use anyhow::anyhow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse_file(file: &Path) -> Result<Vec<String>, anyhow::Error> {
    let file = File::open(file).map_err(|e| anyhow!("Failed to open file {file:?}: {e}"))?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;

    Ok(lines)
}
