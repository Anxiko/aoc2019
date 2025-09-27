use crate::day::{DayPart, DaySolver};
use crate::shared::coord::{Coord, Direction};
use anyhow::Error;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: u32,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let direction = chars
            .next()
            .map(|direction| direction.to_string())
            .ok_or_else(|| anyhow::anyhow!("No direction found"))?;

        let direction = Direction::from_str(&direction)?;
        let length = chars.as_str().parse::<u32>()?;
        Ok(Instruction { direction, length })
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction, self.length)
    }
}

#[derive(Debug)]
struct Wire {
    instructions: Vec<Instruction>,
}

impl Display for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let contents = self
            .instructions
            .iter()
            .map(|instruction| instruction.to_string())
            .join(", ");
        write!(f, "[{}]", contents)
    }
}

impl Wire {
    fn iter(&self) -> impl Iterator<Item = Direction> {
        self.instructions.iter().flat_map(|instruction| {
            std::iter::repeat_n(instruction.direction, instruction.length as usize)
        })
    }

    fn positions(&self) -> impl Iterator<Item = Coord> {
        self.iter().scan(Coord::default(), |acc, direction| {
            *acc += direction.into();
            Some(*acc)
        })
    }

    fn earliest_position(&self) -> HashMap<Coord, u32> {
        self.positions()
            .enumerate()
            .map(|(i, position)| (position, i as u32 + 1))
            .collect_vec()
            .into_iter()
            .rev()
            .collect()
    }
}

type Wires = (Wire, Wire);

pub struct Day3 {}

impl Day3 {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_line(line: &str) -> anyhow::Result<Wire> {
        let instructions = line.split(',').map(Instruction::from_str).try_collect()?;

        Ok(Wire { instructions })
    }

    fn parse_input(input: &[&str]) -> anyhow::Result<Wires> {
        match input {
            [first, second] => {
                let first = Self::parse_line(first)?;
                let second = Self::parse_line(second)?;
                Ok((first, second))
            }
            lines => Err(anyhow::anyhow!(
                "Expected 2 wires to parse, found {}",
                lines.len()
            )),
        }
    }
}

impl DaySolver for Day3 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, Error> {
        let (first, second) = Self::parse_input(input)?;

        match part {
            DayPart::Part1 => {
                let first_positions: HashSet<_> = first.positions().collect();
                let second_positions: HashSet<_> = second.positions().collect();

                let closer_crossing = first_positions
                    .intersection(&second_positions)
                    .copied()
                    .min_by_key(Coord::manhattan)
                    .ok_or_else(|| anyhow::anyhow!("No crossing found"))?;

                Ok(Box::new(closer_crossing.manhattan()))
            }
            DayPart::Part2 => {
                let mapped_first_positions = first.earliest_position();
                let mapped_second_positions = second.earliest_position();

                let (_earliest_crossing, combined_distance) = mapped_first_positions
                    .iter()
                    .flat_map(|(coord, first_distance)| {
                        mapped_second_positions
                            .get(coord)
                            .map(|second_distance| (coord, first_distance + second_distance))
                    })
                    .min_by_key(|(_coord, combined_distance)| *combined_distance)
                    .ok_or_else(|| anyhow::anyhow!("No crossing found"))?;

                Ok(Box::new(combined_distance.to_string()))
            }
        }
    }
}
