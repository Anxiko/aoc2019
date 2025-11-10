use crate::shared::board::{Board, Grid};
use crate::shared::coord::Coord;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct HashBoard<T> {
    pub(crate) coord_mapping: HashMap<Coord, T>,
    default: T,
}

impl<T: Clone + Debug> HashBoard<T> {
    pub(crate) fn new(default: T) -> Self {
        Self {
            coord_mapping: HashMap::new(),
            default,
        }
    }

    pub(crate) fn as_grid(&self) -> Option<(Coord, Grid<T>)> {
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

	    let delta = Coord::new(min_x, min_y);

        self.coord_mapping.iter().for_each(|(&coord, value)| {
            grid.write(coord - delta, value.clone())
                .expect("Copy values to grid");
        });

        Some((delta, grid))
    }
}

impl<T: Clone + Default + Debug> Default for HashBoard<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Clone + Debug> Board<T> for HashBoard<T> {
    fn read(&self, coord: Coord) -> Result<&T, anyhow::Error> {
        Ok(self.coord_mapping.get(&coord).unwrap_or(&self.default))
    }

    fn write(&mut self, coord: Coord, value: T) -> Result<(), anyhow::Error> {
        self.coord_mapping.insert(coord, value);

        Ok(())
    }

    fn elements(&self) -> Vec<(Coord, T)> {
        self.coord_mapping
            .iter()
            .map(|(&coord, value)| (coord, value.clone()))
            .collect_vec()
    }
}
