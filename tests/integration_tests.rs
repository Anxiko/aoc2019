mod common;

use aoc2019::day::solutions::get_day;
use aoc2019::day::{DayPart, solutions};
use aoc2019::parsers::parse_file;
use common::outputs::read_expected_outputs;
use itertools::Itertools;

#[test]
fn solutions() -> anyhow::Result<()> {
    for day in 1..=50 {
        let Ok(day_solver) = get_day(day) else {
            continue;
        };

        let Some(expected_output) = read_expected_outputs(day)? else {
            anyhow::bail!("No expected outputs for day {day}")
        };

        for example in [false, true] {
            let path = solutions::file_path(day, example);
            let input = parse_file(&path)?;
            let input = input.iter().map(String::as_str).collect_vec();

            let Some(scenario) = expected_output.get_scenario(example) else {
                eprintln!("Skipping scenario for day {day} example={example}");
	            continue
            };

            for part in DayPart::values() {
                let Some(value) = scenario.get_part(part) else {
	                eprintln!("Skipping part {part} for day {day} example={example}");
                    continue;
                };

                let actual = day_solver.solve_part(part, example, &input)?.to_string();
                let expected = value.to_string();

                assert_eq!(actual, expected, "Day {day} part {part} example={example}");
            }
        }
    }

    Ok(())
}
