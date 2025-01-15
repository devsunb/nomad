use core::ops::{Deref, DerefMut};

use crate::AsyncCtx;
use crate::backend::Backend;
use crate::notify::{self, Name};

/// TODO: docs.
pub struct AsyncActionCtx<'a, B> {
    async_ctx: AsyncCtx<'a, B>,
    action_name: Name,
}

impl<'a, B: Backend> AsyncActionCtx<'a, B> {
    /// TODO: docs.
    #[inline]
    pub fn emit_err<Err>(&mut self, err: Err)
    where
        Err: notify::Error,
    {
        let action_name = self.action_name;
        self.emit_err_inner(Some(action_name), err)
    }

    #[inline]
    pub(crate) fn new(async_ctx: AsyncCtx<'a, B>, action_name: Name) -> Self {
        Self { async_ctx, action_name }
    }
}

impl<'a, B> Deref for AsyncActionCtx<'a, B> {
    type Target = AsyncCtx<'a, B>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.async_ctx
    }
}

impl<B> DerefMut for AsyncActionCtx<'_, B> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.async_ctx
    }
}
