use aoc2019::args::Args;
use aoc2019::day::Day;
use aoc2019::day1::Day1;
use aoc2019::day2::Day2;
use aoc2019::parsers::parse_file;
use clap::Parser;
use std::path::{Path, PathBuf};

fn get_day(day: u32) -> Result<Box<dyn Day>, anyhow::Error> {
    match day {
        invalid if invalid == 0 || invalid > 50 => anyhow::bail!("Invalid day {invalid}"),
        1 => Ok(Box::new(Day1 {})),
        2 => Ok(Box::new(Day2 {})),
        _unimplemented => anyhow::bail!("Unimplemented day: {day}"),
    }
}

fn file_path(day: u32, example: bool) -> Box<Path> {
    let mut path = PathBuf::from("data/days/");
    path.push(format!("day{day}/"));
    if example {
        path.push("example.txt")
    } else {
        path.push("real.txt");
    }

    path.into_boxed_path()
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let day_solver = get_day(args.day)?;
    let path = file_path(args.day, args.example);

    let lines = parse_file(&path)?;

    let solution = day_solver.solve_part(args.day_part, lines)?;

    println!(
        "Day {} part {}: {}",
        args.day,
        args.day_part,
        solution.to_string()
    );

    Ok(())
}
