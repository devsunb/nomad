use crate::{Metric, Point2};

/// TODO: docs.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point<M: Metric> {
    inner: Point2<M, M>,
}

impl<M: Metric> Default for Point<M> {
    #[inline]
    fn default() -> Self {
        Self { inner: Point2::default() }
    }
}

impl<M: Metric> Point<M> {
    /// Creates a new [`Point`].
    #[inline]
    pub fn new(x: M, y: M) -> Self {
        Self { inner: Point2::new(x, y) }
    }

    /// The `x` coordinate of the point.
    #[inline]
    pub fn x(&self) -> M {
        self.inner.x()
    }

    /// The `y` coordinate of the point.
    #[inline]
    pub fn y(&self) -> M {
        self.inner.y()
    }
}
