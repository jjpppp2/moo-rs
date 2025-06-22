// TODO: make everything local, instead of doing in server struct
// easier to manage in future
#[derive(serde::Serialize, Debug, bincode::Encode, Clone)]
pub struct Player {
    pub id: u64,
    name: String,
    pub x: f32,
    pub y: f32,
    pub last_x: f32,
    pub last_y: f32,
    pub lerp_x: f32,
    pub lerp_y: f32,
    pub last_lerp_x: f32,
    pub last_lerp_y: f32,
    pub delta: f32,
    pub time_1: f32,
    pub time_2: f32,
}

impl Player {
    pub fn new(id: u64, name: String, x: f32, y: f32) -> Self {
        Player {
            id: id,
            name: name,
            x: x,
            y: y,
            last_x: x,
            last_y: y,
            lerp_x: x,
            lerp_y: y,
            last_lerp_x: x,
            last_lerp_y: y,
            delta: 0.0,
            time_1: 0.0,
            time_2: 0.0
        }
    }
}
