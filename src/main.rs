use anyhow::Result;
use clap::Parser;
use log::trace;

fn main() -> Result<()> {
    env_logger::init();

    trace!("Parsing arguments");
    let args = player_rank_cli::Cli::parse();

    player_rank_cli::run(args)?;

    Ok(())
}
