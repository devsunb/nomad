use std::io;

use abs_path::{AbsPath, AbsPathBuf};
use futures_util::{AsyncWriteExt, stream};

use crate::{Directory, IoErrorExt, Metadata, RealFs};

/// TODO: docs.
pub struct File {
    pub(crate) file: Option<async_fs::File>,
    pub(crate) metadata: async_fs::Metadata,
    pub(crate) path: AbsPathBuf,
}

impl File {
    #[inline]
    pub(crate) fn open_options() -> async_fs::OpenOptions {
        let mut opts = async_fs::OpenOptions::new();
        opts.read(true).write(true);
        opts
    }

    #[inline]
    async fn with_file_async<R>(
        &mut self,
        fun: impl AsyncFnOnce(&mut async_fs::File) -> R,
    ) -> Result<R, io::Error> {
        loop {
            match &mut self.file {
                Some(file) => break Ok(fun(file).await),
                None => {
                    self.file =
                        Some(Self::open_options().open(&self.path).await?);
                },
            }
        }
    }
}

impl fs::File for File {
    type EventStream = stream::Pending<fs::FileEvent<RealFs>>;
    type Fs = RealFs;

    type DeleteError = io::Error;
    type MoveError = io::Error;
    type ParentError = io::Error;
    type ReadError = io::Error;
    type WriteError = io::Error;

    #[inline]
    async fn delete(self) -> Result<(), Self::DeleteError> {
        async_fs::remove_file(self.path()).await.with_context(|| {
            format!("couldn't delete file at {}", self.path())
        })
    }

    #[inline]
    fn meta(&self) -> Metadata {
        Metadata {
            inner: self.metadata.clone(),
            node_kind: fs::NodeKind::File,
            node_name: self.name().as_str().into(),
        }
    }

    #[inline]
    async fn r#move(&self, new_path: &AbsPath) -> Result<(), Self::MoveError> {
        crate::move_node(self.path(), new_path).await.with_context(|| {
            format!("couldn't move file at {} to {}", self.path(), new_path)
        })
    }

    #[inline]
    async fn parent(&self) -> Result<Directory, Self::ParentError> {
        let parent_path = self.path().parent().expect("has a parent");
        let metadata =
            async_fs::metadata(parent_path).await.with_context(|| {
                format!(
                    "couldn't get metadata for directory at {}",
                    parent_path
                )
            })?;
        Ok(Directory { path: parent_path.to_owned(), metadata })
    }

    #[inline]
    fn path(&self) -> &AbsPath {
        &self.path
    }

    #[inline]
    async fn read(&self) -> Result<Vec<u8>, Self::ReadError> {
        async_fs::read(self.path())
            .await
            .with_context(|| format!("couldn't read file at {}", self.path()))
    }

    #[inline]
    fn watch(&self) -> Self::EventStream {
        stream::pending()
    }

    #[inline]
    async fn write_chunks<Chunks, Chunk>(
        &mut self,
        chunks: Chunks,
    ) -> Result<(), Self::WriteError>
    where
        Chunks: IntoIterator<Item = Chunk> + Send,
        Chunks::IntoIter: Send,
        Chunk: AsRef<[u8]> + Send,
    {
        self.with_file_async(async move |file| {
            for chunk in chunks {
                file.write_all(chunk.as_ref()).await?;
            }
            file.sync_all().await?;
            Ok(())
        })
        .await
        .with_context(|| {
            format!("couldn't write to file at {}", self.path())
        })?
    }
}
