use std::ffi::NulError;


#[derive(Debug)]
pub struct Player {
    pub id: u64,
    name: String,
    x: u32,
    y: u32,
    moveDir: Option<i8>,
}

impl Player {
    pub fn new(id: u64, name: String, x: u32, y: u32) -> Self {
        Player {
            id: id,
            name: name,
            x: x,
            y: y,
            moveDir: None
        }
    }
}
