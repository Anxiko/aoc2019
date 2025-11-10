use crate::shared::a_star::State;
use crate::shared::board::Board;
use crate::shared::coord::Coord;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub(crate) trait CostCalculator<T> {
    fn calculate_cost(&self, from: (Coord, &T), to: (Coord, &T)) -> Option<u64>;
}

/// Cost calculator for a board of booleans, where true are traversable. Cost between neighbours is always 1
#[allow(unused)]
struct BoolBoardCostCalculator {}

#[allow(unused)]
impl BoolBoardCostCalculator {
    fn new() -> Self {
        Self {}
    }
}

impl CostCalculator<bool> for BoolBoardCostCalculator {
    fn calculate_cost(
        &self,
        _from: (Coord, &bool),
        (_to_coord, to_bool): (Coord, &bool),
    ) -> Option<u64> {
        if *to_bool { Some(1) } else { None }
    }
}

pub(crate) struct MazeSolver<T> {
    destination: Coord,
    cost_calculator: Box<dyn CostCalculator<T>>,
}

impl<T> MazeSolver<T> {
    pub(crate) fn new(destination: Coord, cost_calculator: Box<dyn CostCalculator<T>>) -> Self {
        Self {
            destination,
            cost_calculator,
        }
    }
}

#[derive(Clone)]
pub(crate) struct MazeState<'a, T: Clone> {
    coord: Coord,
    cost: u64,
    board: &'a dyn Board<T>,
    solver: &'a MazeSolver<T>,
    path: Vec<Coord>,
}

impl<'a, T: Clone> MazeState<'a, T> {
    pub(crate) fn initial(coord: Coord, board: &'a dyn Board<T>, solver: &'a MazeSolver<T>) -> Self {
        Self {
            coord,
            cost: 0,
            board,
            solver,
            path: Vec::new(),
        }
    }

    fn neighbour(&self, n_coord: Coord, extra_cost: u64) -> Self {
        let mut n_path = self.path.clone();
        n_path.push(self.coord);

        Self {
            coord: n_coord,
            cost: self.cost + extra_cost,
            board: self.board,
            solver: self.solver,
            path: n_path,
        }
    }
}

impl<'a, T: Clone> PartialEq<Self> for MazeState<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<'a, T: Clone> Eq for MazeState<'a, T> {}

impl<'a, T: Clone> Ord for MazeState<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl<'a, T: Clone> PartialOrd for MazeState<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, T: Clone> Hash for MazeState<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
        self.cost.hash(state);
    }
}

impl<'a, T: Clone> State for MazeState<'a, T> {
    type Position = Coord;

    fn cost(&self) -> u64 {
        self.cost
    }

    fn heuristic(&self) -> u64 {
        (self.solver.destination - self.coord).manhattan().into()
    }

    fn position(&self) -> Self::Position {
        self.coord
    }

    fn is_final(&self) -> bool {
        self.coord == self.solver.destination
    }

    fn neighbours(&self) -> Vec<Self> {
        let tile = self.board.read(self.coord).expect("Read my own coord");
        self.coord
            .cross()
            .filter_map(|neighbour_coord| {
                self.board
                    .read(neighbour_coord)
                    .ok()
                    .map(|tile| (neighbour_coord, tile))
            })
            .filter_map(|(n_coord, n_tile)| {
                self.solver
                    .cost_calculator
                    .calculate_cost((self.coord, tile), (n_coord, n_tile))
                    .map(|extra_cost| (n_coord, extra_cost))
            })
            .map(|(n_coord, extra_cost)| self.neighbour(n_coord, extra_cost))
            .collect()
    }

    fn path(&self) -> Vec<Self::Position> {
        let mut path = self.path.clone();
        path.push(self.coord);

        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::a_star::a_star;
    use crate::shared::board::Grid;
    use crate::shared::coord::Direction;
    use itertools::Itertools;

    #[test]
    fn maze() {
        #[rustfmt::skip]
        let maze = [
            "..#..",
	        "..E##",
	        "....#",
	        "..#.#",
	        "..###",
	        "...#.",
	        "##.#.",
	        ".#S#.",
        ];

        let rows = maze
            .iter()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let grid = Grid::new(rows).unwrap();

        let start = grid
            .elements()
            .iter()
            .find_map(|(coord, element)| if *element == 'S' { Some(coord) } else { None })
            .copied()
            .unwrap();

        let end = grid
            .elements()
            .iter()
            .find_map(|(coord, element)| if *element == 'E' { Some(coord) } else { None })
            .copied()
            .unwrap();

        let grid = grid.map(|char| char != '.');

        let maze_solver = MazeSolver::new(end, Box::new(BoolBoardCostCalculator::new()));
        let initial_state = MazeState::initial(start, &grid, &maze_solver);

        let expected_path = std::iter::once(start)
            .chain(
                vec![
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Up,
                    Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Left,
                ]
                .into_iter()
                .scan(start, |state, dir| {
                    *state += dir.into();
                    Some(*state)
                }),
            )
            .collect_vec();

        let end_state = a_star(initial_state).expect("Find a solution to the maze");
        assert_eq!(end_state.coord, end);
        assert_eq!(end_state.total_cost(), 10);
	    assert_eq!(end_state.heuristic(), 0);
        assert_eq!(end_state.path(), expected_path);
    }
}
