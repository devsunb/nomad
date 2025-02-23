use core::error::Error;

use crate::fs::{FsNodeKind, FsNodeNameBuf};

/// TODO: docs.
pub trait Metadata<Ts> {
    /// TODO: docs.
    type Error: Error;

    /// TODO: docs.
    type NameError: Error;

    /// TODO: docs.
    type NodeKindError: Error;

    /// TODO: docs.
    fn created_at(
        &self,
    ) -> impl Future<Output = Result<Option<Ts>, Self::Error>>;

    /// TODO: docs.
    fn last_modified_at(
        &self,
    ) -> impl Future<Output = Result<Option<Ts>, Self::Error>>;

    /// TODO: docs.
    fn name(
        &self,
    ) -> impl Future<Output = Result<FsNodeNameBuf, Self::NameError>>;

    /// TODO: docs.
    fn node_kind(
        &self,
    ) -> impl Future<Output = Result<FsNodeKind, Self::NodeKindError>>;
}
