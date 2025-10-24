use core::fmt;
use std::io;
use std::path::PathBuf;

use abs_path::{AbsPath, AbsPathBuf};
use fs::Fs;

use crate::{Directory, IoErrorExt, Metadata, RealFs};

/// TODO: docs.
pub struct Symlink {
    pub(crate) metadata: async_fs::Metadata,
    pub(crate) path: AbsPathBuf,
}

impl Symlink {
    async fn read_link(&self) -> Result<PathBuf, io::Error> {
        async_fs::read_link(&*self.path)
            .await
            .with_context(|| format!("couldn't read symlink at {}", self.path))
    }
}

impl fs::Symlink for Symlink {
    type Fs = RealFs;

    type DeleteError = io::Error;
    type FollowError = io::Error;
    type MoveError = io::Error;
    type ParentError = io::Error;
    type ReadError = io::Error;

    #[inline]
    async fn delete(self) -> Result<(), Self::DeleteError> {
        async_fs::remove_file(self.path()).await.with_context(|| {
            format!("couldn't delete symlink at {}", self.path())
        })
    }

    #[inline]
    async fn follow(
        &self,
    ) -> Result<Option<fs::Node<RealFs>>, Self::FollowError> {
        let target_path = self.read_link().await?;
        let path = <&AbsPath>::try_from(&*target_path)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        RealFs::default().node_at_path(path).await
    }

    #[inline]
    async fn follow_recursively(
        &self,
    ) -> Result<Option<fs::Node<RealFs>>, Self::FollowError> {
        let target_path =
            async_fs::canonicalize(&*self.path).await.with_context(|| {
                format!("couldn't canonicalize path at {}", self.path())
            })?;
        let path = <&AbsPath>::try_from(&*target_path)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
        RealFs::default().node_at_path(path).await
    }

    #[inline]
    fn meta(&self) -> Metadata {
        Metadata {
            inner: self.metadata.clone(),
            node_kind: fs::NodeKind::Symlink,
            node_name: self.name().as_str().into(),
        }
    }

    #[inline]
    async fn r#move(&self, new_path: &AbsPath) -> Result<(), Self::MoveError> {
        crate::move_node(self.path(), new_path).await.with_context(|| {
            format!("couldn't move symlink at {} to {}", self.path(), new_path)
        })
    }

    #[inline]
    async fn parent(&self) -> Result<Directory, Self::ParentError> {
        let parent_path = self.path().parent().expect("has a parent");
        let metadata =
            async_fs::metadata(parent_path).await.with_context(|| {
                format!(
                    "couldn't get metadata for directory at {parent_path}",
                )
            })?;
        Ok(Directory { path: parent_path.to_owned(), metadata })
    }

    #[inline]
    fn path(&self) -> &AbsPath {
        &self.path
    }

    #[inline]
    async fn read_path(&self) -> Result<String, Self::ReadError> {
        self.read_link().await.map(|path| path.display().to_string())
    }
}

impl fmt::Debug for Symlink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Symlink").field(&self.path).finish()
    }
}
