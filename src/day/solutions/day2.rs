use crate::day::{DayPart, DaySolver};
use crate::intcode::IntMachine;
use crate::types::IntCell;
use anyhow;

pub struct Day2 {
    target_output: IntCell,
}

type Memory = Vec<IntCell>;

impl Day2 {
    pub(crate) fn new(target_output: IntCell) -> Self {
        Self { target_output }
    }

    fn parse_line(line: &str) -> Result<Memory, anyhow::Error> {
        line.split(',')
            .map(|value| value.parse::<IntCell>().map_err(|e| e.into()))
            .collect()
    }

    fn parse_input(lines: &[&str]) -> Result<Memory, anyhow::Error> {
        match lines {
            [] => Err(anyhow::anyhow!("No input lines")),
            [line] => Self::parse_line(line),
            [..] => Err(anyhow::anyhow!("Too many input lines")),
        }
    }

    fn solve_for_output(
        machine: IntMachine,
        expected_output: IntCell,
    ) -> Result<(IntCell, IntCell), anyhow::Error> {
        for noun in 0..=(99 as IntCell) {
            for verb in 0..=(99 as IntCell) {
                let mut altered_machine = machine.clone();
                altered_machine.write(1, noun)?;
                altered_machine.write(2, verb)?;

                let output = altered_machine.run()?;

                if output == expected_output {
                    return Ok((noun, verb));
                }
            }
        }

        Err(anyhow::anyhow!(
            "Could not find a solution for {expected_output}"
        ))
    }
}

impl DaySolver for Day2 {
    fn solve_part(
        &self,
        part: DayPart,
        example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, anyhow::Error> {
        let memory = Day2::parse_input(input)?;
        let mut machine = IntMachine::new(memory);

        let result: Box<dyn ToString> = match part {
            DayPart::Part1 => {
                if !example {
                    machine.write(1, 12).and_then(|()| machine.write(2, 2))?;
                }

                Box::new(machine.run()?)
            }
            DayPart::Part2 => {
                let (noun, verb) = Self::solve_for_output(machine, self.target_output)?;
                Box::new(100 * noun + verb)
            }
        };

        Ok(result)
    }
}
