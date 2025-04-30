use core::ops::{Deref, DerefMut};
use std::borrow::Cow;

use crop::Rope;
use ed_core::{ByteOffset, backend};

use crate::mock::{self, CallbackKind, Callbacks};

/// TODO: docs.
pub struct Buffer<'a> {
    pub(crate) inner: &'a mut BufferInner,
    pub(crate) callbacks: Callbacks,
}

/// TODO: docs.
pub(crate) struct BufferInner {
    pub(crate) contents: Rope,
    pub(crate) id: BufferId,
    pub(crate) name: String,
}

/// TODO: docs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BufferId(pub(crate) u64);

impl BufferId {
    pub(crate) fn post_inc(&mut self) -> Self {
        let id = *self;
        self.0 += 1;
        id
    }
}

impl<'a> backend::Buffer for Buffer<'a> {
    type Id = BufferId;
    type EventHandle = mock::EventHandle;

    fn byte_len(&self) -> ByteOffset {
        self.contents.byte_len().into()
    }

    fn id(&self) -> Self::Id {
        self.id
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }

    fn on_edited<Fun>(&mut self, _fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Self, &backend::Edit) + 'static,
    {
        todo!()
    }

    fn on_removed<Fun>(&mut self, _fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Self, backend::AgentId) + 'static,
    {
        todo!()
    }

    fn on_saved<Fun>(&mut self, _fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Self, backend::AgentId) + 'static,
    {
        todo!()
    }
}

impl<'a> Deref for Buffer<'a> {
    type Target = BufferInner;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a> DerefMut for Buffer<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}
