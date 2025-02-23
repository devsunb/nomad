use core::error::Error;

use crate::{ByteOffset, fs};

/// TODO: docs.
pub trait File {
    /// TODO: docs.
    type Fs: fs::Fs;

    /// TODO: docs.
    type Error: Error;

    /// TODO: docs.
    fn len(&self) -> impl Future<Output = Result<ByteOffset, Self::Error>>;
}
