use macroquad::rand;

use crate::sprites::{SpriteSheet, items};
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
    fn sprite_index(self) -> u32 {
        match self {
            ItemType::Weapon(WeaponKind::Pistol) => items::PISTOL,
            ItemType::Weapon(WeaponKind::Shotgun) => items::SHOTGUN,
            ItemType::Weapon(WeaponKind::MachinePistol) => items::MACHINE_PISTOL,
            ItemType::Weapon(WeaponKind::Rifle) => items::RIFLE,
            ItemType::HealthPack => items::HEALTH_PACK,
            ItemType::SpeedBoost => items::SPEED_BOOST,
            ItemType::Invulnerability => items::INVULNERABILITY,
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

    pub fn draw(&self, camera_x: f32, camera_y: f32, sprites: &SpriteSheet) {
        if !self.alive {
            return;
        }

        let screen_x = self.x - camera_x;
        let screen_y = self.y - camera_y;
        let sprite_idx = self.item_type.sprite_index();
        sprites.draw_item(sprite_idx, screen_x, screen_y);
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
