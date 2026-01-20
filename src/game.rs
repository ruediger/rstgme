use macroquad::prelude::*;

use crate::entity::{Bot, Player};
use crate::input::{get_mouse_position, get_player_input, get_weapon_switch, is_shooting};
use crate::item::{Item, ItemType};
use crate::projectile::Projectile;
use crate::sprites::SpriteSheet;
use crate::tile_map::{EntityType, TILE_SIZE, TileMap, TileType};

const BOT_HITBOX_SIZE: f32 = TILE_SIZE - 8.0;
const MAP_WIDTH: usize = 60;
const MAP_HEIGHT: usize = 45;
const NUM_BOTS: usize = 10;
const NUM_FLOOR_ITEMS: usize = 15;
const LAVA_DAMAGE_PER_SECOND: i32 = 25;
const HEALTH_PACK_AMOUNT: i32 = 25;
const SPEED_BOOST_DURATION: f32 = 5.0;
const INVULNERABILITY_DURATION: f32 = 3.0;
const MELEE_SWING_DURATION: f32 = 0.15;
const MELEE_SWING_ARC: f32 = std::f32::consts::PI * 0.6; // ~108 degrees

struct MeleeSwing {
    x: f32,
    y: f32,
    angle: f32, // Center angle of the swing
    range: f32,
    timer: f32,
}

impl MeleeSwing {
    fn new(x: f32, y: f32, target_x: f32, target_y: f32, range: f32) -> Self {
        let dx = target_x - x;
        let dy = target_y - y;
        let angle = dy.atan2(dx);
        Self {
            x,
            y,
            angle,
            range,
            timer: MELEE_SWING_DURATION,
        }
    }

    fn update(&mut self, dt: f32) {
        self.timer -= dt;
    }

    fn is_alive(&self) -> bool {
        self.timer > 0.0
    }

    fn draw(&self, camera_x: f32, camera_y: f32) {
        let progress = 1.0 - (self.timer / MELEE_SWING_DURATION);
        let alpha = ((1.0 - progress) * 200.0) as u8;

        // Draw arc segments
        let half_arc = MELEE_SWING_ARC / 2.0;
        let start_angle = self.angle - half_arc;
        let segments = 8;

        let screen_x = self.x - camera_x;
        let screen_y = self.y - camera_y;

        // Animate the swing - starts from one side, sweeps to the other
        let sweep_progress = progress;
        let current_sweep = sweep_progress * MELEE_SWING_ARC;

        for i in 0..segments {
            // Only draw segments that have been "swept" through
            let t0 = i as f32 / segments as f32;
            let t1 = (i + 1) as f32 / segments as f32;

            if t1 <= sweep_progress {
                let a0 = start_angle + t0 * MELEE_SWING_ARC;
                let a1 = start_angle + t1 * MELEE_SWING_ARC;

                let x0 = screen_x + a0.cos() * self.range;
                let y0 = screen_y + a0.sin() * self.range;
                let x1 = screen_x + a1.cos() * self.range;
                let y1 = screen_y + a1.sin() * self.range;

                // Fade out segments that were drawn earlier
                let seg_alpha = (alpha as f32 * (1.0 - t0 * 0.5)) as u8;
                draw_line(
                    x0,
                    y0,
                    x1,
                    y1,
                    3.0,
                    Color::from_rgba(255, 200, 100, seg_alpha),
                );
            }
        }

        // Draw the leading edge of the swing
        if sweep_progress > 0.0 {
            let lead_angle = start_angle + current_sweep;
            let lead_x = screen_x + lead_angle.cos() * self.range;
            let lead_y = screen_y + lead_angle.sin() * self.range;
            draw_line(
                screen_x,
                screen_y,
                lead_x,
                lead_y,
                2.0,
                Color::from_rgba(255, 255, 200, alpha),
            );
        }
    }
}

const DAMAGE_FLASH_DURATION: f32 = 0.35;

pub struct GameState {
    map: TileMap,
    player: Player,
    bots: Vec<Bot>,
    projectiles: Vec<Projectile>,
    melee_swings: Vec<MeleeSwing>,
    items: Vec<Item>,
    score: u32,
    camera_x: f32,
    camera_y: f32,
    lava_damage_accumulator: f32,
    damage_flash_timer: f32,
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

        // Spawn floor items (pistols and health packs)
        let mut items = Vec::new();
        for _ in 0..NUM_FLOOR_ITEMS {
            let (x, y) = Self::find_walkable_spot(&map);
            items.push(Item::random_floor_item(x, y));
        }

        Self {
            map,
            player,
            bots,
            projectiles: Vec::new(),
            melee_swings: Vec::new(),
            items,
            score: 0,
            camera_x: 0.0,
            camera_y: 0.0,
            lava_damage_accumulator: 0.0,
            damage_flash_timer: 0.0,
        }
    }

    fn find_walkable_spot(map: &TileMap) -> (i32, i32) {
        loop {
            let x = rand::gen_range(2, map.width - 2) as i32;
            let y = rand::gen_range(2, map.height - 2) as i32;
            // Use Player entity type - bots may not be able to walk everywhere player can
            if map.is_walkable_by(x, y, EntityType::Player) {
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
        // Check if player is dead and respawn
        if !self.player.is_alive() {
            let (x, y) = Self::find_walkable_spot(&self.map);
            self.player.respawn(x, y);
            self.lava_damage_accumulator = 0.0;
        }

        // Handle weapon switching
        if let Some(weapon_index) = get_weapon_switch() {
            self.player.switch_weapon(weapon_index);
        }

        let input = get_player_input();
        self.player.update(dt, input, &self.map);

        // Update damage flash timer
        if self.damage_flash_timer > 0.0 {
            self.damage_flash_timer -= dt;
        }

        // Apply lava damage (speed boost grants lava immunity)
        if self.map.is_lava_at(self.player.pos.x, self.player.pos.y)
            && self.player.speed_boost_timer <= 0.0
        {
            self.lava_damage_accumulator += LAVA_DAMAGE_PER_SECOND as f32 * dt;
            let damage = self.lava_damage_accumulator as i32;
            if damage > 0 {
                let prev_health = self.player.health;
                self.player.take_damage(damage);
                // Only start a new flash if the previous one has faded
                if self.player.health < prev_health && self.damage_flash_timer <= 0.0 {
                    self.damage_flash_timer = DAMAGE_FLASH_DURATION;
                }
                self.lava_damage_accumulator -= damage as f32;
            }
        } else {
            self.lava_damage_accumulator = 0.0;
        }

        self.update_camera();

        // Handle shooting - convert screen mouse pos to world pos
        if is_shooting() && self.player.weapon().can_fire() {
            let (mx, my) = get_mouse_position();
            let world_mx = mx + self.camera_x;
            let world_my = my + self.camera_y;

            self.player.weapon_mut().fire();

            if self.player.weapon().is_melee {
                let (px, py) = self.player.pos.center_pixel();
                let range = self.player.weapon().range;
                self.melee_swings
                    .push(MeleeSwing::new(px, py, world_mx, world_my, range));
                self.handle_melee_attack(world_mx, world_my);
            } else {
                self.create_projectiles(world_mx, world_my);
            }
        }

        // Update projectiles and handle collisions with tiles
        for projectile in &mut self.projectiles {
            if let Some((tile_x, tile_y)) = projectile.update(dt, &self.map) {
                // Projectile hit a tile - damage it if destructible
                if self.map.is_destructible_at(tile_x, tile_y) {
                    let tile = self.map.get_tile(tile_x as usize, tile_y as usize);
                    let is_crate = tile == Some(TileType::Crate);
                    let destroyed = self.map.damage_tile(tile_x as usize, tile_y as usize);
                    if destroyed {
                        // Roll for item drop
                        let drop = if is_crate {
                            Item::random_crate_drop(tile_x, tile_y)
                        } else {
                            Item::random_wall_drop(tile_x, tile_y)
                        };
                        if let Some(item) = drop {
                            self.items.push(item);
                        }
                    }
                }
            }
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

        // Update melee swings
        for swing in &mut self.melee_swings {
            swing.update(dt);
        }
        self.melee_swings.retain(|s| s.is_alive());

        // Check item pickups
        for item in &mut self.items {
            if !item.alive {
                continue;
            }
            let (ix, iy) = item.tile_position();
            if self.player.pos.x == ix && self.player.pos.y == iy {
                // Pick up the item
                item.alive = false;
                match item.item_type {
                    ItemType::Weapon(kind) => {
                        let weapon = kind.to_weapon();
                        self.player.add_weapon(weapon);
                    }
                    ItemType::HealthPack => {
                        self.player.heal(HEALTH_PACK_AMOUNT);
                    }
                    ItemType::SpeedBoost => {
                        self.player.speed_boost_timer = SPEED_BOOST_DURATION;
                    }
                    ItemType::Invulnerability => {
                        self.player.invulnerability_timer = INVULNERABILITY_DURATION;
                    }
                }
            }
        }
        self.items.retain(|i| i.alive);

        for bot in &mut self.bots {
            bot.update(dt, &self.map);
        }
    }

    pub fn draw(&self, sprites: &SpriteSheet) {
        clear_background(Color::from_rgba(30, 30, 40, 255));

        self.map.draw(self.camera_x, self.camera_y, sprites);

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

        self.player.draw(self.camera_x, self.camera_y, sprites);

        for bot in &self.bots {
            bot.draw(self.camera_x, self.camera_y, sprites);
        }

        for projectile in &self.projectiles {
            projectile.draw(self.camera_x, self.camera_y, sprites);
        }

        for swing in &self.melee_swings {
            swing.draw(self.camera_x, self.camera_y);
        }

        // Draw items
        for item in &self.items {
            item.draw(self.camera_x, self.camera_y, sprites);
        }

        // Draw damage flash overlay
        if self.damage_flash_timer > 0.0 {
            let alpha = (self.damage_flash_timer / DAMAGE_FLASH_DURATION * 100.0) as u8;
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::from_rgba(255, 0, 0, alpha),
            );
        }

        // Draw HUD (fixed on screen)
        draw_text(&format!("Score: {}", self.score), 10.0, 30.0, 30.0, WHITE);

        // Health bar
        let health_bar_width = 150.0;
        let health_bar_height = 16.0;
        let health_x = 10.0;
        let health_y = 40.0;
        let health_pct = self.player.health as f32 / self.player.max_health as f32;

        // Background (empty health)
        draw_rectangle(
            health_x,
            health_y,
            health_bar_width,
            health_bar_height,
            Color::from_rgba(60, 60, 60, 255),
        );
        // Filled health
        let health_color = if health_pct > 0.5 {
            Color::from_rgba(80, 200, 80, 255)
        } else if health_pct > 0.25 {
            Color::from_rgba(200, 200, 80, 255)
        } else {
            Color::from_rgba(200, 80, 80, 255)
        };
        draw_rectangle(
            health_x,
            health_y,
            health_bar_width * health_pct,
            health_bar_height,
            health_color,
        );
        // Health text
        draw_text(
            &format!("{}/{}", self.player.health, self.player.max_health),
            health_x + 5.0,
            health_y + 13.0,
            16.0,
            WHITE,
        );

        draw_text(
            &format!(
                "[{}] {}",
                self.player.current_weapon + 1,
                self.player.weapon().name
            ),
            10.0,
            80.0,
            24.0,
            YELLOW,
        );

        // Show available weapons
        let weapon_list: String = self
            .player
            .weapons
            .iter()
            .enumerate()
            .map(|(i, w)| {
                format!(
                    "{}:{}",
                    i + 1,
                    w.name.split_whitespace().next().unwrap_or(w.name)
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        draw_text(&weapon_list, 10.0, 105.0, 16.0, GRAY);

        // Show active buffs
        let mut buff_y = 125.0;
        if self.player.speed_boost_timer > 0.0 {
            draw_text(
                &format!("SPEED {:.1}s", self.player.speed_boost_timer),
                10.0,
                buff_y,
                16.0,
                Color::from_rgba(60, 150, 220, 255),
            );
            buff_y += 18.0;
        }
        if self.player.invulnerability_timer > 0.0 {
            draw_text(
                &format!("INVULN {:.1}s", self.player.invulnerability_timer),
                10.0,
                buff_y,
                16.0,
                Color::from_rgba(220, 200, 60, 255),
            );
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
