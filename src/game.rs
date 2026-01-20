use macroquad::prelude::*;

use crate::entity::{Bot, Player};
use crate::input::{get_mouse_position, get_player_input, get_weapon_switch, is_shooting};
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

        // Place player at a walkable spot
        let (px, py) = Self::find_walkable_spot(&map);
        let player = Player::new(px, py);

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

    fn handle_melee_attack(&mut self, target_x: f32, target_y: f32) {
        let (px, py) = self.player.pos.center_pixel();
        let range = self.player.weapon().range;

        // Direction to target
        let dx = target_x - px;
        let dy = target_y - py;
        let len = (dx * dx + dy * dy).sqrt();
        if len == 0.0 {
            return;
        }
        let dx = dx / len;
        let dy = dy / len;

        // Check bots in melee range in the direction of attack
        for bot in &mut self.bots {
            if !bot.alive {
                continue;
            }
            let (bx, by) = bot.pos.center_pixel();

            // Vector from player to bot
            let to_bot_x = bx - px;
            let to_bot_y = by - py;
            let dist = (to_bot_x * to_bot_x + to_bot_y * to_bot_y).sqrt();

            if dist > range {
                continue;
            }

            // Check if bot is roughly in the direction of attack
            let dot = (to_bot_x * dx + to_bot_y * dy) / dist;
            if dot > 0.5 {
                bot.kill();
                self.score += 1;
            }
        }
    }

    fn create_projectiles(&mut self, target_x: f32, target_y: f32) {
        let (px, py) = self.player.pos.center_pixel();
        let weapon = self.player.weapon();

        // Calculate base direction
        let dx = target_x - px;
        let dy = target_y - py;
        let base_angle = dy.atan2(dx);

        let pellets = weapon.pellets.max(1);
        let spread = weapon.spread;
        let speed = weapon.bullet_speed;
        let range = weapon.range;

        for i in 0..pellets {
            // Calculate spread angle for this pellet
            let angle_offset = if pellets > 1 {
                let spread_range = spread * 2.0;
                -spread + spread_range * (i as f32 / (pellets - 1) as f32)
            } else if spread > 0.0 {
                // Single pellet with spread (machine pistol) - random spread
                rand::gen_range(-spread, spread)
            } else {
                0.0
            };

            let angle = base_angle + angle_offset;
            let proj_dx = angle.cos();
            let proj_dy = angle.sin();

            let projectile = Projectile::new_with_direction(px, py, proj_dx, proj_dy, speed, range);
            self.projectiles.push(projectile);
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Handle weapon switching
        if let Some(weapon_index) = get_weapon_switch() {
            self.player.switch_weapon(weapon_index);
        }

        let input = get_player_input();
        self.player.update(dt, input, &self.map);

        self.update_camera();

        // Handle shooting - convert screen mouse pos to world pos
        if is_shooting() && self.player.weapon().can_fire() {
            let (mx, my) = get_mouse_position();
            let world_mx = mx + self.camera_x;
            let world_my = my + self.camera_y;

            self.player.weapon_mut().fire();

            if self.player.weapon().is_melee {
                self.handle_melee_attack(world_mx, world_my);
            } else {
                self.create_projectiles(world_mx, world_my);
            }
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

        // Draw HUD (fixed on screen)
        draw_text(&format!("Score: {}", self.score), 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!(
                "[{}] {}",
                self.player.current_weapon + 1,
                self.player.weapon().name
            ),
            10.0,
            60.0,
            24.0,
            YELLOW,
        );
        draw_text(
            "1:Fist 2:Pistol 3:Shotgun 4:MP 5:Rifle",
            10.0,
            85.0,
            16.0,
            GRAY,
        );
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
