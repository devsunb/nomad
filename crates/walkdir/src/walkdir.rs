use nvimx2::fs;

use crate::{Accumulator, Filter};

/// TODO: docs.
pub trait WalkDir<Fs: fs::Fs>: Sized {
    /// TODO: docs.
    #[inline]
    fn accumulate<A>(
        &self,
        _acc: &mut A,
    ) -> impl Future<Output = Result<Fs::Timestamp, AccumulateError<A, Self, Fs>>>
    where
        A: Accumulator<Fs>,
    {
        async move {
            todo!();
        }
    }

    /// TODO: docs.
    #[inline]
    fn filter<F>(self, filter: F) -> Filtered<F, Self>
    where
        F: Filter,
    {
        Filtered { _filter: filter, _inner: self }
    }
}

/// TODO: docs.
pub struct Filtered<F, W> {
    _filter: F,
    _inner: W,
}

/// TODO: docs.
pub enum AccumulateError<A, W, Fs> {
    _Acc(A),
    _Walk(W),
    _Fs(Fs),
}

impl<F, W, Fs> WalkDir<Fs> for Filtered<F, W>
where
    F: Filter,
    W: WalkDir<Fs>,
    Fs: fs::Fs,
{
}
