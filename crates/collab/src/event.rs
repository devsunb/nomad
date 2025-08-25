//! TODO: docs.

use core::ops::Range;

use abs_path::AbsPathBuf;
use editor::{ByteOffset, Editor, Replacement};
use fs::{DirectoryEvent, FileEvent};
use smallvec::SmallVec;

pub use crate::event_stream::{EventError, EventStream};

/// TODO: docs.
#[derive(cauchy::Debug)]
pub enum Event<Ed: Editor> {
    /// TODO: docs.
    Buffer(BufferEvent<Ed>),

    /// TODO: docs.
    Cursor(CursorEvent<Ed>),

    /// TODO: docs.
    Directory(DirectoryEvent<Ed::Fs>),

    /// TODO: docs.
    File(FileEvent<Ed::Fs>),

    /// TODO: docs.
    Selection(SelectionEvent<Ed>),
}

/// TODO: docs.
#[derive(cauchy::Debug)]
pub enum BufferEvent<Ed: Editor> {
    /// TODO: docs.
    Created(Ed::BufferId, AbsPathBuf),

    /// TODO: docs.
    Edited(Ed::BufferId, SmallVec<[Replacement; 1]>),

    /// TODO: docs.
    Removed(Ed::BufferId),
    /// TODO: docs.
    Saved(Ed::BufferId),
}

/// TODO: docs.
#[derive(cauchy::Debug)]
pub struct CursorEvent<Ed: Editor> {
    /// TODO: docs.
    pub cursor_id: Ed::CursorId,

    /// TODO: docs.
    pub kind: CursorEventKind<Ed>,
}

/// TODO: docs.
#[derive(cauchy::Debug)]
pub enum CursorEventKind<Ed: Editor> {
    /// TODO: docs.
    Created(Ed::BufferId, ByteOffset),

    /// TODO: docs.
    Moved(ByteOffset),

    /// TODO: docs.
    Removed,
}

/// TODO: docs.
#[derive(cauchy::Debug)]
pub struct SelectionEvent<Ed: Editor> {
    /// TODO: docs.
    pub selection_id: Ed::SelectionId,
    /// TODO: docs.
    pub kind: SelectionEventKind<Ed>,
}

/// TODO: docs.
#[derive(cauchy::Debug)]
pub enum SelectionEventKind<Ed: Editor> {
    /// TODO: docs.
    Created(Ed::BufferId, Range<ByteOffset>),

    /// TODO: docs.
    Moved(Range<ByteOffset>),

    /// TODO: docs.
    Removed,
}
