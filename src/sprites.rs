use macroquad::prelude::*;

const TILE_SIZE: f32 = 32.0;
const ITEM_SIZE: f32 = 32.0; // Items are in 32px slots in the sheet
const BULLET_SIZE: f32 = 32.0;

/// Sprite sheet layout indices
pub mod tiles {
    pub const FLOOR: u32 = 0;
    pub const WALL: u32 = 1;
    pub const SAND: u32 = 2;
    pub const WATER: u32 = 3;
    pub const LAVA: u32 = 4;
    pub const PIT: u32 = 5;
    pub const DOOR_PLAYER: u32 = 6;
    pub const DOOR_BOT: u32 = 7;
    pub const DOOR_BOTH: u32 = 8;
    pub const CRATE: u32 = 9;
    pub const WALL_DESTRUCTIBLE: u32 = 10;
}

pub mod items {
    pub const PISTOL: u32 = 0;
    pub const SHOTGUN: u32 = 1;
    pub const MACHINE_PISTOL: u32 = 2;
    pub const RIFLE: u32 = 3;
    pub const HEALTH_PACK: u32 = 4;
    pub const SPEED_BOOST: u32 = 5;
    pub const INVULNERABILITY: u32 = 6;
    pub const BULLET: u32 = 15;
}

/// Direction indices for entity rotations (8 directions)
/// Order: Down, Down-Right, Right, Up-Right, Up, Up-Left, Left, Down-Left
pub mod direction {
    pub const DOWN: u32 = 0;
    pub const DOWN_RIGHT: u32 = 1;
    pub const RIGHT: u32 = 2;
    pub const UP_RIGHT: u32 = 3;
    pub const UP: u32 = 4;
    pub const UP_LEFT: u32 = 5;
    pub const LEFT: u32 = 6;
    pub const DOWN_LEFT: u32 = 7;
}

pub struct SpriteSheet {
    texture: Texture2D,
}

impl SpriteSheet {
    pub async fn load() -> Self {
        let texture = load_texture("data/sprites.png")
            .await
            .expect("Failed to load sprites.png");
        texture.set_filter(FilterMode::Nearest);
        Self { texture }
    }

    /// Get source rect for a tile (row 0)
    pub fn tile_rect(&self, index: u32) -> Rect {
        Rect::new(index as f32 * TILE_SIZE, 0.0, TILE_SIZE, TILE_SIZE)
    }

    /// Get source rect for player sprite (row 1) with direction
    pub fn player_rect(&self, direction: u32) -> Rect {
        Rect::new(
            direction as f32 * TILE_SIZE,
            TILE_SIZE, // Row 1
            TILE_SIZE,
            TILE_SIZE,
        )
    }

    /// Get source rect for bot sprite (row 2) with direction
    pub fn bot_rect(&self, direction: u32) -> Rect {
        Rect::new(
            direction as f32 * TILE_SIZE,
            TILE_SIZE * 2.0, // Row 2
            TILE_SIZE,
            TILE_SIZE,
        )
    }

    /// Get source rect for item (row 3)
    pub fn item_rect(&self, index: u32) -> Rect {
        Rect::new(
            index as f32 * ITEM_SIZE,
            TILE_SIZE * 3.0, // Row 3
            ITEM_SIZE,
            ITEM_SIZE,
        )
    }

    /// Get source rect for bullet (row 3, index 15)
    pub fn bullet_rect(&self) -> Rect {
        Rect::new(
            items::BULLET as f32 * BULLET_SIZE,
            TILE_SIZE * 3.0, // Row 3
            BULLET_SIZE,
            BULLET_SIZE,
        )
    }

    /// Draw a tile at the given screen position
    pub fn draw_tile(&self, index: u32, x: f32, y: f32) {
        let src = self.tile_rect(index);
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(src),
                ..Default::default()
            },
        );
    }

    /// Draw a tile with damage darkening (for destructibles)
    pub fn draw_tile_damaged(&self, index: u32, x: f32, y: f32, damage_factor: f32) {
        let src = self.tile_rect(index);
        let brightness = 0.5 + 0.5 * (1.0 - damage_factor);
        let color = Color::new(brightness, brightness, brightness, 1.0);
        draw_texture_ex(
            &self.texture,
            x,
            y,
            color,
            DrawTextureParams {
                source: Some(src),
                ..Default::default()
            },
        );
    }

    /// Draw player at the given screen position with direction
    pub fn draw_player(&self, x: f32, y: f32, direction: u32) {
        let src = self.player_rect(direction);
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(src),
                ..Default::default()
            },
        );
    }

    /// Draw bot at the given screen position with direction
    pub fn draw_bot(&self, x: f32, y: f32, direction: u32) {
        let src = self.bot_rect(direction);
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(src),
                ..Default::default()
            },
        );
    }

    /// Draw bot with a color tint (for hostile bots)
    pub fn draw_bot_tinted(&self, x: f32, y: f32, direction: u32, tint: Color) {
        let src = self.bot_rect(direction);
        draw_texture_ex(
            &self.texture,
            x,
            y,
            tint,
            DrawTextureParams {
                source: Some(src),
                ..Default::default()
            },
        );
    }

    /// Draw item at the given screen position (centered)
    pub fn draw_item(&self, index: u32, x: f32, y: f32) {
        let src = self.item_rect(index);
        // Items are 32x32 in the sheet, draw centered
        draw_texture_ex(
            &self.texture,
            x - ITEM_SIZE / 2.0,
            y - ITEM_SIZE / 2.0,
            WHITE,
            DrawTextureParams {
                source: Some(src),
                ..Default::default()
            },
        );
    }

    /// Draw bullet at the given screen position (centered)
    pub fn draw_bullet(&self, x: f32, y: f32) {
        let src = self.bullet_rect();
        // Scale down bullet to reasonable size
        let display_size = 12.0;
        draw_texture_ex(
            &self.texture,
            x - display_size / 2.0,
            y - display_size / 2.0,
            WHITE,
            DrawTextureParams {
                source: Some(src),
                dest_size: Some(Vec2::new(display_size, display_size)),
                ..Default::default()
            },
        );
    }
}

/// Convert an angle (in radians) to a direction index (0-7)
/// 0 = down, going clockwise
#[allow(dead_code)]
pub fn angle_to_direction(angle: f32) -> u32 {
    // Normalize angle to 0..2PI
    let mut a = angle;
    while a < 0.0 {
        a += std::f32::consts::TAU;
    }
    while a >= std::f32::consts::TAU {
        a -= std::f32::consts::TAU;
    }

    // Convert to 8 directions
    // angle 0 = right, PI/2 = down, PI = left, 3PI/2 = up
    // We want: 0=down, 1=down-right, 2=right, etc.
    // So we need to offset by PI/2 and then divide into 8 sectors

    let sector = ((a + std::f32::consts::FRAC_PI_8) / std::f32::consts::FRAC_PI_4) as u32 % 8;

    // Map from angle-based sectors to sprite indices
    // Angle sectors: 0=right, 1=down-right, 2=down, 3=down-left, 4=left, 5=up-left, 6=up, 7=up-right
    // Sprite indices: 0=down, 1=down-right, 2=right, 3=up-right, 4=up, 5=up-left, 6=left, 7=down-left
    match sector {
        0 => direction::RIGHT,
        1 => direction::DOWN_RIGHT,
        2 => direction::DOWN,
        3 => direction::DOWN_LEFT,
        4 => direction::LEFT,
        5 => direction::UP_LEFT,
        6 => direction::UP,
        7 => direction::UP_RIGHT,
        _ => direction::DOWN,
    }
}

/// Convert dx, dy movement to a direction index
pub fn movement_to_direction(dx: i32, dy: i32) -> u32 {
    match (dx, dy) {
        (0, 1) => direction::DOWN,
        (1, 1) => direction::DOWN_RIGHT,
        (1, 0) => direction::RIGHT,
        (1, -1) => direction::UP_RIGHT,
        (0, -1) => direction::UP,
        (-1, -1) => direction::UP_LEFT,
        (-1, 0) => direction::LEFT,
        (-1, 1) => direction::DOWN_LEFT,
        _ => direction::DOWN, // Default
    }
}
