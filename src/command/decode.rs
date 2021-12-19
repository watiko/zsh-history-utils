use eyre::Result;
use std::path::Path;

use crate::zsh::history::{HistoryEntry, HistoryLines};

pub fn run(path: &Path) -> Result<()> {
    let file = std::fs::File::open(path)?;

    for line in HistoryLines::new(file) {
        let entry = HistoryEntry::parse(&line?)?;
        println!("{}", serde_json::to_string(&entry)?);
    }

    Ok(())
}
