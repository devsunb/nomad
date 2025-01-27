use nvimx2::fs;

use crate::accumulate::{self, AccumulateError, Accumulator};
use crate::filter::{Filter, Filtered};

/// TODO: docs.
pub trait WalkDir<Fs: fs::Fs>: Sized {
    /// TODO: docs.
    #[inline]
    fn accumulate<A>(
        &self,
        acc: &mut A,
        fs: &mut Fs,
    ) -> impl Future<Output = Result<Fs::Timestamp, AccumulateError<A, Self, Fs>>>
    where
        A: Accumulator<Fs>,
    {
        async move { accumulate::accumulate(self, acc, fs).await }
    }

    /// TODO: docs.
    #[inline]
    fn filter<F>(self, filter: F) -> Filtered<F, Self>
    where
        F: Filter,
    {
        Filtered::new(filter, self)
    }
}
