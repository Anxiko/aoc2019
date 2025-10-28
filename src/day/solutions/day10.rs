use crate::day::{DayPart, DaySolver};
use crate::shared::coord::Coord;
use anyhow::{Context, Error};
use gcd::Gcd;
use itertools::Itertools;
use ordered_float::NotNan;

struct Input {
    coords: Vec<Coord>,
    center: Option<Coord>,
}

pub(crate) struct Day10 {}

impl Day10 {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn parse_input(input: &[&str]) -> Input {
        let mut center: Option<Coord> = None;

        let coords = input
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, ch)| (Coord::new(col as i32, row as i32), ch))
            })
            .filter_map(|(coord, ch)| match ch {
                '#' => Some(coord),
                'X' => {
                    center = Some(coord);
                    Some(coord)
                }
                _ => None,
            })
            .collect();

        Input { coords, center }
    }

    fn comparable_coord(coord: Coord) -> Option<(u32, Coord)> {
        let factor: u32 = coord.x.unsigned_abs().gcd(coord.y.unsigned_abs());

        if factor == 0 {
            return None;
        }

        let reduced = Coord {
            x: coord.x / (factor as i32),
            y: coord.y / (factor as i32),
        };

        Some((factor, reduced))
    }

    fn calculate_deltas(center: Coord, others: &[Coord]) -> Vec<Coord> {
        others.iter().map(|&other| other - center).collect()
    }

    fn coord_to_polar(vector: Coord) -> (f64, f64) {
        let angle_from_x_axis = (-vector.y as f64).atan2(-vector.x as f64);
        let angle_from_y_axis =
            (angle_from_x_axis + std::f64::consts::PI * 1.5) % (2.0 * std::f64::consts::PI);

        let module = vector.module();

        (angle_from_y_axis, module)
    }

    fn observable_coords(deltas: &[Coord]) -> usize {
        deltas
            .iter()
            .copied()
            .filter_map(Self::comparable_coord)
            .map(|(_factor, direction)| direction)
            .unique()
            .count()
    }

    fn sorted_deltas(deltas: &[Coord]) -> Vec<Coord> {
        let mut reversed_groups = deltas
            .iter()
            .copied()
            .map(|delta| {
                let (angle, module) = Self::coord_to_polar(delta);
                (delta, angle, module)
            })
            .into_group_map_by(|(_delta, angle, _module)| NotNan::new(*angle).unwrap())
            .into_iter()
            .sorted_by_key(|(angle, _values)| *angle)
            .map(|(_key, values)| {
                let mut group = values
                    .into_iter()
                    .sorted_by_key(|(_delta, _angle, module)| NotNan::new(*module).unwrap())
                    .map(|(delta, _angle, _module)| delta)
                    .collect_vec();

                group.reverse();

                group
            })
            .collect_vec();

        let mut result = Vec::new();

        while let Some(elements) = Self::last_of_each(&mut reversed_groups) {
            result.extend(elements);
        }

        result
    }

    fn last_of_each<T>(groups: &mut Vec<Vec<T>>) -> Option<Vec<T>> {
        let mut result = Vec::new();

        for group in groups {
            if let Some(last) = group.pop() {
                result.push(last);
            }
        }

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}

impl DaySolver for Day10 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, Error> {
        let Input {
            center: maybe_center,
            coords: input,
            ..
        } = Self::parse_input(input);

        let (center, deltas, max_observable) = if let Some(center) = maybe_center {
            let deltas = Self::calculate_deltas(center, &input);
            (center, deltas.clone(), Self::observable_coords(&deltas))
        } else {
            input
                .iter()
                .map(|&center| {
                    let deltas = Self::calculate_deltas(center, &input);
                    (center, deltas.clone(), Self::observable_coords(&deltas))
                })
                .max_by_key(|(_center, _deltas, total_observable)| *total_observable)
                .with_context(|| "Expected at least one asteroid")?
        };

        match part {
            DayPart::Part1 => Ok(Box::new(max_observable)),
            DayPart::Part2 => {
                let sorted_deltas = Self::sorted_deltas(&deltas);

                let target_delta = sorted_deltas
                    .get(200 - 1)
                    .copied()
                    .ok_or(anyhow::anyhow!("Not enough asteroids!"))?;

                let target_coord = center + target_delta;
                let result = target_coord.x * 100 + target_coord.y;

                Ok(Box::new(result))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle() {
        assert_eq!((0.0, 1.0), Day10::coord_to_polar(Coord::new(0, -1)));
        assert_eq!(
            (std::f64::consts::PI / 2.0, 1.0),
            Day10::coord_to_polar(Coord::new(1, 0))
        );
        assert_eq!(
            (std::f64::consts::PI, 1.0),
            Day10::coord_to_polar(Coord::new(0, 1))
        );
        assert_eq!(
            (3.0 * std::f64::consts::PI / 2.0, 1.0),
            Day10::coord_to_polar(Coord::new(-1, 0))
        );
    }
}
