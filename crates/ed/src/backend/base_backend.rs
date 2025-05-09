use abs_path::AbsPath;

use crate::AsyncCtx;
use crate::backend::Backend;

/// TODO: docs.
pub trait BaseBackend: Backend {
    /// TODO: docs.
    fn create_buffer<B: Backend + AsMut<Self>>(
        file_path: &AbsPath,
        ctx: &mut AsyncCtx<'_, B>,
    ) -> impl Future<Output = Result<Self::BufferId, Self::CreateBufferError>>;
}
