use crate::player_rank_lib::*;
use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct PlayerRank<'a> {
    players: &'a Players,
    questions: &'a mut Questions,
    // Curent stage of questioning
    stage: Stage,
    // If we've reached a minimum set of questions to compute a ranking
    minimum_set_reached: bool,
    // Queue of questions ready to be asked
    question_queue: Vec<Question>,
    current_question: Option<Question>,
}

#[derive(Debug)]
pub enum QuestionStatus {
    StartingStage(Stage),
    // Need to pass the stage back because this coincides with starting a new stage regretably
    AllMandatoryQuestionsAnswered(Stage), // TODO: This is just a connection level of 1. Are there other statuses we'd pass back?
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

pub enum NextSectionError {
    MinSetNotReached,
    AllQuestionsAsked,
}

pub enum ResponseError {
    NoActiveQuestion,
    InvalidResponse,
}

impl<'a> PlayerRank<'a> {
    pub fn new(players: &'a Players, questions: &'a mut Questions) -> Self {
        // TODO: Handle non-empty question files, or at least make this nicer
        if !questions.questions.is_empty() {
            panic!("I cannot handle non-empty question files");
        }

        let mut res = PlayerRank {
            players,
            questions,
            stage: Stage::first(),
            minimum_set_reached: false,
            question_queue: Vec::new(),
            current_question: None,
        };

        // Populate queue based on the stage and min questions asked
        res.populate_queue();

        res
    }

    fn get_shuffled_player_list(&self) -> Vec<&Player> {
        // Create shuffled list of all players
        let mut player_list: Vec<&Player> = Vec::new();
        for player in &self.players.players {
            player_list.push(&player);
        }
        let mut rng = thread_rng();
        player_list.shuffle(&mut rng);
        player_list
    }

    // Generate a randomized, minimum set of questions to fully define the vector space(idk if that means anything but it sounds sick lmao)
    fn min_set_populate(&mut self, pos: Position) {
        let player_list = self.get_shuffled_player_list();

        // Create pairs from this shuffled list
        let mut pairs: Vec<(&Player, &Player)> = Vec::new();
        for i in 0..(player_list.len() - 1) {
            pairs.push((player_list[i], player_list[i + 1]));
        }

        // Shuffle those pairs
        let mut rng = thread_rng();
        pairs.shuffle(&mut rng);

        let mut temp_questions = Vec::new(); // Temporary vector to collect questions

        // Convert the player pairs into references
        for pair in pairs {
            let question = Question {
                player1: pair.0.name.clone(),
                pos1: pos,
                player2: pair.1.name.clone(),
                pos2: pos,
            };
            temp_questions.push(question);
        }

        self.question_queue.extend(temp_questions);
    }

    fn min_set_populate_self(&mut self) {
        let player_list = self.get_shuffled_player_list();

        assert!(player_list.len() > 3);

        let mut temp_questions = Vec::new(); // Temporary vector to collect questions

        // Attack-Defense
        temp_questions.push(Question {
            player1: player_list[0].name.clone(),
            pos1: Position::Atk,
            player2: player_list[0].name.clone(),
            pos2: Position::Def,
        });

        // Attack-Goalie
        temp_questions.push(Question {
            player1: player_list[0].name.clone(),
            pos1: Position::Atk,
            player2: player_list[0].name.clone(),
            pos2: Position::Goalie,
        });

        self.question_queue.extend(temp_questions);
    }

    fn populate_queue(&mut self) {
        if !self.minimum_set_reached {
            // Populate with minimum set for the stage
            match self.stage {
                Stage::Position(pos) => self.min_set_populate(pos),
                Stage::SelfRating => self.min_set_populate_self(),
                Stage::Done => { /* Nothing to populate */ }
            }
        } else {
            // TODO: Determine all cominbations and shuffle 'em up
            let pos = match self.stage {
                Stage::Position(p) => p,
                Stage::SelfRating => Position::Atk,
                Stage::Done => return, // Don't populate the queue if we're done
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
    }

    pub fn get_next_question(&mut self) -> (Option<Question>, Option<QuestionStatus>) {
        // Status update for the caller
        let mut status: Option<QuestionStatus> = None;

        // Check if we need to populate the queue
        if self.question_queue.is_empty() {
            // Move to the next stage
            self.stage = self.stage.next();

            // If we're done, we're not starting a new stage, otherwise update
            // user on the new stage
            status = match self.stage {
                Stage::Done => None,
                _ => Some(QuestionStatus::StartingStage(self.stage)),
            };

            // Move from minimum questions to extra questions
            if self.stage == Stage::Done && !self.minimum_set_reached {
                self.minimum_set_reached = true;
                // Reset the stage for asking extra questions
                self.stage = Stage::first();
                // Inform user we're done min question set
                status = Some(QuestionStatus::AllMandatoryQuestionsAnswered(self.stage));
            }

            // Populate queue based on the stage and min questions asked
            self.populate_queue();
        }

        // Set the current question
        self.current_question = self.question_queue.pop();

        // Return the current question to the user
        (self.current_question.clone(), status)
    }

    pub fn next_section(&mut self) -> Result<(), NextSectionError> {
        if self.minimum_set_reached {
            if self.stage == Stage::Done {
                Err(NextSectionError::AllQuestionsAsked)
            } else {
                // Empty the question queue. The next time a question is requested, it'll move to
                // the next section to refill the queue
                self.question_queue.clear();
                Ok(())
            }
        } else {
            Err(NextSectionError::MinSetNotReached)
        }
    }

    pub fn give_response(&mut self, response: f64) -> Result<(), ResponseError> {
        if let Some(question) = self.current_question.clone() {
            // Check that the response is allowed
            if !response.is_finite() || response.is_sign_negative() {
                Err(ResponseError::InvalidResponse)
            } else {
                // Add to our list of answered questions
                self.questions
                    .questions
                    .push(AnsweredQuestion { question, response });

                // Clear the current question
                self.current_question = None;

                Ok(())
            }
        } else {
            Err(ResponseError::NoActiveQuestion)
        }
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
