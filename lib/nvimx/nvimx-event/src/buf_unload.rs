use core::marker::PhantomData;
use core::ops::Deref;

use nvimx_action::{Action, ActionName, IntoModuleName};
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

/// TODO: docs.
pub struct BufUnload<A, M> {
    action: BufUnloadAction<A, M>,
    buffer_id: Option<BufferId>,
}

/// TODO: docs.
#[derive(Debug, Copy, Clone)]
pub struct BufUnloadArgs {
    /// The [`ActorId`] that unloaded the buffer.
    pub actor_id: ActorId,
}

pub struct BufUnloadAction<A, M> {
    action: A,
    module_name: PhantomData<M>,
}

impl<A, M> BufUnload<A, M> {
    /// TODO: docs.
    pub fn buffer_id(mut self, buffer_id: BufferId) -> Self {
        self.buffer_id = Some(buffer_id);
        self
    }

    /// Creates a new [`BufUnload`] with the given action.
    pub fn new(action: A) -> Self {
        Self {
            action: BufUnloadAction { action, module_name: PhantomData },
            buffer_id: None,
        }
    }
}

impl<A, M> AutoCommand for BufUnload<A, M>
where
    A: for<'ctx> Action<M, Args = BufUnloadArgs, Ctx<'ctx> = BufferCtx<'ctx>>,
    A::Return: Into<ShouldDetach>,
    M: IntoModuleName + 'static,
{
    type Action = BufUnloadAction<A, M>;
    type OnModule = M;

    fn into_action(self) -> Self::Action {
        self.action
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

impl<A, M> Action<M> for BufUnloadAction<A, M>
where
    A: for<'ctx> Action<M, Args = BufUnloadArgs, Ctx<'ctx> = BufferCtx<'ctx>>,
    A::Return: Into<ShouldDetach>,
    M: IntoModuleName + 'static,
{
    const NAME: ActionName = A::NAME;
    type Args = ActorId;
    type Ctx<'ctx> = &'ctx AutoCommandCtx<'ctx>;
    type Docs = A::Docs;
    type Return = A::Return;

    fn execute<'a>(
        &'a mut self,
        actor_id: Self::Args,
        ctx: Self::Ctx<'a>,
    ) -> impl MaybeResult<Self::Return> {
        let args = BufUnloadArgs { actor_id };
        let buffer_ctx = ctx.deref().clone().into_current_buffer();
        self.action.execute(args, buffer_ctx)
    }

    fn docs(&self) -> Self::Docs {
        self.action.docs()
    }
}
