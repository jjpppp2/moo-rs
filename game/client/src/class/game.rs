use notan::prelude::*;

use crate::class::player::Player;
use crate::class::render::RenderUtil;

#[derive(AppState)]
pub struct Game {
    pub my_player: Option<Player>,
    pub all_players: Vec<Player>,

    pub render: RenderUtil,
}

impl Game {
    pub fn new() -> Self {
        Game {
            my_player: None,
            all_players: Vec::new(),
            render: RenderUtil::new(),
        }
    }

    pub fn create_player(&mut self, id: u64, x: f32, y: f32, name: String) -> Player {
        Player::new(id, name, x, y)
    }
}
