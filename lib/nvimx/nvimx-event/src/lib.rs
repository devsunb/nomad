//! TODO: docs.

mod buf_add;
mod buf_enter;
mod buf_leave;
mod buf_unload;
mod cursor;
mod cursor_moved;
mod cursor_moved_i;
mod on_bytes;

pub use buf_add::BufAdd;
pub use buf_enter::{BufEnter, BufEnterArgs};
pub use buf_leave::{BufLeave, BufLeaveArgs};
pub use buf_unload::{BufUnload, BufUnloadArgs};
pub use cursor::CursorArgs;
pub use cursor_moved::{CursorMoved, CursorMovedArgs};
pub use cursor_moved_i::CursorMovedI;
pub use on_bytes::OnBytes;

/// TODO: docs.
pub trait Event: Sized {
    /// TODO: docs.
    type Ctx<'a>;

    /// TODO: docs.
    fn register(self, ctx: Self::Ctx<'_>);
}

impl<A: nvimx_ctx::AutoCommand> Event for A {
    type Ctx<'a> = nvimx_ctx::NeovimCtx<'a>;

    fn register(self, ctx: Self::Ctx<'_>) {
        ctx.register_auto_command(self);
    }
}
