use std::fmt::Debug;
use crate::shared::coord::Coord;

mod grid;
mod hash_board;

pub(crate) use grid::Grid;
pub(crate) use hash_board::HashBoard;

pub(crate) trait Board<T: Clone> : Debug {
    fn read(&self, coord: Coord) -> Result<&T, anyhow::Error>;

    fn write(&mut self, coord: Coord, value: T) -> Result<(), anyhow::Error>;

    fn elements(&self) -> Vec<(Coord, T)>;
}
