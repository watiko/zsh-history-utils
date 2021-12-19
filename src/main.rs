use eyre::Result;

mod cli;
mod command;
mod zsh;

use crate::cli::{Args, Commands, Parser};

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::Decode { path } => {
            command::decode::run(path)?;
        }
        Commands::Encode { path } => {
            command::encode::run(path)?;
        }
        Commands::Merge { path } => {
            command::merge::run(path)?;
        }
    }

    Ok(())
}
