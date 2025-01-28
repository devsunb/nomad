use futures_util::Stream;
use nvimx2::fs;

use crate::accumulate::{self, AccumulateError, Accumulator};
use crate::filter::{Filter, Filtered};

/// TODO: docs.
pub trait WalkDir: Sized {
    /// TODO: docs.
    type DirEntry: fs::DirEntry;

    /// TODO: docs.
    type DirEntryError;

    /// TODO: docs.
    type ReadDirError;

    /// TODO: docs.
    fn read_dir(
        &self,
        dir_path: &fs::AbsPath,
    ) -> impl Future<
        Output = Result<
            impl Stream<Item = Result<Self::DirEntry, Self::DirEntryError>>,
            Self::ReadDirError,
        >,
    >;

    /// TODO: docs.
    #[inline]
    fn accumulate<A, Fs>(
        &self,
        acc: &mut A,
        fs: &mut Fs,
    ) -> impl Future<Output = Result<Fs::Timestamp, AccumulateError<A, Self, Fs>>>
    where
        A: Accumulator<Fs>,
        Fs: fs::Fs,
    {
        async move { accumulate::accumulate(self, acc, fs).await }
    }

    /// TODO: docs.
    #[inline]
    fn filter<F>(self, filter: F) -> Filtered<F, Self>
    where
        F: Filter<Self>,
    {
        Filtered::new(filter, self)
    }
}

impl<Fs: fs::Fs> WalkDir for Fs {
    type DirEntry = <Self as fs::Fs>::DirEntry;
    type DirEntryError = <Self as fs::Fs>::DirEntryError;
    type ReadDirError = <Self as fs::Fs>::ReadDirError;

    async fn read_dir(
        &self,
        _dir_path: &fs::AbsPath,
    ) -> Result<<Self as fs::Fs>::ReadDir, Self::ReadDirError> {
        todo!()
    }
}
