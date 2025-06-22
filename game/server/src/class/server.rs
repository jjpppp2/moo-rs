use std::ptr::NonNull;

use super::player::Player;
use dashmap::DashMap;
use futures::{SinkExt, stream::SplitSink};
use quadtree_f32::{Point, QuadTree};
use rand::Rng;
use rayon::prelude::*;
use serial_int::SerialGenerator;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, warn};

use crate::packets::OutgoingPackets;
use crate::packets::SetInitPacket;
use crate::packets::UpdatePlayersPacket;

pub struct Server {
    pub ids: SerialGenerator<u64>,
    pub player_ws: DashMap<u64, SplitSink<WebSocketStream<TcpStream>, Message>>,
    pub players: Vec<Player>,
    tick: u64,
    pub region: String,
    pub instance_id: u64,
    quadtree: QuadTree,
    quadtree_id: SerialGenerator<u32>,
    bincode_config: bincode::config::Configuration,
}

impl Server {
    pub fn new() -> Self {
        Self {
            player_ws: DashMap::new(),
            players: Vec::new(),
            tick: 0,
            ids: SerialGenerator::<u64>::new(),
            region: String::from("US East"),
            instance_id: rand::random(),
            bincode_config: bincode::config::standard(),
            quadtree: QuadTree::new(),
            quadtree_id: SerialGenerator::<u32>::new(),
        }
    }

    pub async fn add(
        &mut self,
        ws_writer: SplitSink<WebSocketStream<TcpStream>, Message>,
        name: String,
    ) {
        info!("whaaaa wat da fuck man");

        let id = self.ids.generate();
        let (x, y) = {
            let mut rng = rand::rng();
            (
                rng.random_range(0.0..=14400.0),
                (rng.random_range(0.0..=14400.0)),
            )
        };

        let player = Player::new(id, name.clone(), x, y);

        self.player_ws.insert(id, ws_writer);
        self.players.push(player);

        // send to everyone else the player data
        self.send_to_all_except(
            id,
            OutgoingPackets::SetInit(SetInitPacket {
                is_mine: false,
                id,
                x,
                y,
                name: name.clone(),
            }),
        )
        .await;

        // send to the specific client the other data
        self.send_to_client(
            &OutgoingPackets::SetInit(SetInitPacket {
                is_mine: true,
                id,
                x,
                y,
                name: name.clone(),
            }),
            id,
        )
        .await;

        //self.send_to_all(OutgoingPackets::SetInit(SetInitPacket { id: id, x, y, name }))
        //.await;
        //id
    }

    pub fn get_player_by_id(&mut self, id: &u64) -> Option<&mut Player> {
        self.players.iter_mut().find(|x| x.id == *id)
    }

    // TODO: paralellize (is that how it's spelt??) serialization in batches ONLY IF NECESSARY!!
    // do NOT do it otherwise
    /*
    pub async fn send_to_client(
        &self,
        packet_type: &OutgoingPackets,
        id: u64, //ws_writer: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    ) {
        if let Some(mut ws_writer) = self.player_ws.get_mut(&id) {
            let encoded = match bincode::encode_to_vec(&packet_type, self.bincode_config) {
                Ok(v) => v,
                Err(err) => {
                    warn!(
                        "Error serializing data of type {:?}: {:?}",
                        packet_type, err
                    );
                    return;
                }
            };

            match ws_writer.send(encoded.into()).await {
                Ok(v) => v,
                Err(err) => {
                    warn!("Error sending serialized content to client: {:?}", err);
                }
            }
        }
    }
    */

    // i find this weird, taking out the writer and inserting it back in
    // surely that can't be efficient, right?
    pub async fn send_to_client(&self, packet_type: &OutgoingPackets, id: u64) {
        let encoded = match bincode::encode_to_vec(&packet_type, self.bincode_config) {
            Ok(v) => v,
            Err(err) => {
                warn!(
                    "Error serializing data of type {:?}: {:?}",
                    packet_type, err
                );
                return;
            }
        };

        let mut ws_writer = match self.player_ws.remove(&id) {
            Some((_, writer)) => writer,
            None => {
                println!("no ws for id");
                return;
            }
        };

        match ws_writer.send(encoded.into()).await {
            Ok(_) => {
                self.player_ws.insert(id, ws_writer);
                warn!("tried to send data to client uhhh");
            }
            Err(err) => {
                warn!("Error sending serialized content to client: {:?}", err);
                self.player_ws.insert(id, ws_writer);
            }
        }
    }

    pub async fn send_to_all(&mut self, packet_type: OutgoingPackets) {
        /*for entry in self.player_ws.iter_mut() {
            println!("{}", *entry.key());
            self.send_to_client(&packet_type, *entry.key()).await;
        }*/

        let ids: Vec<u64> = self.player_ws.iter().map(|x| *x.key()).collect();
        for id in ids {
            self.send_to_client(&packet_type, id).await;
        }
    }

    pub async fn send_to_all_except(&mut self, id: u64, packet_type: OutgoingPackets) {
        let ids: Vec<u64> = self
            .player_ws
            .iter()
            .filter(|x| *x.key() != id)
            .map(|x| *x.key())
            .collect();
        for id in ids {
            self.send_to_client(&packet_type, id).await;
        }
    }

    pub async fn update(&mut self) {
        self.tick += 1;
        /*println!(
            "{} tick. players {}, id: {}",
            self.tick,
            self.player_ws.len(),
            self.instance_id
        );*/

        /*
        self.players.par_iter_mut().for_each(|player| {
            match player.move_dir {
                None => {
                    let old_x_vel = player.x_vel;
                    let old_y_vel = player.y_vel;

                    // decel player if they arent moving
                    player.x_vel -= player.x_vel * 0.8;
                    player.y_vel -= player.y_vel * 0.8;

                    // clip player to 0 vel, so we dont have to give a fuck about teeny tiny values
                    if player.x_vel.abs() < 0.1 {
                        player.x_vel = 0.0;
                    }

                    if player.y_vel.abs() < 0.1 {
                        player.y_vel = 0.0;
                    }

                    player.x = player.x_vel;
                    player.y = player.y_vel;

                    player.x_accel = player.x_vel - old_x_vel;
                    player.y_accel = player.y_vel - old_y_vel;
                }
                Some(move_dir) => {
                    let base_movement_speed = 20.0; // 83.33333

                    // set acceleration values to velocity values temporarily
                    player.x_accel = player.x_vel;
                    player.y_accel = player.y_vel;

                    // TODO: OPTIMISE!!!
                    let mut x_vel_temp = move_dir.cos();
                    let mut y_vel_temp = move_dir.sin();

                    if x_vel_temp != 0.0 {
                        player.x_vel += x_vel_temp * base_movement_speed;
                    }

                    if y_vel_temp != 0.0 {
                        player.y_vel += y_vel_temp * base_movement_speed;
                    }

                    player.x = player.x_vel;
                    player.y = player.y_vel;

                    // now we can come back and define accel values, though not much use local
                    player.x_accel = player.x_vel - player.x_accel;
                    player.y_accel = player.y_vel - player.y_accel;
                }
            }
        });
        */

        self.players.par_iter_mut().for_each(|player| {
            match player.move_dir {
                None => {
                    // apply deceleration
                    player.x_vel -= player.x_vel * 0.6;
                    player.y_vel -= player.y_vel * 0.6;
                    //player.x_vel *= 0.9;
                    //player.y_vel *= 0.9;
                }

                Some(dir) => {
                    let base_spd = 55.0;

                    let x_d = dir.cos();
                    let y_d = dir.sin();

                    if x_d != 0.0 {
                        player.x_vel = base_spd * x_d;
                    }
                    if y_d != 0.0 {
                        player.y_vel = base_spd * y_d;
                    }
                }
            }

            player.x += player.x_vel;
            player.y += player.y_vel;

            player.x = player.x.clamp(0.0 + 35.0, 14400.0 - 35.0);
            player.y = player.y.clamp(0.0 + 35.0, 14400.0 - 35.0);
        });

        self.send_to_all(OutgoingPackets::UpdatePlayers(UpdatePlayersPacket {
            data: self
                .players
                .iter()
                .map(|x| (x.id, x.x, x.y))
                .collect::<Vec<_>>()
                .clone(),
        }))
        .await;
    }
}
