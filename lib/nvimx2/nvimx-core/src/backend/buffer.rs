use std::borrow::Cow;

use crate::ByteOffset;
use crate::backend::Backend;

/// TODO: docs.
pub trait Buffer<B: Backend> {
    /// TODO: docs.
    fn byte_len(&self) -> ByteOffset;

    /// TODO: docs.
    fn id(&self) -> B::BufferId;

    /// TODO: docs.
    fn name(&self) -> Cow<'_, str>;
}
