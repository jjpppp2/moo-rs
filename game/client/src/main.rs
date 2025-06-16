use bincode::{decode_from_slice, encode_to_vec, Decode, Encode};
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use log::{error, info};
use notan::draw::*;
use notan::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod packets;
use packets::IncomingPackets;

use crate::packets::OutgoingPackets;
use crate::packets::SetIDPacket;
use crate::packets::SpawnPacket;
use crate::packets::UpdatePlayersPacket;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().add_config(DrawConfig).draw(draw).build()
}

fn draw(gfx: &mut Graphics) {
    let x_off = 300.0;
    let y_off = 300.0;

    static INIT_WS: std::sync::Once = std::sync::Once::new();
    INIT_WS.call_once(|| {
        init_websocket();
    });

    info!("render!");

    let mut draw = gfx.create_draw();
    
    //110.0 / 255.0, 140.0 / 255.0, 90.0 / 255.0
    draw.clear(Color::BLACK);

    draw.rect((x_off, y_off), (14400.0, 2400.0)).color(Color::from_rgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0));
    draw.rect((x_off, y_off + 2400.0), (14400.0, 14000.0 - 2400.0)).color(Color::from_rgb(149.0 / 255.0, 196.0 / 255.0, 100.0 / 255.0));

    draw.set_alpha(0.35);
    draw.rect((x_off, y_off), (draw.width(), draw.height())).color(Color { r: 0.0, g: 0.0, b: 70.0, a: 0.35 });

    let mut i = 0.0;
    while i < draw.width() {
        draw.line((i, 0.0), (i, draw.height())).color(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.06 });

        i += 30.0;
    }

    let mut i = 0.0;
    while i < draw.height() {
        draw.line((0.0, i), (draw.width(), i)).color(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.06 });

        i += 30.0;
    }


    gfx.render(&draw);
}

fn init_websocket() {
    let ws = WebSocket::open("ws://localhost:8089");
    let ws = match ws {
        Ok(ws) => ws,
        Err(err) => {
            error!("WebSocket failed to open: {:?}", err);
            return;
        }
    };
    let (mut write, mut read) = ws.split();
    let bincode_default_config = bincode::config::standard();

    spawn_local(async move {
        let encoded = match bincode::encode_to_vec(
            &OutgoingPackets::Spawn(SpawnPacket {
                name: String::from("Test"),
            }),
            bincode_default_config,
        ) {
            Ok(v) => v,
            Err(err) => {
                error!("Error serializing data {:?}", err);
                return;
            }
        };

        match write.send(Message::Bytes(encoded)).await {
            Ok(v) => v,
            Err(err) => {
                error!("Error sending serialized content to client: {:?}", err);
            }
        }
    });

    error!("lala");

    spawn_local(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Bytes(data)) => {
                    let (packet, _): (IncomingPackets, _) =
                        match decode_from_slice(&data, bincode_default_config) {
                            Ok(v) => v,
                            Err(e) => {
                                error!("failed to decode msg {:?}", e);
                                continue;
                            }
                        };

                    error!("packet {:?}", packet);

                    match packet {
                        IncomingPackets::UpdatePlayers(UpdatePlayersPacket { data }) => {
                            info!("lalala {:?}", data)
                        }

                        IncomingPackets::SetID(SetIDPacket { id }) => {
                            error!("wha {}", id);
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }
        info!("WebSocket connection closed");
    });
}
