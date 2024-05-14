use crate::{Bound, Metric};

/// TODO: docs.
pub enum RequestedBound<T: Metric> {
    /// TODO: docs.
    Exact(Bound<T>),

    /// TODO: docs.
    Available,
}

impl<T: Metric> RequestedBound<T> {
    /// Creates a new empty `RequestedBound`.
    #[inline]
    pub fn empty() -> Self {
        Self::Exact(Bound::empty())
    }

    /// Maps a `RequestedBound<T>` to a `RequestedBound<U>` by applying the
    /// given function to the `Bound<T>` if self is `Exact`, or returns
    /// `Available` otherwise.
    #[inline]
    pub fn map<F, U>(self, f: F) -> RequestedBound<U>
    where
        F: FnOnce(Bound<T>) -> Bound<U>,
        U: Metric,
    {
        match self {
            Self::Exact(bound) => RequestedBound::Exact(f(bound)),
            Self::Available => RequestedBound::Available,
        }
    }
}
