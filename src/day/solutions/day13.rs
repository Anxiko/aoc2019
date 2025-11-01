use crate::day::{DayPart, DaySolver};
use crate::intcode::IntMachine;
use crate::parsers::parse_intmachine_input;
use crate::shared::board::{Board, HashBoard};
use crate::shared::coord::Coord;
use crate::types::IntCell;
use anyhow::{Context, Error};
use itertools::Itertools;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => '*',
            Tile::Paddle => '=',
            Tile::Ball => '@',
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum DisplayOutput {
    PaintTile { tile: Tile, coord: Coord },
    Score(IntCell),
}

impl DisplayOutput {
    fn try_from_slice(slice: &[IntCell]) -> anyhow::Result<Self> {
        let [x, y, tile] = slice.try_into()?;

        if x == -1 && y == 0 {
            Ok(Self::Score(tile))
        } else {
            let tile = u8::try_from(tile)?;
            let tile = Tile::try_from(tile)?;
            Ok(Self::PaintTile {
                tile,
                coord: Coord::new(x as i32, y as i32),
            })
        }
    }
}

struct ParsedOutput {
    ball: Coord,
    paddle: Coord,
    score: IntCell,
    blocks: usize,
}

pub(crate) struct Day13 {}

impl Day13 {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn parse_machine_output(
        board: &mut HashBoard<Tile>,
        machine: &mut IntMachine,
    ) -> anyhow::Result<ParsedOutput> {
        let outputs: Vec<DisplayOutput> = machine
            .get_output()
            .chunks(3)
            .map(DisplayOutput::try_from_slice)
            .try_collect()
            .with_context(|| "Failed to parse paint instructions from output")?;
        machine.clear_output();

        let mut player_score = 0;

        for display_output in outputs {
            match display_output {
                DisplayOutput::PaintTile { tile, coord } => {
                    board.write(coord, tile)?;
                }
                DisplayOutput::Score(score) => {
                    player_score = score;
                }
            }
        }

        let ball = board
            .elements()
            .find_map(|(coord, tile)| {
                if tile == &Tile::Ball {
                    Some(coord)
                } else {
                    None
                }
            })
            .ok_or(anyhow::anyhow!("Could not find ball"))?;

        let paddle = board
            .elements()
            .find_map(|(coord, tile)| {
                if tile == &Tile::Paddle {
                    Some(coord)
                } else {
                    None
                }
            })
            .ok_or(anyhow::anyhow!("Could not find paddle"))?;

        let blocks = board
            .elements()
            .filter(|(_coord, tile)| **tile == Tile::Block)
            .count();

        Ok(ParsedOutput {
            ball,
            paddle,
            score: player_score,
            blocks,
        })
    }
}

impl DaySolver for Day13 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, Error> {
        let memory = parse_intmachine_input(input)?;
        let mut machine = IntMachine::new(memory);

        match part {
            DayPart::Part1 => {
                machine.run()?;
                let output = machine.get_output();

                let outputs: Vec<DisplayOutput> = output
                    .chunks(3)
                    .map(DisplayOutput::try_from_slice)
                    .try_collect()
                    .with_context(|| "Failed to parse paint instructions from output")?;

                let mut board = HashBoard::new(Tile::Empty);

                for display_output in outputs {
                    match display_output {
                        DisplayOutput::PaintTile { tile, coord } => {
                            board.write(coord, tile)?;
                        }
                        DisplayOutput::Score(_) => {}
                    }
                }
                let blocks = board
                    .coord_mapping
                    .values()
                    .filter(|&&tile| tile == Tile::Block)
                    .count();

                Ok(Box::new(blocks))
            }
            DayPart::Part2 => {
                machine.write(0, 2)?; // Play for free!
                let mut board = HashBoard::new(Tile::Empty);

                let score = loop {
                    machine.run_until_input()?;

                    let parsed_output = Self::parse_machine_output(&mut board, &mut machine)?;

                    if parsed_output.blocks == 0 {
                        break parsed_output.score;
                    }

                    let joystick = match parsed_output.paddle.x.cmp(&parsed_output.ball.x) {
                        Ordering::Equal => 0,
                        Ordering::Less => 1,
                        Ordering::Greater => -1,
                    };

                    machine.add_input(joystick);
                };

                Ok(Box::new(score))
            }
        }
    }
}
