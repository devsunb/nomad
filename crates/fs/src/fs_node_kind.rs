/// TODO: docs.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FsNodeKind {
    /// TODO: docs.
    File,

    /// TODO: docs.
    Directory,

    /// TODO: docs.
    Symlink,
}

impl FsNodeKind {
    /// TODO: docs.
    pub fn is_directory(&self) -> bool {
        matches!(self, Self::Directory)
    }

    /// TODO: docs.
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File)
    }

    /// TODO: docs.
    pub fn is_symlink(&self) -> bool {
        matches!(self, Self::Symlink)
    }
}
