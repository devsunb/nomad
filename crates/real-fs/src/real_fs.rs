//! TODO: docs.

use std::io;
use std::time::SystemTime;

use abs_path::{AbsPath, AbsPathBuf, AbsPathFromPathError};
use either::Either;

use crate::{Directory, File, Metadata, Symlink};

/// TODO: docs.
pub type Inode = u64;

/// TODO: docs.
#[derive(Debug, Default, Copy, Clone)]
pub struct RealFs {}

impl fs::Fs for RealFs {
    type Directory = Directory;
    type File = File;
    type Symlink = Symlink;
    type Metadata = Metadata;
    type NodeId = Inode;
    type Timestamp = SystemTime;

    type CreateDirectoriesError = io::Error;
    type HomeError = Either<io::Error, fs::GetDirError<Self>>;
    type NodeAtPathError = io::Error;

    #[inline]
    async fn create_all_missing_directories<P: AsRef<AbsPath> + Send>(
        &self,
        path: P,
    ) -> Result<Self::Directory, Self::CreateDirectoriesError> {
        let path = path.as_ref();
        async_fs::create_dir_all(path).await?;
        let metadata = async_fs::metadata(path).await?;
        Ok(Directory { path: path.to_owned(), metadata })
    }

    #[inline]
    async fn home(&self) -> Result<Option<Self::Directory>, Self::HomeError> {
        let Some(home_path) = home_path().map_err(Either::Left)? else {
            return Ok(None);
        };
        self.dir(home_path).await.map_err(Either::Right).map(Some)
    }

    #[inline]
    async fn node_at_path<P: AsRef<AbsPath> + Send>(
        &self,
        path: P,
    ) -> Result<Option<fs::Node<Self>>, Self::NodeAtPathError> {
        let path = path.as_ref();
        let metadata = match async_fs::symlink_metadata(path).await {
            Ok(metadata) => metadata,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e),
        };
        let Ok(file_type) = metadata.file_type().try_into() else {
            return Ok(None);
        };
        Ok(Some(match file_type {
            fs::NodeKind::File => {
                fs::Node::File(File::new(metadata, path.to_owned()))
            },
            fs::NodeKind::Directory => fs::Node::Directory(Directory {
                metadata,
                path: path.to_owned(),
            }),
            fs::NodeKind::Symlink => {
                fs::Node::Symlink(Symlink { metadata, path: path.to_owned() })
            },
        }))
    }

    #[inline]
    fn now(&self) -> Self::Timestamp {
        SystemTime::now()
    }
}

fn home_path() -> io::Result<Option<AbsPathBuf>> {
    let path = match home::home_dir() {
        Some(path) if !path.as_os_str().is_empty() => path,
        _ => return Ok(None),
    };

    (&*path)
        .try_into()
        .map_err(|err| {
            let msg = match err {
                AbsPathFromPathError::NotAbsolute => {
                    format!("home directory path is not absolute: {path:?}")
                },
                AbsPathFromPathError::NotUtf8 => {
                    format!("home directory path is not valid UTF-8: {path:?}")
                },
            };
            io::Error::new(io::ErrorKind::InvalidInput, msg)
        })
        .map(Some)
}
