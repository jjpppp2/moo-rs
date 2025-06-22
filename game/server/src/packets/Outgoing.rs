use serde::Serialize;
use bincode::Encode;
use crate::class::Player;

// why so many tags rust??!?
// there COULD be a better way to handle this, but i really like this method
#[derive(Serialize, Debug, Encode, Clone)]
pub enum OutgoingPackets {
    AddPlayer(AddPlayerPacket),
    SetInit(SetInitPacket),
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
pub struct SetInitPacket {
    pub is_mine: bool,
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub name: String
}

#[derive(Serialize, Debug, Encode, Clone)]
pub struct AddPlayerPacket {
    pub id: u64,
    pub name: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Debug, Encode, Clone)]
pub struct RemovePlayerPacket {
    pub id: u64,
}

#[derive(Serialize, Debug, Encode, Clone)]
pub struct UpdatePlayersPacket {
    pub data: Vec<(u64, f32, f32)>
}
