use std::io;
use std::path::PathBuf;

use abs_path::{AbsPath, AbsPathBuf, NodeName};
use futures_util::select_biased;
use futures_util::stream::{self, Stream, StreamExt};

use crate::{File, Metadata, RealFs, Symlink};

/// TODO: docs.
#[derive(Clone)]
pub struct Directory {
    pub(crate) metadata: async_fs::Metadata,
    pub(crate) path: AbsPathBuf,
}

impl fs::Directory for Directory {
    type EventStream = stream::Pending<fs::DirectoryEvent<RealFs>>;
    type Fs = RealFs;

    type ClearError = io::Error;
    type CreateDirectoryError = io::Error;
    type CreateFileError = io::Error;
    type CreateSymlinkError = io::Error;
    type DeleteError = io::Error;
    type ListError = io::Error;
    type MoveError = io::Error;
    type ParentError = io::Error;
    type ReadMetadataError = io::Error;

    #[inline]
    async fn create_directory(
        &self,
        directory_name: &NodeName,
    ) -> Result<Self, Self::CreateDirectoryError> {
        let path = self.path.clone().join(directory_name);
        async_fs::create_dir(&path).await?;
        let metadata = async_fs::metadata(&path).await?;
        Ok(Self { metadata, path })
    }

    #[inline]
    async fn create_file(
        &self,
        file_name: &NodeName,
    ) -> Result<File, Self::CreateFileError> {
        let path = self.path.clone().join(file_name);
        let file = File::open_options().create_new(true).open(&path).await?;
        let metadata = file.metadata().await?;
        Ok(File { file: file.into(), metadata, path })
    }

    #[inline]
    async fn create_symlink(
        &self,
        symlink_name: &NodeName,
        target_path: &str,
    ) -> Result<Symlink, Self::CreateSymlinkError> {
        #[cfg(unix)]
        {
            let path = self.path.clone().join(symlink_name);
            async_fs::unix::symlink(target_path, &path).await?;
            let metadata = async_fs::metadata(&path).await?;
            Ok(Symlink { metadata, path })
        }
    }

    #[inline]
    async fn clear(&self) -> Result<(), Self::ClearError> {
        async_fs::remove_dir_all(self.path()).await?;
        async_fs::create_dir(self.path()).await?;
        Ok(())
    }

    #[inline]
    async fn delete(self) -> Result<(), Self::DeleteError> {
        async_fs::remove_dir_all(self.path()).await
    }

    #[inline]
    async fn list_metas(
        &self,
    ) -> Result<
        impl Stream<Item = Result<Metadata, Self::ReadMetadataError>> + use<>,
        Self::ListError,
    > {
        let read_dir = async_fs::read_dir(self.path()).await?.fuse();
        let get_metadata = stream::FuturesUnordered::new();
        Ok(Box::pin(stream::unfold(
            (read_dir, get_metadata, self.path().to_owned()),
            move |(mut read_dir, mut get_metadata, dir_path)| async move {
                let metadata_res = loop {
                    select_biased! {
                        res = read_dir.select_next_some() => {
                            let dir_entry = match res {
                                Ok(entry) => entry,
                                Err(err) => break Err(err),
                            };
                            let dir_path = dir_path.clone();
                            get_metadata.push(async move {
                                let node_name = dir_entry.file_name();
                                let entry_path =
                                    PathBuf::from(dir_path.as_str())
                                        .join(&node_name);
                                let meta =
                                    async_fs::symlink_metadata(entry_path)
                                        .await?;
                                Ok((meta, node_name))
                            });
                        },
                        res = get_metadata.select_next_some() => {
                            let (metadata, node_name) = match res {
                                Ok(tuple) => tuple,
                                Err(err) => break Err(err),
                            };
                            let file_type = metadata.file_type();
                            let node_kind = if file_type.is_dir() {
                                fs::NodeKind::Directory
                            } else if file_type.is_file() {
                                fs::NodeKind::File
                            } else if file_type.is_symlink() {
                                fs::NodeKind::Symlink
                            } else {
                                continue
                            };
                            break Ok(Metadata {
                                inner: metadata,
                                node_kind,
                                node_name,
                            })
                        },
                        complete => return None,
                    }
                };
                Some((metadata_res, (read_dir, get_metadata, dir_path)))
            },
        )))
    }

    #[inline]
    fn meta(&self) -> Metadata {
        Metadata {
            inner: self.metadata.clone(),
            node_kind: fs::NodeKind::Directory,
            node_name: self
                .name()
                .map(|n| n.as_str().into())
                .unwrap_or_default(),
        }
    }

    #[inline]
    async fn r#move(&self, new_path: &AbsPath) -> Result<(), Self::MoveError> {
        crate::move_node(self.path(), new_path).await
    }

    #[inline]
    async fn parent(&self) -> Result<Option<Self>, Self::ParentError> {
        let Some(parent_path) = self.path().parent() else { return Ok(None) };
        let metadata = async_fs::metadata(parent_path).await?;
        Ok(Some(Directory { path: parent_path.to_owned(), metadata }))
    }

    #[inline]
    fn path(&self) -> &AbsPath {
        &self.path
    }

    #[inline]
    fn watch(&self) -> Self::EventStream {
        stream::pending()
    }
}

impl AsRef<RealFs> for Directory {
    fn as_ref(&self) -> &RealFs {
        &RealFs {}
    }
}
