mod audio;
mod entity;
mod game;
mod input;
mod item;
mod projectile;
mod sprites;
mod terminal;
mod tile_map;
mod weapon;

use audio::AudioManager;
use game::GameState;
use macroquad::prelude::*;
use sprites::SpriteSheet;

fn window_conf() -> Conf {
    Conf {
        window_title: "rstgme".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Seed random number generator with current time
    rand::srand(macroquad::miniquad::date::now() as u64);

    let sprites = SpriteSheet::load().await;
    let audio = AudioManager::load().await;
    let mut game = GameState::new(audio);

    loop {
        let dt = get_frame_time();

        game.update(dt);
        game.draw(&sprites);

        next_frame().await
    }
}
