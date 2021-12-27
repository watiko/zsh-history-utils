use eyre::Result;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use super::helper;
use crate::zsh::history::{HistoryEntry, HistoryLines};

pub fn run(path: &Path) -> Result<()> {
    let file = helper::open(path)?;

    let mut stdout = BufWriter::new(io::stdout());
    for line in HistoryLines::new(file) {
        let entry = HistoryEntry::parse(&line?)?;
        writeln!(&mut stdout, "{}", serde_json::to_string(&entry)?)?;
    }

    Ok(())
}
