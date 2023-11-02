use anyhow::{Context, Result};
use csv;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io;

use crate::player_rank_lib;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
struct ParsedPlayer {
    name: String,
    goalie: bool,
    week1: bool,
    week2: bool,
}

pub fn parse_player_file(player_file: &std::path::PathBuf) -> Result<player_rank_lib::Players> {
    let mut players = player_rank_lib::Players::new();

    let file = File::open(player_file)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        // Must provide a type hint for automatic deserialization.
        let player: ParsedPlayer = result?;

        // Add the parsed player to the list of players
        players.players.push(player_rank_lib::Player {
            name: (player.name),
            goalie: (player.goalie),
            avail1: (player.week1),
            avail2: (player.week2),
        });
    }
    Ok(players)
}

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize, Serialize)]
struct ParsedQuestion {
    player1: String,
    player1_pos: String,
    player2: String,
    player2_pos: String,
    skill_factor: f64,
}

impl player_rank_lib::Position {
    // Try to create a position from a string
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Atk" => Some(player_rank_lib::Position::Atk),
            "Def" => Some(player_rank_lib::Position::Def),
            "Goalie" => Some(player_rank_lib::Position::Goalie),
            _ => None, // Handle unrecognized strings
        }
    }

    fn to_str(&self) -> String {
        match self {
            player_rank_lib::Position::Atk => String::from("Atk"),
            player_rank_lib::Position::Def => String::from("Def"),
            player_rank_lib::Position::Goalie => String::from("Goalie"),
        }
    }
}

// Convert a string to a position enum, handling errors
fn string_to_position(pos_string: &str) -> Result<player_rank_lib::Position> {
    match player_rank_lib::Position::from_str(pos_string) {
        Some(pos) => Ok(pos),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot parse questions",
            ))
            .with_context(|| format!("Unknown position `{}`", pos_string))
        }
    }
}

pub fn parse_question_file(
    question_file: &std::path::PathBuf,
) -> Result<player_rank_lib::Questions> {
    let mut questions = player_rank_lib::Questions::new();

    let file = File::open(question_file)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        // Must provide a type hint for automatic deserialization.
        let parsed_question: ParsedQuestion = result?;

        // Add the parsed player to the list of players
        questions.questions.push(player_rank_lib::AnsweredQuestion {
            question: player_rank_lib::Question {
                player1: parsed_question.player1,
                pos1: string_to_position(&parsed_question.player1_pos)?,
                player2: parsed_question.player2,
                pos2: string_to_position(&parsed_question.player2_pos)?,
            },
            response: parsed_question.skill_factor,
        });
    }
    Ok(questions)
}

pub fn write_question_file(
    question_file: &std::path::PathBuf,
    questions: &player_rank_lib::Questions,
) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(false)
        .open(question_file)?;
    let mut wtr = csv::Writer::from_writer(file);

    for question in &questions.questions {
        // Convert from a player_rank_lib question struct to one I can serialize
        let parsed_question = ParsedQuestion {
            player1: question.question.player1.clone(),
            player1_pos: question.question.pos1.to_str(),
            player2: question.question.player2.clone(),
            player2_pos: question.question.pos2.to_str(),
            skill_factor: question.response.clone(),
        };
        wtr.serialize(parsed_question)?;
    }
    wtr.flush()?;
    Ok(())
}

#[derive(Serialize)]
struct ParsedRank {
    pub name: String,
    pub atk: f64,
    pub def: f64,
    pub goalie: Option<f64>,
}

pub fn write_rank_file(
    rank_file: &std::path::PathBuf,
    ranks: &player_rank_lib::Ranks,
) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(false)
        .open(rank_file)?;
    let mut wtr = csv::Writer::from_writer(file);

    for rank in &ranks.ranks {
        // Convert from a player_rank_lib rank struct to one I can serialize
        let parsed_rank = ParsedRank {
            name: rank.name.clone(),
            atk: rank.atk,
            def: rank.def,
            goalie: rank.goalie,
        };
        wtr.serialize(parsed_rank)?;
    }
    wtr.flush()?;
    Ok(())
}
