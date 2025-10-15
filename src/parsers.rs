use crate::types::IntCell;
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

pub fn single_input_line<'a>(lines: &[&'a str]) -> Result<&'a str, anyhow::Error> {
    match lines {
        [] => Err(anyhow!("No lines in input")),
        [line] => Ok(line),
        lines => Err(anyhow!("Too many lines: {lines:?}")),
    }
}

pub fn parse_intmachine_input(lines: &[&str]) -> Result<Vec<IntCell>, anyhow::Error> {
    match lines {
        [] => Err(anyhow::anyhow!("No input lines")),
        [line] => line
            .split(',')
            .map(|value| value.parse::<IntCell>().map_err(|e| e.into()))
            .collect(),
        [..] => Err(anyhow::anyhow!("Too many input lines")),
    }
}
