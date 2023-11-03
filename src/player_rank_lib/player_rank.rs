use crate::player_rank_lib::*;
use anyhow::Result;

pub struct PlayerRank<'a> {
    players: &'a Players,
    questions: &'a mut Questions,
}

#[derive(Debug)]
pub enum QuestionStatus {
    AllMandatoryQuestionsAnswered, // TODO: This is just a connection level of 1. Are there other statuses we'd pass back?
    ConnectionLevelReached(i32),
}

// Question asking is broken into stages, these are them
enum Stage {
    position(Position),
    self_rating
}

impl Stage{
    // The first stage in the ordering
    fn first() -> Self{
        Stage::position(Position::Atk)
    }

    // The stages has an ordering. This method defines that ordering
    // None implies the ordering has been reached
    fn next(&self) -> Option<Self>{
        match self{
            Stage::position(pos) => {
                match pos{
                    Position::Atk => Some(Stage::position(Position::Def)),
                    Position::Def => Some(Stage::position(Position::Goalie)),
                    Position::Goalie => Some(Stage::self_rating)
                }
            },
            Stage::self_rating => None
        }
    }
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