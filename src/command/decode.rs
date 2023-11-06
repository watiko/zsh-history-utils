use eyre::Result;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use super::helper;
use crate::zsh::history::{HistoryEntry, HistoryLines};

pub fn run(path: &Path) -> Result<()> {
    let file = helper::open(path)?;

    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    for line in HistoryLines::new(file) {
        let entry = HistoryEntry::parse(&line?)?;
        writeln!(stdout, "{}", serde_json::to_string(&entry)?)?;
    }

    Ok(())
}
