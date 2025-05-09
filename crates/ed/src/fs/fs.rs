use core::error::Error;
use core::fmt::Debug;
use core::future::Future;
use core::hash::Hash;

use abs_path::AbsPathBuf;

use crate::fs::{AbsPath, Directory, File, FsNode, Metadata, Symlink};

/// TODO: docs.
pub trait Fs: Clone + Send + Sync + 'static {
    /// TODO: docs.
    type Directory: Directory<Fs = Self>;

    /// TODO: docs.
    type File: File<Fs = Self>;

    /// TODO: docs.
    type Symlink: Symlink<Fs = Self>;

    /// TODO: docs.
    type Metadata: Metadata<Fs = Self>;

    /// TODO: docs.
    type NodeId: Debug + Clone + Eq + Hash + Send + Sync;

    /// TODO: docs.
    type Timestamp: Clone + Ord;

    /// TODO: docs.
    type CreateDirectoriesError: Error + Send;

    /// TODO: docs.
    type NodeAtPathError: Error + Send;

    /// TODO: docs.
    fn create_all_missing_directories<P: AsRef<AbsPath> + Send>(
        &self,
        path: P,
    ) -> impl Future<
        Output = Result<Self::Directory, Self::CreateDirectoriesError>,
    > + Send;

    /// TODO: docs.
    fn node_at_path<P: AsRef<AbsPath> + Send>(
        &self,
        path: P,
    ) -> impl Future<
        Output = Result<Option<FsNode<Self>>, Self::NodeAtPathError>,
    > + Send;

    /// TODO: docs.
    fn now(&self) -> Self::Timestamp;

    /// TODO: docs.
    fn exists<P: AsRef<AbsPath> + Send>(
        &self,
        path: P,
    ) -> impl Future<Output = Result<bool, Self::NodeAtPathError>> {
        async move { self.node_at_path(path).await.map(|opt| opt.is_some()) }
    }

    /// TODO: docs.
    fn is_dir<P: AsRef<AbsPath> + Send>(
        &self,
        path: P,
    ) -> impl Future<Output = Result<bool, Self::NodeAtPathError>> {
        async move {
            self.node_at_path(path).await.map(|maybe_node| {
                maybe_node.map(|node| node.is_dir()).unwrap_or(false)
            })
        }
    }

    /// TODO: docs.
    fn is_file<P: AsRef<AbsPath> + Send>(
        &self,
        path: P,
    ) -> impl Future<Output = Result<bool, Self::NodeAtPathError>> {
        async move {
            self.node_at_path(path).await.map(|maybe_node| {
                maybe_node.map(|node| node.is_dir()).unwrap_or(false)
            })
        }
    }

    /// TODO: docs.
    #[inline]
    fn read<P: AsRef<AbsPath> + Send>(
        &self,
        _path: P,
    ) -> impl Future<Output = Result<Vec<u8>, ReadFileError<Self>>> + Send
    {
        async move {
            todo!();
        }
    }

    /// TODO: docs.
    #[inline]
    fn read_to_string<P: AsRef<AbsPath> + Send>(
        &self,
        _path: P,
    ) -> impl Future<Output = Result<String, ReadFileToStringError<Self>>> + Send
    {
        async move {
            todo!();
        }
    }
}

/// TODO: docs.
#[derive(
    cauchy::Debug,
    derive_more::Display,
    cauchy::Error,
    cauchy::PartialEq,
    cauchy::Eq,
)]
pub enum ReadFileError<Fs: self::Fs> {
    /// TODO: docs.
    #[display("{_0}")]
    NodeAtPath(Fs::NodeAtPathError),

    /// TODO: docs.
    #[display("{_0}")]
    ReadFile(<Fs::File as File>::ReadError),

    /// TODO: docs.
    #[display("{_0}")]
    FollowSymlink(<Fs::Symlink as Symlink>::FollowError),

    /// TODO: docs.
    #[display("no file or directory at {_0}")]
    NoNodeAtPath(AbsPathBuf),

    /// TODO: docs.
    #[display("node at {_0} is a directory, but expected a file")]
    DirectoryAtPath(AbsPathBuf),
}

/// TODO: docs.
#[derive(
    cauchy::Debug,
    derive_more::Display,
    cauchy::Error,
    cauchy::PartialEq,
    cauchy::Eq,
)]
pub enum ReadFileToStringError<Fs: self::Fs> {
    /// TODO: docs.
    #[display("{_0}")]
    ReadFile(ReadFileError<Fs>),

    /// TODO: docs.
    #[display(
        "tried to read contents of file {_0} into a string, but it contains \
         binary data"
    )]
    FileIsNotUtf8(AbsPathBuf),
}
