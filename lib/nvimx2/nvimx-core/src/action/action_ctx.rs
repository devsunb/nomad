use core::ops::{Deref, DerefMut};

use crate::NeovimCtx;
use crate::backend::Backend;
use crate::module::Module;
use crate::notify::{self, Name, NotificationId};

/// TODO: docs.
pub struct ActionCtx<'a, M, B> {
    neovim_ctx: NeovimCtx<'a, M, B>,
    action_name: Name,
}

impl<'a, M, B> ActionCtx<'a, M, B>
where
    M: Module<B>,
    B: Backend,
{
    /// TODO: docs.
    #[inline]
    pub fn emit_info(&mut self, message: notify::Message) -> NotificationId {
        self.neovim_ctx.emit_info_inner(message, None)
    }

    #[inline]
    pub(crate) fn emit_err<Err>(&mut self, err: Err)
    where
        Err: notify::Error,
    {
        self.neovim_ctx.emit_err(Some(self.action_name), err);
    }

    #[inline]
    pub(crate) fn new(
        neovim_ctx: NeovimCtx<'a, M, B>,
        action_name: Name,
    ) -> Self {
        Self { neovim_ctx, action_name }
    }
}

impl<'a, M, B> Deref for ActionCtx<'a, M, B> {
    type Target = NeovimCtx<'a, M, B>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.neovim_ctx
    }
}

impl<M, B> DerefMut for ActionCtx<'_, M, B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.neovim_ctx
    }
}
