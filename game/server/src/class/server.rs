use super::player::Player;
use bincode::Encode;
use dashmap::DashMap;
use futures::{SinkExt, stream::SplitSink};
use rayon::prelude::*;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use tracing::{info, warn};

pub struct Server {
    pub ids: u64,
    pub player_ws: DashMap<u64, SplitSink<WebSocketStream<TcpStream>, Message>>,
    pub players: Vec<Player>,
    tick: u64,
    pub region: String,
    pub instance_id: u64,
    bincode_config: bincode::config::Configuration,
}

impl Server {
    pub fn new() -> Self {
        Self {
            player_ws: DashMap::new(),
            players: Vec::new(),
            tick: 0,
            ids: 0,
            region: String::from("US East"),
            instance_id: rand::random(),
            bincode_config: bincode::config::standard(),
        }
    }

    pub async fn add(&mut self, ws_writer: SplitSink<WebSocketStream<TcpStream>, Message>, name: String) {
        self.ids += 1;

        let player = Player::new(self.ids, name, 0.0, 0.0);
        let id = player.id;

        self.player_ws.insert(self.ids, ws_writer);
        self.players.push(player);

        self.send_to_all(OutgoingPackets::SetID(SetIDPacket { id: id }))
            .await;
    }

    pub fn get_player_by_id(&self, id: &u64) -> Option<&Player> {
        self.players.iter().find(|&x| x.id == *id)
    }

    // TODO: paralellize (is that how it's spelt??) serialization in batches ONLY IF NECESSARY!!
    // do NOT do it otherwise
    pub async fn send_to_client(
        &self,
        packet_type: &OutgoingPackets,
        ws_writer: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    ) {
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

    pub async fn send_to_all(&mut self, packet_type: OutgoingPackets) {
        for mut entry in self.player_ws.iter_mut() {
            let ws_writer = entry.value_mut();
            self.send_to_client(&packet_type, ws_writer).await;
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
                None => {}
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
            data: self.players.clone()
        })).await;
    }
}

// why so many tags rust??!?
// there COULD be a better way to handle this, but i really like this method
#[derive(Serialize, Debug, Encode, Clone)]
pub enum OutgoingPackets {
    AddPlayer(AddPlayerPacket),
    SetID(SetIDPacket),
    RemovePlayer(RemovePlayerPacket),
    UpdatePlayers(UpdatePlayersPacket),
    AddBuilding,
    RemoveBuilding,
    UpdateBuilding,
    AddAnimal,
    RemoveAnimal,
    UpdateAnimals,
}

#[derive(Serialize, Debug, Encode, Clone)]
pub struct SetIDPacket {
    id: u64,
}

#[derive(Serialize, Debug, Encode, Clone)]
struct AddPlayerPacket {
    id: u64,
    name: String,
    x: f32,
    y: f32,
}

#[derive(Serialize, Debug, Encode, Clone)]
struct RemovePlayerPacket {
    id: u64,
}

#[derive(Serialize, Debug, Encode, Clone)]
struct UpdatePlayersPacket {
    data: Vec<Player>
}
