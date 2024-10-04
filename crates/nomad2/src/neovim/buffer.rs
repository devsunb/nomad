use alloc::borrow::Cow;
use core::cmp::Ordering;
use core::ops::RangeBounds;

use collab_fs::AbsUtf8Path;
use nvim_oxi::api::Buffer as NvimBuffer;

use super::Neovim;
use crate::{ByteOffset, Text};

/// TODO: docs.
pub struct Buffer {
    id: BufferId,
}

impl Buffer {
    pub(super) fn new(id: BufferId) -> Self {
        Self { id }
    }
}

impl crate::Buffer<Neovim> for Buffer {
    type Id = BufferId;

    fn byte_len(&self) -> usize {
        todo!()
    }

    fn get_text<R>(&self, byte_range: R) -> Text
    where
        R: RangeBounds<ByteOffset>,
    {
        todo!()
    }

    fn id(&self) -> Self::Id {
        todo!()
    }

    fn path(&self) -> Option<Cow<'_, AbsUtf8Path>> {
        todo!()
    }

    fn set_text<R, T>(&mut self, replaced_range: R, new_text: T) -> Text
    where
        R: RangeBounds<ByteOffset>,
        T: AsRef<str>,
    {
        todo!()
    }
}

/// TODO: docs.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BufferId {
    inner: NvimBuffer,
}

impl BufferId {
    pub(super) fn is_of_text_buffer(&self) -> bool {
        todo!();
    }

    pub(super) fn new(inner: NvimBuffer) -> Self {
        Self { inner }
    }
}

impl PartialOrd for BufferId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BufferId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.handle().cmp(&other.inner.handle())
    }
}
