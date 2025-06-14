use serde::Deserialize;
use bincode::Decode;

#[derive(Deserialize, Decode)]
#[serde(tag = "type", content = "data")]
pub enum IncomingPackets {
    Spawn(SpawnPacket),
    Move(MovePacket),
    Aim,
    Hit,
    Place,
}

#[derive(Deserialize, Decode)]
pub struct SpawnPacket {
    pub name: String,
}

#[derive(Deserialize, Decode)]
pub struct MovePacket {
    pub direction: i8,
}

#[derive(Deserialize, Decode)]
pub struct AimPacket {
    pub direction: i8,
}

#[derive(Deserialize, Decode)]
pub struct HitPacket {}

#[derive(Deserialize)]
pub struct PlacePacket {
    pub item: u8,
}
