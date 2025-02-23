use core::error::Error;

use crate::fs::{self, FsNode};

/// TODO: docs.
pub trait Symlink {
    /// TODO: docs.
    type Fs: fs::Fs;

    /// TODO: docs.
    type FollowError: Error;

    /// TODO: docs.
    fn follow(
        &self,
    ) -> impl Future<Output = Result<Option<FsNode<Self::Fs>>, Self::FollowError>>;

    /// TODO: docs.
    fn follow_recursively(
        &self,
    ) -> impl Future<Output = Result<Option<FsNode<Self::Fs>>, Self::FollowError>>;
}
