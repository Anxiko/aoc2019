use crate::day::DaySolver;
use crate::day::solutions::day1::Day1;
use crate::day::solutions::day2::Day2;
use crate::day::solutions::day3::Day3;
use std::path::{Path, PathBuf};

pub mod day1;
pub mod day2;
mod day3;

pub fn get_day(day: u32) -> Result<Box<dyn DaySolver>, anyhow::Error> {
    match day {
        invalid if invalid == 0 || invalid > 50 => anyhow::bail!("Invalid day {invalid}"),
        1 => Ok(Box::new(Day1::new())),
        2 => Ok(Box::new(Day2::new(19690720))),
        3 => Ok(Box::new(Day3::new())),
        _unimplemented => anyhow::bail!("Unimplemented day: {day}"),
    }
}

pub fn file_path(day: u32, example: bool) -> Box<Path> {
    let mut path = PathBuf::from("data/days/");
    path.push(format!("day{day}/"));
    path.push("input/");
    if example {
        path.push("example.txt")
    } else {
        path.push("real.txt");
    }

    path.into_boxed_path()
}
