use async_std::{stream::Once, task};
use async_tungstenite::async_std::connect_async;
use futures_util::{SinkExt, StreamExt};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use tungstenite::client::IntoClientRequest;

static mut SERVER_MSG: Option<String> = None;

#[macroquad::main("Meedy is bad")]
async fn main() {
    console_error_panic_hook::set_once();

    // ws
    task::spawn(async {
        let request = "ws://localhost:8080".into_client_request().unwrap();
        let (mut ws_stream, _) = connect_async(request)
            .await
            .expect("WebSocket Connection to server failed.");

        while let Some(msg) = ws_stream.next().await {
            if let Ok(text) = msg {
                if let Ok(txt) = text.into_text() {
                    //miniquad::log!(miniquad::log::Level::Info, "from server {}", &txt);

                    unsafe {
                        SERVER_MSG = Some(txt.to_string());
                    }
                }
            }
        }
    });

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
