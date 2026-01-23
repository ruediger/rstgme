use crate::sprites::SpriteSheet;
use crate::tile_map::TILE_SIZE;
use macroquad::prelude::*;

// Hacking constants
pub const HACK_DURATION: f32 = 7.0; // Seconds of active hacking to complete
pub const HACK_WINDOW: f32 = 18.0; // Total seconds before hack fails
pub const HACK_RANGE: i32 = 1; // Tiles from terminal to interact
pub const FAIL_BOT_SPAWN: usize = 3; // Extra hostile bots spawned on failure

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HackState {
    Idle,
    InProgress { progress: f32, elapsed: f32 },
    Complete,
}

pub struct Terminal {
    pub x: f32, // Pixel coordinates (centered in tile)
    pub y: f32,
    pub state: HackState,
}

impl Terminal {
    pub fn new(tile_x: i32, tile_y: i32) -> Self {
        Self {
            x: tile_x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
            y: tile_y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
            state: HackState::Idle,
        }
    }

    /// Get the tile position of this terminal
    pub fn tile_position(&self) -> (i32, i32) {
        ((self.x / TILE_SIZE) as i32, (self.y / TILE_SIZE) as i32)
    }

    /// Relocate terminal to a new position and reset state
    pub fn relocate(&mut self, tile_x: i32, tile_y: i32) {
        self.x = tile_x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        self.y = tile_y as f32 * TILE_SIZE + TILE_SIZE / 2.0;
        self.state = HackState::Idle;
    }

    /// Check if player is within interaction range
    pub fn is_player_nearby(&self, player_x: i32, player_y: i32) -> bool {
        let (tx, ty) = self.tile_position();
        let dist = (player_x - tx).abs() + (player_y - ty).abs();
        dist <= HACK_RANGE
    }

    /// Draw the terminal
    pub fn draw(&self, camera_x: f32, camera_y: f32, sprites: &SpriteSheet) {
        let screen_x = self.x - TILE_SIZE / 2.0 - camera_x;
        let screen_y = self.y - TILE_SIZE / 2.0 - camera_y;

        match self.state {
            HackState::Complete => {
                // Draw completed terminal with green tint
                sprites.draw_terminal_tinted(
                    screen_x,
                    screen_y,
                    Color::from_rgba(100, 255, 100, 255),
                );
            }
            _ => {
                sprites.draw_terminal(screen_x, screen_y);
            }
        }
    }

    /// Draw interaction prompt if player is nearby and terminal is hackable
    pub fn draw_prompt(&self, camera_x: f32, camera_y: f32, player_x: i32, player_y: i32) {
        if self.state == HackState::Complete {
            return;
        }

        if self.is_player_nearby(player_x, player_y) {
            let screen_x = self.x - TILE_SIZE / 2.0 - camera_x;
            let screen_y = self.y - TILE_SIZE / 2.0 - camera_y;

            // Draw "[E] Hack" prompt above terminal
            draw_text(
                "[E] Hack",
                screen_x - 8.0,
                screen_y - 5.0,
                16.0,
                Color::from_rgba(255, 255, 100, 255),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_creation() {
        let terminal = Terminal::new(5, 10);
        assert_eq!(terminal.tile_position(), (5, 10));
        assert_eq!(terminal.state, HackState::Idle);
    }

    #[test]
    fn test_terminal_relocate() {
        let mut terminal = Terminal::new(5, 10);
        terminal.state = HackState::InProgress {
            progress: 0.5,
            elapsed: 5.0,
        };

        terminal.relocate(20, 30);
        assert_eq!(terminal.tile_position(), (20, 30));
        assert_eq!(terminal.state, HackState::Idle);
    }

    #[test]
    fn test_player_nearby() {
        let terminal = Terminal::new(10, 10);

        // Adjacent tiles should be nearby
        assert!(terminal.is_player_nearby(10, 10)); // Same tile
        assert!(terminal.is_player_nearby(11, 10)); // Right
        assert!(terminal.is_player_nearby(10, 11)); // Below

        // 2 tiles away should not be nearby
        assert!(!terminal.is_player_nearby(12, 10));
        assert!(!terminal.is_player_nearby(10, 12));
    }
}
