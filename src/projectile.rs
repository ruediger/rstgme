use crate::sprites::SpriteSheet;
use crate::tile_map::{TILE_SIZE, TileMap};

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
    pub from_player: bool,
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
            from_player: true,
        }
    }

    pub fn new_player(x: f32, y: f32, dx: f32, dy: f32, speed: f32, max_range: f32) -> Self {
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
            from_player: true,
        }
    }

    pub fn new_bot(x: f32, y: f32, dx: f32, dy: f32, speed: f32, max_range: f32) -> Self {
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
            from_player: false,
        }
    }

    /// Update projectile position. Returns Some((x, y)) if hit a blocking tile.
    pub fn update(&mut self, dt: f32, map: &TileMap) -> Option<(i32, i32)> {
        if !self.alive {
            return None;
        }

        self.x += self.dx * self.speed * dt;
        self.y += self.dy * self.speed * dt;

        // Check range
        let dist_x = self.x - self.start_x;
        let dist_y = self.y - self.start_y;
        let distance = (dist_x * dist_x + dist_y * dist_y).sqrt();
        if distance > self.max_range {
            self.alive = false;
            return None;
        }

        // Check tile collision (walls, doors, crates block; pits don't)
        let tile_x = (self.x / TILE_SIZE) as i32;
        let tile_y = (self.y / TILE_SIZE) as i32;

        if map.blocks_projectile_at(tile_x, tile_y) {
            self.alive = false;
            return Some((tile_x, tile_y));
        }

        None
    }

    pub fn draw(&self, camera_x: f32, camera_y: f32, sprites: &SpriteSheet) {
        if !self.alive {
            return;
        }

        let screen_x = self.x - camera_x;
        let screen_y = self.y - camera_y;
        sprites.draw_bullet(screen_x, screen_y);
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
