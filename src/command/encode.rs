use eyre::Result;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

use super::helper;
use crate::zsh::history::HistoryEntry;

pub fn run(path: &Path) -> Result<()> {
    let file = BufReader::new(helper::open(path)?);

    let iter = serde_json::Deserializer::from_reader(file).into_iter::<HistoryEntry>();
    let mut stdout = BufWriter::new(io::stdout());
    for entry in iter {
        stdout.write_all(&entry?.to_bytes())?;
    }

    Ok(())
}
