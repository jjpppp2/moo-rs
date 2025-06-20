use bincode::{decode_from_slice, encode_to_vec, Decode, Encode};
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use log::warn;
use log::{error, info};
use notan::draw::*;
use notan::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod packets;
mod class;

use packets::IncomingPackets;
use class::Game;

use crate::packets::OutgoingPackets;
use crate::packets::SetInitPacket;
use crate::packets::SpawnPacket;
use crate::packets::UpdatePlayersPacket;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init).add_config(DrawConfig).draw(draw).build()
}

fn init() -> Game {
    let mut game = Game::new();
    init_websocket(&mut game);
    game
}

fn draw(gfx: &mut Graphics, game: &mut Game) {
    game.render.draw(gfx);
}

fn init_websocket(game: &mut Game) {
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
                            //info!("lalala {:?}", data)
                        }

                        IncomingPackets::SetInit(SetInitPacket { is_mine, id, x, y, name }) => {
                            //game.my_player = Some(game.create_player(id, x, y, name));
                            warn!("whaaaa {:?}", data);
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
