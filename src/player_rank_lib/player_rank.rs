use crate::player_rank_lib::*;
use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct PlayerRank<'a> {
    players: &'a Players,
    questions: &'a mut Questions,
    // Curent stage of questioning
    stage: Stage,
    // If we've reached a minimum set of questions to compute a ranking
    minimum_set_reached: bool,
    // Queue of questions ready to be asked
    min_set_question_queue: Vec<RefQuestion<'a>>,
    current_question: Option<RefQuestion<'a>>,
    skipped_questions: HashMap<Stage, Vec<RefQuestion<'a>>>,
    answered_questions: HashMap<Stage, Vec<RefQuestion<'a>>>,
    minimum_linkage: HashMap<Stage, usize>,
}

#[derive(PartialEq, Copy, Clone)]
struct RefQuestion<'a> {
    pub player1: &'a Player,
    pub pos1: Position,
    pub player2: &'a Player,
    pub pos2: Position,
}

// Add method to create a Question from a RefQuestion
impl Question {
    fn from_opt_refq(q: &Option<RefQuestion>) -> Option<Self> {
        if let Some(q) = q {
            Some(Question {
                player1: q.player1.name.clone(),
                pos1: q.pos1,
                player2: q.player2.name.clone(),
                pos2: q.pos2,
            })
        } else {
            None
        }
    }

    fn from_refq(q: &RefQuestion) -> Self {
        Question {
            player1: q.player1.name.clone(),
            pos1: q.pos1,
            player2: q.player2.name.clone(),
            pos2: q.pos2,
        }
    }
}

#[derive(Debug)]
pub enum QuestionStatus {
    StartingStage(Stage),
    // Need to pass the stage back because this coincides with starting a new stage regretably
    AllMandatoryQuestionsAnswered(Stage), // TODO: This is just a connection level of 1. Are there other statuses we'd pass back?
    AllQuestionsSkipped,
    ConnectionLevelReached(usize),
}

// Question asking is broken into stages, these are them
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
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
            min_set_question_queue: Vec::new(),
            current_question: None,
            skipped_questions: HashMap::new(),
            answered_questions: HashMap::new(),
            minimum_linkage: HashMap::new(),
        };

        // Populate queue based on the stage and min questions asked
        res.populate_min_set_queue();

        res
    }

    fn get_shuffled_player_list(&self) -> Vec<&'a Player> {
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
    fn min_set_populate_position(&mut self, pos: Position) {
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
            let question = RefQuestion {
                player1: pair.0,
                pos1: pos,
                player2: pair.1,
                pos2: pos,
            };
            temp_questions.push(question);
        }

        self.min_set_question_queue.extend(temp_questions);
    }

    fn min_set_populate_self(&mut self) {
        let player_list = self.get_shuffled_player_list();

        assert!(player_list.len() > 3);

        let mut temp_questions = Vec::new(); // Temporary vector to collect questions

        // Attack-Defense
        temp_questions.push(RefQuestion {
            player1: player_list[0],
            pos1: Position::Atk,
            player2: player_list[0],
            pos2: Position::Def,
        });

        // Attack-Goalie
        temp_questions.push(RefQuestion {
            player1: player_list[0],
            pos1: Position::Atk,
            player2: player_list[0],
            pos2: Position::Goalie,
        });

        self.min_set_question_queue.extend(temp_questions);
    }

    fn populate_min_set_queue(&mut self) {
        // Populate with minimum set for the stage
        match self.stage {
            Stage::Position(pos) => self.min_set_populate_position(pos),
            Stage::SelfRating => self.min_set_populate_self(),
            Stage::Done => { /* Nothing to populate */ }
        }
    }

    fn get_skip_replacement_position(&self) -> Option<RefQuestion<'a>> {
        let pos = match self.stage {
            Stage::Position(pos) => pos,
            _ => return None,
        };

        let curr_q = if let Some(curr_q) = self.current_question {
            curr_q
        } else {
            return None;
        };

        // Find all numbers connected to each number in the skipped question
        let mut lhs: Vec<&Player> = vec![curr_q.player1];
        let mut rhs: Vec<&Player> = vec![curr_q.player2];

        // Create a list of all the asked and about-to-be-asked questions, not including the skipped question
        let mut all_questions = self.min_set_question_queue.clone(); // Only ever contain questions for a single stage
        if self.answered_questions.contains_key(&self.stage) {
            all_questions.extend(self.answered_questions[&self.stage].clone());
        }

        // Iterate through all the questions
        // If one of them connects to a known number, add it to the list and start looking for another
        while !all_questions.is_empty() {
            for i in (0..all_questions.len()).rev() {
                let pair = all_questions[i];
                let mut found = false;
                if lhs.contains(&pair.player1) {
                    lhs.push(pair.player2);
                    found = true;
                } else if lhs.contains(&pair.player2) {
                    lhs.push(pair.player1);
                    found = true;
                } else if rhs.contains(&pair.player1) {
                    rhs.push(pair.player2);
                    found = true;
                } else if rhs.contains(&pair.player2) {
                    rhs.push(pair.player1);
                    found = true;
                }
                if found {
                    all_questions.remove(i);
                    break;
                }
            }
        }

        // Generate list of all potential replacement questions, not including the skipped ones
        let mut potential_replacements: Vec<RefQuestion> = Vec::new();
        for left in &lhs {
            for right in &rhs {
                let potential_question = RefQuestion {
                    player1: *left,
                    pos1: pos,
                    player2: *right,
                    pos2: pos,
                };
                let potential_question_rev = RefQuestion {
                    player1: *right,
                    pos1: pos,
                    player2: *left,
                    pos2: pos,
                };
                if self.skipped_questions.contains_key(&self.stage)
                    && !self.skipped_questions[&self.stage].contains(&potential_question)
                    && !self.skipped_questions[&self.stage].contains(&potential_question_rev)
                {
                    potential_replacements.push(potential_question);
                }
            }
        }

        // If all the options are skipped, return an error
        if potential_replacements.is_empty() {
            return None;
        }

        // Choose a random one and add it to the upcoming list
        let rand = rand::random::<usize>() % potential_replacements.len();
        Some(potential_replacements[rand])
    }

    fn get_skip_replacement_self_rating(&self) -> Option<RefQuestion<'a>> {
        // Make sure we're in the right stage
        if self.stage != Stage::SelfRating {
            return None;
        }

        // Make sure there's a question being skipped
        let curr_q = if let Some(curr_q) = self.current_question {
            curr_q
        } else {
            return None;
        };

        // Shuffle up the players
        let player_list = self.get_shuffled_player_list();

        // There's one possible replacement question for each player.
        // Try each player, make sure it hasn't been skipped already
        for player in player_list {
            // The potential replacement question for the skipped question
            let potential_question = RefQuestion {
                player1: player,
                pos1: curr_q.pos1,
                player2: player,
                pos2: curr_q.pos2,
            };

            // If we've already answered this question
            if self.answered_questions.contains_key(&Stage::SelfRating)
                && self.answered_questions[&Stage::SelfRating].contains(&potential_question)
            {
                continue;
            }

            // If we've already skipped this question
            if self.skipped_questions.contains_key(&Stage::SelfRating)
                && self.skipped_questions[&Stage::SelfRating].contains(&potential_question)
            {
                // Keep looking for another question
                continue;
            }
            return Some(potential_question);
        }
        None
    }

    // When a question gets skipped, find a question to replace it while maintaining the requirements of a fully connected graph
    fn get_skip_replacement(&self) -> Option<RefQuestion<'a>> {
        match self.stage {
            Stage::Position(_) => self.get_skip_replacement_position(),
            Stage::SelfRating => self.get_skip_replacement_self_rating(),
            Stage::Done => None,
        }
    }

    // For a given question, determine how many other answered questions reference each player in the question
    fn count_connections(&self, question: &RefQuestion) -> usize {
        let mut left_cnt = 0;
        let mut right_cnt = 0;

        if !self.answered_questions.contains_key(&self.stage) {
            return 0;
        }

        for answered in &self.answered_questions[&self.stage] {
            if answered.player1 == question.player1 || answered.player2 == question.player1 {
                left_cnt += 1;
            }
            if answered.player1 == question.player2 || answered.player2 == question.player2 {
                right_cnt += 1;
            }
        }
        left_cnt + right_cnt
    }

    fn list_remaining_questions_position(&self) -> Vec<RefQuestion<'a>> {
        let pos = match self.stage {
            Stage::Position(pos) => pos,
            _ => return Vec::new(),
        };

        let mut remaining_questions: Vec<RefQuestion> = Vec::new();
        for p1 in 0..self.players.players.len() {
            for p2 in (p1 + 1)..self.players.players.len() {
                let question = RefQuestion {
                    player1: &self.players.players[p1],
                    pos1: pos,
                    player2: &self.players.players[p2],
                    pos2: pos,
                };
                let question_rev = RefQuestion {
                    player1: &self.players.players[p2],
                    pos1: pos,
                    player2: &self.players.players[p1],
                    pos2: pos,
                };

                // If we've already answered this question, ignore it
                if self.answered_questions.contains_key(&self.stage)
                    && (self.answered_questions[&self.stage].contains(&question)
                        || self.answered_questions[&self.stage].contains(&question_rev))
                {
                    continue;
                }

                // If we've already skipped this question, ignore it
                if self.skipped_questions.contains_key(&self.stage)
                    && (!self.skipped_questions[&self.stage].contains(&question)
                        || self.skipped_questions[&self.stage].contains(&question_rev))
                {
                    continue;
                }

                remaining_questions.push(question);
            }
        }

        // Shuffle up the remaing questions
        remaining_questions.shuffle(&mut thread_rng());
        remaining_questions
    }

    fn list_remaining_questions_self_rating(&self) -> Vec<RefQuestion<'a>> {
        let player_list = self.get_shuffled_player_list();

        // Go through all the possible questions, and filter out the ones we've answered or skipped already
        let mut remaining_questions = Vec::new();
        for player in player_list {
            let potential_questions: Vec<RefQuestion> = vec![
                RefQuestion {
                    player1: player,
                    pos1: Position::Atk,
                    player2: player,
                    pos2: Position::Def,
                },
                RefQuestion {
                    player1: player,
                    pos1: Position::Atk,
                    player2: player,
                    pos2: Position::Goalie,
                },
                RefQuestion {
                    player1: player,
                    pos1: Position::Def,
                    player2: player,
                    pos2: Position::Goalie,
                },
            ];

            for pot_q in potential_questions {
                let pot_q_rev = RefQuestion {
                    player1: pot_q.player1,
                    pos1: pot_q.pos2,
                    player2: pot_q.player2,
                    pos2: pot_q.pos1,
                };
                // If we've already answered this question, ignore it
                if self.answered_questions.contains_key(&self.stage)
                    && (self.answered_questions[&self.stage].contains(&pot_q)
                        || self.answered_questions[&self.stage].contains(&pot_q_rev))
                {
                    continue;
                }

                // If we've already skipped this question, ignore it
                if self.skipped_questions.contains_key(&self.stage)
                    && (!self.skipped_questions[&self.stage].contains(&pot_q)
                        || self.skipped_questions[&self.stage].contains(&pot_q_rev))
                {
                    continue;
                }
                remaining_questions.push(pot_q);
            }
        }

        // Shuffle up all the questions
        remaining_questions.shuffle(&mut thread_rng());

        remaining_questions
    }

    fn get_regular_question(&mut self) -> (Option<RefQuestion<'a>>, Option<QuestionStatus>) {
        // List all remaining questions
        let remaining_questions = match self.stage {
            Stage::Position(_) => self.list_remaining_questions_position(),
            Stage::SelfRating => self.list_remaining_questions_self_rating(),
            Stage::Done => Vec::new(),
        };

        // Find minimum linked question in the list
        let mut min_links = self.players.players.len();
        let mut min_question: Option<RefQuestion> = None;
        for question in remaining_questions {
            let pair_links = self.count_connections(&question) / 2;
            if pair_links < min_links {
                min_links = pair_links;
                min_question = Some(question);
            }
        }

        // Update user on how connected the graph is
        // This is sorta like a confidence measure
        let status = {
            if !self.minimum_linkage.contains_key(&self.stage) {
                self.minimum_linkage.insert(self.stage, 0);
            }

            let min_linkage = self.minimum_linkage[&self.stage];

            if min_linkage != min_links {
                // Update the minimum linkage
                self.minimum_linkage.insert(self.stage, min_links);
                // Notify the user we've reached a new connection level
                Some(QuestionStatus::ConnectionLevelReached(min_linkage))
            } else {
                None
            }
        };

        (min_question, status)
    }

    fn get_min_set_question(&mut self) -> (Option<RefQuestion<'a>>, Option<QuestionStatus>) {
        let mut status = None;

        if self.min_set_question_queue.is_empty() {
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

                // Now give the first regular question
                let regular_question = self.get_regular_question();
                if let Some(ignored_status) = regular_question.1 {
                    println!("Warning, ignored status message {:?}", ignored_status);
                }
                return (regular_question.0, status);
            }

            // Populate queue based on the stage and min questions asked
            self.populate_min_set_queue();
        }

        (self.min_set_question_queue.pop(), status)
    }

    pub fn get_next_question(&mut self) -> (Option<Question>, Option<QuestionStatus>) {
        // If there's still a current question, then it's being skipped, perform skipping logic
        if let Some(current_question) = self.current_question {
            // Add the current question to the skipped questions list
            match self.skipped_questions.entry(self.stage) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(current_question);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![current_question]);
                }
            }

            // Only perform skip replacement if we're determining the minimum set
            if !self.minimum_set_reached {
                // Figure out the replacement
                let replacement = self.get_skip_replacement();

                if let Some(replacement) = replacement {
                    self.min_set_question_queue.push(replacement);
                } else {
                    // Report error status to user, no questions left
                    return (None, Some(QuestionStatus::AllQuestionsSkipped));
                }
            }

            self.current_question = None;
        }

        // Status update for the caller
        let status;

        // Get the next question
        (self.current_question, status) = match self.minimum_set_reached {
            true => self.get_regular_question(),
            false => self.get_min_set_question(),
        };

        // Return the current question to the user
        (Question::from_opt_refq(&self.current_question), status)
    }

    pub fn next_section(&mut self) -> Result<(), NextSectionError> {
        if self.minimum_set_reached {
            if self.stage == Stage::Done {
                Err(NextSectionError::AllQuestionsAsked)
            } else {
                // Empty the question queue. The next time a question is requested, it'll move to
                // the next section to refill the queue
                self.min_set_question_queue.clear();
                Ok(())
            }
        } else {
            Err(NextSectionError::MinSetNotReached)
        }
    }

    pub fn give_response(&mut self, response: f64) -> Result<(), ResponseError> {
        if let Some(question) = &self.current_question {
            // Check that the response is allowed
            if !response.is_finite() || response.is_sign_negative() {
                Err(ResponseError::InvalidResponse)
            } else {
                // Add to our list of answered questions
                // Add the current question to the skipped questions list
                match self.answered_questions.entry(self.stage) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(question.clone());
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![question.clone()]);
                    }
                }

                // Also add to the user-facing list of answered questions
                self.questions.questions.push(AnsweredQuestion {
                    question: Question::from_refq(question),
                    response,
                });

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
