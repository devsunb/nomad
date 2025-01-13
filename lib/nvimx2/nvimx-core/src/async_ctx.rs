use core::marker::PhantomData;

use crate::NeovimCtx;
use crate::backend::{
    Backend,
    BackendHandle,
    BackgroundExecutor,
    TaskBackground,
};
use crate::module::Module;
use crate::notify::ModulePath;

/// TODO: docs.
pub struct AsyncCtx<'a, M, B> {
    backend: BackendHandle<B>,
    module_path: ModulePath,
    module: PhantomData<M>,
    _non_static: PhantomData<&'a ()>,
}

impl<M, B> AsyncCtx<'_, M, B>
where
    M: Module<B>,
    B: Backend,
{
    /// TODO: docs.
    #[inline]
    pub fn spawn_background<Fut>(
        &self,
        fut: Fut,
    ) -> TaskBackground<Fut::Output, B>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        let task = self
            .backend
            .with_mut(|mut backend| backend.background_executor().spawn(fut));
        TaskBackground::new(task)
    }

    /// TODO: docs.
    #[inline]
    pub fn with_ctx<Fun, Out>(&self, fun: Fun) -> Out
    where
        Fun: FnOnce(&mut NeovimCtx<M, B>) -> Out,
    {
        self.backend.with_mut(|backend| {
            let mut ctx = NeovimCtx::new(backend, &self.module_path);
            fun(&mut ctx)
        })
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn new(
        backend: BackendHandle<B>,
        module_path: ModulePath,
    ) -> Self {
        Self {
            backend,
            module_path,
            module: PhantomData,
            _non_static: PhantomData,
        }
    }
}
