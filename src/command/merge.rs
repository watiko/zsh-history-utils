use eyre::Result;
use std::path::PathBuf;

pub fn run(paths: &Vec<PathBuf>) -> Result<()> {
    let paths = paths
        .iter()
        .map(|p| p.to_str().unwrap())
        .collect::<Vec<_>>();
    for path in paths {
        println!("{}", path);
    }

    Ok(())
}
