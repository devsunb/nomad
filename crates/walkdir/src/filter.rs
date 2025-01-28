use nvimx2::fs;

use crate::WalkDir;

/// TODO: docs.
pub trait Filter<W: WalkDir> {
    type Error;

    /// TODO: docs.
    fn should_filter(
        &self,
        dir_path: &fs::AbsPath,
        dir_entry: &W::DirEntry,
    ) -> impl Future<Output = Result<bool, Self::Error>>;
}

/// TODO: docs.
pub struct Filtered<F, W> {
    _filter: F,
    _walker: W,
}

impl<F, W> Filtered<F, W> {
    /// TODO: docs.
    #[inline]
    pub(crate) fn new(filter: F, walker: W) -> Self {
        Self { _filter: filter, _walker: walker }
    }
}

impl<F, W> WalkDir for Filtered<F, W>
where
    F: Filter<W>,
    W: WalkDir,
{
    type DirEntry = W::DirEntry;
    type ReadDir = W::ReadDir;
    type DirEntryError = W::DirEntryError;
    type ReadDirError = W::ReadDirError;

    async fn read_dir(
        &self,
        _path: &fs::AbsPath,
    ) -> Result<Self::ReadDir, Self::ReadDirError> {
        todo!()
    }
}
