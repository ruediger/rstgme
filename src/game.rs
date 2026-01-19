use macroquad::prelude::*;

use crate::entity::{Bot, Player};
use crate::input::{get_mouse_position, get_player_input, is_shooting};
use crate::projectile::Projectile;
use crate::tile_map::TileMap;

pub struct GameState {
    map: TileMap,
    player: Player,
    bots: Vec<Bot>,
    projectiles: Vec<Projectile>,
}

impl GameState {
    pub fn new() -> Self {
        let map = TileMap::create_test_level();

        // Place player in a walkable spot
        let player = Player::new(2, 2);

        // Add some bots
        let bots = vec![Bot::new(5, 8), Bot::new(15, 3), Bot::new(10, 10)];

        Self {
            map,
            player,
            bots,
            projectiles: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let input = get_player_input();
        self.player.update(dt, input, &self.map);

        // Handle shooting
        if is_shooting() && self.player.weapon.can_fire() {
            let (px, py) = self.player.pos.center_pixel();
            let (mx, my) = get_mouse_position();

            self.player.weapon.fire();
            let projectile = Projectile::new(px, py, mx, my, self.player.weapon.bullet_speed);
            self.projectiles.push(projectile);
        }

        // Update projectiles
        for projectile in &mut self.projectiles {
            projectile.update(dt, &self.map);
        }

        // Remove dead projectiles
        self.projectiles.retain(|p| p.alive);

        for bot in &mut self.bots {
            bot.update(dt, &self.map);
        }
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(30, 30, 40, 255));

        self.map.draw();

        // Draw aim line
        let (px, py) = self.player.pos.center_pixel();
        let (mx, my) = get_mouse_position();
        draw_line(px, py, mx, my, 1.0, Color::from_rgba(255, 255, 255, 80));

        self.player.draw();

        for bot in &self.bots {
            bot.draw();
        }

        for projectile in &self.projectiles {
            projectile.draw();
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
