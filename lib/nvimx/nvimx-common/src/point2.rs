use crate::Metric;

/// TODO: docs.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point2<X: Metric, Y: Metric> {
    x: X,
    y: Y,
}

impl<X: Metric, Y: Metric> Default for Point2<X, Y> {
    #[inline]
    fn default() -> Self {
        Self { x: X::zero(), y: Y::zero() }
    }
}

impl<X: Metric, Y: Metric> Point2<X, Y> {
    /// Creates a new [`Point2`].
    #[inline]
    pub fn new(x: X, y: Y) -> Self {
        Self { x, y }
    }

    /// The `x` coordinate of the point.
    #[inline]
    pub fn x(&self) -> X {
        self.x
    }

    /// The `y` coordinate of the point.
    #[inline]
    pub fn y(&self) -> Y {
        self.y
    }
}
