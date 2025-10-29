use crate::shared::board::{Board, Grid};
use crate::shared::coord::Coord;
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::Infallible;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct HashBoard<T> {
    coord_mapping: HashMap<Coord, T>,
    default: T,
}

impl<T: Clone> HashBoard<T> {
    pub(crate) fn new(default: T) -> Self {
        Self {
            coord_mapping: HashMap::new(),
            default,
        }
    }

    pub(crate) fn as_grid(&self) -> Option<Grid<T>> {
        let (min_x, max_x) = self
            .coord_mapping
            .keys()
            .copied()
            .map(|coord| coord.x)
            .minmax()
            .into_option()?;

        let (min_y, max_y) = self
            .coord_mapping
            .keys()
            .copied()
            .map(|coord| coord.y)
            .minmax()
            .into_option()?;

        let width = max_x.abs_diff(min_x) as usize + 1;
        let height = max_y.abs_diff(min_y) as usize + 1;

        let mut grid = Grid::with_value(width, height, self.default.clone());

        self.coord_mapping.iter().for_each(|(&coord, value)| {
            grid.write(coord, value.clone())
                .expect("Copy values to grid");
        });

        Some(grid)
    }
}

impl<T> Board<T> for HashBoard<T> {
    type Error = Infallible;

    fn read(&self, coord: Coord) -> Result<&T, Self::Error> {
        Ok(self.coord_mapping.get(&coord).unwrap_or(&self.default))
    }

    fn write(&mut self, coord: Coord, value: T) -> Result<(), Self::Error> {
        self.coord_mapping.insert(coord, value);

        Ok(())
    }
}
