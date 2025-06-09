pub struct Player {
    id: u64,
    name: String,
    x: u32,
    y: u32,
}

impl Player {
    pub fn new(id: u64, name: String, x: u32, y: u32) -> Self {
        Player {
            id: id,
            name: name,
            x: x,
            y: y,
        }
    }

    
}
