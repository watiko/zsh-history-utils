use std::path::PathBuf;

pub use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[clap(about = "manipulate the history file of zsh")]
#[clap(version)]
#[clap(propagate_version = true)]
#[clap(subcommand_required = true, arg_required_else_help = true)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert a history file to a JSON Lines
    #[clap(arg_required_else_help = true)]
    Decode {
        #[clap(required = true, value_parser)]
        path: PathBuf,
    },

    /// Convert a JSON Lines to a history file
    #[clap(arg_required_else_help = true)]
    Encode {
        #[clap(required = true, value_parser)]
        path: PathBuf,
    },

    /// Merge multiple history files into a single history file
    #[clap(arg_required_else_help = true)]
    Merge {
        #[clap(required = true, value_parser)]
        path: Vec<PathBuf>,
    },
}
