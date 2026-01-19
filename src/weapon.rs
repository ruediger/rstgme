#[derive(Clone, Debug)]
pub struct Weapon {
    pub fire_rate: f32,
    pub bullet_speed: f32,
    pub cooldown: f32,
}

impl Weapon {
    pub fn pistol() -> Self {
        Self {
            fire_rate: 4.0,
            bullet_speed: 400.0,
            cooldown: 0.0,
        }
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
}
