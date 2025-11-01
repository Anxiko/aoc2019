use crate::day::{DayPart, DaySolver};
use crate::shared::vect3::Vect3;
use itertools::Itertools;
use num::Integer;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt::Display;
use std::sync::LazyLock;

#[derive(Clone, Eq, PartialEq)]
struct Body {
    pos: Vect3,
    vel: Vect3,
}

impl Body {
    fn new(pos: Vect3) -> Self {
        Self {
            pos,
            vel: Default::default(),
        }
    }

    fn move_(&mut self) {
        self.pos += self.vel;
    }

    fn accelerate(&mut self, gravity: Vect3) {
        self.vel += gravity;
    }

    fn accel_towards_body(&self, other: &Self) -> Vect3 {
        let diff = other.pos - self.pos;
        Vect3::new(
            Self::diff_to_accel(diff.x),
            Self::diff_to_accel(diff.y),
            Self::diff_to_accel(diff.z),
        )
    }

    fn diff_to_accel(diff: i32) -> i32 {
        match diff.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    fn vect3_value(vect3: &Vect3) -> u32 {
        vect3.x.unsigned_abs() + vect3.y.unsigned_abs() + vect3.z.unsigned_abs()
    }

    fn potential_energy(&self) -> u32 {
        Self::vect3_value(&self.pos)
    }

    fn kinetic_energy(&self) -> u32 {
        Self::vect3_value(&self.vel)
    }

    fn total_energy(&self) -> u32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn all_vect3s(&self) -> Vec<Vect3> {
        vec![self.pos, self.vel]
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos={}, vel={}", self.pos, self.vel)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Simulation {
    bodies: Vec<Body>,
}

impl Simulation {
    fn new(bodies: Vec<Body>) -> Self {
        Self { bodies }
    }

    fn tick(&mut self) {
        for left_idx in 0..self.bodies.len() {
            for right_idx in 0..self.bodies.len() {
                if left_idx == right_idx {
                    continue;
                }

                let accelerating = self.bodies.get(left_idx).unwrap();
                let attracting = self.bodies.get(right_idx).unwrap();

                let acceleration = accelerating.accel_towards_body(attracting);

                self.bodies[left_idx].accelerate(acceleration);
            }
        }

        for body in &mut self.bodies {
            body.move_();
        }
    }

    fn total_energy(&self) -> u32 {
        self.bodies.iter().map(Body::total_energy).sum()
    }

    fn all_vect3s(&self) -> Vec<Vect3> {
        self.bodies.iter().flat_map(Body::all_vect3s).collect_vec()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum DimensionSelector {
    X,
    Y,
    Z,
}

impl DimensionSelector {
    fn extract_dimension(&self, vect3: &Vect3) -> i32 {
        match self {
            Self::X => vect3.x,
            Self::Y => vect3.y,
            Self::Z => vect3.z,
        }
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for body in &self.bodies {
            writeln!(f, "{body}")?;
        }

        Ok(())
    }
}

static VECT3_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^<x=([+-]?\d+), y=([+-]?\d+), z=([+-]?\d+)>$").unwrap());

pub(crate) struct Day12 {}

impl Day12 {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn parse_vect3(vect3: &str) -> anyhow::Result<Vect3> {
        let captures = VECT3_PATTERN
            .captures(vect3)
            .ok_or_else(|| anyhow::anyhow!("Could not match {vect3}"))?;

        let (_full, [x, y, z]) = captures.extract();
        let x = x.parse::<i32>()?;
        let y = y.parse::<i32>()?;
        let z = z.parse::<i32>()?;

        Ok(Vect3::new(x, y, z))
    }

    fn parse_input(input: &[&str]) -> anyhow::Result<Simulation> {
        let positions = input
            .iter()
            .copied()
            .map(Self::parse_vect3)
            .collect::<anyhow::Result<Vec<_>>>()?;

        let bodies = positions.into_iter().map(Body::new).collect_vec();

        Ok(Simulation::new(bodies))
    }

    fn equal_by_dimension(
        lhs: &Simulation,
        rhs: &Simulation,
        dimension_selector: DimensionSelector,
    ) -> bool {
        let lefts = lhs
            .all_vect3s()
            .into_iter()
            .map(|vect3| dimension_selector.extract_dimension(&vect3))
            .collect_vec();

        let rights = rhs
            .all_vect3s()
            .into_iter()
            .map(|vect3| dimension_selector.extract_dimension(&vect3))
            .collect_vec();

        lefts == rights
    }

    fn loop_for_dimension(simulation: &Simulation, dimension_selector: DimensionSelector) -> u64 {
        let mut current = simulation.clone();
        current.tick();
        let mut ticks = 1;

        while !Self::equal_by_dimension(&current, &simulation, dimension_selector) {
            current.tick();
            ticks += 1;
        }

        ticks
    }
}

const SIMULATIONS_TICKS: u32 = 1000;

impl DaySolver for Day12 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, anyhow::Error> {
        let mut simulation = Self::parse_input(input)?;

        match part {
            DayPart::Part1 => {
                for _tick in 0..SIMULATIONS_TICKS {
                    // println!("At tick {tick}:\n{simulation}");

                    simulation.tick();
                }

                // println!("Final:\n{simulation}");

                Ok(Box::new(simulation.total_energy()))
            }
            DayPart::Part2 => {
                let x_loops = Self::loop_for_dimension(&simulation, DimensionSelector::X);
                let y_loops = Self::loop_for_dimension(&simulation, DimensionSelector::Y);
                let z_loops = Self::loop_for_dimension(&simulation, DimensionSelector::Z);

	            let loop_ticks = x_loops.lcm(&y_loops).lcm(&z_loops);

                Ok(Box::new(loop_ticks))
            }
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
    fn parse_vect3() {
        assert_eq!(
            Day12::parse_vect3("<x=-1, y=0, z=2>").expect("Vect3 to parse"),
            Vect3::new(-1, 0, 2)
        );
    }
}
