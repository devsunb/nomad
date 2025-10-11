use std::sync::Arc;

use collab_types::bytes::Bytes;
use collab_types::crop::Rope;
use collab_types::puff;
use puff::builder::CreateError;
use puff::directory::LocalDirectoryId;
use puff::file::LocalFileId;
use puff::node::LocalNodeId;

use crate::abs_path::AbsPath;
use crate::fs::{FileContents, FsBuilder};
use crate::symlink::SymlinkContents;
use crate::text::TextContents;
use crate::{Project, binary};

/// TODO: docs.
pub struct ProjectBuilder {
    pub(crate) inner: FsBuilder,
    pub(crate) binary_ctx: binary::BinaryCtx,
}

impl ProjectBuilder {
    /// TODO: docs.
    #[inline]
    pub fn build(self) -> Project {
        Project::from_builder(self)
    }

    /// TODO: docs.
    #[inline]
    pub fn push_binary_file(
        &mut self,
        file_path: impl AsRef<AbsPath>,
        file_contents: impl Into<Bytes>,
    ) -> Result<LocalFileId, CreateError> {
        let contents =
            FileContents::Binary(binary::BinaryContents::new_local(
                self.inner.peer_id().into(),
                file_contents.into(),
                &mut self.binary_ctx,
            ));
        self.inner.push_file(file_path, contents)
    }

    /// TODO: docs.
    #[inline]
    pub fn push_symlink(
        &mut self,
        symlink_path: impl AsRef<AbsPath>,
        symlink_target_path: impl Into<Arc<str>>,
    ) -> Result<LocalFileId, CreateError> {
        let contents = FileContents::Symlink(SymlinkContents::new(
            symlink_target_path.into(),
        ));
        self.inner.push_file(symlink_path, contents)
    }

    /// TODO: docs.
    #[inline]
    pub fn push_text_file(
        &mut self,
        file_path: impl AsRef<AbsPath>,
        file_contents: impl Into<Rope>,
    ) -> Result<LocalFileId, CreateError> {
        let contents =
            FileContents::Text(TextContents::new(file_contents.into()));

        self.inner.push_file(file_path, contents)
    }

    /// TODO: docs.
    #[inline]
    pub fn push_directory(
        &mut self,
        directory_path: impl AsRef<AbsPath>,
    ) -> Result<LocalDirectoryId, CreateError> {
        match self.inner.push_directory(directory_path, ()) {
            Ok(dir_id) => Ok(dir_id),
            Err(CreateError::PushedRoot) => Err(CreateError::PushedRoot),
            Err(CreateError::NameConflicts(node_id)) => {
                match node_id {
                    LocalNodeId::File(file_id) => {
                        Err(CreateError::NameConflicts(file_id.into()))
                    },
                    // Directories don't have any associated metadata, so they
                    // can be pushed multiple times without conflicts.
                    LocalNodeId::Directory(dir_id) => Ok(dir_id),
                }
            },
        }
    }
}
