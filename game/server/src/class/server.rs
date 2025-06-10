use std::ops::Deref;

use crate::class::player;

use super::player::Player;
use dashmap::DashMap;
use futures::{SinkExt, stream::SplitSink};
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

pub struct Server {
    pub ids: u64,
    pub playerWS: DashMap<u64, SplitSink<WebSocketStream<TcpStream>, Message>>,
    pub players: Vec<Player>,
    tick: u64,
    pub region: String,
    pub instance_id: u64,
}

impl Server {
    pub fn new() -> Self {
        Self {
            playerWS: DashMap::new(),
            players: Vec::new(),
            tick: 0,
            ids: 0,
            region: String::from("US East"),
            instance_id: rand::random(),
        }
    }

    pub async fn add(&mut self, mut ws_writer: SplitSink<WebSocketStream<TcpStream>, Message>) {
        self.ids += 1;

        let player = Player::new(self.ids, String::from("awds"), 0, 0);

        let _ = ws_writer.send(Message::Text("sano".into())).await;

        self.playerWS.insert(self.ids, ws_writer);
        self.players.push(player);
    }

    /*
    pub async fn init_game_loop(&mut self) {
        loop {
            self.update().await;
            sleep(Duration::from_millis(100)).await;
        }
    }*/

    pub fn get_player_by_id(&self, id: &u64) -> Option<&Player> {
        self.players.iter().find(|&x| x.id == *id)
    }

    pub async fn update(&mut self) {
        //self.tick += 1;
        //println!("{} tick. players {}, id: {}", self.tick, self.playerWS.len(), self.instance_id);

        let ids: Vec<u64> = self.playerWS.iter().map(|e| *e.key()).collect();
        for id in ids {
            let player = self.get_player_by_id(&id);

            if let Some(mut ws_writer) = self.playerWS.get_mut(&id) {
                match player {
                    Some(player) => {
                        let _ = ws_writer.send(Message::Text("sndaodsiosdan".into())).await;
                    }
                    None => {}
                }
            }
        }
    }
}

enum OutgoingPackets {
    AddPlayer,
    RemovePlayer,
    UpdatePlayers,
    AddBuilding,
    RemoveBuilding,
    UpdateBuilding,
    AddAnimal,
    RemoveAnimal,
    UpdateAnimals,
}
