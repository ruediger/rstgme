use macroquad::prelude::*;

use crate::input::MoveDirection;
use crate::tile_map::{TILE_SIZE, TileMap};
use crate::weapon::Weapon;

const MOVE_SPEED: f32 = 5.0;

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

    pub fn update_visual(&mut self, dt: f32) {
        let target_x = self.x as f32;
        let target_y = self.y as f32;

        self.visual_x += (target_x - self.visual_x) * MOVE_SPEED * dt * 10.0;
        self.visual_y += (target_y - self.visual_y) * MOVE_SPEED * dt * 10.0;

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

pub struct Player {
    pub pos: Position,
    pub weapon: Weapon,
    color: Color,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: Position::new(x, y),
            weapon: Weapon::pistol(),
            color: Color::from_rgba(80, 180, 80, 255),
        }
    }

    pub fn update(&mut self, dt: f32, input: MoveDirection, map: &TileMap) {
        // Only allow new movement when at target position
        if self.pos.is_at_target() && input.is_moving() {
            let new_x = self.pos.x + input.dx;
            let new_y = self.pos.y + input.dy;

            if map.is_walkable(new_x, new_y) {
                self.pos.x = new_x;
                self.pos.y = new_y;
            }
        }

        self.pos.update_visual(dt);
        self.weapon.update(dt);
    }

    pub fn draw(&self) {
        let padding = 2.0;
        draw_rectangle(
            self.pos.visual_x * TILE_SIZE + padding,
            self.pos.visual_y * TILE_SIZE + padding,
            TILE_SIZE - padding * 2.0,
            TILE_SIZE - padding * 2.0,
            self.color,
        );
    }
}

pub struct Bot {
    pub pos: Position,
    spawn_pos: Position,
    color: Color,
    move_timer: f32,
    move_interval: f32,
    pub alive: bool,
    respawn_timer: f32,
}

impl Bot {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: Position::new(x, y),
            spawn_pos: Position::new(x, y),
            color: Color::from_rgba(180, 80, 80, 255),
            move_timer: 0.0,
            move_interval: 0.5 + rand::gen_range(0.0, 0.5),
            alive: true,
            respawn_timer: 0.0,
        }
    }

    pub fn kill(&mut self) {
        self.alive = false;
        self.respawn_timer = rand::gen_range(5.0, 15.0);
    }

    pub fn update(&mut self, dt: f32, map: &TileMap) {
        if !self.alive {
            self.respawn_timer -= dt;
            if self.respawn_timer <= 0.0 {
                self.alive = true;
                self.pos = self.spawn_pos;
            }
            return;
        }

        self.move_timer += dt;

        if self.pos.is_at_target() && self.move_timer >= self.move_interval {
            self.move_timer = 0.0;

            // Random direction
            let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            let (dx, dy) = directions[rand::gen_range(0, 4)];

            let new_x = self.pos.x + dx;
            let new_y = self.pos.y + dy;

            if map.is_walkable(new_x, new_y) {
                self.pos.x = new_x;
                self.pos.y = new_y;
            }
        }

        self.pos.update_visual(dt);
    }

    pub fn draw(&self) {
        if !self.alive {
            return;
        }

        let padding = 4.0;
        draw_rectangle(
            self.pos.visual_x * TILE_SIZE + padding,
            self.pos.visual_y * TILE_SIZE + padding,
            TILE_SIZE - padding * 2.0,
            TILE_SIZE - padding * 2.0,
            self.color,
        );
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
    }

    #[test]
    fn test_bot_creation() {
        let bot = Bot::new(7, 8);
        assert_eq!(bot.pos.x, 7);
        assert_eq!(bot.pos.y, 8);
    }
}
