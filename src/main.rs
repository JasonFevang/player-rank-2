use clap::Parser;
use std::fmt;
use std::fs;
use std::io;

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

fn file_exists(file_path: &std::path::PathBuf) -> io::Result<()> {
    let metadata = fs::metadata(file_path)?;
    if !metadata.is_file(){
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The path is not a file",
        ));
    }
    Ok(())
}

fn validate_arguments(args: &Cli) -> io::Result<()> {
    file_exists(&args.player_file)?;
    file_exists(&args.question_file)?;
    file_exists(&args.output_file)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments
    let args = Cli::parse();
    println!("{:?}", args);

    validate_arguments(&args)?;
    Ok(())
}
