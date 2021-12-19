use eyre::Result;

mod cli;

use crate::cli::{Args, Commands, Parser};

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::Decode { path } => {
            println!("{}", path.to_str().unwrap());
        }
        Commands::Encode { path } => {
            println!("{}", path.to_str().unwrap())
        }
        Commands::Merge { path } => {
            let paths = path.iter().map(|p| p.to_str().unwrap()).collect::<Vec<_>>();
            for path in paths {
                println!("{}", path);
            }
        }
    }

    Ok(())
}
