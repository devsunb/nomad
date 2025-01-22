use std::borrow::Cow;

use crate::backend::{Backend, Buffer, BufferId};

/// TODO: docs.
pub struct BufferCtx<'a, B: Backend> {
    inner: B::Buffer,
    _non_static: core::marker::PhantomData<&'a ()>,
}

impl<B: Backend> BufferCtx<'_, B> {
    /// TODO: docs.
    #[inline]
    pub fn id(&self) -> BufferId<B> {
        self.inner.id()
    }

    /// TODO: docs.
    #[inline]
    pub fn name(&self) -> Cow<'_, str> {
        self.inner.name()
    }

    #[inline]
    pub(crate) fn new(inner: B::Buffer) -> Self {
        Self { inner, _non_static: core::marker::PhantomData }
    }
}
