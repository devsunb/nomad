use core::error::Error;

use futures_lite::Stream;

use crate::fs::{AbsPath, Fs, Metadata};

/// TODO: docs.
pub trait Directory {
    /// TODO: docs.
    type Fs: Fs;

    /// TODO: docs.
    type Metadata: Metadata<Timestamp = <Self::Fs as Fs>::Timestamp>;

    /// TODO: docs.
    type ReadEntryError: Error;

    /// TODO: docs.
    type ReadError: Error;

    /// TODO: docs.
    fn read(
        &self,
    ) -> impl Future<
        Output = Result<
            impl Stream<Item = Result<Self::Metadata, Self::ReadEntryError>>
            + use<Self>,
            Self::ReadError,
        >,
    >;

    /// TODO: docs.
    fn parent(
        &self,
    ) -> impl Future<Output = Option<<Self::Fs as Fs>::Directory>>;

    /// TODO: docs.
    fn path(&self) -> &AbsPath;
}
