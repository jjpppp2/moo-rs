#[derive(serde::Serialize, Debug, bincode::Encode, Clone)]
pub struct Object {
    pub id: u64,
    x: u32,
    y: u32,
    scale: u32,
    owner: Option<u32>,
}