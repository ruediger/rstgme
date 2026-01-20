use macroquad::prelude::*;
use std::collections::HashMap;

use crate::sprites::{SpriteSheet, tiles};

pub const TILE_SIZE: f32 = 32.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum EntityType {
    Player,
    Bot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Floor,
    Wall,
    Sand,
    Water,
    Lava,
    Pit,
    DoorPlayer,
    DoorBot,
    DoorBoth,
    Crate,
    WallDestructible,
}

impl TileType {
    pub fn is_walkable_by(self, entity_type: EntityType) -> bool {
        match self {
            TileType::Floor | TileType::Sand | TileType::Water | TileType::Lava => true,
            TileType::DoorPlayer => entity_type == EntityType::Player,
            TileType::DoorBot => entity_type == EntityType::Bot,
            TileType::DoorBoth => true,
            TileType::Wall | TileType::Pit | TileType::Crate | TileType::WallDestructible => false,
        }
    }

    pub fn speed_multiplier(self) -> f32 {
        match self {
            TileType::Sand => 0.5,
            TileType::Water => 0.3,
            TileType::Lava => 0.4, // Slow but not as slow as water
            _ => 1.0,
        }
    }

    pub fn blocks_projectile(self) -> bool {
        match self {
            TileType::Wall
            | TileType::DoorPlayer
            | TileType::DoorBot
            | TileType::DoorBoth
            | TileType::Crate
            | TileType::WallDestructible => true,
            // Pit, Lava, Floor, Sand, Water let projectiles pass
            _ => false,
        }
    }

    pub fn is_destructible(self) -> bool {
        matches!(self, TileType::Crate | TileType::WallDestructible)
    }

    pub fn max_health(self) -> u8 {
        match self {
            TileType::Crate => 1,
            TileType::WallDestructible => 3,
            _ => 0,
        }
    }

    fn sprite_index(self) -> u32 {
        match self {
            TileType::Floor => tiles::FLOOR,
            TileType::Wall => tiles::WALL,
            TileType::Sand => tiles::SAND,
            TileType::Water => tiles::WATER,
            TileType::Lava => tiles::LAVA,
            TileType::Pit => tiles::PIT,
            TileType::DoorPlayer => tiles::DOOR_PLAYER,
            TileType::DoorBot => tiles::DOOR_BOT,
            TileType::DoorBoth => tiles::DOOR_BOTH,
            TileType::Crate => tiles::CRATE,
            TileType::WallDestructible => tiles::WALL_DESTRUCTIBLE,
        }
    }
}

pub struct TileMap {
    tiles: Vec<Vec<TileType>>,
    tile_health: HashMap<(usize, usize), u8>,
    pub width: usize,
    pub height: usize,
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![TileType::Floor; width]; height];
        Self {
            tiles,
            tile_health: HashMap::new(),
            width,
            height,
        }
    }

    pub fn create_random(width: usize, height: usize) -> Self {
        let mut map = Self::new(width, height);

        // Add walls around the border
        for x in 0..width {
            map.set_tile(x, 0, TileType::Wall);
            map.set_tile(x, height - 1, TileType::Wall);
        }
        for y in 0..height {
            map.set_tile(0, y, TileType::Wall);
            map.set_tile(width - 1, y, TileType::Wall);
        }

        // Add random wall clusters
        let num_clusters = (width * height) / 50;
        for _ in 0..num_clusters {
            let cx = rand::gen_range(3, width - 3);
            let cy = rand::gen_range(3, height - 3);
            let cluster_size = rand::gen_range(2, 6);

            for _ in 0..cluster_size {
                let ox = rand::gen_range(0, 3) as i32 - 1;
                let oy = rand::gen_range(0, 3) as i32 - 1;
                let wx = (cx as i32 + ox) as usize;
                let wy = (cy as i32 + oy) as usize;
                if wx > 1 && wx < width - 2 && wy > 1 && wy < height - 2 {
                    // Mix of wall types
                    let tile = if rand::gen_range(0, 4) == 0 {
                        TileType::WallDestructible
                    } else {
                        TileType::Wall
                    };
                    map.set_tile(wx, wy, tile);
                }
            }
        }

        // Add horizontal wall segments with doors
        let num_h_walls = height / 8;
        for _ in 0..num_h_walls {
            let y = rand::gen_range(3, height - 3);
            let x_start = rand::gen_range(2, width / 2);
            let length = rand::gen_range(4, width / 3);
            for x in x_start..(x_start + length).min(width - 2) {
                map.set_tile(x, y, TileType::Wall);
            }
            // Add a door in the gap
            let gap = rand::gen_range(x_start, (x_start + length).min(width - 2));
            let door_type = match rand::gen_range(0, 3) {
                0 => TileType::DoorPlayer,
                1 => TileType::DoorBot,
                _ => TileType::DoorBoth,
            };
            map.set_tile(gap, y, door_type);
        }

        // Add vertical wall segments with doors
        let num_v_walls = width / 8;
        for _ in 0..num_v_walls {
            let x = rand::gen_range(3, width - 3);
            let y_start = rand::gen_range(2, height / 2);
            let length = rand::gen_range(4, height / 3);
            for y in y_start..(y_start + length).min(height - 2) {
                map.set_tile(x, y, TileType::Wall);
            }
            // Add a door in the gap
            let gap = rand::gen_range(y_start, (y_start + length).min(height - 2));
            let door_type = match rand::gen_range(0, 3) {
                0 => TileType::DoorPlayer,
                1 => TileType::DoorBot,
                _ => TileType::DoorBoth,
            };
            map.set_tile(x, gap, door_type);
        }

        // Add sand patches
        let num_sand = (width * height) / 80;
        for _ in 0..num_sand {
            let cx = rand::gen_range(3, width - 3);
            let cy = rand::gen_range(3, height - 3);
            let size = rand::gen_range(2, 5);
            for dx in 0..size {
                for dy in 0..size {
                    let x = cx + dx;
                    let y = cy + dy;
                    if x < width - 1
                        && y < height - 1
                        && map.get_tile(x, y) == Some(TileType::Floor)
                    {
                        map.set_tile(x, y, TileType::Sand);
                    }
                }
            }
        }

        // Add water patches
        let num_water = (width * height) / 120;
        for _ in 0..num_water {
            let cx = rand::gen_range(3, width - 3);
            let cy = rand::gen_range(3, height - 3);
            let size = rand::gen_range(2, 4);
            for dx in 0..size {
                for dy in 0..size {
                    let x = cx + dx;
                    let y = cy + dy;
                    if x < width - 1
                        && y < height - 1
                        && (map.get_tile(x, y) == Some(TileType::Floor)
                            || map.get_tile(x, y) == Some(TileType::Sand))
                    {
                        map.set_tile(x, y, TileType::Water);
                    }
                }
            }
        }

        // Add lava pools (small and dangerous)
        let num_lava = (width * height) / 200;
        for _ in 0..num_lava {
            let cx = rand::gen_range(4, width - 4);
            let cy = rand::gen_range(4, height - 4);
            let size = rand::gen_range(1, 3);
            for dx in 0..size {
                for dy in 0..size {
                    let x = cx + dx;
                    let y = cy + dy;
                    if x < width - 1
                        && y < height - 1
                        && map.get_tile(x, y) == Some(TileType::Floor)
                    {
                        map.set_tile(x, y, TileType::Lava);
                    }
                }
            }
        }

        // Add pits
        let num_pits = (width * height) / 150;
        for _ in 0..num_pits {
            let cx = rand::gen_range(3, width - 3);
            let cy = rand::gen_range(3, height - 3);
            let size = rand::gen_range(1, 3);
            for dx in 0..size {
                for dy in 0..size {
                    let x = cx + dx;
                    let y = cy + dy;
                    if x < width - 1
                        && y < height - 1
                        && map.get_tile(x, y) == Some(TileType::Floor)
                    {
                        map.set_tile(x, y, TileType::Pit);
                    }
                }
            }
        }

        // Add crates scattered around
        let num_crates = (width * height) / 60;
        for _ in 0..num_crates {
            let x = rand::gen_range(2, width - 2);
            let y = rand::gen_range(2, height - 2);
            if map.get_tile(x, y) == Some(TileType::Floor) {
                map.set_tile(x, y, TileType::Crate);
            }
        }

        map
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<TileType> {
        self.tiles.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileType) {
        if y < self.height && x < self.width {
            self.tiles[y][x] = tile;
            // Initialize health for destructible tiles
            if tile.is_destructible() {
                self.tile_health.insert((x, y), tile.max_health());
            } else {
                self.tile_health.remove(&(x, y));
            }
        }
    }

    pub fn is_walkable_by(&self, x: i32, y: i32, entity_type: EntityType) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        self.get_tile(x as usize, y as usize)
            .map(|t| t.is_walkable_by(entity_type))
            .unwrap_or(false)
    }

    pub fn get_speed_at(&self, x: i32, y: i32) -> f32 {
        if x < 0 || y < 0 {
            return 1.0;
        }
        self.get_tile(x as usize, y as usize)
            .map(|t| t.speed_multiplier())
            .unwrap_or(1.0)
    }

    pub fn blocks_projectile_at(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return true;
        }
        self.get_tile(x as usize, y as usize)
            .map(|t| t.blocks_projectile())
            .unwrap_or(true)
    }

    pub fn damage_tile(&mut self, x: usize, y: usize) -> bool {
        if let Some(tile) = self.get_tile(x, y)
            && tile.is_destructible()
            && let Some(health) = self.tile_health.get_mut(&(x, y))
        {
            *health = health.saturating_sub(1);
            if *health == 0 {
                self.set_tile(x, y, TileType::Floor);
                return true;
            }
        }
        false
    }

    pub fn is_destructible_at(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        self.get_tile(x as usize, y as usize)
            .map(|t| t.is_destructible())
            .unwrap_or(false)
    }

    pub fn is_lava_at(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        self.get_tile(x as usize, y as usize) == Some(TileType::Lava)
    }

    pub fn draw(&self, camera_x: f32, camera_y: f32, sprites: &SpriteSheet) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                let screen_x = x as f32 * TILE_SIZE - camera_x;
                let screen_y = y as f32 * TILE_SIZE - camera_y;
                let sprite_idx = tile.sprite_index();

                // Show damage on destructible tiles
                if tile.is_destructible()
                    && let Some(&health) = self.tile_health.get(&(x, y))
                {
                    let max = tile.max_health();
                    if health < max {
                        let damage_factor = 1.0 - (health as f32 / max as f32);
                        sprites.draw_tile_damaged(sprite_idx, screen_x, screen_y, damage_factor);
                    } else {
                        sprites.draw_tile(sprite_idx, screen_x, screen_y);
                    }
                } else {
                    sprites.draw_tile(sprite_idx, screen_x, screen_y);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_walkability() {
        assert!(TileType::Floor.is_walkable_by(EntityType::Player));
        assert!(TileType::Floor.is_walkable_by(EntityType::Bot));
        assert!(!TileType::Wall.is_walkable_by(EntityType::Player));
        assert!(!TileType::Wall.is_walkable_by(EntityType::Bot));
        // Lava is walkable (but damages player)
        assert!(TileType::Lava.is_walkable_by(EntityType::Player));
        assert!(TileType::Lava.is_walkable_by(EntityType::Bot));
    }

    #[test]
    fn test_door_access() {
        assert!(TileType::DoorPlayer.is_walkable_by(EntityType::Player));
        assert!(!TileType::DoorPlayer.is_walkable_by(EntityType::Bot));
        assert!(!TileType::DoorBot.is_walkable_by(EntityType::Player));
        assert!(TileType::DoorBot.is_walkable_by(EntityType::Bot));
        assert!(TileType::DoorBoth.is_walkable_by(EntityType::Player));
        assert!(TileType::DoorBoth.is_walkable_by(EntityType::Bot));
    }

    #[test]
    fn test_speed_multipliers() {
        assert_eq!(TileType::Floor.speed_multiplier(), 1.0);
        assert_eq!(TileType::Sand.speed_multiplier(), 0.5);
        assert_eq!(TileType::Water.speed_multiplier(), 0.3);
        assert_eq!(TileType::Lava.speed_multiplier(), 0.4);
    }

    #[test]
    fn test_projectile_blocking() {
        assert!(TileType::Wall.blocks_projectile());
        assert!(TileType::Crate.blocks_projectile());
        assert!(!TileType::Floor.blocks_projectile());
        assert!(!TileType::Pit.blocks_projectile());
        assert!(!TileType::Lava.blocks_projectile());
    }

    #[test]
    fn test_destructible() {
        let mut map = TileMap::new(10, 10);
        map.set_tile(5, 5, TileType::Crate);
        assert!(map.is_destructible_at(5, 5));

        // Crate should be destroyed in 1 hit
        let destroyed = map.damage_tile(5, 5);
        assert!(destroyed);
        assert_eq!(map.get_tile(5, 5), Some(TileType::Floor));
    }

    #[test]
    fn test_destructible_wall() {
        let mut map = TileMap::new(10, 10);
        map.set_tile(5, 5, TileType::WallDestructible);

        // Should take 3 hits
        assert!(!map.damage_tile(5, 5));
        assert!(!map.damage_tile(5, 5));
        assert!(map.damage_tile(5, 5));
        assert_eq!(map.get_tile(5, 5), Some(TileType::Floor));
    }

    #[test]
    fn test_map_boundaries() {
        let map = TileMap::create_random(20, 15);
        // Corners should be walls (border)
        assert!(!map.is_walkable_by(0, 0, EntityType::Player));
        assert!(!map.is_walkable_by(19, 0, EntityType::Player));
        assert!(!map.is_walkable_by(0, 14, EntityType::Player));
        assert!(!map.is_walkable_by(19, 14, EntityType::Player));
    }

    #[test]
    fn test_out_of_bounds() {
        let map = TileMap::new(10, 10);
        assert!(!map.is_walkable_by(-1, 0, EntityType::Player));
        assert!(!map.is_walkable_by(0, -1, EntityType::Player));
        assert!(!map.is_walkable_by(100, 0, EntityType::Player));
        assert!(!map.is_walkable_by(0, 100, EntityType::Player));
    }
}
