use crate::shared::board::Board;
use crate::shared::coord::Coord;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Grid<T> {
    rows: Vec<Vec<T>>,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GridAccessError {
    #[error("invalid row {0}")]
    InvalidRow(i32),
    #[error("invalid column {0}")]
    InvalidCol(i32),
    #[error("row {0} out of bounds")]
    OutOfBoundsRow(usize),
    #[error("column {0} out of bounds")]
    OutOfBoundsCol(usize),
}

impl<T: Clone> Grid<T> {
    pub(crate) fn new(rows: Vec<Vec<T>>) -> Result<Self, anyhow::Error> {
        match rows.iter().map(Vec::len).all_equal_value() {
            Ok(_) | Err(None) => Ok(Self { rows }),
            Err(Some((first, second))) => Err(anyhow::anyhow!(
                "Mismatch on board row sizes, found {first} and {second}"
            )),
        }
    }

    pub(crate) fn with_value(width: usize, height: usize, value: T) -> Self {
        Self::new(vec![vec![value; width]; height]).expect("Build an empty board")
    }

    #[allow(dead_code)]
    pub(crate) fn width(&self) -> usize {
        self.rows.first().map(Vec::len).unwrap_or_default()
    }

    #[allow(dead_code)]
    pub(crate) fn height(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn coord_to_col_row(coord: Coord) -> Result<(usize, usize), GridAccessError> {
        let col = usize::try_from(coord.x).map_err(|_| GridAccessError::InvalidCol(coord.x))?;
        let row = usize::try_from(coord.y).map_err(|_| GridAccessError::InvalidRow(coord.y))?;

        Ok((col, row))
    }

    pub(crate) fn map<O: Clone>(self, mapper: fn(T) -> O) -> Grid<O> {
        let rows = self
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(mapper).collect_vec())
            .collect_vec();

        Grid::new(rows).expect("Map a grid into another")
    }

	#[allow(unused)]
    pub(crate) fn map_tuples<O: Clone, F: Fn((Coord, T)) -> O + Copy>(self, mapper: F) -> Grid<O> {
        let rows = self
            .rows
            .into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(x, element)| (Coord::new(x as i32, y as i32), element))
	                .map(mapper)
                    .collect_vec()
            })
            .collect_vec();

        Grid::new(rows).expect("Map a grid into another")
    }
}

impl<T: Clone + Debug> Board<T> for Grid<T> {
    fn read(&self, coord: Coord) -> Result<&T, anyhow::Error> {
        let (col, row) = Self::coord_to_col_row(coord)?;

        let row = self
            .rows
            .get(row)
            .ok_or(GridAccessError::OutOfBoundsRow(row))?;
        let element = row.get(col).ok_or(GridAccessError::OutOfBoundsCol(col))?;

        Ok(element)
    }

    fn write(&mut self, coord: Coord, value: T) -> Result<(), anyhow::Error> {
        let (col, row) = Self::coord_to_col_row(coord)?;

        let row = self
            .rows
            .get_mut(row)
            .ok_or(GridAccessError::OutOfBoundsRow(row))?;
        let element = row
            .get_mut(col)
            .ok_or(GridAccessError::OutOfBoundsCol(col))?;

        *element = value;

        Ok(())
    }

    fn elements(&self) -> Vec<(Coord, T)> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, element)| (Coord::new(x as i32, y as i32), element.clone()))
            })
            .collect_vec()
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for element in row {
                write!(f, "{element}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
