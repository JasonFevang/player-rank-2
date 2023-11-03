#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Position {
    Atk,
    Def,
    Goalie,
}

impl Position {
    // Try to create a position from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Atk" => Some(Position::Atk),
            "Def" => Some(Position::Def),
            "Goalie" => Some(Position::Goalie),
            _ => None, // Handle unrecognized strings
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Position::Atk => String::from("Atk"),
            Position::Def => String::from("Def"),
            Position::Goalie => String::from("Goalie"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Question {
    pub player1: String,
    pub pos1: Position,
    pub player2: String,
    pub pos2: Position,
}

#[derive(Debug)]
pub struct AnsweredQuestion {
    pub question: Question,
    pub response: f64,
}

/// Player Rank Interface: Input/output, questions list
pub struct Questions {
    pub questions: Vec<AnsweredQuestion>,
}

impl Questions {
    pub fn new() -> Self {
        Questions {
            questions: Vec::new(),
        }
    }
}
