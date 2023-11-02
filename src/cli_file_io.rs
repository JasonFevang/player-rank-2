use anyhow::Result;
use std::fs::File;
use csv;
use serde::Deserialize;

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

pub fn parse_question_file(_question_file: &std::path::PathBuf) -> player_rank_lib::Questions {
    let mut questions = player_rank_lib::Questions::new();

    // Add a dummy question
    questions.questions.push(player_rank_lib::AnsweredQuestion {
        question: player_rank_lib::Question {
            player1: String::from("Jason"),
            pos1: player_rank_lib::Position::Atk,
            player2: String::from("Max"),
            pos2: player_rank_lib::Position::Def,
        },
        response: 1.2,
    });
    questions
}

pub fn write_question_file(
    _file: &std::path::PathBuf,
    _questions: &player_rank_lib::Questions,
) -> Result<()> {
    Ok(())
}

pub fn write_rank_file(_file: &std::path::PathBuf, _ranks: &player_rank_lib::Ranks) -> Result<()> {
    Ok(())
}
