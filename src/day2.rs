use crate::day::{Day, DayPart};
use crate::intcode::IntMachine;
use anyhow;
use std::fmt::{Display, Formatter};

pub struct Day2 {}

type Memory = Vec<u32>;

impl Day2 {
    fn parse_line(line: &str) -> Result<Memory, anyhow::Error> {
        line.split(',')
            .map(|value| value.parse::<u32>().map_err(|e| e.into()))
            .collect()
    }

    fn parse_input(lines: Vec<String>) -> Result<Vec<Memory>, anyhow::Error> {
        lines
            .into_iter()
            .map(|line| Self::parse_line(&line))
            .collect()
    }

    fn solve_for_output(
        machine: IntMachine,
        expected_output: u32,
    ) -> Result<(u32, u32), anyhow::Error> {
        for noun in 0..=99u32 {
            for verb in 0..=99u32 {
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

enum Day2Solution {
    Outputs(Vec<u32>),
    Inputs(Vec<(u32, u32)>),
}

impl Day2Solution {
    fn outputs(results: Vec<u32>) -> Self {
        Self::Outputs(results)
    }

    fn inputs(inputs: Vec<(u32, u32)>) -> Self {
        Self::Inputs(inputs)
    }
}

impl Display for Day2Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Outputs(outputs) => {
                for (idx, result) in outputs.iter().enumerate() {
                    writeln!(f, "{idx} => {result}")?;
                }
            }
            Self::Inputs(inputs) => {
                for (idx, (noun, verb)) in inputs.iter().enumerate() {
                    writeln!(f, "{idx} => ({noun}, {verb})")?;
                }
            }
        }

        Ok(())
    }
}

impl Day for Day2 {
    fn solve_part(
        &self,
        part: DayPart,
        input: Vec<String>,
    ) -> Result<Box<dyn ToString>, anyhow::Error> {
        let memories = Day2::parse_input(input)?;

        let result = match part {
            DayPart::Part1 => {
                let outputs = memories
                    .into_iter()
                    .map(IntMachine::new)
                    .map(|mut machine| machine.run())
                    .collect::<Result<Vec<_>, _>>()?;
                Day2Solution::outputs(outputs)
            }
            DayPart::Part2 => {
                let inputs = memories
                    .into_iter()
                    .map(IntMachine::new)
                    .map(|machine| Self::solve_for_output(machine, 19690720))
                    .collect::<Result<Vec<_>, _>>()?;
                Day2Solution::inputs(inputs)
            }
        };

        Ok(Box::new(result))
    }
}
