use crate::Filter;

/// TODO: docs.
pub trait WalkDir: Sized {
    /// TODO: docs.
    #[inline]
    fn filter<F: Filter>(self, filter: F) -> Filtered<F, Self> {
        Filtered { _filter: filter, _inner: self }
    }
}

/// TODO: docs.
pub struct Filtered<F, W> {
    _filter: F,
    _inner: W,
}

impl<F, W> WalkDir for Filtered<F, W>
where
    F: Filter,
    W: WalkDir,
{
}
