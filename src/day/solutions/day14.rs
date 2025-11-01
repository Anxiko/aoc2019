use crate::day::{DayPart, DaySolver};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Mul;
use std::str::FromStr;
use std::sync::LazyLock;

const FUEL: &str = "FUEL";
const ORE: &str = "ORE";

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct Chemical {
    name: String,
    count: u64,
}

impl Chemical {
    fn with_count(&self, new_count: u64) -> Self {
        Self {
            name: self.name.clone(),
            count: new_count,
        }
    }
}

impl Mul<u64> for Chemical {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self {
            name: self.name,
            count: self.count * rhs,
        }
    }
}

static CHEMICAL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<count>\d+) (?P<name>\w+)").expect("Chemical pattern to compile")
});

impl FromStr for Chemical {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let named_captures = CHEMICAL_PATTERN
            .captures(s)
            .ok_or(anyhow::anyhow!("Failed to parse chemical"))?;

        let count = &named_captures["count"];
        let count = count
            .parse()
            .map_err(|_| anyhow::anyhow!("Failed to parse count: {count}"))?;
        let name = named_captures["name"].to_owned();

        Ok(Self { name, count })
    }
}

impl Display for Chemical {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.count, self.name)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

static REACTION_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<inputs>\d+ \w+(, \d+ \w+)*) => (?P<output>\d+ \w+)$")
        .expect("Reaction pattern to compile")
});

impl FromStr for Reaction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let named_captures = REACTION_PATTERN
            .captures(s)
            .ok_or(anyhow::anyhow!("Failed to parse chemical: {s}"))?;

        let inputs = &named_captures["inputs"];
        let output = &named_captures["output"];

        let inputs = inputs
            .split(", ")
            .map(Chemical::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let output = Chemical::from_str(output)?;

        Ok(Self { inputs, output })
    }
}

impl Display for Reaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let inputs = self.inputs.iter().map(ToString::to_string).join(", ");
        write!(f, "{inputs} => {}", self.output)
    }
}

struct MiningOperation {
    cookbook: HashMap<String, (u64, Vec<Chemical>)>,
    chemical_count: HashMap<String, i64>, // Negative numbers imply we need that amount of the chemical
}

impl MiningOperation {
    fn new(cookbook: HashMap<String, (u64, Vec<Chemical>)>, fuel_requirement: u64) -> Self {
        Self {
            cookbook,
            chemical_count: HashMap::from([(FUEL.to_owned(), -(fuel_requirement as i64))]), // We need 1 FUEL
        }
    }

    fn from_reactions(reactions: Vec<Reaction>, fuel_requirement: u64) -> Self {
        let cookbook: HashMap<_, _> = reactions
            .into_iter()
            .map(|reaction| {
                (
                    reaction.output.name,
                    (reaction.output.count, reaction.inputs),
                )
            })
            .collect();

        Self::new(cookbook, fuel_requirement)
    }

    fn pop_next_required(&mut self) -> Option<Chemical> {
        let needed_chemical = self
            .chemical_count
            .iter()
            .find(|&(name, &count)| name != ORE && count < 0)
            .map(|(name, count)| Chemical {
                name: name.to_owned(),
                count: count.unsigned_abs(),
            });

        if let Some(chemical) = &needed_chemical {
            self.chemical_count.remove(&chemical.name);
        };

        needed_chemical
    }

    fn add_to_required(&mut self, chemical: &Chemical) {
        let entry = self
            .chemical_count
            .entry(chemical.name.clone())
            .or_insert(0);
        *entry -= chemical.count as i64;
    }

    fn add_to_obtained(&mut self, chemical: &Chemical) {
        let entry = self
            .chemical_count
            .entry(chemical.name.clone())
            .or_insert(0);
        *entry += chemical.count as i64;
    }

    fn procure_required(&mut self) -> bool {
        if let Some(needed_chemical) = self.pop_next_required() {
            let (output_count, required_chemicals) = self.cookbook[&needed_chemical.name].clone();
            let multiplier = needed_chemical.count.div_ceil(output_count);
            let extra_procured = output_count * multiplier - needed_chemical.count;

            if extra_procured > 0 {
                self.add_to_obtained(&needed_chemical.with_count(extra_procured));
            }

            for required in required_chemicals {
                self.add_to_required(&(required * multiplier))
            }

            false
        } else {
            true
        }
    }

    fn ore_required(&self) -> anyhow::Result<u64> {
        let ore_count = self
            .chemical_count
            .get(ORE)
            .copied()
            .ok_or(anyhow::anyhow!("ORE requirements not found"))?;
        if ore_count <= 0 {
            Ok(ore_count.unsigned_abs())
        } else {
            Err(anyhow::anyhow!("Ore requirements not negative"))
        }
    }

	fn solve_for(reactions: Vec<Reaction>, fuel_requirement: u64) -> anyhow::Result<u64> {
		let mut mining_operation = Self::from_reactions(reactions, fuel_requirement);

		while !mining_operation.procure_required() {}
        mining_operation.ore_required()
	}
}

pub(crate) struct Day14 {}

impl Day14 {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn parse_input(input: &[&str]) -> anyhow::Result<Vec<Reaction>> {
        input.iter().copied().map(Reaction::from_str).collect()
    }
}

impl DaySolver for Day14 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, anyhow::Error> {
        let reactions = Self::parse_input(input)?;
	    let ore_for_1_fuel = MiningOperation::solve_for(reactions.clone(), 1)?;

        match part {
            DayPart::Part1 => {

                Ok(Box::new(ore_for_1_fuel))
            }
            DayPart::Part2 => {
	            const ORE_AVAILABLE: u64 = 1_000_000_000_000;
	            let mut min_fuel_target = ORE_AVAILABLE / ore_for_1_fuel;
	            let mut max_fuel_target = min_fuel_target * 2;

	            while max_fuel_target - min_fuel_target > 1 {
		            let mid_fuel_target = (min_fuel_target + max_fuel_target) / 2;

		            let ore_needed = MiningOperation::solve_for(reactions.clone(), mid_fuel_target)?;
		            if ore_needed < ORE_AVAILABLE {
			            min_fuel_target = mid_fuel_target;
		            } else {
			            max_fuel_target = mid_fuel_target;
		            }
	            }

	            Ok(Box::new(min_fuel_target))
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_reactions() {
        assert_eq!(
            Reaction::from_str("2 AB, 3 BC, 4 CA => 1 FUEL").unwrap(),
            Reaction {
                inputs: vec![
                    Chemical {
                        name: "AB".to_owned(),
                        count: 2
                    },
                    Chemical {
                        name: "BC".to_owned(),
                        count: 3
                    },
                    Chemical {
                        name: "CA".to_owned(),
                        count: 4
                    }
                ],
                output: Chemical {
                    name: "FUEL".to_owned(),
                    count: 1
                },
            }
        );

        assert_eq!(
            Reaction::from_str("9 ORE => 2 A").unwrap(),
            Reaction {
                inputs: vec![Chemical {
                    name: "ORE".to_owned(),
                    count: 9
                }],
                output: Chemical {
                    name: "A".to_owned(),
                    count: 2
                },
            }
        );
    }
}
