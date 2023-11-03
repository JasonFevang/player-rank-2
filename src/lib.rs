use anyhow::{Context, Result};
use clap::Parser;
use log::trace;
use std::fmt;
use std::fs;
use std::io;

mod cli_file_io;
mod player_rank_lib;

// The triple-slash comments can be read by Rust's procedural macros and are used to populate the help message. That's  crazy
/// This command is used to determine relative player rankings through a series of questions comparing two players' abilities. Provide a list of player's names to begin
#[derive(Parser)]
pub struct Cli {
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
enum UserResponse {
    Value(f64),
    Skip,
    Quit,
}

fn ask_question(question: &player_rank_lib::Question) -> Result<UserResponse> {
    // Ask the question
    println!(
        "{} {} vs {} {}",
        question.player1,
        question.pos1.to_str(),
        question.player2,
        question.pos2.to_str()
    );

    // Get a response
    loop {
        let mut input = String::new();

        // Read a line from the user
        io::stdin()
            .read_line(&mut input)
            .with_context(|| format!("Failed to read user input"))?;
        let input = input.trim();

        // Check for specific commands
        match input {
            "s" => return Ok(UserResponse::Skip),
            "q" => return Ok(UserResponse::Quit),
            _ => {}
        }

        // Check for rating
        if let Ok(val) = input.parse::<f64>() {
            return Ok(UserResponse::Value(val));
        };
    }
}

fn run_ranking(player_rank: player_rank_lib::PlayerRank) -> Result<player_rank_lib::Ranks> {
    loop {
        // Get a question from player_rank
        let (question, status) = player_rank.get_next_question();
        if let Some(status) = status {
            // Report status to user, or otherwise act on it
            println!("Question status: {:?}", status);
        }

        if let Some(question) = question {
            let response = ask_question(&question)?;
            match response {
                UserResponse::Value(value) => player_rank.give_response(value)?,
                UserResponse::Skip => { /* Do nothing*/ }
                UserResponse::Quit => break,
            };
        } else {
            // No question available, we've run out of questions
            println!("All possible combinations have been asked or skipped, you're done! :)");
            break;
        }
    }
    player_rank.get_ranking()
}

pub fn run(args: Cli) -> Result<()> {
    validate_arguments(&args)?;
    println!("{:?}", args);
    // Convert files into their respective structs
    let players = cli_file_io::parse_player_file(&args.player_file)?;
    let mut questions = cli_file_io::parse_question_file(&args.question_file)?;

    // Print parsed players
    for player in &players.players {
        println!("{:?}", player);
    }
    // Print parsed players
    for question in &questions.questions {
        println!("{:?}", question);
    }

    let player_rank = player_rank_lib::PlayerRank::new(&players, &mut questions);

    let ranks = run_ranking(player_rank)?;

    // Write the outputs back to file
    cli_file_io::write_question_file(&args.question_file, &questions)?;
    cli_file_io::write_rank_file(&args.output_file, &ranks)?;
    Ok(())
}
