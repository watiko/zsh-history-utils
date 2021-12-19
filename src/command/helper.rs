use eyre::Result;
use std::{fs, io, path::Path};

pub fn open(path: &Path) -> Result<Box<dyn io::Read>> {
    let file: Box<dyn io::Read> = if path.as_os_str() == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(path)?)
    };

    Ok(file)
}
