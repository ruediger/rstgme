use macroquad::prelude::*;

use crate::tile_map::{TILE_SIZE, TileMap};

const PROJECTILE_SIZE: f32 = 6.0;

pub struct Projectile {
    pub x: f32,
    pub y: f32,
    dx: f32,
    dy: f32,
    speed: f32,
    pub alive: bool,
}

impl Projectile {
    pub fn new(x: f32, y: f32, target_x: f32, target_y: f32, speed: f32) -> Self {
        let dx = target_x - x;
        let dy = target_y - y;
        let len = (dx * dx + dy * dy).sqrt();

        let (dx, dy) = if len > 0.0 {
            (dx / len, dy / len)
        } else {
            (1.0, 0.0)
        };

        Self {
            x,
            y,
            dx,
            dy,
            speed,
            alive: true,
        }
    }

    pub fn update(&mut self, dt: f32, map: &TileMap) {
        if !self.alive {
            return;
        }

        self.x += self.dx * self.speed * dt;
        self.y += self.dy * self.speed * dt;

        // Check wall collision
        let tile_x = (self.x / TILE_SIZE) as i32;
        let tile_y = (self.y / TILE_SIZE) as i32;

        if !map.is_walkable(tile_x, tile_y) {
            self.alive = false;
        }
    }

    pub fn draw(&self) {
        if !self.alive {
            return;
        }

        draw_circle(self.x, self.y, PROJECTILE_SIZE / 2.0, YELLOW);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projectile_direction() {
        let p = Projectile::new(0.0, 0.0, 100.0, 0.0, 100.0);
        assert!((p.dx - 1.0).abs() < 0.001);
        assert!(p.dy.abs() < 0.001);
    }

    #[test]
    fn test_projectile_diagonal() {
        let p = Projectile::new(0.0, 0.0, 100.0, 100.0, 100.0);
        let expected = 1.0 / 2.0_f32.sqrt();
        assert!((p.dx - expected).abs() < 0.001);
        assert!((p.dy - expected).abs() < 0.001);
    }

    #[test]
    fn test_projectile_zero_distance() {
        let p = Projectile::new(50.0, 50.0, 50.0, 50.0, 100.0);
        assert!(p.alive);
        assert_eq!(p.dx, 1.0);
    }
}
