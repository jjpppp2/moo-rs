use crate::class::player;

use super::player::Player;
use dashmap::DashMap;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio::time::{sleep, Duration};
use futures::stream::SplitSink;
use tokio_tungstenite::tungstenite::Message;

pub struct Server {
    pub ids: u64,
    pub playerWS: DashMap<u64, SplitSink<WebSocketStream<TcpStream>, Message>>,
    pub players: Vec<Player>,
    tick: u64,
    pub region: String,
    pub instance_id: u64
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

    pub fn add(&mut self, ws_writer: SplitSink<WebSocketStream<TcpStream>, Message>) {
        self.ids += 1;

        let player = Player::new(self.ids, String::from("BISMILLAH"), 0, 0);

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

    pub async fn update(&mut self) {
        self.tick += 1;
        println!("{} tick. players {}, id: {}", self.tick, self.playerWS.len(), self.instance_id);
    }
}

enum IncomingPackets {
    Spawn,
    Move,
    Aim,
    Hit,
    Place,
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