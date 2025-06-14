use macroquad::prelude::*;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

mod config;
mod game;
mod ws;


#[macroquad::main("Tribalcraft")]
async fn main() {
    

    let mut game = game::Game::new();
    

    loop {
        // update ws
        //game.ws.update();

        clear_background(BLACK);

        game.render.render_background();
        game.render.render_grid_lines();

        //draw_rectangle(0.0 - game.render.x_offset, 0.0 - game.render.y_offset, 10000.0, 2000.0, Color::from_hex(0x768f5b));

        next_frame().await;
    }
}
