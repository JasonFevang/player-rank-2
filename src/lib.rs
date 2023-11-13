use anyhow::{Context, Result, anyhow};
use clap::Parser;
use log::trace;
use std::fmt;
use std::fs;
use std::io;

mod cli_file_io;
pub mod player_rank_lib;

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
    NextSection,
    Quit,
}

fn ask_question(question: &player_rank_lib::Question) {
    println!(
        "{} {} vs {} {}",
        question.player1,
        question.pos1.to_str(),
        question.player2,
        question.pos2.to_str()
    );
}

fn get_response() -> Result<UserResponse> {
    return Ok(UserResponse::Value(1.0));
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
            "n" => return Ok(UserResponse::NextSection),
            "q" => return Ok(UserResponse::Quit),
            _ => {}
        }

        // Check for rating
        if let Ok(val) = input.parse::<f64>() {
            return Ok(UserResponse::Value(val));
        };
    }
}

fn run_ranking(player_rank: &mut player_rank_lib::PlayerRank) -> Result<player_rank_lib::Ranks> {
    'ranking_loop: loop {
        // Get a question from player_rank
        let (question, status) = player_rank.get_next_question();
        if let Some(status) = status {
            // Report status to user, or otherwise act on it
            println!("Question status: {:?}", status);
        }

        if let Some(question) = question {
            ask_question(&question);

            // Get a valid response
            let mut get_another_response = true;
            while get_another_response{
                get_another_response = false; // Assume we won't need to get another response

                let response = get_response()?;
                match response {
                    UserResponse::Value(value) => {
                        if let Err(err) = player_rank.give_response(value){
                            match err{
                                player_rank_lib::ResponseError::InvalidResponse => {
                                    println!("Invalid response");
                                    get_another_response = true;
                                }
                                player_rank_lib::ResponseError::NoActiveQuestion => {
                                    return Err(anyhow!("Internal logic error: Gave a response with no active question"));
                                }
                            }
                        }
                    }
                    UserResponse::Skip => { /* Do nothing*/ }
                    UserResponse::NextSection => {
                        if let Err(err) = player_rank.next_section() {
                            // If we can't skip sections, get another response
                            get_another_response = true;
                            match  err {
                                player_rank_lib::NextSectionError::MinSetNotReached => println!("Can't skip sections until the minimum question set has been reached"),
                                player_rank_lib::NextSectionError::AllQuestionsAsked => println!("All questions have been asked! You gotta quit now"),
                            }
                        }
                    }
                    UserResponse::Quit => break 'ranking_loop,
                };
            }
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

    // Create a PlayerRank object that handles figuring out what questions to ask and creating the ranking
    let mut player_rank = player_rank_lib::PlayerRank::new(&players, &mut questions, None);

    // Run the routine of asking the user questions and parsing responses
    let ranks = run_ranking(&mut player_rank)?;

    // Write the outputs back to file
    cli_file_io::write_question_file(&args.question_file, &questions)?;
    cli_file_io::write_rank_file(&args.output_file, &ranks)?;
    Ok(())
}
