use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::ByteOffset;

/// A byte offset in a buffer.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteLen(usize);

impl ByteLen {
    /// Returns the byte len as a `usize`.
    #[inline]
    pub fn as_usize(&self) -> usize {
        self.0
    }

    /// Creates a new `ByteLen` with the given len.
    #[inline]
    pub fn new(len: usize) -> Self {
        Self(len)
    }
}

impl Add<Self> for ByteLen {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self(self.as_usize() + other.as_usize())
    }
}

impl Add<ByteOffset> for ByteLen {
    type Output = ByteOffset;

    #[inline]
    fn add(self, offset: ByteOffset) -> ByteOffset {
        ByteOffset::new(self.as_usize() + offset.as_usize())
    }
}

impl AddAssign<Self> for ByteLen {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.as_usize();
    }
}

impl Sub<Self> for ByteLen {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.as_usize() - rhs.as_usize())
    }
}

impl SubAssign<Self> for ByteLen {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.as_usize();
    }
}

impl From<usize> for ByteLen {
    #[inline]
    fn from(offset: usize) -> Self {
        Self::new(offset)
    }
}

impl From<ByteLen> for usize {
    #[inline]
    fn from(offset: ByteLen) -> usize {
        offset.as_usize()
    }
}
