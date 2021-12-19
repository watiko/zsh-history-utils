use std::path::PathBuf;

pub use clap::Parser;
use clap::{AppSettings, Subcommand};

#[derive(Parser)]
#[clap(about = "manipulate the history file of zsh")]
#[clap(version)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
#[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert a history file to a JSON Lines
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Decode {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },

    /// Convert a JSON Lines to a history file
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Encode {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },

    /// Merge multiple history files into a single history file
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Merge {
        #[clap(required = true, parse(from_os_str))]
        path: Vec<PathBuf>,
    },
}
