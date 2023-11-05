#[derive(Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub goalie: bool,
    pub avail1: bool,
    pub avail2: bool,
}

pub struct Players {
    pub players: Vec<Player>,
}

impl Players {
    pub fn new() -> Self {
        Players {
            players: Vec::new(),
        }
    }
}
