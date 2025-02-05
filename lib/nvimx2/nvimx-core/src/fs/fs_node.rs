use crate::fs::{self, FsNodeKind};

/// TODO: docs.
pub enum FsNode<Fs: fs::Fs> {
    /// TODO: docs.
    File(Fs::File),

    /// TODO: docs.
    Directory(Fs::Directory),

    /// TODO: docs.
    Symlink(Fs::Symlink),
}

impl<Fs: fs::Fs> FsNode<Fs> {
    /// TODO: docs.
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.kind().is_dir()
    }

    /// TODO: docs.
    #[inline]
    pub fn is_file(&self) -> bool {
        self.kind().is_file()
    }

    /// TODO: docs.
    #[inline]
    pub fn kind(&self) -> FsNodeKind {
        match self {
            Self::File(_) => FsNodeKind::File,
            Self::Directory(_) => FsNodeKind::Directory,
            Self::Symlink(_) => FsNodeKind::Symlink,
        }
    }
}
