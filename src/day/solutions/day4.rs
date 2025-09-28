use crate::day::{DayPart, DaySolver};
use anyhow::Error;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day4 {}
impl Day4 {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn parse_input(input: &[&str]) -> anyhow::Result<(u32, u32)> {
        match input {
            [line] => Self::parse_range(line),
            lines => Err(anyhow::anyhow!(
                "Expected 1 line of input, found {}",
                lines.len()
            )),
        }
    }

    fn parse_range(range: &str) -> anyhow::Result<(u32, u32)> {
        match range.split('-').collect_vec().as_slice() {
            &[start, end] => {
                let range = u32::from_str(start)
                    .and_then(|start| u32::from_str(end).map(|end| (start, end)))?;

                Ok(range)
            }
            _ => Err(anyhow::anyhow!(
                "Expected a range delimited by '-', found {}",
                range
            )),
        }
    }

    fn number_digits_is_correct(digits: &[u8]) -> bool {
        digits.len() == 6
    }

    fn adjacent_digits_equal(digits: &[u8], strict_pairs: bool) -> bool {
        digits
            .iter()
            .chunk_by(|&&digit| digit)
            .into_iter()
            .map(|(_digit, chunk)| chunk.count())
            .any(|count| count == 2 || (count > 2 && !strict_pairs))
    }

    fn non_decreasing_digits(digits: &[u8]) -> bool {
        digits.iter().tuple_windows().all(|(a, b)| a <= b)
    }

    fn digits(number: u32) -> Vec<u8> {
        number
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    }
}

impl DaySolver for Day4 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, Error> {
        let (min, max) = Self::parse_input(input)?;

        let possible_passwords = (min..=max)
            .filter(|&n| {
                let digits = Self::digits(n);

                Self::number_digits_is_correct(&digits)
                    && Self::non_decreasing_digits(&digits)
                    && Self::adjacent_digits_equal(&digits, part.is_part2())
            })
            .count();

        Ok(Box::new(possible_passwords))
    }
}
