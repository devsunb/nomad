use core::ops::{Deref, DerefMut};

use smallvec::{SmallVec, smallvec};

use crate::notify::NotificationId;
use crate::{Backend, Name, NeovimCtx, Plugin, notify};

/// TODO: docs.
pub struct ActionCtx<'a, P, B> {
    neovim_ctx: NeovimCtx<'a, P, B>,
    action_name: Name,
}

/// TODO: docs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePath {
    names: SmallVec<[Name; 2]>,
}

impl<'a, P, B> ActionCtx<'a, P, B>
where
    P: Plugin<B>,
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
        Err: notify::Error<B>,
    {
        self.neovim_ctx.emit_err(Some(self.action_name), err);
    }

    /// Constructs a new [`ActionCtx`].
    #[inline]
    pub(crate) fn new(
        neovim_ctx: NeovimCtx<'a, P, B>,
        action_name: Name,
    ) -> Self {
        Self { neovim_ctx, action_name }
    }
}

impl ModulePath {
    /// TODO: docs.
    #[inline]
    pub fn names(&self) -> impl ExactSizeIterator<Item = Name> + '_ {
        self.names.iter().copied()
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn new(base_module: Name) -> Self {
        Self { names: smallvec![base_module] }
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn push(&mut self, module_name: Name) {
        self.names.push(module_name);
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn pop(&mut self) {
        self.names.pop();
    }
}

impl<'a, P, B> Deref for ActionCtx<'a, P, B> {
    type Target = NeovimCtx<'a, P, B>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.neovim_ctx
    }
}

impl<P, B> DerefMut for ActionCtx<'_, P, B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.neovim_ctx
    }
}
