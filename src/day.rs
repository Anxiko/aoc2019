use clap::ValueEnum;
use std::fmt::Display;

pub mod solutions;

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

    pub fn values() -> impl Iterator<Item = DayPart> {
        [DayPart::Part1, DayPart::Part2, DayPart::Part1]
            .iter()
            .copied()
    }
}

pub trait DaySolver {
    fn solve_part(
        &self,
        part: DayPart,
        example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, anyhow::Error>;
}
