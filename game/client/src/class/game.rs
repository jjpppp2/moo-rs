use super::render::RenderUtil;
use gloo_net::websocket::{Message, futures::WebSocket};

pub struct Game {
    render: RenderUtil,
    ws: WebSocket,
}

impl Game {
    
}