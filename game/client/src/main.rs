use notan::prelude::*;
use notan::draw::*;
use gloo_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures_util::{StreamExt, SinkExt};
use log::{info, error};

mod class;
use class::Game;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init()
    .add_config(DrawConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics) {
    static INIT_WS: std::sync::Once = std::sync::Once::new();
    INIT_WS.call_once(|| {
        init_websocket();
    });

    info!("render!");

    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0))
        .color(Color { r: 255.0, g: 0.0, b: 0.0, a: 1.0 });
    gfx.render(&draw);
}

fn init_websocket() {
    let ws = WebSocket::open("wss://echo.websocket.org");
    let mut ws = match ws {
        Ok(ws) => ws,
        Err(err) => {
            error!("WebSocket failed to open: {:?}", err);
            return;
        }
    };
    let (mut write, mut read) = ws.split();

    spawn_local(async move {
        if let Err(e) = write.send(Message::Text("hello".into())).await {
            error!("WS send error: {:?}", e);
        }
    });

    spawn_local(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => info!("Received text: {}", text),
                Ok(Message::Bytes(bytes)) => info!("Received {} bytes", bytes.len()),
                Err(e) => error!("WebSocket error: {:?}", e),
            }
        }
        info!("WebSocket connection closed");
    });
}
