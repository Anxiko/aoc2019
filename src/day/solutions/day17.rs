use crate::day::{DayPart, DaySolver};
use crate::intcode::IntMachine;
use crate::parsers::parse_intmachine_input;
use crate::shared::board::{Board, HashBoard};
use crate::shared::coord::{Coord, Direction};
use crate::types::IntCell;
use anyhow::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Robot {
    pos: Coord,
    dir: Direction,
}

impl Robot {
    fn new(pos: Coord, dir: Direction) -> Self {
        Robot { pos, dir }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Scaffold,
    Empty,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CharCell {
    Robot(Direction),
    Scaffold,
    Empty,
    LineJump,
}

impl CharCell {
    fn maybe_cell(&self) -> Option<Cell> {
        match self {
            Self::Robot(_) | Self::Scaffold => Some(Cell::Scaffold),
            Self::Empty => Some(Cell::Empty),
            Self::LineJump => None,
        }
    }

    fn maybe_direction(&self) -> Option<Direction> {
        match self {
            &Self::Robot(direction) => Some(direction),
            _ => None,
        }
    }

    fn is_line_jump(&self) -> bool {
        self == &Self::LineJump
    }
}

impl TryFrom<char> for CharCell {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Scaffold),
            '.' => Ok(Self::Empty),
            '^' => Ok(Self::Robot(Direction::Up)),
            'v' => Ok(Self::Robot(Direction::Down)),
            '>' => Ok(Self::Robot(Direction::Right)),
            '<' => Ok(Self::Robot(Direction::Left)),
            '\n' => Ok(Self::LineJump),
            error => Err(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    board: HashBoard<Cell>,
    robot: Robot,
}

impl State {
    fn new(board: HashBoard<Cell>, robot: Robot) -> Self {
        State { board, robot }
    }

    fn crossings(&self) -> Vec<Coord> {
        self.board
            .elements()
            .into_iter()
            .filter(|(_coord, cell)| *cell == Cell::Scaffold)
            .map(|(coord, _cell)| coord)
            .filter(|coord| {
                coord.cross().all(|neighbour| {
                    self.board.read(neighbour).expect("Read the board") == &Cell::Scaffold
                })
            })
            .collect()
    }
}

pub(crate) struct Day17 {}

impl Day17 {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn parse_output(machine_output: &[IntCell]) -> anyhow::Result<State> {
        let mut cursor = Coord::default();
        let mut maybe_robot = None;
        let mut board = HashBoard::new(Cell::Empty);

        for &input in machine_output {
            let char: char = char::from_u32(input as u32)
                .ok_or_else(|| anyhow::anyhow!("Couldn't convert {input} to char"))?;

            let char_cell = CharCell::try_from(char)
                .map_err(|char| anyhow::anyhow!("Couldn't interpret {char:?}"))?;

            if let Some(cell) = char_cell.maybe_cell() {
                board.write(cursor, cell).expect("Write to board");
            }

            if let Some(direction) = char_cell.maybe_direction() {
                if let Some(robot) = maybe_robot {
                    return Err(anyhow::anyhow!(
                        "Found a robot at {cursor} where one already exists: {robot:?}"
                    ));
                }

                maybe_robot = Some(Robot::new(cursor, direction));
            }

            if char_cell.is_line_jump() {
                cursor = Coord::new(0, cursor.y + 1);
            } else {
                cursor += Coord::new(1, 0);
            }
        }

        if let Some(robot) = maybe_robot {
            Ok(State::new(board, robot))
        } else {
            Err(anyhow::anyhow!("No robot found"))
        }
    }
}

impl DaySolver for Day17 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, Error> {
        let input = parse_intmachine_input(input)?;
        let mut machine = IntMachine::new(input);

        match part {
            DayPart::Part1 => {
                machine.run()?;

                let output = machine.get_output();
                let state = Self::parse_output(&output)?;

                let score = state
                    .crossings()
                    .into_iter()
                    .map(|Coord { x, y }| x * y)
                    .sum::<i32>();

                Ok(Box::new(score))
            }
            DayPart::Part2 => todo!(),
        }
    }
}
