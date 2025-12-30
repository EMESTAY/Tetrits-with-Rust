mod background;
mod bidule;
mod constants;
mod draw;
mod effects;
mod game;
mod grid;
mod sound_effects;

use crate::sound_effects::AudioSystem;
use game::Game;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Tetris Jelly".to_owned(),
        window_width: 1920,
        window_height: 1080,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font_res = load_ttf_font("src/assets/Jellies.ttf").await;
    let font = match font_res {
        Ok(f) => Some(f),
        Err(e) => {
            println!("Failed to load font: {:?}", e);
            None
        }
    };

    let audio = AudioSystem::new().await;
    let mut game = Game::new(font, audio);

    loop {
        clear_background(BLACK);

        game.update();
        draw::draw_game(&game);

        next_frame().await
    }
}
