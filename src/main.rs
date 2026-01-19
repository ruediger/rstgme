mod entity;
mod game;
mod input;
mod projectile;
mod tile_map;
mod weapon;

use game::GameState;
use macroquad::prelude::*;

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
    let mut game = GameState::new();

    loop {
        let dt = get_frame_time();

        game.update(dt);
        game.draw();

        next_frame().await
    }
}
