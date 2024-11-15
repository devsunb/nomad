use core::marker::PhantomData;
use core::ops::Deref;

use nvimx_action::{Action, IntoModuleName};
use nvimx_common::MaybeResult;
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

/// TODO: docs.
pub struct BufUnload<A, M> {
    action: A,
    buffer_id: Option<BufferId>,
    module_name: PhantomData<M>,
}

/// TODO: docs.
#[derive(Debug, Copy, Clone)]
pub struct BufUnloadArgs {
    /// The [`ActorId`] that unloaded the buffer.
    pub actor_id: ActorId,
}

impl<A, M> BufUnload<A, M> {
    /// TODO: docs.
    pub fn buffer_id(mut self, buffer_id: BufferId) -> Self {
        self.buffer_id = Some(buffer_id);
        self
    }

    /// Creates a new [`BufUnload`] with the given action.
    pub fn new(action: A) -> Self {
        Self { action, module_name: PhantomData, buffer_id: None }
    }
}

impl<A, M> AutoCommand for BufUnload<A, M>
where
    A: for<'ctx> Action<M, Args = BufUnloadArgs, Ctx<'ctx> = BufferCtx<'ctx>>,
    A::Return: Into<ShouldDetach>,
    M: IntoModuleName + 'static,
{
    const MODULE_NAME: Option<&'static str> = M::NAME;
    const CALLBACK_NAME: Option<&'static str> = Some(A::NAME.as_str());

    fn into_callback(
        mut self,
    ) -> impl for<'ctx> FnMut(
        ActorId,
        &'ctx AutoCommandCtx<'ctx>,
    ) -> Result<ShouldDetach, DiagnosticMessage> {
        move |actor_id, ctx| {
            let args = BufUnloadArgs { actor_id };
            let buffer_ctx = ctx.deref().clone().into_current_buffer();
            self.action
                .execute(args, buffer_ctx)
                .into_result()
                .map(Into::into)
                .map_err(Into::into)
        }
    }

    fn on_event(&self) -> AutoCommandEvent {
        AutoCommandEvent::BufUnload
    }

    fn on_buffer(&self) -> Option<BufferId> {
        self.buffer_id
    }

    fn take_actor_id(_: &AutoCommandCtx<'_>) -> ActorId {
        // TODO: Implement this.
        ActorId::unknown()
    }
}
