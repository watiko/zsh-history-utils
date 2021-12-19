use eyre::Result;
use std::path::PathBuf;

pub fn run(path: &PathBuf) -> Result<()> {
    println!("{}", path.to_str().unwrap());

    Ok(())
}
