use abs_path::AbsPath;

use crate::{Directory, File, NodeKind, Symlink};

/// A node in a file system tree.
#[derive(cauchy::Debug, cauchy::PartialEq)]
pub enum Node<Fs: crate::Fs> {
    /// A file node.
    File(Fs::File),

    /// A directory node.
    Directory(Fs::Directory),

    /// A symlink node.
    Symlink(Fs::Symlink),
}

/// The type of error that can occur when deleting a node in a file system.
#[derive(
    cauchy::Debug, derive_more::Display, cauchy::Error, cauchy::PartialEq,
)]
#[display("{_0}")]
pub enum NodeDeleteError<Fs: crate::Fs> {
    /// Deleting a file failed.
    File(<Fs::File as File>::DeleteError),

    /// Deleting a directory failed..
    Directory(<Fs::Directory as Directory>::DeleteError),

    /// Deleting a symling failed.
    Symlink(<Fs::Symlink as Symlink>::DeleteError),
}

/// The type of error that can occur when moving a node in a file system.
#[derive(
    cauchy::Debug, derive_more::Display, cauchy::Error, cauchy::PartialEq,
)]
#[display("{_0}")]
pub enum NodeMoveError<Fs: crate::Fs> {
    /// Moving a file failed.
    File(<Fs::File as File>::MoveError),

    /// Moving a directory failed.
    Directory(<Fs::Directory as Directory>::MoveError),

    /// Moving a symlink failed.
    Symlink(<Fs::Symlink as Symlink>::MoveError),
}

impl<Fs: crate::Fs> Node<Fs> {
    /// Deletes this file system node.
    #[inline]
    pub async fn delete(self) -> Result<(), NodeDeleteError<Fs>> {
        match self {
            Self::File(file) => {
                file.delete().await.map_err(NodeDeleteError::File)
            },
            Self::Directory(dir) => {
                dir.delete().await.map_err(NodeDeleteError::Directory)
            },
            Self::Symlink(symlink) => {
                symlink.delete().await.map_err(NodeDeleteError::Symlink)
            },
        }
    }

    /// Returns the node's ID.
    #[inline]
    pub fn id(&self) -> Fs::NodeId {
        match self {
            Self::File(file) => file.id(),
            Self::Directory(dir) => dir.id(),
            Self::Symlink(symlink) => symlink.id(),
        }
    }

    /// Returns whether the node is a [`Directory`](Self::Directory).
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.kind().is_dir()
    }

    /// Returns whether the node is a [`File`](Self::File).
    #[inline]
    pub fn is_file(&self) -> bool {
        self.kind().is_file()
    }

    /// Returns the node's kind.
    #[inline]
    pub fn kind(&self) -> NodeKind {
        match self {
            Self::File(_) => NodeKind::File,
            Self::Directory(_) => NodeKind::Directory,
            Self::Symlink(_) => NodeKind::Symlink,
        }
    }

    /// Returns the node's metadata.
    #[inline]
    pub fn meta(&self) -> Fs::Metadata {
        match self {
            Self::File(file) => file.meta(),
            Self::Directory(dir) => dir.meta(),
            Self::Symlink(symlink) => symlink.meta(),
        }
    }

    /// Moves the node to the given path.
    #[inline]
    pub async fn r#move(
        &self,
        new_path: &AbsPath,
    ) -> Result<(), NodeMoveError<Fs>> {
        match self {
            Self::File(file) => {
                file.r#move(new_path).await.map_err(NodeMoveError::File)
            },
            Self::Directory(dir) => {
                dir.r#move(new_path).await.map_err(NodeMoveError::Directory)
            },
            Self::Symlink(symlink) => {
                symlink.r#move(new_path).await.map_err(NodeMoveError::Symlink)
            },
        }
    }

    /// Returns the node's path in the file system.
    #[inline]
    pub fn path(&self) -> &AbsPath {
        match self {
            Self::File(file) => file.path(),
            Self::Directory(directory) => directory.path(),
            Self::Symlink(symlink) => symlink.path(),
        }
    }

    /// Returns the node as a [`Directory`](Self::Directory).
    ///
    /// # Panics
    ///
    /// Panics if the node is not a [`Directory`](Self::Directory).
    #[track_caller]
    #[inline]
    pub fn unwrap_directory(self) -> Fs::Directory {
        match self {
            Self::Directory(dir) => dir,
            other => panic!("expected directory, got {:?}", other.kind()),
        }
    }

    /// Returns the node as a [`File`](Self::File).
    ///
    /// # Panics
    ///
    /// Panics if the node is not a [`File`](Self::File).
    #[track_caller]
    #[inline]
    pub fn unwrap_file(self) -> Fs::File {
        match self {
            Self::File(file) => file,
            other => panic!("expected file, got {:?}", other.kind()),
        }
    }

    /// Returns the node as a [`Symlink`](Self::Symlink).
    ///
    /// # Panics
    ///
    /// Panics if the node is not a [`Symlink`](Self::Symlink).
    #[track_caller]
    #[inline]
    pub fn unwrap_symlink(self) -> Fs::Symlink {
        match self {
            Self::Symlink(symlink) => symlink,
            other => panic!("expected symlink, got {:?}", other.kind()),
        }
    }
}
