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
