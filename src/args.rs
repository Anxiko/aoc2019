use crate::day::DayPart;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    pub day: u32,
    pub day_part: DayPart,
    #[arg(short, long)]
    example: bool,
}