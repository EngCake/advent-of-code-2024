use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Position {
    pub i: i32,
    pub j: i32,
}

impl Position {
    pub fn rotate_clockwise(&self) -> Self {
        Self {
            i: self.j,
            j: -self.i,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { i: 0, j: 0 }
    }
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self {
            i: value.0,
            j: value.1,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            i: value.0 as i32,
            j: value.1 as i32,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.i += rhs.i;
        self.j += rhs.j;
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.i -= rhs.i;
        self.j -= rhs.j;
    }
}
