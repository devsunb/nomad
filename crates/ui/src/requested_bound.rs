use crate::Bound;

/// TODO: docs.
pub enum RequestedBound<T> {
    /// TODO: docs.
    Exact(Bound<T>),

    /// TODO: docs.
    Available,
}

impl<T> RequestedBound<T> {
    /// Maps a `RequestedBound<T>` to a `RequestedBound<U>` by applying the
    /// given function to the `Bound<T>` if self is `Exact`, or returns
    /// `Available` otherwise.
    #[inline]
    pub fn map<F, U>(self, f: F) -> RequestedBound<U>
    where
        F: FnOnce(Bound<T>) -> Bound<U>,
    {
        match self {
            Self::Exact(bound) => RequestedBound::Exact(f(bound)),
            Self::Available => RequestedBound::Available,
        }
    }
}
