use crate::day::{DayPart, DaySolver};
use crate::intcode::IntMachine;
use crate::parsers::parse_intmachine_input;
use crate::types::IntCell;
use itertools::Itertools;

pub(crate) struct Day7 {}

const AMPLIFIERS_COUNT: usize = 5;

struct AmplifierCircuit {
    machines: [IntMachine; AMPLIFIERS_COUNT],
}

impl AmplifierCircuit {
    fn new(configurations: [IntCell; AMPLIFIERS_COUNT], memory: Vec<IntCell>) -> Self {
        let machines = configurations
            .iter()
            .copied()
            .map(|config| {
                let mut machine = IntMachine::new(memory.clone());
                machine.add_input(config);

                machine
            })
            .collect_vec();

        Self {
            machines: machines.try_into().unwrap(),
        }
    }

    fn run(&mut self, input: IntCell) -> anyhow::Result<IntCell> {
        self.machines.iter_mut().try_fold(input, |acc, machine| {
            machine.add_input(acc);
            machine.run_until_input()?;
            machine.pop_output()
        })
    }

    fn is_halted(&self) -> anyhow::Result<bool> {
        let result = self
            .machines
            .iter()
            .map(IntMachine::is_halted)
            .all_equal_value();

        match result {
            Ok(are_halted) => Ok(are_halted),
            Err(None) => unreachable!("There should be {AMPLIFIERS_COUNT} machines"),
            Err(Some(_)) => Err(anyhow::anyhow!(
                "Some machines have halted, while others haven't"
            )),
        }
    }
}

impl Day7 {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn evaluate_configuration(
        memory: Vec<IntCell>,
        configurations: &[IntCell; AMPLIFIERS_COUNT],
        looping: bool,
    ) -> anyhow::Result<IntCell> {
        let mut circuit = AmplifierCircuit::new(configurations.to_owned(), memory);
        let mut output = circuit.run(0)?;

        if looping {
            while !circuit.is_halted()? {
                output = circuit.run(output)?;
            }
        }

        Ok(output)
    }
}

impl DaySolver for Day7 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> anyhow::Result<Box<dyn ToString>> {
        let memory = parse_intmachine_input(input)?;
        let looping = part.is_part2();

        let configurations = if !looping {
            0..AMPLIFIERS_COUNT as IntCell
        } else {
            AMPLIFIERS_COUNT as IntCell..2 * AMPLIFIERS_COUNT as IntCell
        };

        let max_output = configurations
            .permutations(AMPLIFIERS_COUNT)
            .map(|configurations| configurations.try_into().unwrap())
            .map(|configurations| {
                Self::evaluate_configuration(memory.clone(), &configurations, looping).unwrap()
            })
            .max()
            .unwrap_or_default();

        Ok(Box::new(max_output))
    }
}
