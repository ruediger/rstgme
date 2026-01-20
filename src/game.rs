use macroquad::prelude::*;

use crate::entity::{Bot, Player};
use crate::input::{get_mouse_position, get_player_input, is_shooting};
use crate::projectile::Projectile;
use crate::tile_map::{TILE_SIZE, TileMap};

const BOT_HITBOX_SIZE: f32 = TILE_SIZE - 8.0;
const MAP_WIDTH: usize = 60;
const MAP_HEIGHT: usize = 45;
const NUM_BOTS: usize = 10;

pub struct GameState {
    map: TileMap,
    player: Player,
    bots: Vec<Bot>,
    projectiles: Vec<Projectile>,
    score: u32,
    camera_x: f32,
    camera_y: f32,
}

impl GameState {
    pub fn new() -> Self {
        let map = TileMap::create_random(MAP_WIDTH, MAP_HEIGHT);

        // Place player in center-ish area
        let player = Player::new(MAP_WIDTH as i32 / 2, MAP_HEIGHT as i32 / 2);

        // Add bots at random walkable positions
        let mut bots = Vec::with_capacity(NUM_BOTS);
        for _ in 0..NUM_BOTS {
            let (x, y) = Self::find_walkable_spot(&map);
            bots.push(Bot::new(x, y));
        }

        Self {
            map,
            player,
            bots,
            projectiles: Vec::new(),
            score: 0,
            camera_x: 0.0,
            camera_y: 0.0,
        }
    }

    fn find_walkable_spot(map: &TileMap) -> (i32, i32) {
        loop {
            let x = rand::gen_range(2, map.width - 2) as i32;
            let y = rand::gen_range(2, map.height - 2) as i32;
            if map.is_walkable(x, y) {
                return (x, y);
            }
        }
    }

    fn update_camera(&mut self) {
        let (px, py) = self.player.pos.center_pixel();
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Target camera position (centered on player)
        let target_x = px - screen_w / 2.0;
        let target_y = py - screen_h / 2.0;

        // Clamp to map bounds
        let max_x = (self.map.width as f32 * TILE_SIZE - screen_w).max(0.0);
        let max_y = (self.map.height as f32 * TILE_SIZE - screen_h).max(0.0);

        self.camera_x = target_x.clamp(0.0, max_x);
        self.camera_y = target_y.clamp(0.0, max_y);
    }

    pub fn update(&mut self, dt: f32) {
        let input = get_player_input();
        self.player.update(dt, input, &self.map);

        self.update_camera();

        // Handle shooting - convert screen mouse pos to world pos
        if is_shooting() && self.player.weapon.can_fire() {
            let (px, py) = self.player.pos.center_pixel();
            let (mx, my) = get_mouse_position();
            let world_mx = mx + self.camera_x;
            let world_my = my + self.camera_y;

            self.player.weapon.fire();
            let projectile =
                Projectile::new(px, py, world_mx, world_my, self.player.weapon.bullet_speed);
            self.projectiles.push(projectile);
        }

        // Update projectiles
        for projectile in &mut self.projectiles {
            projectile.update(dt, &self.map);
        }

        // Check projectile-bot collisions
        for projectile in &mut self.projectiles {
            if !projectile.alive {
                continue;
            }
            for bot in &mut self.bots {
                if !bot.alive {
                    continue;
                }
                let (bx, by) = bot.pos.center_pixel();
                let half_size = BOT_HITBOX_SIZE / 2.0;
                if projectile.x >= bx - half_size
                    && projectile.x <= bx + half_size
                    && projectile.y >= by - half_size
                    && projectile.y <= by + half_size
                {
                    projectile.alive = false;
                    bot.kill();
                    self.score += 1;
                }
            }
        }

        // Remove dead projectiles
        self.projectiles.retain(|p| p.alive);

        for bot in &mut self.bots {
            bot.update(dt, &self.map);
        }
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(30, 30, 40, 255));

        self.map.draw(self.camera_x, self.camera_y);

        // Draw aim line (in screen space)
        let (px, py) = self.player.pos.center_pixel();
        let screen_px = px - self.camera_x;
        let screen_py = py - self.camera_y;
        let (mx, my) = get_mouse_position();
        draw_line(
            screen_px,
            screen_py,
            mx,
            my,
            1.0,
            Color::from_rgba(255, 255, 255, 80),
        );

        self.player.draw(self.camera_x, self.camera_y);

        for bot in &self.bots {
            bot.draw(self.camera_x, self.camera_y);
        }

        for projectile in &self.projectiles {
            projectile.draw(self.camera_x, self.camera_y);
        }

        // Draw score (fixed on screen)
        draw_text(&format!("Score: {}", self.score), 10.0, 30.0, 30.0, WHITE);
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
