use crate::day::{DayPart, DaySolver};
use anyhow;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day6 {}

struct Orbit {
    center: String,
    orbiter: String,
}

impl FromStr for Orbit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(')') {
            Some((center, orbiter)) => Ok(Self {
                center: center.to_owned(),
                orbiter: orbiter.to_owned(),
            }),
            _ => Err(anyhow::anyhow!("Invalid orbit: {s}")),
        }
    }
}

impl Day6 {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_input(input: &[&str]) -> anyhow::Result<Vec<Orbit>> {
        input.iter().copied().map(Orbit::from_str).collect()
    }

    fn chain(
        orbiter: &str,
        orbiter_mapper: &HashMap<String, String>,
    ) -> impl Iterator<Item = String> {
        let mut current_node = Some(orbiter.to_owned());

        std::iter::from_fn(move || {
            if let Some(node) = &current_node {
                let emitted_node = node.clone();
                current_node = orbiter_mapper.get(&emitted_node).cloned();
                Some(emitted_node)
            } else {
                None
            }
        })
    }

    fn chain_length(orbiter: &str, orbiter_mapper: &HashMap<String, String>) -> u32 {
        match orbiter_mapper.get(orbiter) {
            Some(parent) => 1 + Self::chain_length(parent, orbiter_mapper),
            None => 0,
        }
    }
}

impl DaySolver for Day6 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, anyhow::Error> {
        let input = Day6::parse_input(input)?;
        let orbiter_to_center_mapper: HashMap<_, _> = input
            .into_iter()
            .map(|Orbit { center, orbiter }| (orbiter, center))
            .collect();

        match part {
            DayPart::Part1 => {
                let total: u32 = orbiter_to_center_mapper
                    .keys()
                    .map(|orbiter| Self::chain_length(orbiter, &orbiter_to_center_mapper))
                    .sum();

                Ok(Box::new(total))
            }
            DayPart::Part2 => {
                let you = Self::chain("YOU", &orbiter_to_center_mapper).collect_vec();
                let santa = Self::chain("SAN", &orbiter_to_center_mapper).collect_vec();

                let you_iter = you.iter().rev();
                let santa_iter = santa.iter().rev();

                let prefix_size = you_iter
                    .zip(santa_iter)
                    .take_while(|(left, right)| left == right)
                    .count();

                let you_unshared = you.len() - prefix_size;
                let santa_unshared = santa.len() - prefix_size;

                /*
                YOU and SAN aren't actually objects, they're just markers.
                That means that your real starting point is the center to YOU,
                and your destination is the center to SAN.
                 */
                let orbital_transfers = (you_unshared - 2) + (santa_unshared - 2) + 2;

                Ok(Box::new(orbital_transfers))
            }
        }
    }
}
