use crate::day::{DaySolver, DayPart};

pub struct Day1 {}

impl Day1 {
	pub(crate) fn new() -> Self {
		Self{}
	}
	
    fn parse_input(input: &[&str]) -> Result<Vec<u32>, anyhow::Error> {
        let numbers = input
            .iter()
            .map(|line| line.parse())
            .collect::<Result<Vec<u32>, _>>()?;

        Ok(numbers)
    }

    fn calc_fuel(mass: u32, looping: bool) -> u32 {
        if mass == 0 {
            return 0;
        }

        let fuel = (mass / 3).saturating_sub(2);

        if looping {
            fuel + Self::calc_fuel(fuel, true)
        } else {
            fuel
        }
    }
}

impl DaySolver for Day1 {
    fn solve_part(
	    &self,
	    part: DayPart,
	    _example: bool,
	    input: &[&str],
    ) -> Result<Box<dyn ToString>, anyhow::Error> {
        let total_fuel: u32 = Self::parse_input(input)?
            .into_iter()
            .map(|module_mass| Self::calc_fuel(module_mass, part.is_part2()))
            .sum();

        Ok(Box::new(total_fuel))
    }
}
