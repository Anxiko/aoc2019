use aoc2019::args::Args;
use aoc2019::day::solutions;
use aoc2019::parsers::parse_file;
use clap::Parser;
use itertools::Itertools;

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let day_solver = solutions::get_day(args.day)?;
    let path = solutions::file_path(args.day, args.example);

    let lines = parse_file(&path)?;
    let input = lines.iter().map(String::as_str).collect_vec();

    let solution = day_solver.solve_part(args.day_part, args.example, &input)?;

    println!(
        "Day {} part {}:\n{}",
        args.day,
        args.day_part,
        solution.to_string()
    );

    Ok(())
}
