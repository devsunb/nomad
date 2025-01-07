use core::marker::PhantomData;

use crate::executor::{BackgroundExecutor, TaskBackground};
use crate::{Backend, BackendHandle, NeovimCtx, Plugin};

/// TODO: docs.
pub struct AsyncCtx<'a, P, B> {
    backend: BackendHandle<B>,
    plugin: PhantomData<P>,
    _non_static: PhantomData<&'a ()>,
}

impl<P, B> AsyncCtx<'_, P, B>
where
    P: Plugin<B>,
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
        Fun: FnOnce(&mut NeovimCtx<P, B>) -> Out,
    {
        self.backend.with_mut(|backend| fun(&mut NeovimCtx::new(backend)))
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn new(backend: BackendHandle<B>) -> Self {
        Self { backend, plugin: PhantomData, _non_static: PhantomData }
    }
}
