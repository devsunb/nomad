use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::ByteLen;

/// A byte offset in a buffer.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteOffset(usize);

impl ByteOffset {
    /// Returns the byte offset as a `usize`.
    #[inline]
    pub fn as_usize(&self) -> usize {
        self.0
    }

    /// Creates a new `ByteOffset` with the given offset.
    #[inline]
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }
}

impl Add<ByteLen> for ByteOffset {
    type Output = Self;

    #[inline]
    fn add(self, len: ByteLen) -> Self {
        Self(self.as_usize() + len.as_usize())
    }
}

impl AddAssign<ByteLen> for ByteOffset {
    #[inline]
    fn add_assign(&mut self, len: ByteLen) {
        self.0 += len.as_usize();
    }
}

impl Sub<ByteLen> for ByteOffset {
    type Output = Self;

    #[inline]
    fn sub(self, len: ByteLen) -> Self {
        Self(self.as_usize() - len.as_usize())
    }
}

impl SubAssign<ByteLen> for ByteOffset {
    #[inline]
    fn sub_assign(&mut self, len: ByteLen) {
        self.0 -= len.as_usize();
    }
}

impl From<usize> for ByteOffset {
    #[inline]
    fn from(offset: usize) -> Self {
        Self::new(offset)
    }
}

impl From<ByteOffset> for usize {
    #[inline]
    fn from(offset: ByteOffset) -> usize {
        offset.as_usize()
    }
}
