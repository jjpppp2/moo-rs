// TODO: make everything local, instead of doing in server struct
// easier to manage in future
#[derive(serde::Serialize, Debug, bincode::Encode, Clone)]
pub struct Player {
    pub id: u64,
    name: String,
    pub x: f32,
    pub y: f32,
    pub move_dir: Option<f32>,
    pub x_vel: f32,
    pub y_vel: f32,
    pub x_accel: f32,
    pub y_accel: f32,
    pub lock_movement: bool
}

impl Player {
    pub fn new(id: u64, name: String, x: f32, y: f32) -> Self {
        Player {
            id: id,
            name: name,
            x: x,
            y: y,
            move_dir: None,
            x_vel: 0.0,
            y_vel: 0.0,
            x_accel: 0.0,
            y_accel: 0.0,
            lock_movement: false
        }
    }
}
