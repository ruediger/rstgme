use macroquad::prelude::*;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct MoveDirection {
    pub dx: i32,
    pub dy: i32,
}

impl MoveDirection {
    pub fn is_moving(&self) -> bool {
        self.dx != 0 || self.dy != 0
    }
}

pub fn get_player_input() -> MoveDirection {
    let mut dir = MoveDirection::default();

    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        dir.dy = -1;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        dir.dy = 1;
    }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        dir.dx = -1;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        dir.dx = 1;
    }

    dir
}

pub fn get_mouse_position() -> (f32, f32) {
    mouse_position()
}

pub fn is_shooting() -> bool {
    is_mouse_button_down(MouseButton::Left)
}

pub fn get_weapon_switch() -> Option<usize> {
    if is_key_pressed(KeyCode::Key1) {
        Some(0)
    } else if is_key_pressed(KeyCode::Key2) {
        Some(1)
    } else if is_key_pressed(KeyCode::Key3) {
        Some(2)
    } else if is_key_pressed(KeyCode::Key4) {
        Some(3)
    } else if is_key_pressed(KeyCode::Key5) {
        Some(4)
    } else {
        None
    }
}

/// Check if player pressed the interact key (E)
pub fn is_interact_pressed() -> bool {
    is_key_pressed(KeyCode::E)
}

/// Check if player is holding the interact key (E)
pub fn is_interact_held() -> bool {
    is_key_down(KeyCode::E)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_direction_default() {
        let dir = MoveDirection::default();
        assert!(!dir.is_moving());
    }

    #[test]
    fn test_move_direction_moving() {
        let dir = MoveDirection { dx: 1, dy: 0 };
        assert!(dir.is_moving());
    }
}
