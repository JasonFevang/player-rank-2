use clap::Parser;
use std::fmt;

// The triple-slash comments can be read by Rust's procedural macros and are used to populate the help message. That's  crazy
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    player_file: std::path::PathBuf,
    question_file: std::path::PathBuf,
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

fn main() {
    let _args = Cli::parse();
    println!("{:?}", _args)
}
