pub struct Rank {
    pub name: String,
    pub atk: f64,
    pub def: f64,
    pub goalie: Option<f64>,
}

pub struct Ranks {
    pub ranks: Vec<Rank>,
}

impl Ranks {
    pub fn new() -> Self {
        Ranks { ranks: Vec::new() }
    }
}
