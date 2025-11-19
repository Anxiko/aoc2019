use crate::day::{DayPart, DaySolver};
use anyhow::Error;
use itertools::Itertools;
use std::iter::{repeat_n, successors};

type Int = i32;

const BASE_PATTERN: [Int; 4] = [0, 1, 0, -1];
const TARGET_ITERATTION: usize = 100;
const REQUESTED_DIGITS: usize = 8;
const OFFSET_DIGITS: usize = 7;
const INPUT_REPEATS: usize = 10_000;

pub(crate) struct Day16 {}

impl Day16 {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn compute_digit(digits: &[Int], repeat_count: usize) -> Int {
        let pattern = BASE_PATTERN
            .iter()
            .flat_map(|digit| repeat_n(digit, repeat_count))
            .cycle()
            .skip(1)
            .copied();

        digits
            .iter()
            .zip(pattern)
            // .inspect(|pair| println!("{pair:?}"))
            .map(|(digit, pattern)| digit * pattern)
            // .reduce(|x, y| (x + y) % 10)
            // .unwrap_or_default()
            .sum::<Int>()
            .abs()
            % 10
    }

    fn apply_fft(digits: &[Int]) -> Vec<Int> {
        (1..=digits.len())
            .map(|offset| Self::compute_digit(digits, offset))
            .collect()
    }

    fn parse_input(input: &[&str]) -> Vec<Int> {
        match input {
            &[line] => line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as Int)
                .collect(),
            [] => panic!("No lines!"),
            lines => panic!("Expected 1 line, founnd {}", lines.len()),
        }
    }

    fn from_digits(digits: &[Int]) -> Int {
        digits.iter().fold(0, |acc, &digit| acc * 10 + digit)
    }
}

impl DaySolver for Day16 {
    fn solve_part(
        &self,
        part: DayPart,
        example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, Error> {
        let digits = Self::parse_input(input);

        let iterations = if !example { TARGET_ITERATTION } else { 5 };

        let digits = match part {
            DayPart::Part1 => successors(Some(digits), |digits| Some(Self::apply_fft(digits)))
                .nth(iterations)
                .unwrap(),
            DayPart::Part2 => {
                let offset =
                    Self::from_digits(&digits.iter().copied().take(OFFSET_DIGITS).collect_vec());

                let mut digits = repeat_n(digits, INPUT_REPEATS)
                    .flatten()
                    .skip(offset as usize)
                    .collect_vec();

                // I copied this from elsewhere :(
                for _ in 0..iterations {
                    let mut sum = 0;
                    for x in digits.iter_mut().rev() {
                        sum = (sum + *x as usize) % 10;
                        *x = sum as Int;
                    }
                }

                digits
            }
        };

        let digits: String = digits
            .into_iter()
            .take(REQUESTED_DIGITS)
            .map(|digit| char::from_digit(digit as u32, 10).unwrap())
            .collect();

        Ok(Box::new(digits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_digit() {
        assert_eq!(
            Day16::compute_digit(&Day16::parse_input(&["12345678"]), 1),
            4
        );
    }

    #[test]
    fn compute_other_digit() {
        assert_eq!(
            Day16::compute_digit(&Day16::parse_input(&["48226158"]), 2),
            4
        );
    }
}
