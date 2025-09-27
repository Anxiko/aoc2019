use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};
use strum_macros::EnumString;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    pub(crate) fn manhattan(&self) -> u32 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<Coord> for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Default for Coord {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumString, strum_macros::Display)]
pub enum Direction {
    #[strum(serialize = "U")]
    Up,
    #[strum(serialize = "R")]
    Right,
    #[strum(serialize = "D")]
    Down,
    #[strum(serialize = "L")]
    Left,
}

impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Coord::new(0, -1),
            Direction::Right => Coord::new(1, 0),
            Direction::Down => Coord::new(0, 1),
            Direction::Left => Coord::new(-1, 0),
        }
    }
}
