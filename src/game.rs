use macroquad::prelude::*;

use crate::entity::{Bot, Player};
use crate::input::get_player_input;
use crate::tile_map::TileMap;

pub struct GameState {
    map: TileMap,
    player: Player,
    bots: Vec<Bot>,
}

impl GameState {
    pub fn new() -> Self {
        let map = TileMap::create_test_level();

        // Place player in a walkable spot
        let player = Player::new(2, 2);

        // Add some bots
        let bots = vec![Bot::new(5, 8), Bot::new(15, 3), Bot::new(10, 10)];

        Self { map, player, bots }
    }

    pub fn update(&mut self, dt: f32) {
        let input = get_player_input();
        self.player.update(dt, input, &self.map);

        for bot in &mut self.bots {
            bot.update(dt, &self.map);
        }
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(30, 30, 40, 255));

        self.map.draw();
        self.player.draw();

        for bot in &self.bots {
            bot.draw();
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
