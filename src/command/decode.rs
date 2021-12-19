use eyre::Result;
use std::path::Path;

use super::helper;
use crate::zsh::history::{HistoryEntry, HistoryLines};

pub fn run(path: &Path) -> Result<()> {
    let file = helper::open(path)?;

    for line in HistoryLines::new(file) {
        let entry = HistoryEntry::parse(&line?)?;
        println!("{}", serde_json::to_string(&entry)?);
    }

    Ok(())
}
