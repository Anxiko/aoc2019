use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub(crate) struct Vect3 {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}

impl Display for Vect3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

impl Vect3 {
    pub(crate) fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Add<Vect3> for Vect3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vect3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub<Vect3> for Vect3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
