use crate::day::{DayPart, DaySolver};
use crate::intcode::IntMachine;
use crate::parsers::parse_intmachine_input;
use anyhow::Error;

pub struct Day5 {}

impl Day5 {
    pub fn new() -> Self {
        Self {}
    }
}

impl DaySolver for Day5 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, Error> {
        let memory = parse_intmachine_input(input)?;
        let mut machine = IntMachine::new(memory);

        match part {
            DayPart::Part1 => {
                machine.with_input(vec![1].into());

                machine.run()?;

                let output = machine.get_output();

                let result = match output.as_slice() {
                    [checks @ .., diagnostic_code]
                        if checks.iter().all(|&check_ok| check_ok == 0) =>
                    {
                        *diagnostic_code
                    }
                    output => {
                        anyhow::bail!("Invalid output: {:?}", output);
                    }
                };

                Ok(Box::new(result))
            }
            DayPart::Part2 => {
                machine.with_input(vec![5].into());

                machine.run()?;

                let output = machine.get_output();

                match output.as_slice() {
                    [diagnostic_code] => Ok(Box::new(*diagnostic_code)),
                    unexpected_output => Err(anyhow::anyhow!(
                        "Unexpected output: {:?}",
                        unexpected_output
                    )),
                }
            }
        }
    }
}
