use e31e::fs::AbsPathBuf;
use e31e::FileRefMut;
use nohash::IntMap as NoHashMap;
use nomad::ctx::NeovimCtx;
use nomad::{ActorId, BufferId, Shared, ShouldDetach};

#[derive(Clone)]
pub(super) struct SessionCtx {
    /// The [`ActorId`] of the [`Session`].
    pub(super) actor_id: ActorId,

    /// Map from [`BufferId`]
    pub(super) buffer_actions: NoHashMap<BufferId, Shared<ShouldDetach>>,

    /// An instance of the [`NeovimCtx`].
    pub(super) neovim_ctx: NeovimCtx<'static>,

    /// The absolute path to the root of the project.
    pub(super) project_root: AbsPathBuf,

    /// The [`Replica`](e31e::Replica) used to integrate remote messages on the
    /// project at [`project_root`](Self::project_root).
    pub(super) replica: e31e::Replica,
}

impl SessionCtx {
    /// Returns the [`FileRefMut`] corresponding to the file that's currently
    /// being edited in the buffer with the given [`BufferId`], if any.
    pub(super) fn file_mut_of_buffer_id(
        &mut self,
        buffer_id: BufferId,
    ) -> Option<FileRefMut<'_>> {
        let file_ctx = self
            .neovim_ctx
            .reborrow()
            .into_buffer(buffer_id)
            .and_then(|ctx| ctx.into_file())?;

        let file_path = file_ctx.path().strip_prefix(&self.project_root)?;

        match self.replica.file_mut_at_path(file_path) {
            Ok(Some(file)) => Some(file),
            _ => None,
        }
    }
}
