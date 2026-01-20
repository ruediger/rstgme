use macroquad::prelude::*;

use crate::tile_map::TILE_SIZE;
use crate::weapon::Weapon;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WeaponKind {
    Pistol,
    Shotgun,
    MachinePistol,
    Rifle,
}

impl WeaponKind {
    pub fn to_weapon(self) -> Weapon {
        match self {
            WeaponKind::Pistol => Weapon::pistol(),
            WeaponKind::Shotgun => Weapon::shotgun(),
            WeaponKind::MachinePistol => Weapon::machine_pistol(),
            WeaponKind::Rifle => Weapon::rifle(),
        }
    }

    #[allow(dead_code)]
    pub fn name(self) -> &'static str {
        match self {
            WeaponKind::Pistol => "Pistol",
            WeaponKind::Shotgun => "Shotgun",
            WeaponKind::MachinePistol => "Machine Pistol",
            WeaponKind::Rifle => "Rifle",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemType {
    Weapon(WeaponKind),
    HealthPack,
    SpeedBoost,
    Invulnerability,
}

impl ItemType {
    fn color(self) -> Color {
        match self {
            ItemType::Weapon(WeaponKind::Pistol) => Color::from_rgba(180, 180, 180, 255),
            ItemType::Weapon(WeaponKind::Shotgun) => Color::from_rgba(139, 90, 43, 255),
            ItemType::Weapon(WeaponKind::MachinePistol) => Color::from_rgba(100, 100, 120, 255),
            ItemType::Weapon(WeaponKind::Rifle) => Color::from_rgba(60, 80, 60, 255),
            ItemType::HealthPack => Color::from_rgba(220, 60, 60, 255),
            ItemType::SpeedBoost => Color::from_rgba(60, 150, 220, 255),
            ItemType::Invulnerability => Color::from_rgba(220, 200, 60, 255),
        }
    }
}

pub struct Item {
    pub x: f32,
    pub y: f32,
    pub item_type: ItemType,
    pub alive: bool,
}

impl Item {
    pub fn new(tile_x: i32, tile_y: i32, item_type: ItemType) -> Self {
        Self {
            x: tile_x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
            y: tile_y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
            item_type,
            alive: true,
        }
    }

    pub fn tile_position(&self) -> (i32, i32) {
        ((self.x / TILE_SIZE) as i32, (self.y / TILE_SIZE) as i32)
    }

    /// Random item for floor spawns (common items only)
    pub fn random_floor_item(tile_x: i32, tile_y: i32) -> Self {
        let item_type = match rand::gen_range(0, 10) {
            0..=4 => ItemType::Weapon(WeaponKind::Pistol), // 50% pistol
            5..=9 => ItemType::HealthPack,                 // 50% health
            _ => ItemType::HealthPack,
        };
        Self::new(tile_x, tile_y, item_type)
    }

    /// Random item drop from destroyed crate
    pub fn random_crate_drop(tile_x: i32, tile_y: i32) -> Option<Self> {
        // 60% chance to drop something
        if rand::gen_range(0, 10) >= 6 {
            return None;
        }

        let item_type = match rand::gen_range(0, 20) {
            0..=5 => ItemType::HealthPack,                          // 30% health
            6..=10 => ItemType::Weapon(WeaponKind::Pistol),         // 25% pistol
            11..=14 => ItemType::Weapon(WeaponKind::Shotgun),       // 20% shotgun
            15..=16 => ItemType::Weapon(WeaponKind::MachinePistol), // 10% MP
            17..=18 => ItemType::SpeedBoost,                        // 10% speed
            19 => ItemType::Invulnerability,                        // 5% invuln
            _ => ItemType::HealthPack,
        };
        Some(Self::new(tile_x, tile_y, item_type))
    }

    /// Random item drop from destroyed wall (higher tier)
    pub fn random_wall_drop(tile_x: i32, tile_y: i32) -> Option<Self> {
        // 40% chance to drop something
        if rand::gen_range(0, 10) >= 4 {
            return None;
        }

        let item_type = match rand::gen_range(0, 20) {
            0..=3 => ItemType::HealthPack,                         // 20% health
            4..=7 => ItemType::Weapon(WeaponKind::Shotgun),        // 20% shotgun
            8..=11 => ItemType::Weapon(WeaponKind::MachinePistol), // 20% MP
            12..=15 => ItemType::Weapon(WeaponKind::Rifle),        // 20% rifle
            16..=17 => ItemType::SpeedBoost,                       // 10% speed
            18..=19 => ItemType::Invulnerability,                  // 10% invuln
            _ => ItemType::HealthPack,
        };
        Some(Self::new(tile_x, tile_y, item_type))
    }

    pub fn draw(&self, camera_x: f32, camera_y: f32) {
        if !self.alive {
            return;
        }

        let screen_x = self.x - camera_x;
        let screen_y = self.y - camera_y;
        let size = 12.0;
        let half = size / 2.0;

        let color = self.item_type.color();

        match self.item_type {
            ItemType::Weapon(_) => {
                // Draw weapon as a small square
                draw_rectangle(screen_x - half, screen_y - half, size, size, color);
                // Add a small highlight
                draw_rectangle(
                    screen_x - half + 2.0,
                    screen_y - half + 2.0,
                    4.0,
                    4.0,
                    Color::from_rgba(255, 255, 255, 100),
                );
            }
            ItemType::HealthPack => {
                // Draw as a cross
                draw_rectangle(screen_x - half, screen_y - 2.0, size, 4.0, color);
                draw_rectangle(screen_x - 2.0, screen_y - half, 4.0, size, color);
            }
            ItemType::SpeedBoost => {
                // Draw as a diamond
                let points = [
                    (screen_x, screen_y - half),
                    (screen_x + half, screen_y),
                    (screen_x, screen_y + half),
                    (screen_x - half, screen_y),
                ];
                for i in 0..4 {
                    let (x1, y1) = points[i];
                    let (x2, y2) = points[(i + 1) % 4];
                    draw_line(x1, y1, x2, y2, 2.0, color);
                }
            }
            ItemType::Invulnerability => {
                // Draw as a star/shield shape
                draw_circle(screen_x, screen_y, half, color);
                draw_circle(
                    screen_x,
                    screen_y,
                    half - 3.0,
                    Color::from_rgba(255, 255, 200, 255),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::new(5, 10, ItemType::HealthPack);
        assert!(item.alive);
        assert_eq!(item.tile_position(), (5, 10));
    }

    #[test]
    fn test_weapon_kind_to_weapon() {
        let weapon = WeaponKind::Pistol.to_weapon();
        assert_eq!(weapon.name, "Pistol");
    }

    #[test]
    fn test_floor_item_types() {
        // Just verify it doesn't panic
        for _ in 0..20 {
            let item = Item::random_floor_item(0, 0);
            assert!(matches!(
                item.item_type,
                ItemType::Weapon(WeaponKind::Pistol) | ItemType::HealthPack
            ));
        }
    }
}
