use clap::ValueEnum;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum DayPart {
    Part1,
    Part2,
}

impl From<DayPart> for u8 {
    fn from(value: DayPart) -> Self {
        match value {
            DayPart::Part1 => 1,
            DayPart::Part2 => 2,
        }
    }
}

impl Display for DayPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

impl DayPart {
    pub(crate) fn is_part2(&self) -> bool {
        *self == DayPart::Part2
    }
}

pub trait Day {
    fn solve_part(
        &self,
        part: DayPart,
        input: Vec<String>,
    ) -> Result<Box<dyn ToString>, anyhow::Error>;
}
