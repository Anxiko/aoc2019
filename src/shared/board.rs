use crate::shared::coord::Coord;

mod grid;
mod hash_board;

pub(crate) use grid::Grid;
pub(crate) use hash_board::HashBoard;

pub(crate) trait Board<T> {
    type Error;

    fn read(&self, coord: Coord) -> Result<&T, Self::Error>;

    fn write(&mut self, coord: Coord, value: T) -> Result<(), Self::Error>;

    fn elements<'a>(&'a self) -> impl Iterator<Item = (Coord, &'a T)>
    where
        T: 'a;
}
