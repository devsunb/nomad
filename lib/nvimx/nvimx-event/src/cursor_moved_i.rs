use nvimx_action::{Action, IntoModuleName};
use nvimx_ctx::{
    ActorId,
    AutoCommand,
    AutoCommandCtx,
    AutoCommandEvent,
    BufferCtx,
    BufferId,
    ShouldDetach,
};
use nvimx_diagnostics::DiagnosticMessage;

use crate::cursor_moved::{CursorMoved, CursorMovedArgs};

/// TODO: docs.
pub struct CursorMovedI<A, M> {
    inner: CursorMoved<A, M>,
    buffer_id: Option<BufferId>,
}

impl<A, M> CursorMovedI<A, M> {
    /// TODO: docs.
    pub fn buffer_id(mut self, buffer_id: BufferId) -> Self {
        self.buffer_id = Some(buffer_id);
        self
    }

    /// Creates a new [`CursorMovedI`] with the given action.
    pub fn new(action: A) -> Self {
        Self { inner: CursorMoved::new(action), buffer_id: None }
    }
}

impl<A, M> AutoCommand for CursorMovedI<A, M>
where
    A: for<'ctx> Action<
        M,
        Args = CursorMovedArgs,
        Ctx<'ctx> = BufferCtx<'ctx>,
    >,
    A::Return: Into<ShouldDetach>,
    M: IntoModuleName + 'static,
{
    const MODULE_NAME: Option<&'static str> = M::NAME;
    const CALLBACK_NAME: Option<&'static str> = Some(A::NAME.as_str());

    fn into_callback(
        self,
    ) -> impl for<'ctx> FnMut(
        ActorId,
        &'ctx AutoCommandCtx<'ctx>,
    ) -> Result<ShouldDetach, DiagnosticMessage> {
        self.inner.into_callback()
    }

    fn on_event(&self) -> AutoCommandEvent {
        AutoCommandEvent::CursorMovedI
    }

    fn on_buffer(&self) -> Option<BufferId> {
        self.buffer_id
    }

    fn take_actor_id(ctx: &AutoCommandCtx<'_>) -> ActorId {
        <CursorMoved<A, M>>::take_actor_id(ctx)
    }
}
