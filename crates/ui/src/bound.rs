use core::ops::{Add, AddAssign};

use crate::ExpandRect;

/// TODO: docs
#[derive(Copy, Clone, Debug)]
pub struct Bound<T> {
    height: T,
    width: T,
}

impl<T> AddAssign<ExpandRect<T>> for Bound<T>
where
    T: Copy + Add<Output = T>,
{
    #[inline]
    fn add_assign(&mut self, expand: ExpandRect<T>) {
        self.height = self.height + expand.top + expand.bottom;
        self.width = self.width + expand.left + expand.right;
    }
}

impl<T> Add<ExpandRect<T>> for Bound<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(mut self, expand: ExpandRect<T>) -> Self {
        self += expand;
        self
    }
}
