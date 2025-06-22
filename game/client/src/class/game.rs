use std::collections::HashMap;

use notan::prelude::*;

use crate::class::player::Player;
use crate::class::render::RenderUtil;
use log::error;

#[derive(AppState)]
pub struct Game {
    pub my_player: Option<Player>,
    pub my_player_id: Option<u64>,
    pub all_players: Vec<Player>,
    pub keys: HashMap<KeyCode, bool>,
    pub move_dir: Option<f32>,

    pub render: RenderUtil,
}

impl Game {
    pub fn new() -> Self {
        Game {
            my_player: None,
            my_player_id: None,
            all_players: Vec::new(),
            render: RenderUtil::new(),
            keys: HashMap::new(),
            move_dir: None,
        }
    }

    pub fn get_player_by_id(&mut self, id: u64) -> Option<&mut Player> {
        self.all_players.iter_mut().find(|x| x.id == id)
    }

    pub fn create_player(&mut self, is_mine: bool, id: u64, x: f32, y: f32, name: String) {
        let player = Player::new(id, name, x, y);

        if is_mine {
            self.my_player = Some(player.clone());
            self.my_player_id = Some(id);
        }

        self.all_players.push(player);
        error!("{:?}", self.all_players);
    }
}
