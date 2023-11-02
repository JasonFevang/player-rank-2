use anyhow::{Context, Result};
use clap::Parser;
use log::trace;
use std::fmt;
use std::fs;
use std::io;

mod player_rank_lib;

// The triple-slash comments can be read by Rust's procedural macros and are used to populate the help message. That's  crazy
/// This command is used to determine relative player rankings through a series of questions comparing two players' abilities. Provide a list of player's names to begin
#[derive(Parser)]
struct Cli {
    /// CSV with a list of players and information about them
    player_file: std::path::PathBuf,
    /// CSV with a list of questions with the provided comparisions. May or may not already exist
    question_file: std::path::PathBuf,
    /// CSV output file with relative rankings for each player
    output_file: std::path::PathBuf,
}

impl fmt::Debug for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cli")
            .field("player_file", &self.player_file)
            .field("question_file", &self.question_file)
            .field("output_file", &self.output_file)
            .finish()
    }
}

fn file_exists(file_path: &std::path::PathBuf) -> Result<()> {
    let metadata = fs::metadata(file_path)
        .with_context(|| format!("Invalid file `{}`", file_path.to_string_lossy()))?;

    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The path is not a file",
        ))
        .with_context(|| format!("Invalid file `{}`", file_path.to_string_lossy()));
    }
    Ok(())
}

fn validate_arguments(args: &Cli) -> Result<()> {
    trace!("Validating arguments");
    file_exists(&args.player_file)?;
    file_exists(&args.question_file)?;
    file_exists(&args.output_file)?;
    Ok(())
}

fn parse_player_file(_player_file: &std::path::PathBuf) -> player_rank_lib::Players {
    let mut players = player_rank_lib::Players::new();
    players.players.push(player_rank_lib::Player {
        name: String::from("Jason"),
        goalie: true,
        avail1: true,
        avail2: true,
    });
    players
}

fn parse_question_file(_question_file: &std::path::PathBuf) -> player_rank_lib::Questions {
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

fn write_question_file(
    _file: &std::path::PathBuf,
    _questions: &player_rank_lib::Questions,
) -> Result<()> {
    Ok(())
}

fn write_rank_file(_file: &std::path::PathBuf, _ranks: &player_rank_lib::Ranks) -> Result<()> {
    Ok(())
}

fn run_ranking() -> Result<player_rank_lib::Ranks> {
    Ok(player_rank_lib::Ranks::new())
}

fn main() -> Result<()> {
    env_logger::init();

    trace!("Parsing arguments");
    let args = Cli::parse();

    validate_arguments(&args)?;
    println!("{:?}", args);

    // Convert files into their respective structs
    let players = parse_player_file(&args.player_file);
    let questions = parse_question_file(&args.player_file);

    // Print parsed players
    for player in &players.players {
        println!("{:?}", player);
    }

    let ranks = run_ranking()?;

    // Write the outputs back to file
    write_question_file(&args.question_file, &questions)?;
    write_rank_file(&args.output_file, &ranks)?;
    Ok(())
}
