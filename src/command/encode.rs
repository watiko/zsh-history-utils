use eyre::Result;
use std::io::{self, Write};
use std::path::Path;

use super::helper;
use crate::zsh::history::HistoryEntry;

pub fn run(path: &Path) -> Result<()> {
    let file = helper::open(path)?;

    let iter = serde_json::Deserializer::from_reader(file).into_iter::<HistoryEntry>();
    for entry in iter {
        io::stdout().write_all(&entry?.to_bytes())?;
    }

    Ok(())
}
