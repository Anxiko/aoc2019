use aoc2019::day::DayPart;
use serde::Deserialize;
use serde_json;
use std::fmt::Display;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum Value {
    OneLine(String),
    ManyLines(Vec<String>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::OneLine(line) => write!(f, "{line}"),
            Value::ManyLines(lines) => {
                let joined = lines.join("\n");
                write!(f, "{joined}")
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ScenarioOutput {
    part1: Option<Value>,
    part2: Option<Value>,
}

impl ScenarioOutput {
    pub(crate) fn get_part(&self, part: DayPart) -> Option<&Value> {
        match part {
            DayPart::Part1 => self.part1.as_ref(),
            DayPart::Part2 => self.part2.as_ref(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ExpectedOutput {
    real: Option<ScenarioOutput>,
    example: Option<ScenarioOutput>,
}

impl ExpectedOutput {
    pub(crate) fn get_scenario(&self, example: bool) -> Option<&ScenarioOutput> {
        if example {
            self.example.as_ref()
        } else {
            self.real.as_ref()
        }
    }
}

fn get_day_output_path(day: u32) -> Box<Path> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data/")
        .join("days/")
        .join(format!("day{day}"))
        .join("outputs.json")
        .into_boxed_path()
}

pub(crate) fn read_expected_outputs(day: u32) -> anyhow::Result<Option<ExpectedOutput>> {
    let file = get_day_output_path(day);
    if !file.exists() {
        return Ok(None);
    }

    let file = File::open(file)?;
    let expected_outputs: ExpectedOutput = serde_json::from_reader(file)?;

    Ok(Some(expected_outputs))
}
