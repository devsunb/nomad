use core::marker::PhantomData;

use nvimx_common::{ByteOffset, MaybeResult, Shared};
use nvimx_ctx::{ActorId, BufferCtx, BufferId, ShouldDetach};
use nvimx_plugin::{Action, ActionName, Module};

use crate::{
    BufEnter,
    BufEnterArgs,
    BufLeave,
    BufLeaveArgs,
    CursorMoved,
    CursorMovedArgs,
    CursorMovedI,
    Event,
};

/// TODO: docs.
pub struct Cursor<A, M> {
    action: A,
    module: PhantomData<M>,
}

/// TODO: docs.
#[derive(Clone)]
pub struct CursorArgs {
    /// TODO: docs.
    pub kind: CursorKind,

    /// TODO: docs.
    pub buffer_id: BufferId,

    /// TODO: docs.
    pub moved_by: ActorId,
}

/// TODO: docs.
#[derive(Clone, Copy)]
pub enum CursorKind {
    /// The cursor has been moved into the buffer at the given offset.
    Created(ByteOffset),

    /// The cursor has been moved to the given offset.
    Moved(ByteOffset),

    /// The cursor has been moved away from the buffer.
    Removed,
}

struct CursorMovedAction<A> {
    action: A,
    has_just_entered_buf: Shared<bool>,
    should_detach: Shared<ShouldDetach>,
}

struct BufEnterAction<M> {
    has_just_entered_buf: Shared<bool>,
    should_detach: Shared<ShouldDetach>,
    module: PhantomData<M>,
}

struct BufLeaveAction<A> {
    action: A,
    should_detach: Shared<ShouldDetach>,
}

impl<A, M> Cursor<A, M> {
    /// Creates a new [`Cursor`] with the given action.
    pub fn new(action: A) -> Self {
        Self { action, module: PhantomData }
    }
}

impl<A: Clone> CursorMovedAction<A> {
    fn clone(&self) -> Self {
        Self {
            action: self.action.clone(),
            has_just_entered_buf: self.has_just_entered_buf.clone(),
            should_detach: self.should_detach.clone(),
        }
    }
}

impl<A, M> Event for Cursor<A, M>
where
    A: for<'ctx> Action<
            M,
            Ctx<'ctx> = BufferCtx<'ctx>,
            Args = CursorArgs,
            Return: Into<ShouldDetach>,
        > + Clone,
    M: Module + 'static,
{
    type Ctx<'a> = BufferCtx<'a>;

    fn register(self, ctx: Self::Ctx<'_>) {
        let action = self.action;
        let should_detach = Shared::new(ShouldDetach::No);
        let has_just_entered_buf = Shared::new(false);

        let cursor_moved_action = CursorMovedAction {
            action: action.clone(),
            has_just_entered_buf: has_just_entered_buf.clone(),
            should_detach: should_detach.clone(),
        };

        let buf_enter_action = BufEnterAction::<M> {
            has_just_entered_buf: has_just_entered_buf.clone(),
            should_detach: should_detach.clone(),
            module: PhantomData,
        };

        let buf_leave_action = BufLeaveAction { action, should_detach };

        CursorMoved::new(cursor_moved_action.clone())
            .buffer_id(ctx.buffer_id())
            .register((*ctx).reborrow());

        CursorMovedI::new(cursor_moved_action)
            .buffer_id(ctx.buffer_id())
            .register((*ctx).reborrow());

        BufEnter::new(buf_enter_action)
            .buffer_id(ctx.buffer_id())
            .register((*ctx).reborrow());

        BufLeave::new(buf_leave_action)
            .buffer_id(ctx.buffer_id())
            .register((*ctx).reborrow());
    }
}

impl<A, M> Action<M> for CursorMovedAction<A>
where
    A: for<'ctx> Action<M, Args = CursorArgs, Ctx<'ctx> = BufferCtx<'ctx>>,
    A::Return: Into<ShouldDetach>,
    M: Module + 'static,
{
    const NAME: ActionName = A::NAME;
    type Args = CursorMovedArgs;
    type Ctx<'ctx> = BufferCtx<'ctx>;
    type Docs = ();
    type Return = ShouldDetach;

    fn execute<'a>(
        &'a mut self,
        args: CursorMovedArgs,
        ctx: Self::Ctx<'a>,
    ) -> impl MaybeResult<Self::Return> {
        let cursor_action = if self.has_just_entered_buf.take() {
            CursorKind::Created(args.moved_to)
        } else {
            CursorKind::Moved(args.moved_to)
        };
        let cursor = CursorArgs {
            kind: cursor_action,
            buffer_id: ctx.buffer_id(),
            moved_by: args.actor_id,
        };
        self.action.execute(cursor, ctx).into_result().map(|ret| {
            self.should_detach.set(ret.into());
            self.should_detach.get()
        })
    }

    fn docs(&self) {}
}

impl<M: Module> Action<M> for BufEnterAction<M> {
    const NAME: ActionName = ActionName::from_str("");
    type Args = BufEnterArgs;
    type Ctx<'ctx> = BufferCtx<'ctx>;
    type Docs = ();
    type Return = ShouldDetach;

    fn execute<'a>(
        &'a mut self,
        _: BufEnterArgs,
        _: Self::Ctx<'a>,
    ) -> Self::Return {
        self.has_just_entered_buf.set(true);
        self.should_detach.get()
    }

    fn docs(&self) {}
}

impl<A, M> Action<M> for BufLeaveAction<A>
where
    A: for<'ctx> Action<M, Args = CursorArgs, Ctx<'ctx> = BufferCtx<'ctx>>,
    A::Return: Into<ShouldDetach>,
    M: Module + 'static,
{
    const NAME: ActionName = A::NAME;
    type Args = BufLeaveArgs;
    type Ctx<'ctx> = BufferCtx<'ctx>;
    type Docs = ();
    type Return = ShouldDetach;

    fn execute<'a>(
        &'a mut self,
        args: BufLeaveArgs,
        ctx: Self::Ctx<'a>,
    ) -> impl MaybeResult<Self::Return> {
        self.action
            .execute(
                CursorArgs {
                    kind: CursorKind::Removed,
                    buffer_id: ctx.buffer_id(),
                    moved_by: args.actor_id,
                },
                ctx,
            )
            .into_result()
            .map(|ret| {
                self.should_detach.set(ret.into());
                self.should_detach.get()
            })
    }

    fn docs(&self) {}
}
