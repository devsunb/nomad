use core::ops::{Add, AddAssign};

use crate::{ExpandRect, Metric};

/// TODO: docs
#[derive(Copy, Clone, Debug)]
pub struct Bound<T: Metric> {
    height: T,
    width: T,
}

impl<T: Metric> Bound<T> {
    /// Creates a new empty `Bound`.
    #[inline]
    pub(crate) fn empty() -> Self {
        Self { height: T::zero(), width: T::zero() }
    }
}

impl<T: Metric> AddAssign<ExpandRect<T>> for Bound<T> {
    #[inline]
    fn add_assign(&mut self, expand: ExpandRect<T>) {
        self.height = self.height + expand.top + expand.bottom;
        self.width = self.width + expand.left + expand.right;
    }
}

impl<T: Metric> Add<ExpandRect<T>> for Bound<T> {
    type Output = Self;

    #[inline]
    fn add(mut self, expand: ExpandRect<T>) -> Self {
        self += expand;
        self
    }
}
