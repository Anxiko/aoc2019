use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use strum_macros::EnumString;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Ord, PartialOrd)]
pub struct Coord {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Coord {
    pub(crate) fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    pub(crate) fn manhattan(&self) -> u32 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }

    pub(crate) fn module(&self) -> f64 {
        let x = self.x as f64;
        let y = self.y as f64;

        (x * x + y * y).sqrt()
    }

    pub(crate) fn cross(&self) -> impl Iterator<Item = Self> {
        CLOCKWISE
            .iter()
            .copied()
            .map(Self::from)
            .map(|delta| *self + delta)
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
        *self = *self + rhs;
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Coord {
    type Output = Coord;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
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

const CLOCKWISE: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

impl Direction {
    pub(crate) fn turn_right(&self) -> Self {
        let idx = CLOCKWISE
            .iter()
            .position(|d| d == self)
            .expect("Find direction position");

        let idx = (idx + 1) % CLOCKWISE.len();

        CLOCKWISE[idx]
    }

    pub(crate) fn turn_left(&self) -> Self {
        let idx = CLOCKWISE
            .iter()
            .position(|d| d == self)
            .expect("Find direction position");

        let idx = (CLOCKWISE.len() + idx - 1) % CLOCKWISE.len();

        CLOCKWISE[idx]
    }
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

impl TryFrom<Coord> for Direction {
    type Error = Coord;

    fn try_from(value: Coord) -> Result<Self, Self::Error> {
        match value {
            Coord { x: 0, y: -1 } => Ok(Direction::Up),
            Coord { x: 1, y: 0 } => Ok(Direction::Right),
            Coord { x: 0, y: 1 } => Ok(Direction::Down),
            Coord { x: -1, y: 0 } => Ok(Direction::Left),
            value => Err(value),
        }
    }
}
