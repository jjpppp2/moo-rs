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
        /*self.send_to_all_except(
            id,
            OutgoingPackets::SetInit(SetInitPacket {
                is_mine: false,
                id,
                x,
                y,
                name: name.clone(),
            }),
        )
        .await;*/

        // send to the specific client the other data
        self.send_to_client(&OutgoingPackets::SetInit(SetInitPacket { is_mine: true, id, x, y, name: name.clone() }), id).await;

        //self.send_to_all(OutgoingPackets::SetInit(SetInitPacket { id: id, x, y, name }))
        //.await;
    }

    pub fn get_player_by_id(&self, id: &u64) -> Option<&Player> {
        self.players.iter().find(|&x| x.id == *id)
    }

    // TODO: paralellize (is that how it's spelt??) serialization in batches ONLY IF NECESSARY!!
    // do NOT do it otherwise
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

    pub async fn send_to_all(&mut self, packet_type: OutgoingPackets) {
        for entry in self.player_ws.iter_mut() {
            self.send_to_client(&packet_type, *entry.key()).await;
        }
    }

    pub async fn send_to_all_except(&mut self, id: u64, packet_type: OutgoingPackets) {
        for entry in self.player_ws.iter_mut() {
            let player_id = entry.key();
            if *player_id == id {
                continue;
            }

            self.send_to_client(&packet_type, id).await;
        }
    }

    pub async fn update(&mut self) {
        self.tick += 1;
        println!(
            "{} tick. players {}, id: {}",
            self.tick,
            self.player_ws.len(),
            self.instance_id
        );

        // paralellizing for performance, there will soon be INSANELY heavily logic in here
        // preparing for doomsday
        self.players.par_iter_mut().for_each(|player| {
            match player.move_dir {
                None => {
                    // decel player if they arent moving
                    player.x_vel -= player.x_vel * 0.8;
                    player.y_vel -= player.y_vel * 0.8;

                    // clip player to 0 vel, so we dont have to give a fuck about teeny tiny values
                    _ = player.x_vel.clamp(0.1, 300.0);
                    _ = player.y_vel.clamp(0.1, 300.0);

                    player.x = player.x_vel;
                    player.y = player.y_vel;
                }
                Some(move_dir) => {
                    let base_movement_speed = 1000.0 / 12.0; // 83.33333

                    // set acceleration values to velocity values temporarily
                    player.x_accel = player.x_vel;
                    player.y_accel = player.y_vel;

                    // TODO: OPTIMISE!!!
                    let mut x_vel_temp = move_dir.cos();
                    let mut y_vel_temp = move_dir.sin();
                    let magnitude = (x_vel_temp * x_vel_temp + y_vel_temp * y_vel_temp).sqrt();
                    if magnitude != 0.0 {
                        x_vel_temp /= magnitude;
                        y_vel_temp /= magnitude;
                    }

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

        self.send_to_all(OutgoingPackets::UpdatePlayers(UpdatePlayersPacket {
            data: self.players.clone(),
        }))
        .await;
    }
}
