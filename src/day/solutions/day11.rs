use crate::day::{DayPart, DaySolver};
use crate::intcode::IntMachine;
use crate::parsers::parse_intmachine_input;
use crate::shared::board::{Board, Grid, HashBoard};
use crate::shared::coord::{Coord, Direction};
use crate::types::IntCell;
use std::collections::HashSet;

pub(crate) struct Day11 {}

type Paint = bool;
const BLACK: Paint = false;
#[allow(dead_code)]
const WHITE: Paint = true;

struct Robot {
    pos: Coord,
    dir: Direction,
    painted: HashSet<Coord>,
    board: HashBoard<Paint>,
    machine: IntMachine,
}

impl Robot {
    fn new(machine: IntMachine, initial: Paint) -> Self {
        let mut robot = Self {
            pos: Coord::new(0, 0),
            dir: Direction::Up,
            painted: HashSet::new(),
            board: HashBoard::new(BLACK),
            machine,
        };

        robot
            .board
            .write(robot.pos, initial)
            .expect("Paint initial floor coord");
        robot
    }

    fn tick(&mut self) -> anyhow::Result<bool> {
        self.machine.run_until_input()?;

        if self.machine.is_halted() {
            return Ok(true);
        }

        let input: IntCell = self.board.read(self.pos).copied()?.into();
        self.machine.add_input(input);

        let paint_color = self.machine.run_until_output()? == 1;
        self.paint_current(paint_color);

        let turn = self.machine.run_until_output()?;
        self.dir = if turn == 1 {
            self.dir.turn_right()
        } else {
            self.dir.turn_left()
        };
        self.step_forward();

        Ok(false)
    }

    fn get_grid(&self) -> Grid<Paint> {
        self.board.as_grid().expect("Transform board into grid")
    }

    fn paint_current(&mut self, paint: Paint) {
        self.board
            .write(self.pos, paint)
            .expect("Paint current position");
        self.painted.insert(self.pos);
    }

    fn step_forward(&mut self) {
        self.pos += self.dir.into();
    }

    fn count_painted(&self) -> usize {
        self.painted.len()
    }
}

impl Day11 {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl DaySolver for Day11 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> anyhow::Result<Box<dyn ToString>> {
        let memory = parse_intmachine_input(input)?;
        let machine = IntMachine::new(memory);
        let mut robot = Robot::new(machine, part.is_part2());

        match part {
            DayPart::Part1 => {
                while !robot.tick()? {}

                Ok(Box::new(robot.count_painted()))
            }
            DayPart::Part2 => {
                while !robot.tick()? {}
                let grid = robot.get_grid().map(|paint| if paint { '#' } else { ' ' });
                let response = grid.to_string();

                Ok(Box::new(response))
            }
        }
    }
}
