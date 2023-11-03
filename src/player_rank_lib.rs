use anyhow::Result;
use std::fmt;

//// Structures ////
pub struct Player {
    pub name: String,
    pub goalie: bool,
    pub avail1: bool,
    pub avail2: bool,
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Player")
            .field("name", &self.name)
            .field("goalie", &self.goalie)
            .field("avail1", &self.avail1)
            .field("avail2", &self.avail2)
            .finish()
    }
}

/// Player Rank Interface: Input, player list
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

#[derive(Debug)]
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

#[derive(Debug)]
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

/// Player Rank Interface: Output, rank list
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

//// Methods ////

pub struct PlayerRank<'a> {
    players: &'a Players,
    questions: &'a mut Questions,
}

#[derive(Debug)]
pub enum QuestionStatus {
    AllMandatoryQuestionsAnswered, // TODO: This is just a connection level of 1. Are there other statuses we'd pass back?
    ConnectionLevelReached(i32),
}

impl<'a> PlayerRank<'a> {
    pub fn new(players: &'a Players, questions: &'a mut Questions) -> Self {
        // TODO: Handle non-empty question files, or at least make this nicer
        if !questions.questions.is_empty() {
            panic!("I cannot handle non-empty question files");
        }
        PlayerRank { players, questions }
    }

    pub fn get_next_question(&self) -> (Option<Question>, Option<QuestionStatus>) {
        // Return a dummy question, and status is nothing for now
        (
            Some(Question {
                player1: String::from("player1"),
                pos1: Position::Atk,
                player2: String::from("player2"),
                pos2: Position::Atk,
            }),
            None,
        )
    }

    pub fn next_section(&self) -> Result<()>{
        Ok(())
    }

    pub fn give_response(&self, _response: f64) -> Result<()> {
        Ok(())
    }

    pub fn get_ranking(&self) -> Result<Ranks> {
        let mut ranks = Ranks::new();
        // Add some test ranks for now
        ranks.ranks.push(Rank {
            name: String::from("player1 Name"),
            atk: 1.2,
            def: 1.5,
            goalie: Some(2.2),
        });
        ranks.ranks.push(Rank {
            name: String::from("player2 Name"),
            atk: 0.8,
            def: 1.24,
            goalie: None,
        });
        Ok(ranks)
    }
}
