use crate::tile_map::TILE_SIZE;

#[derive(Clone, Debug)]
pub struct Weapon {
    pub name: &'static str,
    pub fire_rate: f32,
    pub bullet_speed: f32,
    pub range: f32,
    pub spread: f32,
    pub pellets: u8,
    pub is_melee: bool,
    pub cooldown: f32,
}

impl Weapon {
    pub fn knife() -> Self {
        Self {
            name: "Knife",
            fire_rate: 2.0,
            bullet_speed: 0.0,
            range: TILE_SIZE * 1.5,
            spread: 0.0,
            pellets: 0,
            is_melee: true,
            cooldown: 0.0,
        }
    }

    pub fn pistol() -> Self {
        Self {
            name: "Pistol",
            fire_rate: 4.0,
            bullet_speed: 400.0,
            range: TILE_SIZE * 8.0,
            spread: 0.0,
            pellets: 1,
            is_melee: false,
            cooldown: 0.0,
        }
    }

    pub fn shotgun() -> Self {
        Self {
            name: "Shotgun",
            fire_rate: 1.0,
            bullet_speed: 350.0,
            range: TILE_SIZE * 5.0,
            spread: 0.26, // ~15 degrees in radians
            pellets: 5,
            is_melee: false,
            cooldown: 0.0,
        }
    }

    pub fn machine_pistol() -> Self {
        Self {
            name: "Machine Pistol",
            fire_rate: 10.0,
            bullet_speed: 350.0,
            range: TILE_SIZE * 6.0,
            spread: 0.09, // ~5 degrees in radians
            pellets: 1,
            is_melee: false,
            cooldown: 0.0,
        }
    }

    pub fn rifle() -> Self {
        Self {
            name: "Rifle",
            fire_rate: 1.0,
            bullet_speed: 600.0,
            range: TILE_SIZE * 20.0,
            spread: 0.0,
            pellets: 1,
            is_melee: false,
            cooldown: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn all_weapons() -> Vec<Weapon> {
        vec![
            Self::knife(),
            Self::pistol(),
            Self::shotgun(),
            Self::machine_pistol(),
            Self::rifle(),
        ]
    }

    pub fn can_fire(&self) -> bool {
        self.cooldown <= 0.0
    }

    pub fn fire(&mut self) {
        self.cooldown = 1.0 / self.fire_rate;
    }

    pub fn update(&mut self, dt: f32) {
        if self.cooldown > 0.0 {
            self.cooldown -= dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pistol_creation() {
        let pistol = Weapon::pistol();
        assert_eq!(pistol.fire_rate, 4.0);
        assert!(pistol.can_fire());
        assert!(!pistol.is_melee);
    }

    #[test]
    fn test_fire_cooldown() {
        let mut pistol = Weapon::pistol();
        assert!(pistol.can_fire());

        pistol.fire();
        assert!(!pistol.can_fire());

        pistol.update(0.5);
        assert!(pistol.can_fire());
    }

    #[test]
    fn test_all_weapons() {
        let weapons = Weapon::all_weapons();
        assert_eq!(weapons.len(), 5);
        assert_eq!(weapons[0].name, "Knife");
        assert!(weapons[0].is_melee);
    }
}
