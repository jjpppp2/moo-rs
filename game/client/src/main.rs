use bincode::decode_from_slice;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use log::{error, info};
use notan::draw::*;
use notan::prelude::*;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::u64;
use wasm_bindgen_futures::spawn_local;
use std::time::Instant;

mod class;
mod packets;

use class::Game;
use packets::IncomingPackets;

use crate::packets::SetInitPacket;
use crate::packets::SpawnPacket;
use crate::packets::UpdatePlayersPacket;
use crate::packets::{MovePacket, OutgoingPackets};

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(DrawConfig)
        .add_config(
            WindowConfig::new()
                .set_maximized(true)
                .set_resizable(true)
                .set_high_dpi(true),
        )
        .update(update)
        .draw(draw)
        .build()
}

pub struct Apps(Arc<Mutex<Game>>);
impl AppState for Apps {}

fn init(app: &mut App) -> Apps {
    let game = Arc::new(Mutex::new(Game::new()));
    init_websocket(app, game.clone());
    Apps(game)
}

fn update(app: &mut App, game: &mut Apps) {
    let mut lock = game.0.lock();
    let game = lock.as_mut().unwrap();

    let mut t_x = 0.0;
    let mut t_y = 0.0;
    let directions: HashMap<KeyCode, (f32, f32)> = [
        (KeyCode::W, (0.0, -1.0)),
        (KeyCode::A, (-1.0, 0.0)),
        (KeyCode::S, (0.0, 1.0)),
        (KeyCode::D, (1.0, 0.0)),
        (KeyCode::Up, (0.0, -1.0)),
        (KeyCode::Left, (-1.0, 0.0)),
        (KeyCode::Down, (0.0, 1.0)),
        (KeyCode::Right, (1.0, 0.0)),
    ]
    .into_iter()
    .collect();

    for (key, &(dx, dy)) in directions.iter() {
        if app.keyboard.is_down(*key) {
            t_x += dx;
            t_y += dy;
        }
    }

    if t_x != 0.0 || t_y != 0.0 {
        let angle = t_y.atan2(t_x);
        game.move_dir = Some((angle * 100.0).round() / 100.0); // round to 2 decimal places
    } else {
        game.move_dir = None;
    }

    drop(lock);
}

fn draw(app: &mut App, gfx: &mut Graphics, game: &mut Apps) {
    let mut lock = game.0.lock();
    let game2 = lock.as_mut().unwrap();

    let players = game2.all_players.clone();
    let my_id = game2.my_player_id.unwrap_or(u64::MIN);
    game2.render.draw(app, gfx, my_id, players);

    drop(lock);
}

fn init_websocket(app: &mut App, game: Arc<Mutex<Game>>) {
    let ws = WebSocket::open("ws://localhost:8089"); //WebSocket::open("wss://grim-maude-jjpppp2-ca6ee7f1.koyeb.app/");
    let ws = match ws {
        Ok(ws) => ws,
        Err(err) => {
            error!("WebSocket failed to open: {:?}", err);
            return;
        }
    };

    let (mut write, mut read) = ws.split();
    let bincode_default_config = bincode::config::standard();
    let (mut websocket_write_tx, mut websocket_write_rx) = futures::channel::mpsc::channel(1024);

    spawn_local(async move {
        while let Some(msg) = websocket_write_rx.next().await {
            match msg {
                Message::Bytes(data) => {
                    if let Err(err) = write.send(Message::Bytes(data)).await {
                        error!("error sending to server: {:?}", err);
                        continue;
                    }
                    if let Err(err) = write.flush().await {
                        error!("error flushing writer: {:?}", err);
                    }
                }

                _ => {
                    error!("unsupported message type");
                }
            }
        }

        //info!("sender loop closed");
    });

    {
        let mut websocket_write_tx = websocket_write_tx.clone();
        spawn_local(async move {
            let encoded = match bincode::encode_to_vec(
                //OutgoingPackets::SetMoveDir(MovePacket { dir: Some(3.14) }),
                OutgoingPackets::Spawn(SpawnPacket {
                    name: String::from("test"),
                }),
                bincode_default_config,
            ) {
                Ok(v) => {
                    info!("encoded bytes: {:?}", v);
                    v
                }
                Err(err) => {
                    error!("error serializing data {:?}", err);
                    return;
                }
            };

            if let Err(err) = websocket_write_tx.try_send(Message::Bytes(encoded)) {
                error!("error queuing serialized content: {:?}", err);
            }
        });
    }

    {
        let mut websocket_write_tx = websocket_write_tx.clone();
        spawn_local(async move {
            let encoded = match bincode::encode_to_vec(
                OutgoingPackets::SetMoveDir(MovePacket { dir: Some(3.14) }),
                bincode_default_config,
            ) {
                Ok(v) => v,
                Err(err) => {
                    error!("error serializing data {:?}", err);
                    return;
                }
            };

            if let Err(err) = websocket_write_tx.try_send(Message::Bytes(encoded)) {
                error!("error queuing serialized content: {:?}", err);
            }
        });
    }

    //5203-5601-2744-2124 11/29 352

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

                    let mut lock = game.lock();
                    let game: &mut std::sync::MutexGuard<'_, Game> = lock.as_mut().unwrap();
                    match packet {
                        IncomingPackets::SetInit(SetInitPacket {
                            is_mine,
                            id,
                            x,
                            y,
                            name,
                        }) => {
                            info!("{}, {}", x, y);
                            game.create_player(is_mine, id, x, y, name);
                        }

                        IncomingPackets::UpdatePlayers(UpdatePlayersPacket { data }) => {
                            for (id, x, y) in data {
                                //let my_id = game.my_player_id.unwrap();
                                let player = game.get_player_by_id(id);
                                match player {
                                    Some(player) => {
                                        info!("{:?}", player);
                                        player.last_x = player.x;
                                        player.last_y = player.y;
                                        player.x = x;
                                        player.y = y;
                                        player.time_1 = player.time_2;
                                        player.last_lerp_x = player.lerp_x;
                                        player.last_lerp_y = player.lerp_y;

                                        //player.time_1 = timtime(&mut app)
                                    }

                                    None => {}
                                }
                            }

                            {
                                let dir = game.move_dir;
                                let mut websocket_write_tx = websocket_write_tx.clone();
                                spawn_local(async move {
                                    let encoded = match bincode::encode_to_vec(
                                        OutgoingPackets::SetMoveDir(MovePacket { dir }),
                                        bincode_default_config,
                                    ) {
                                        Ok(v) => {
                                            info!("encoded bytes: {:?}", v);
                                            v
                                        }
                                        Err(err) => {
                                            error!("error serializing data {:?}", err);
                                            return;
                                        }
                                    };

                                    if let Err(err) =
                                        websocket_write_tx.try_send(Message::Bytes(encoded))
                                    {
                                        error!("error queuing serialized content: {:?}", err);
                                    }
                                });
                            }
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        //drop(lock);
        info!("WebSocket connection closed");
    });
}

fn timtime(app: &mut App) -> f32 {
    app.timer.elapsed_f32()
}

/*
fn send_msg(mut writer: SplitSink<WebSocket, Message>, packet_type: OutgoingPackets) {
    spawn_local(async move {
        let encoded = match bincode::encode_to_vec(&packet_type, bincode::config::standard()) {
            Ok(v) => v,
            Err(err) => {
                error!("Error serializing data {:?}", err);
                return;
            }
        };


        match writer.send(Message::Bytes(encoded)).await {
            Ok(_) => (),
            Err(err) => {
                error!("Error sending serialized content to client: {:?}", err);
            }
        }
    });
}
    */
