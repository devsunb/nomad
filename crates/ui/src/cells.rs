use core::ops::{Add, AddAssign};

/// TODO: docs
#[derive(Debug, Copy, Clone, Default)]
pub struct Cells(u32);

impl From<u32> for Cells {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Add for Cells {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl AddAssign for Cells {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
