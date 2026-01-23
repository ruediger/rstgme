use macroquad::prelude::*;
use std::collections::VecDeque;

use crate::input::MoveDirection;
use crate::sprites::{SpriteSheet, direction, direction_to_vector, movement_to_direction};
use crate::tile_map::{EntityType, TILE_SIZE, TileMap};
use crate::weapon::Weapon;

const MOVE_SPEED: f32 = 1.5;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub visual_x: f32,
    pub visual_y: f32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            visual_x: x as f32,
            visual_y: y as f32,
        }
    }

    pub fn update_visual(&mut self, dt: f32, speed_mult: f32) {
        let target_x = self.x as f32;
        let target_y = self.y as f32;

        let speed = MOVE_SPEED * speed_mult;
        self.visual_x += (target_x - self.visual_x) * speed * dt * 10.0;
        self.visual_y += (target_y - self.visual_y) * speed * dt * 10.0;

        // Snap if very close
        if (self.visual_x - target_x).abs() < 0.01 {
            self.visual_x = target_x;
        }
        if (self.visual_y - target_y).abs() < 0.01 {
            self.visual_y = target_y;
        }
    }

    pub fn is_at_target(&self) -> bool {
        (self.visual_x - self.x as f32).abs() < 0.1 && (self.visual_y - self.y as f32).abs() < 0.1
    }

    pub fn center_pixel(&self) -> (f32, f32) {
        (
            self.visual_x * TILE_SIZE + TILE_SIZE / 2.0,
            self.visual_y * TILE_SIZE + TILE_SIZE / 2.0,
        )
    }
}

const PLAYER_MAX_HEALTH: i32 = 100;

pub struct Player {
    pub pos: Position,
    pub weapons: Vec<Weapon>,
    pub current_weapon: usize,
    pub health: i32,
    pub max_health: i32,
    pub speed_boost_timer: f32,
    pub invulnerability_timer: f32,
    facing: u32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: Position::new(x, y),
            weapons: vec![Weapon::knife()], // Start with only knife
            current_weapon: 0,
            health: PLAYER_MAX_HEALTH,
            max_health: PLAYER_MAX_HEALTH,
            speed_boost_timer: 0.0,
            invulnerability_timer: 0.0,
            facing: direction::DOWN,
        }
    }

    pub fn take_damage(&mut self, amount: i32) {
        // Invulnerability prevents all damage
        if self.invulnerability_timer > 0.0 {
            return;
        }
        self.health = (self.health - amount).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    #[allow(dead_code)]
    pub fn is_invulnerable(&self) -> bool {
        self.invulnerability_timer > 0.0
    }

    #[allow(dead_code)]
    pub fn has_speed_boost(&self) -> bool {
        self.speed_boost_timer > 0.0
    }

    pub fn respawn(&mut self, x: i32, y: i32) {
        self.pos = Position::new(x, y);
        self.health = self.max_health;
        self.speed_boost_timer = 0.0;
        self.invulnerability_timer = 0.0;
    }

    pub fn has_weapon(&self, name: &str) -> bool {
        self.weapons.iter().any(|w| w.name == name)
    }

    pub fn add_weapon(&mut self, weapon: Weapon) {
        if !self.has_weapon(weapon.name) {
            self.weapons.push(weapon);
        }
    }

    pub fn weapon(&self) -> &Weapon {
        &self.weapons[self.current_weapon]
    }

    pub fn weapon_mut(&mut self) -> &mut Weapon {
        &mut self.weapons[self.current_weapon]
    }

    pub fn switch_weapon(&mut self, index: usize) {
        if index < self.weapons.len() {
            self.current_weapon = index;
        }
    }

    pub fn update(&mut self, dt: f32, input: MoveDirection, map: &TileMap) {
        // Update buff timers
        if self.speed_boost_timer > 0.0 {
            self.speed_boost_timer -= dt;
        }
        if self.invulnerability_timer > 0.0 {
            self.invulnerability_timer -= dt;
        }

        // Only allow new movement when at target position
        if self.pos.is_at_target() && input.is_moving() {
            let new_x = self.pos.x + input.dx;
            let new_y = self.pos.y + input.dy;

            // Update facing direction
            self.facing = movement_to_direction(input.dx, input.dy);

            if map.is_walkable_by(new_x, new_y, EntityType::Player) {
                self.pos.x = new_x;
                self.pos.y = new_y;
            }
        }

        // Apply speed multiplier (tile speed * boost)
        let mut speed_mult = map.get_speed_at(self.pos.x, self.pos.y);
        if self.speed_boost_timer > 0.0 {
            speed_mult *= 2.0;
        }
        self.pos.update_visual(dt, speed_mult);

        for weapon in &mut self.weapons {
            weapon.update(dt);
        }
    }

    pub fn draw(&self, camera_x: f32, camera_y: f32, sprites: &SpriteSheet) {
        let screen_x = self.pos.visual_x * TILE_SIZE - camera_x;
        let screen_y = self.pos.visual_y * TILE_SIZE - camera_y;
        sprites.draw_player(screen_x, screen_y, self.facing);
    }
}

pub struct Bot {
    pub pos: Position,
    spawn_pos: Position,
    pub facing: u32,
    move_timer: f32,
    move_interval: f32,
    pub alive: bool,
    respawn_timer: f32,
    pub hostile: bool,
    pub shoot_cooldown: f32,
    // Pathfinding
    path: VecDeque<(i32, i32)>,
    path_target: Option<(i32, i32)>,
    path_recalc_timer: f32,
}

impl Bot {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: Position::new(x, y),
            spawn_pos: Position::new(x, y),
            facing: direction::DOWN,
            move_timer: 0.0,
            move_interval: 0.5 + rand::gen_range(0.0, 0.5),
            alive: true,
            respawn_timer: 0.0,
            hostile: false,
            shoot_cooldown: 0.0,
            path: VecDeque::new(),
            path_target: None,
            path_recalc_timer: 0.0,
        }
    }

    pub fn new_hostile(x: i32, y: i32) -> Self {
        Self {
            pos: Position::new(x, y),
            spawn_pos: Position::new(x, y),
            facing: direction::DOWN,
            move_timer: 0.0,
            move_interval: 0.2 + rand::gen_range(0.0, 0.15), // Fast movement
            alive: true,
            respawn_timer: 0.0,
            hostile: true,
            shoot_cooldown: rand::gen_range(0.0, 1.0), // Stagger initial shots
            path: VecDeque::new(),
            path_target: None,
            path_recalc_timer: 0.0,
        }
    }

    pub fn kill(&mut self) {
        self.alive = false;
        self.respawn_timer = rand::gen_range(5.0, 15.0);
    }

    /// Turn this bot hostile (infected by another hostile bot)
    pub fn infect(&mut self) {
        self.hostile = true;
        self.move_interval = 0.3 + rand::gen_range(0.0, 0.2);
    }

    pub fn update(&mut self, dt: f32, map: &TileMap, target_pos: Option<(i32, i32)>) {
        if !self.alive {
            self.respawn_timer -= dt;
            if self.respawn_timer <= 0.0 {
                self.alive = true;
                self.pos = self.spawn_pos;
                // 50% chance to respawn as hostile
                if rand::gen_range(0.0, 1.0) < 0.5 {
                    self.hostile = true;
                    self.move_interval = 0.2 + rand::gen_range(0.0, 0.15);
                } else {
                    self.hostile = false;
                    self.move_interval = 0.5 + rand::gen_range(0.0, 0.5);
                }
                self.shoot_cooldown = rand::gen_range(0.0, 1.0);
                // Reset pathfinding
                self.path.clear();
                self.path_target = None;
                self.path_recalc_timer = 0.0;
            }
            return;
        }

        // Update shoot cooldown
        if self.shoot_cooldown > 0.0 {
            self.shoot_cooldown -= dt;
        }

        // Update path recalc timer
        self.path_recalc_timer -= dt;

        self.move_timer += dt;

        if self.pos.is_at_target() && self.move_timer >= self.move_interval {
            self.move_timer = 0.0;

            // Hostile bots stop moving when close to target (stand and shoot)
            let should_stand = if self.hostile {
                if let Some((tx, ty)) = target_pos {
                    let dist = (tx - self.pos.x).abs() + (ty - self.pos.y).abs();
                    dist <= 3 // Stand and shoot when within 3 tiles
                } else {
                    false
                }
            } else {
                false
            };

            if should_stand {
                // Just face the target, don't move
                if let Some((tx, ty)) = target_pos {
                    let dx = (tx - self.pos.x).signum();
                    let dy = (ty - self.pos.y).signum();
                    if dx != 0 || dy != 0 {
                        self.facing = movement_to_direction(dx, dy);
                    }
                }
            } else if self.hostile {
                // Use BFS pathfinding for hostile bots
                self.move_with_pathfinding(map, target_pos);
            } else {
                // Random direction for non-hostile bots
                let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                let (dx, dy) = directions[rand::gen_range(0, 4)];
                let new_x = self.pos.x + dx;
                let new_y = self.pos.y + dy;

                if dx != 0 || dy != 0 {
                    self.facing = movement_to_direction(dx, dy);
                }

                if map.is_walkable_by(new_x, new_y, EntityType::Bot) {
                    self.pos.x = new_x;
                    self.pos.y = new_y;
                }
            }
        }

        let speed_mult = map.get_speed_at(self.pos.x, self.pos.y);
        self.pos.update_visual(dt, speed_mult);
    }

    /// Move hostile bot using BFS pathfinding
    fn move_with_pathfinding(&mut self, map: &TileMap, target_pos: Option<(i32, i32)>) {
        let Some((tx, ty)) = target_pos else {
            // No target - random wander
            let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            let (dx, dy) = directions[rand::gen_range(0, 4)];
            let new_x = self.pos.x + dx;
            let new_y = self.pos.y + dy;
            if map.is_walkable_by(new_x, new_y, EntityType::Bot) {
                self.pos.x = new_x;
                self.pos.y = new_y;
                self.facing = movement_to_direction(dx, dy);
            }
            return;
        };

        // Recalculate path if target changed, path is empty, or timer expired
        let need_recalc = self.path_target != Some((tx, ty))
            || self.path.is_empty()
            || self.path_recalc_timer <= 0.0;

        if need_recalc {
            self.path = Self::find_path(self.pos.x, self.pos.y, tx, ty, map);
            self.path_target = Some((tx, ty));
            self.path_recalc_timer = 0.5 + rand::gen_range(0.0, 0.3); // Recalc every 0.5-0.8s
        }

        // Follow the path
        if let Some((next_x, next_y)) = self.path.front().copied() {
            let dx = next_x - self.pos.x;
            let dy = next_y - self.pos.y;

            if map.is_walkable_by(next_x, next_y, EntityType::Bot) {
                self.pos.x = next_x;
                self.pos.y = next_y;
                self.path.pop_front();

                if dx != 0 || dy != 0 {
                    self.facing = movement_to_direction(dx, dy);
                }
            } else {
                // Path blocked, recalculate next frame
                self.path.clear();
            }
        }
    }

    /// BFS pathfinding to find shortest path from (sx, sy) to (tx, ty)
    fn find_path(sx: i32, sy: i32, tx: i32, ty: i32, map: &TileMap) -> VecDeque<(i32, i32)> {
        const MAX_SEARCH: usize = 2000; // Limit search to prevent lag

        if sx == tx && sy == ty {
            return VecDeque::new();
        }

        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let mut came_from: std::collections::HashMap<(i32, i32), (i32, i32)> =
            std::collections::HashMap::new();

        queue.push_back((sx, sy));
        came_from.insert((sx, sy), (sx, sy)); // Mark start as visited

        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut found = false;
        let mut iterations = 0;

        while let Some((cx, cy)) = queue.pop_front() {
            iterations += 1;
            if iterations > MAX_SEARCH {
                break;
            }

            if cx == tx && cy == ty {
                found = true;
                break;
            }

            for (dx, dy) in directions {
                let nx = cx + dx;
                let ny = cy + dy;

                if came_from.contains_key(&(nx, ny)) {
                    continue;
                }

                if map.is_walkable_by(nx, ny, EntityType::Bot) {
                    came_from.insert((nx, ny), (cx, cy));
                    queue.push_back((nx, ny));
                }
            }
        }

        if !found {
            return VecDeque::new();
        }

        // Reconstruct path from target back to start
        let mut path = VecDeque::new();
        let mut current = (tx, ty);

        while current != (sx, sy) {
            path.push_front(current);
            current = came_from[&current];
        }

        path
    }

    /// Check if hostile bot can shoot and return target direction if so
    pub fn try_shoot(&mut self, player_x: i32, player_y: i32) -> Option<(f32, f32)> {
        if !self.hostile || !self.alive || self.shoot_cooldown > 0.0 {
            return None;
        }

        let (bx, by) = (self.pos.x, self.pos.y);
        let dx = player_x - bx;
        let dy = player_y - by;
        let dist_sq = dx * dx + dy * dy;

        // Only shoot if within range (8 tiles)
        if dist_sq <= 64 {
            self.shoot_cooldown = 1.0 + rand::gen_range(0.0, 0.5); // Faster shooting: 1.0-1.5s

            // Return normalized direction
            let dist = (dist_sq as f32).sqrt();
            if dist > 0.0 {
                return Some((dx as f32 / dist, dy as f32 / dist));
            } else {
                // On top of player - shoot in facing direction
                let (fdx, fdy) = direction_to_vector(self.facing);
                return Some((fdx, fdy));
            }
        }

        None
    }

    pub fn draw(&self, camera_x: f32, camera_y: f32, sprites: &SpriteSheet) {
        if !self.alive {
            return;
        }

        let screen_x = self.pos.visual_x * TILE_SIZE - camera_x;
        let screen_y = self.pos.visual_y * TILE_SIZE - camera_y;

        if self.hostile {
            // Hostile bots get a red tint
            let tint = Color::from_rgba(255, 100, 100, 255);
            sprites.draw_bot_tinted(screen_x, screen_y, self.facing, tint);
        } else {
            sprites.draw_bot(screen_x, screen_y, self.facing);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        let pos = Position::new(5, 10);
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 10);
        assert_eq!(pos.visual_x, 5.0);
        assert_eq!(pos.visual_y, 10.0);
    }

    #[test]
    fn test_position_is_at_target() {
        let pos = Position::new(5, 5);
        assert!(pos.is_at_target());

        let mut pos2 = Position::new(5, 5);
        pos2.visual_x = 4.0;
        assert!(!pos2.is_at_target());
    }

    #[test]
    fn test_player_creation() {
        let player = Player::new(3, 4);
        assert_eq!(player.pos.x, 3);
        assert_eq!(player.pos.y, 4);
        assert_eq!(player.health, 100);
        assert!(player.is_alive());
        // Player starts with only knife
        assert_eq!(player.weapons.len(), 1);
        assert_eq!(player.weapons[0].name, "Knife");
    }

    #[test]
    fn test_player_damage() {
        let mut player = Player::new(0, 0);
        player.take_damage(30);
        assert_eq!(player.health, 70);
        assert!(player.is_alive());

        player.take_damage(100);
        assert_eq!(player.health, 0);
        assert!(!player.is_alive());
    }

    #[test]
    fn test_player_invulnerability() {
        let mut player = Player::new(0, 0);
        player.invulnerability_timer = 3.0;

        player.take_damage(50);
        assert_eq!(player.health, 100); // No damage taken
        assert!(player.is_invulnerable());
    }

    #[test]
    fn test_player_add_weapon() {
        let mut player = Player::new(0, 0);
        assert!(!player.has_weapon("Pistol"));

        player.add_weapon(Weapon::pistol());
        assert!(player.has_weapon("Pistol"));
        assert_eq!(player.weapons.len(), 2);

        // Adding same weapon again should not duplicate
        player.add_weapon(Weapon::pistol());
        assert_eq!(player.weapons.len(), 2);
    }

    #[test]
    fn test_player_respawn() {
        let mut player = Player::new(0, 0);
        player.take_damage(100);
        assert!(!player.is_alive());

        player.respawn(5, 5);
        assert!(player.is_alive());
        assert_eq!(player.health, 100);
        assert_eq!(player.pos.x, 5);
        assert_eq!(player.pos.y, 5);
    }

    #[test]
    fn test_bot_creation() {
        let bot = Bot::new(7, 8);
        assert_eq!(bot.pos.x, 7);
        assert_eq!(bot.pos.y, 8);
    }
}
