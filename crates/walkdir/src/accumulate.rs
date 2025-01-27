use nvimx2::fs;

use crate::WalkDir;

/// TODO: docs.
pub trait Accumulator<Fs: fs::Fs> {
    /// TODO: docs.
    type Error;

    /// TODO: docs.
    fn accumulate_entry(
        &mut self,
        dir_path: &fs::AbsPath,
        entry: Fs::DirEntry,
        fs: &mut Fs,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// TODO: docs.
    fn accumulate_fs_event(
        &mut self,
        event: fs::FsEvent<Fs>,
        fs: &mut Fs,
    ) -> impl Future<Output = Result<(), Self::Error>>;
}

/// TODO: docs.
pub enum AccumulateError<A, W, Fs> {
    /// TODO: docs.
    _Acc(A),

    /// TODO: docs.
    _Walk(W),

    /// TODO: docs.
    _Fs(Fs),
}

/// TODO: docs.
pub(crate) async fn accumulate<A, W, Fs>(
    _walker: &W,
    _acc: &mut A,
    _fs: &mut Fs,
) -> Result<Fs::Timestamp, AccumulateError<A, W, Fs>>
where
    A: Accumulator<Fs>,
    W: WalkDir<Fs>,
    Fs: fs::Fs,
{
    todo!();
}
