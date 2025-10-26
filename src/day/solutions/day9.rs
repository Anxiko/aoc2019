use crate::day::{DayPart, DaySolver};
use crate::intcode::IntMachine;
use crate::parsers::parse_intmachine_input;
use crate::types::IntCell;
use anyhow::Error;

pub(crate) struct Day9 {}

impl Day9 {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl DaySolver for Day9 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, Error> {
        let mem = parse_intmachine_input(input)?;
        let mut machine = IntMachine::new(mem);

        let input: IntCell = match part {
            DayPart::Part1 => 1,
            DayPart::Part2 => 2,
        };

        machine.with_input(vec![input].into());

        machine.run()?;

        let output = match machine.get_output().as_slice() {
            [code] => *code,
            outputs => anyhow::bail!("Expected 1 output, got {}", outputs.len()),
        };

        Ok(Box::new(output))
    }
}
