use serde::Serialize;
use bincode::Encode;

#[derive(Serialize, Debug, Encode, Clone)]
pub enum OutgoingPackets {
    Spawn(SpawnPacket),
    SetMoveDir(MovePacket)
}

#[derive(Serialize, Debug, Encode, Clone)]
pub struct SpawnPacket {
    pub name: String
}

#[derive(Serialize, Debug, Encode, Clone)]
pub struct MovePacket {
    pub dir: Option<f32>
}
