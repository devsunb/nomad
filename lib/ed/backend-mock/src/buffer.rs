use core::ops::{Deref, DerefMut};
use std::borrow::Cow;

use crop::Rope;
use ed_core::ByteOffset;
use ed_core::backend::{self, AgentId, Edit, Replacement};

use crate::mock::{self, CallbackKind, Callbacks};

/// TODO: docs.
pub struct Buffer<'a> {
    pub(crate) inner: &'a mut BufferInner,
    pub(crate) callbacks: Callbacks,
}

/// TODO: docs.
#[doc(hidden)]
pub struct BufferInner {
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

impl backend::Buffer for Buffer<'_> {
    type Backend = mock::Mock;
    type Id = BufferId;
    type EventHandle = mock::EventHandle;

    fn byte_len(&self) -> ByteOffset {
        self.contents.byte_len().into()
    }

    fn id(&self) -> Self::Id {
        self.id
    }

    fn edit<R>(&mut self, replacements: R, agent_id: AgentId)
    where
        R: IntoIterator<Item = Replacement>,
    {
        let edit = Edit {
            made_by: agent_id,
            replacements: replacements.into_iter().collect(),
        };

        for replacement in &edit.replacements {
            let range = replacement.removed_range();
            self.contents.replace(
                usize::from(range.start)..usize::from(range.end),
                replacement.inserted_text(),
            );
        }

        self.callbacks.with_mut(|callbacks| {
            for cb_kind in callbacks.values_mut() {
                if let CallbackKind::OnBufferEdited(buf_id, fun) = cb_kind {
                    if *buf_id == self.id() {
                        fun(self, &edit);
                    }
                }
            }
        });
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }

    fn on_edited<Fun>(&mut self, fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Buffer<'_>, &Edit) + 'static,
    {
        let cb_kind = CallbackKind::OnBufferEdited(self.id(), Box::new(fun));
        self.callbacks.insert(cb_kind)
    }

    fn on_removed<Fun>(&mut self, fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Buffer<'_>, AgentId) + 'static,
    {
        let cb_kind = CallbackKind::OnBufferRemoved(self.id(), Box::new(fun));
        self.callbacks.insert(cb_kind)
    }

    fn on_saved<Fun>(&mut self, fun: Fun) -> Self::EventHandle
    where
        Fun: FnMut(&Buffer<'_>, AgentId) + 'static,
    {
        let cb_kind = CallbackKind::OnBufferSaved(self.id(), Box::new(fun));
        self.callbacks.insert(cb_kind)
    }
}

impl Deref for Buffer<'_> {
    type Target = BufferInner;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl DerefMut for Buffer<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}
