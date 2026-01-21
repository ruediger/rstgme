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

    #[allow(dead_code)] // Kept for tests and potential alternative game modes
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

    /// Creates a labyrinth-style map using recursive backtracking algorithm.
    /// This generates proper corridors and rooms instead of random tile placement.
    pub fn create_labyrinth(width: usize, height: usize) -> Self {
        let mut map = Self::new(width, height);

        // Fill with walls
        for y in 0..height {
            for x in 0..width {
                map.set_tile(x, y, TileType::Wall);
            }
        }

        // Generate maze using iterative backtracking (avoid stack overflow)
        map.carve_maze(1, 1);

        // Add rooms (creates open areas for combat)
        let room_count = (width * height) / 400 + 2; // Scale with map size
        map.add_rooms(room_count);

        // Add loops to create alternative paths
        let loop_count = width * height / 50;
        map.add_loops(loop_count);

        // Add terrain features
        map.add_terrain();

        // Add doors at corridor junctions
        map.add_doors();

        // Add crates scattered around
        map.add_labyrinth_crates();

        map
    }

    /// Carve a maze using iterative depth-first backtracking.
    /// Uses an explicit stack to avoid stack overflow on large maps.
    fn carve_maze(&mut self, start_x: usize, start_y: usize) {
        let mut stack = vec![(start_x, start_y)];
        self.set_tile(start_x, start_y, TileType::Floor);

        while let Some(&(x, y)) = stack.last() {
            // Get unvisited neighbors 2 cells away
            let mut neighbors = Vec::new();
            let directions: [(i32, i32); 4] = [(0, -2), (0, 2), (-2, 0), (2, 0)];

            for (dx, dy) in directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx > 0
                    && (nx as usize) < self.width - 1
                    && ny > 0
                    && (ny as usize) < self.height - 1
                    && self.get_tile(nx as usize, ny as usize) == Some(TileType::Wall)
                {
                    neighbors.push((nx as usize, ny as usize, dx, dy));
                }
            }

            if neighbors.is_empty() {
                // Backtrack
                stack.pop();
            } else {
                // Choose random neighbor
                let idx = rand::gen_range(0, neighbors.len());
                let (nx, ny, dx, dy) = neighbors[idx];

                // Carve the wall between current and next
                let wx = (x as i32 + dx / 2) as usize;
                let wy = (y as i32 + dy / 2) as usize;
                self.set_tile(wx, wy, TileType::Floor);
                self.set_tile(nx, ny, TileType::Floor);

                stack.push((nx, ny));
            }
        }
    }

    /// Add rectangular rooms to create open areas for combat.
    fn add_rooms(&mut self, count: usize) {
        for _ in 0..count {
            let room_w = rand::gen_range(3, 7);
            let room_h = rand::gen_range(3, 7);

            // Ensure room fits within map bounds
            if room_w + 4 >= self.width || room_h + 4 >= self.height {
                continue;
            }

            let rx = rand::gen_range(2, self.width - room_w - 2);
            let ry = rand::gen_range(2, self.height - room_h - 2);

            for y in ry..ry + room_h {
                for x in rx..rx + room_w {
                    self.set_tile(x, y, TileType::Floor);
                }
            }
        }
    }

    /// Add loops by removing some walls to create alternative paths.
    fn add_loops(&mut self, count: usize) {
        let mut added = 0;
        let max_attempts = count * 10;
        let mut attempts = 0;

        while added < count && attempts < max_attempts {
            attempts += 1;
            let x = rand::gen_range(2, self.width - 2);
            let y = rand::gen_range(2, self.height - 2);

            if self.get_tile(x, y) != Some(TileType::Wall) {
                continue;
            }

            // Check if removing would connect two floor tiles
            let h_connect = self.get_tile(x.wrapping_sub(1), y) == Some(TileType::Floor)
                && self.get_tile(x + 1, y) == Some(TileType::Floor);
            let v_connect = self.get_tile(x, y.wrapping_sub(1)) == Some(TileType::Floor)
                && self.get_tile(x, y + 1) == Some(TileType::Floor);

            if h_connect || v_connect {
                self.set_tile(x, y, TileType::Floor);
                added += 1;
            }
        }
    }

    /// Add terrain features (sand, water, lava, pits) to corridors and rooms.
    fn add_terrain(&mut self) {
        // Add sand patches in corridors
        let num_sand = (self.width * self.height) / 100;
        for _ in 0..num_sand {
            let x = rand::gen_range(2, self.width - 2);
            let y = rand::gen_range(2, self.height - 2);
            if self.get_tile(x, y) == Some(TileType::Floor) {
                self.set_tile(x, y, TileType::Sand);
                // Expand sand slightly
                for (dx, dy) in [(0, 1), (1, 0), (0, -1_i32), (-1, 0)] {
                    let nx = (x as i32 + dx) as usize;
                    let ny = (y as i32 + dy) as usize;
                    if rand::gen_range(0, 3) == 0 && self.get_tile(nx, ny) == Some(TileType::Floor)
                    {
                        self.set_tile(nx, ny, TileType::Sand);
                    }
                }
            }
        }

        // Add water pools in rooms (larger areas)
        let num_water = (self.width * self.height) / 200;
        for _ in 0..num_water {
            let x = rand::gen_range(3, self.width - 3);
            let y = rand::gen_range(3, self.height - 3);
            let tile = self.get_tile(x, y);
            if tile == Some(TileType::Floor) || tile == Some(TileType::Sand) {
                self.set_tile(x, y, TileType::Water);
                // Expand water
                for (dx, dy) in [(0, 1), (1, 0), (0, -1_i32), (-1, 0), (1, 1), (-1, -1)] {
                    let nx = (x as i32 + dx) as usize;
                    let ny = (y as i32 + dy) as usize;
                    if rand::gen_range(0, 2) == 0 {
                        let ntile = self.get_tile(nx, ny);
                        if ntile == Some(TileType::Floor) || ntile == Some(TileType::Sand) {
                            self.set_tile(nx, ny, TileType::Water);
                        }
                    }
                }
            }
        }

        // Add lava hazards (small and strategic)
        let num_lava = (self.width * self.height) / 300;
        for _ in 0..num_lava {
            let x = rand::gen_range(4, self.width - 4);
            let y = rand::gen_range(4, self.height - 4);
            if self.get_tile(x, y) == Some(TileType::Floor) {
                self.set_tile(x, y, TileType::Lava);
                // Maybe add one adjacent lava tile
                if rand::gen_range(0, 3) == 0 {
                    let dirs = [(0, 1), (1, 0), (0, -1_i32), (-1, 0)];
                    let (dx, dy) = dirs[rand::gen_range(0, 4)];
                    let nx = (x as i32 + dx) as usize;
                    let ny = (y as i32 + dy) as usize;
                    if self.get_tile(nx, ny) == Some(TileType::Floor) {
                        self.set_tile(nx, ny, TileType::Lava);
                    }
                }
            }
        }

        // Add pits (block movement but not projectiles)
        let num_pits = (self.width * self.height) / 250;
        for _ in 0..num_pits {
            let x = rand::gen_range(3, self.width - 3);
            let y = rand::gen_range(3, self.height - 3);
            if self.get_tile(x, y) == Some(TileType::Floor) {
                self.set_tile(x, y, TileType::Pit);
            }
        }
    }

    /// Add doors at corridor junctions and choke points.
    fn add_doors(&mut self) {
        let num_doors = (self.width * self.height) / 150;
        let mut added = 0;
        let max_attempts = num_doors * 20;
        let mut attempts = 0;

        while added < num_doors && attempts < max_attempts {
            attempts += 1;
            let x = rand::gen_range(2, self.width - 2);
            let y = rand::gen_range(2, self.height - 2);

            if self.get_tile(x, y) != Some(TileType::Floor) {
                continue;
            }

            // Check if this is a corridor (walls on two opposite sides, floor on the other two)
            let north = self.get_tile(x, y.wrapping_sub(1));
            let south = self.get_tile(x, y + 1);
            let east = self.get_tile(x + 1, y);
            let west = self.get_tile(x.wrapping_sub(1), y);

            let is_h_corridor = north == Some(TileType::Wall)
                && south == Some(TileType::Wall)
                && (east == Some(TileType::Floor) || east == Some(TileType::Sand))
                && (west == Some(TileType::Floor) || west == Some(TileType::Sand));

            let is_v_corridor = east == Some(TileType::Wall)
                && west == Some(TileType::Wall)
                && (north == Some(TileType::Floor) || north == Some(TileType::Sand))
                && (south == Some(TileType::Floor) || south == Some(TileType::Sand));

            if is_h_corridor || is_v_corridor {
                let door_type = match rand::gen_range(0, 4) {
                    0 => TileType::DoorPlayer,
                    1 => TileType::DoorBot,
                    _ => TileType::DoorBoth, // More common
                };
                self.set_tile(x, y, door_type);
                added += 1;
            }
        }
    }

    /// Add crates scattered in floor areas of the labyrinth.
    fn add_labyrinth_crates(&mut self) {
        let num_crates = (self.width * self.height) / 80;
        let mut added = 0;
        let max_attempts = num_crates * 5;
        let mut attempts = 0;

        while added < num_crates && attempts < max_attempts {
            attempts += 1;
            let x = rand::gen_range(2, self.width - 2);
            let y = rand::gen_range(2, self.height - 2);

            if self.get_tile(x, y) != Some(TileType::Floor) {
                continue;
            }

            // Prefer placing crates in rooms (areas with more open space)
            let mut floor_neighbors = 0;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1_i32), (0, 1)] {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if let Some(tile) = self.get_tile(nx, ny)
                    && tile.is_walkable_by(EntityType::Player)
                {
                    floor_neighbors += 1;
                }
            }

            // Place crate if it's in an open area (at least 3 walkable neighbors)
            // or randomly in corridors
            if floor_neighbors >= 3 || rand::gen_range(0, 4) == 0 {
                // Mix of crate types
                let tile = if rand::gen_range(0, 5) == 0 {
                    TileType::WallDestructible
                } else {
                    TileType::Crate
                };
                self.set_tile(x, y, tile);
                added += 1;
            }
        }
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
