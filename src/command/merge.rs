use eyre::Result;
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::zsh::history::{HistoryEntry, HistoryLines};

pub fn run(paths: &[PathBuf]) -> Result<()> {
    let mut entries_map: BTreeMap<u64, Vec<HistoryEntry>> = BTreeMap::new();

    for path in paths {
        let file = std::fs::File::open(path)?;
        for line in HistoryLines::new(file) {
            let entry = HistoryEntry::parse(&line?)?;
            let key = entry.start_time;
            if let Some(entries) = entries_map.get_mut(&key) {
                entries.push(entry);
                continue;
            }
            entries_map.insert(key, vec![entry]);
        }
    }

    for entries in entries_map.values() {
        for entry in entries {
            io::stdout().write_all(&entry.to_bytes())?;
        }
    }

    Ok(())
}
