use macroquad::prelude::*;

use crate::tile_map::{TILE_SIZE, TileMap};

const PROJECTILE_SIZE: f32 = 6.0;

pub struct Projectile {
    pub x: f32,
    pub y: f32,
    start_x: f32,
    start_y: f32,
    dx: f32,
    dy: f32,
    speed: f32,
    max_range: f32,
    pub alive: bool,
}

impl Projectile {
    #[cfg(test)]
    pub fn new(x: f32, y: f32, target_x: f32, target_y: f32, speed: f32, max_range: f32) -> Self {
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
            start_x: x,
            start_y: y,
            dx,
            dy,
            speed,
            max_range,
            alive: true,
        }
    }

    pub fn new_with_direction(
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        speed: f32,
        max_range: f32,
    ) -> Self {
        Self {
            x,
            y,
            start_x: x,
            start_y: y,
            dx,
            dy,
            speed,
            max_range,
            alive: true,
        }
    }

    pub fn update(&mut self, dt: f32, map: &TileMap) {
        if !self.alive {
            return;
        }

        self.x += self.dx * self.speed * dt;
        self.y += self.dy * self.speed * dt;

        // Check range
        let dist_x = self.x - self.start_x;
        let dist_y = self.y - self.start_y;
        let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();
        if distance > self.max_range {
            self.alive = false;
            return;
        }

        // Check wall collision
        let tile_x = (self.x / TILE_SIZE) as i32;
        let tile_y = (self.y / TILE_SIZE) as i32;

        if !map.is_walkable(tile_x, tile_y) {
            self.alive = false;
        }
    }

    pub fn draw(&self, camera_x: f32, camera_y: f32) {
        if !self.alive {
            return;
        }

        draw_circle(
            self.x - camera_x,
            self.y - camera_y,
            PROJECTILE_SIZE / 2.0,
            YELLOW,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projectile_direction() {
        let p = Projectile::new(0.0, 0.0, 100.0, 0.0, 100.0, 500.0);
        assert!((p.dx - 1.0).abs() < 0.001);
        assert!(p.dy.abs() < 0.001);
    }

    #[test]
    fn test_projectile_diagonal() {
        let p = Projectile::new(0.0, 0.0, 100.0, 100.0, 100.0, 500.0);
        let expected = 1.0 / 2.0_f32.sqrt();
        assert!((p.dx - expected).abs() < 0.001);
        assert!((p.dy - expected).abs() < 0.001);
    }

    #[test]
    fn test_projectile_zero_distance() {
        let p = Projectile::new(50.0, 50.0, 50.0, 50.0, 100.0, 500.0);
        assert!(p.alive);
        assert_eq!(p.dx, 1.0);
    }

    #[test]
    fn test_projectile_range() {
        let mut p = Projectile::new(0.0, 0.0, 100.0, 0.0, 1000.0, 50.0);
        let map = TileMap::new(10, 10);

        // Move past range
        p.update(0.1, &map);
        assert!(!p.alive);
    }
}
