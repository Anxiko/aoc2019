use crate::day::{DayPart, DaySolver};
use crate::intcode::IntMachine;
use crate::parsers::parse_intmachine_input;
use crate::shared::a_star::maze::{CostCalculator, MazeSolver, MazeState};
use crate::shared::a_star::{State, a_star};
use crate::shared::board::{Board, HashBoard};
use crate::shared::coord::{Coord, Direction};
use crate::types::IntCell;
use anyhow::Error;
use itertools::Itertools;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Display, Formatter};

pub(crate) struct Day15 {}

impl Day15 {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
enum Tile {
    #[default]
    Unknown,
    Space,
    Wall,
}

impl Tile {
    fn is_space(&self) -> bool {
        matches!(self, Self::Space)
    }

    fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum MovementResult {
    WallHit = 0,
    Moved = 1,
    ReachedDestination = 2,
}

impl MovementResult {
    fn moved_ok(&self) -> bool {
        match self {
            Self::Moved | Self::ReachedDestination => true,
            Self::WallHit => false,
        }
    }

    fn is_destination(&self) -> bool {
        self == &Self::ReachedDestination
    }
}

impl From<MovementResult> for Tile {
    fn from(value: MovementResult) -> Self {
        if value.moved_ok() {
            Self::Space
        } else {
            Self::Wall
        }
    }
}

struct TileBoardCostCalculator {
    destination: Coord,
    allow_all_unknown: bool,
}
impl TileBoardCostCalculator {
    fn new(destination: Coord, allow_all_unknown: bool) -> Self {
        Self {
            destination,
            allow_all_unknown,
        }
    }
}

impl CostCalculator<Tile> for TileBoardCostCalculator {
    fn calculate_cost(
        &self,
        _from: (Coord, &Tile),
        (to_coord, to_tile): (Coord, &Tile),
    ) -> Option<u64> {
        if to_tile.is_space()
            || (to_tile.is_unknown() && (self.allow_all_unknown || to_coord == self.destination))
        {
            Some(1)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Droid {
    machine: IntMachine,
    position: Coord,
    board: Box<HashBoard<Tile>>,
}

impl Droid {
    fn new(memory: Vec<IntCell>) -> Self {
        let machine = IntMachine::new(memory);

        let position = Coord::new(0, 0);
        let mut board = Box::new(HashBoard::default());
        board
            .write(position, Tile::Space)
            .expect("Initialize board");

        Self {
            machine,
            position,
            board,
        }
    }

    fn closest_unknown(&self) -> Coord {
        let mut active = BinaryHeap::from([Reverse((0, self.position))]);
        let mut seen = HashSet::from([self.position]);

        while let Some(Reverse((cost, coord))) = active.pop() {
            match self.board.read(coord).expect("Read board") {
                Tile::Unknown => return coord,
                Tile::Wall => continue,
                Tile::Space => {
                    let candidates = coord.cross().filter(|c| !seen.contains(c)).collect_vec();

                    seen.extend(candidates.clone());
                    active.extend(candidates.into_iter().map(|c| Reverse((cost + 1, c))));
                }
            }
        }

        panic!("Could not find any unknown tile from {}", self.position);
    }

    fn move_(&mut self, direction: Direction) -> MovementResult {
        self.machine
            .add_input(Self::direction_to_intcell(direction));
        let movement_result = self.machine.run_until_output().expect("Move droid");

        let movement_result = u8::try_from(movement_result).expect("Movement result to u8");
        let movement_result: MovementResult =
            movement_result.try_into().expect("Parse movement result");

        let destination = self.position + direction.into();

        let tile = if movement_result.moved_ok() {
            self.position = destination;
            Tile::Space
        } else {
            Tile::Wall
        };
        self.board.write(destination, tile).expect("Update board");

        movement_result
    }

    fn move_to(&mut self, destination: Coord) -> MovementResult {
        let mut path = self.shortest_path(self.position, destination, false);
        let final_direction = path
            .pop()
            .unwrap_or_else(|| panic!("Found no path to {destination} from {}", self.position));

        for direction in path {
            match self.move_(direction) {
                MovementResult::Moved => {}
                MovementResult::WallHit => {
                    panic!("Hit a wall approaching destination: {destination}")
                }
                MovementResult::ReachedDestination => {
                    panic!("Reached target approaching destination: {destination}")
                }
            }
        }

        let final_movement = self.move_(final_direction);

        self.board
            .write(destination, final_movement.into())
            .expect("Write to board");

        final_movement
    }

    fn direction_to_intcell(dir: Direction) -> IntCell {
        match dir {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Right => 4,
        }
    }

    fn shortest_path(
        &self,
        start: Coord,
        destination: Coord,
        allow_all_unknown: bool,
    ) -> Vec<Direction> {
        let maze_solver = MazeSolver::new(
            destination,
            Box::new(TileBoardCostCalculator::new(destination, allow_all_unknown)),
        );

        let initial_state = MazeState::initial(start, self.board.as_ref(), &maze_solver);

        let final_state =
            a_star(initial_state).unwrap_or_else(|| panic!("Found no path to {destination}"));

        final_state
            .path()
            .into_iter()
            .tuple_windows()
            .map(|(from, to)| Direction::try_from(to - from))
            .try_collect()
            .expect("Should be able to turn path into directions")
    }

    fn explore_closest(&mut self) -> Option<Coord> {
        let closest = self.closest_unknown();
        let movement_result = self.move_to(closest);

        if movement_result.is_destination() {
            Some(closest)
        } else {
            None
        }
    }

    fn find_shortest_path(&mut self, destination: Coord) -> Vec<Direction> {
        loop {
            let path = self.shortest_path(Coord::default(), destination, true);
            if self.verify_path(path.clone()) {
                return path;
            }
        }
    }

    fn verify_path(&mut self, mut path: Vec<Direction>) -> bool {
        let start = Coord::default();
        if self.position != start && self.move_to(start) != MovementResult::Moved {
            panic!("Failed to moved to the start");
        }

        let last_direction = path.pop().expect("At least one direction in path");

        for direction in path {
            match self.move_(direction) {
                MovementResult::Moved => {}
                MovementResult::WallHit => {
                    self.board
                        .write(self.position + direction.into(), Tile::Wall)
                        .expect("Write to board");
                    return false;
                }
                MovementResult::ReachedDestination => {
                    unreachable!("Should not reach destination mid path")
                }
            }
        }

        match self.move_(last_direction) {
            MovementResult::ReachedDestination => true,
            MovementResult::WallHit => false,
            MovementResult::Moved => panic!("Did not reach destination in last step"),
        }
    }

    fn check_if_coord_is_space(&mut self, coord: Coord) -> bool {
        match self.board.read(coord).expect("Read the board") {
            Tile::Space => true,
            Tile::Wall => false,
            Tile::Unknown => self.move_to(coord).moved_ok(),
        }
    }

    fn gas_fill(&mut self) -> u32 {
        let mut active = vec![self.position];
        let mut current_distance = 0;
        let mut seen: HashSet<Coord> = HashSet::new();

        loop {
            let next_active = active
                .iter()
	            .copied()
                .filter(|coord| !seen.contains(coord))
                .filter(|coord| self.check_if_coord_is_space(*coord))
                .flat_map(|coord| coord.cross().collect_vec())
                .collect_vec();

            seen.extend(active.iter().copied());

            if next_active.is_empty() {
                return current_distance;
            }

            current_distance += 1;
            active = next_active;
        }
    }
}

impl Display for Droid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Droid at {}\n{:?}", self.position, self.board)
    }
}

impl DaySolver for Day15 {
    fn solve_part(
        &self,
        part: DayPart,
        _example: bool,
        input: &[&str],
    ) -> Result<Box<dyn ToString>, Error> {
        let memory = parse_intmachine_input(input)?;
        let mut droid = Droid::new(memory);

        let destination = loop {
            if let Some(destination) = droid.explore_closest() {
                break destination;
            }
        };

        match part {
            DayPart::Part1 => {
                let shortest_path = droid.find_shortest_path(destination);

                Ok(Box::new(shortest_path.len()))
            }
            DayPart::Part2 => {
                let gas_fill_time = droid.gas_fill();
                Ok(Box::new(gas_fill_time - 1))
            }
        }
    }
}
