use crate::player_rank_lib::*;
use anyhow::Result;

pub struct PlayerRank<'a> {
    players: &'a Players,
    questions: &'a mut Questions,
    // Curent stage of questioning
    stage: Stage,
    // If we've reached a minimum set of questions to compute a ranking
    minimum_set_reached: bool,
    // Queue of questions ready to be asked
    question_queue: Vec<Question>,
}

#[derive(Debug)]
pub enum QuestionStatus {
    StartingStage(Stage),
    AllMandatoryQuestionsAnswered, // TODO: This is just a connection level of 1. Are there other statuses we'd pass back?
    ConnectionLevelReached(i32),
}

// Question asking is broken into stages, these are them
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Stage {
    Position(Position),
    SelfRating,
    Done,
}

impl Stage {
    // The first stage in the ordering
    fn first() -> Self {
        Stage::Position(Position::Atk)
    }

    // The stages have an ordering. This method defines that ordering
    fn next(&self) -> Self {
        match self {
            Stage::Position(pos) => match pos {
                Position::Atk => Stage::Position(Position::Def),
                Position::Def => Stage::Position(Position::Goalie),
                Position::Goalie => Stage::SelfRating,
            },
            Stage::SelfRating => Stage::Done,
            Stage::Done => Stage::Done, // Stay in Done forever
        }
    }
}

impl<'a> PlayerRank<'a> {
    pub fn new(players: &'a Players, questions: &'a mut Questions) -> Self {
        // TODO: Handle non-empty question files, or at least make this nicer
        if !questions.questions.is_empty() {
            panic!("I cannot handle non-empty question files");
        }

        PlayerRank {
            players,
            questions,
            stage: Stage::first(),
            minimum_set_reached: false,
            question_queue: Vec::new(),
        }
    }

    fn populate_queue(&mut self) {
        let pos = match self.stage{
            Stage::Position(p) => p,
            Stage::SelfRating => Position::Atk,
            Stage::Done => return // Don't populate the queue if we're done
        };
        // Generate questions to go on the queue
        self.question_queue.push(Question {
            player1: String::from("player1"),
            pos1: pos,
            player2: String::from("player2"),
            pos2: pos,
        });

        self.question_queue.push(Question {
            player1: String::from("player3"),
            pos1: pos,
            player2: String::from("player4"),
            pos2: pos,
        });

        self.question_queue.push(Question {
            player1: String::from("player5"),
            pos1: pos,
            player2: String::from("player6"),
            pos2: pos,
        });
    }

    pub fn get_next_question(&mut self) -> (Option<Question>, Option<QuestionStatus>) {
        // Status update for the caller
        let mut status : Option<QuestionStatus> = None;

        // Check if we need to populate the queue
        if self.question_queue.is_empty() {
            // If we're done, we're not starting a new stage, otherwise update 
            // user on the new stage
            status = match self.stage{
                Stage::Done => None,
                _ => Some(QuestionStatus::StartingStage(self.stage))
            };

            // Populate queue based on the stage and min questions asked
            self.populate_queue();
        }

        let res = (self.question_queue.pop(), status);

        if self.question_queue.is_empty() {
            // Move to the next stage
            self.stage = self.stage.next();

            // Move from minimum questions to extra questions
            if self.stage == Stage::Done && !self.minimum_set_reached {
                self.minimum_set_reached = true;
                // Reset the stage for asking extra questions
                self.stage = Stage::first();
            }
        }

        res
    }

    pub fn next_section(&self) -> Result<()> {
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
