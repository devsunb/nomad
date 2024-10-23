use nvim_oxi::api::types;

use crate::ctx::{BufferCtx, FileCtx, TextBufferCtx};
use crate::neovim::BufferId;

/// TODO: docs.
#[derive(Clone)]
pub struct TextFileCtx<'ctx> {
    ctx: BufferCtx<'ctx>,
}

impl<'ctx> TextFileCtx<'ctx> {
    pub(crate) fn from_file(file_ctx: FileCtx<'ctx>) -> Option<Self> {
        todo!();
    }

    pub(crate) fn from_text_buffer(
        text_buffer_ctx: TextBufferCtx<'ctx>,
    ) -> Option<Self> {
        todo!();
    }
}
