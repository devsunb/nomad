use std::sync::Arc;

use collab_types::bytes::Bytes;
use collab_types::crop::Rope;
use collab_types::{PeerId, puff};
use puff::builder::CreateError;
use puff::directory::LocalDirectoryId;
use puff::file::LocalFileId;

use crate::Project;
use crate::abs_path::AbsPath;
use crate::binary::BinaryContents;
use crate::fs::{FileContents, ProjectTreeBuilder};
use crate::project::Contexts;
use crate::symlink::SymlinkContents;
use crate::text::TextContents;

/// TODO: docs.
pub struct ProjectBuilder {
    inner: ProjectTreeBuilder,
    contexts: Contexts,
}

impl ProjectBuilder {
    /// TODO: docs.
    #[inline]
    pub fn build(self) -> Project {
        let tree = self.inner.build();
        Project { backlog: Default::default(), contexts: self.contexts, tree }
    }

    /// TODO: docs.
    #[inline]
    pub fn new(peer_id: PeerId) -> Self {
        Self {
            inner: ProjectTreeBuilder::new(peer_id.into()),
            contexts: Contexts::new(peer_id),
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn push_binary_file(
        &mut self,
        file_path: impl AsRef<AbsPath>,
        file_contents: impl Into<Bytes>,
    ) -> Result<LocalFileId, CreateError> {
        let contents = FileContents::Binary(BinaryContents::new_local(
            file_contents.into(),
            &mut self.contexts.binary,
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
        let local_id = self.contexts.text.cursors.local_id();

        let contents = FileContents::Text(Box::new(TextContents::new(
            local_id,
            file_contents.into(),
        )));

        self.inner.push_file(file_path, contents)
    }

    /// TODO: docs.
    #[inline]
    pub fn push_directory(
        &mut self,
        directory_path: impl AsRef<AbsPath>,
    ) -> Result<LocalDirectoryId, CreateError> {
        self.inner.push_directory(directory_path, ())
    }
}
