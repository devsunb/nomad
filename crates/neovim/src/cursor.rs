//! TODO: docs.

use ed::ByteOffset;
use ed::backend::{AgentId, Buffer, Cursor};

use crate::Neovim;
use crate::buffer::{BufferId, NeovimBuffer, Point};
use crate::events::{self, EventHandle, Events};
use crate::oxi::api;

/// TODO: docs.
#[derive(Copy, Clone)]
pub struct NeovimCursor<'a> {
    buffer: NeovimBuffer<'a>,
}

impl<'a> NeovimCursor<'a> {
    /// Returns the [`NeovimBuffer`] this cursor is in.
    #[inline]
    pub(crate) fn buffer(&self) -> NeovimBuffer<'a> {
        self.buffer
    }

    #[inline]
    pub(crate) fn new(buffer: NeovimBuffer<'a>) -> Self {
        debug_assert!(buffer.is_focused());
        Self { buffer }
    }

    /// Returns the [`Point`] this cursor is currently at.
    fn point(&self) -> Point {
        let (row, col) =
            api::Window::current().get_cursor().expect("couldn't get cursor");
        Point { line_idx: row - 1, byte_offset: col.into() }
    }
}

impl Cursor for NeovimCursor<'_> {
    type Backend = Neovim;

    #[inline]
    fn buffer_id(&self) -> BufferId {
        self.buffer().id()
    }

    #[inline]
    fn byte_offset(&self) -> ByteOffset {
        self.buffer().byte_offset_of_point(self.point())
    }

    #[inline]
    fn id(&self) -> BufferId {
        self.buffer().id()
    }

    #[inline]
    fn r#move(&mut self, offset: ByteOffset, _agent_id: AgentId) {
        let point = self.buffer().point_of_byte_offset(offset);

        api::Window::current()
            .set_cursor(point.line_idx + 1, point.byte_offset.into())
            .expect("couldn't set cursor");
    }

    #[inline]
    fn on_moved<Fun>(&self, mut fun: Fun) -> EventHandle
    where
        Fun: FnMut(&NeovimCursor<'_>, AgentId) + 'static,
    {
        Events::insert(
            self.buffer().events(),
            events::CursorMoved(self.buffer_id()),
            move |(this, moved_by)| fun(this, moved_by),
        )
    }

    #[inline]
    fn on_removed<Fun>(&self, mut fun: Fun) -> EventHandle
    where
        Fun: FnMut(BufferId, AgentId) + 'static,
    {
        Events::insert(
            self.buffer().events(),
            events::BufLeave(self.buffer_id()),
            move |(&buf, unfocused_by)| fun(buf.id(), unfocused_by),
        )
    }
}
