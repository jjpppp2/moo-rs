use serde::Serialize;
use bincode::Encode;

#[derive(Serialize, Debug, Encode, Clone)]
pub enum OutgoingPackets {
    Spawn(SpawnPacket),
}

#[derive(Serialize, Debug, Encode, Clone)]
pub struct SpawnPacket {
    pub name: String
}
