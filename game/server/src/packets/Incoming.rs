use serde::Deserialize;
use bincode::Decode;

#[derive(Deserialize, Decode, Debug)]
#[serde(tag = "type", content = "data")]
pub enum IncomingPackets {
    Spawn(SpawnPacket),
    SetMoveDir(MovePacket),
    Aim,
    Hit,
    Place,
}

#[derive(Deserialize, Decode, Debug)]
pub struct SpawnPacket {
    pub name: String,
}

#[derive(Deserialize, Debug, Decode, Clone)]
pub struct MovePacket {
    pub dir: Option<f32>
}

#[derive(Deserialize, Decode, Debug)]
pub struct AimPacket {
    pub direction: i8,
}

#[derive(Deserialize, Decode, Debug)]
pub struct HitPacket {}

#[derive(Deserialize, Decode, Debug)]
pub struct PlacePacket {
    pub item: u8,
}
