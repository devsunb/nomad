use core::marker::PhantomData;

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

use crate::cursor_moved::{CursorMoved, CursorMovedAction, CursorMovedArgs};

/// TODO: docs.
pub struct CursorMovedI<A, M> {
    action: CursorMovedAction<A, M>,
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
        Self {
            action: CursorMovedAction { action, module_name: PhantomData },
            buffer_id: None,
        }
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
    type Action = CursorMovedAction<A, M>;
    type OnModule = M;

    fn into_action(self) -> Self::Action {
        self.action
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
