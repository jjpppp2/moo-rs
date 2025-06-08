use crate::class::player;

use super::player::Player;
use dashmap::DashMap;
use futures::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;

pub struct Server {
    pub ids: u64,
    pub players: DashMap<u64, WebSocketStream<TcpStream>>,
    tick: u64,
}

impl Server {
    pub fn new() -> Self {
        Self {
            players: DashMap::new(),
            tick: 0,
            ids: 0,
        }
    }

    pub fn add(&mut self, ws: WebSocketStream<TcpStream>) {
        self.ids += 1;
        let player = Player::new(self.ids);
        self.players.insert(self.ids, ws);

        //let (mut read, mut write) = ws.split();
    }

    pub async fn update(&mut self) {
        self.tick += 1;
        println!("tick {}", self.tick);
    }
}

enum ClientMessages {
    Spawn,
}
