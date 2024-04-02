use alloc::rc::Rc;
use core::cell::RefCell;
use core::ops::Range;

use cola::{Anchor, Deletion, Insertion, Replica};
use crop::{Rope, RopeBuilder};
use nvim::api::{self, opts, Buffer as NvimBuffer};

use crate::editor::{
    ByteChange,
    ByteOffset,
    Point,
    RemoteDeletion,
    RemoteInsertion,
};
use crate::streams::{AppliedDeletion, AppliedInsertion};

/// TODO: docs
#[derive(Clone)]
pub(crate) struct BufferState {
    /// TODO: docs
    bound_to: Option<NvimBuffer>,

    /// TODO: docs
    inner: Rc<RefCell<BufferInner>>,
}

/// TODO: docs
struct BufferInner {
    /// TODO: docs
    replica: Replica,

    /// TODO: docs
    text: Rope,
}

impl Clone for BufferInner {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            replica: self.replica.fork(self.replica.id()),
            text: self.text.clone(),
        }
    }
}

impl BufferInner {
    /// TODO: docs
    #[inline]
    pub fn delete(&mut self, range: Range<ByteOffset>) -> Deletion {
        self.text.delete(range.clone());
        self.replica.deleted(range)
    }

    /// TODO: docs
    #[inline]
    pub fn edit<E>(&mut self, edit: E) -> E::Diff
    where
        E: Edit,
    {
        edit.apply(self)
    }

    /// TODO: docs
    #[inline]
    pub fn insert(&mut self, offset: ByteOffset, text: &str) -> Insertion {
        self.text.insert(offset, text);
        self.replica.inserted(offset, text.len())
    }

    /// TODO: docs
    #[inline]
    pub fn integrate_deletion(
        &mut self,
        deletion: &Deletion,
    ) -> Vec<Range<ByteOffset>> {
        let byte_ranges = self.replica.integrate_deletion(&deletion);
        byte_ranges.iter().rev().for_each(|r| self.text.delete(r.clone()));
        byte_ranges
    }

    /// TODO: docs
    #[inline]
    pub fn integrate_insertion(
        &mut self,
        insertion: &Insertion,
        text: &str,
    ) -> Option<ByteOffset> {
        let offset = self.replica.integrate_insertion(insertion)?;
        self.text.insert(offset, text);
        Some(offset)
    }

    /// Transforms the 1-dimensional byte offset into a 2-dimensional
    /// [`Point`].
    #[inline]
    pub fn point_of_offset(&self, byte_offset: ByteOffset) -> Point {
        let row = self.text.line_of_byte(byte_offset);
        let row_offset = self.text.byte_of_line(row);
        let col = byte_offset - row_offset;
        Point { row, col }
    }

    /// TODO: docs
    #[inline]
    pub fn resolve_anchor(&self, anchor: &Anchor) -> Option<ByteOffset> {
        self.replica.resolve_anchor(*anchor)
    }
}

/// TODO: docs
pub trait Edit {
    /// TODO: docs
    type Diff;

    /// TODO: docs
    fn apply(self, buf: &mut BufferInner) -> Self::Diff;
}

impl Edit for ByteChange {
    type Diff = (Option<AppliedDeletion>, Option<AppliedInsertion>);

    #[inline]
    fn apply(self, buf: &mut BufferInner) -> Self::Diff {
        let mut applied_del = None;
        let mut applied_ins = None;

        if !self.byte_range().is_empty() {
            let del = buf.delete(self.byte_range());
            applied_del = Some(AppliedDeletion::new(del));
        }

        if !self.text().is_empty() {
            let ins = buf.insert(self.byte_range().start, self.text());
            applied_ins = Some(AppliedInsertion::new(ins, self.into_text()));
        }

        (applied_del, applied_ins)
    }
}

/// TODO: docs
pub struct LocalDeletion {
    range: Range<Anchor>,
}

impl Edit for LocalDeletion {
    type Diff = Option<(AppliedDeletion, Range<Point>)>;

    fn apply(self, buf: &mut BufferInner) -> Self::Diff {
        let start_anchor = &self.range.start;

        let end_anchor = &self.range.end;

        let Some(start_offset) = buf.resolve_anchor(&start_anchor) else {
            panic_couldnt_resolve_anchor(&start_anchor);
        };

        let Some(end_offset) = buf.resolve_anchor(&end_anchor) else {
            panic_couldnt_resolve_anchor(&end_anchor);
        };

        if start_offset == end_offset {
            return None;
        }

        let start_point = buf.point_of_offset(start_offset);

        let end_point = buf.point_of_offset(end_offset);

        let deletion = buf.delete(start_offset..end_offset);

        Some((AppliedDeletion::new(deletion), start_point..end_point))
    }
}

/// TODO: docs
pub struct LocalInsertion {
    insert_at: Anchor,
    text: String,
}

impl Edit for LocalInsertion {
    type Diff = (AppliedInsertion, Point);

    fn apply(self, buf: &mut BufferInner) -> Self::Diff {
        let Some(byte_offset) = buf.resolve_anchor(&self.insert_at) else {
            panic_couldnt_resolve_anchor(&self.insert_at);
        };

        let point = buf.point_of_offset(byte_offset);

        let insertion = buf.insert(byte_offset, &self.text);

        (AppliedInsertion::new(insertion, self.text), point)
    }
}

impl Edit for &RemoteDeletion {
    type Diff = Vec<Range<Point>>;

    fn apply(self, buf: &mut BufferInner) -> Self::Diff {
        let buf_prev = buf.clone();

        let byte_ranges = buf.integrate_deletion(self.inner());

        byte_ranges
            .into_iter()
            .map(|range| {
                let start = buf_prev.point_of_offset(range.start);
                let end = buf_prev.point_of_offset(range.end);
                start..end
            })
            .collect()
    }
}

impl Edit for &RemoteInsertion {
    type Diff = Option<Point>;

    fn apply(self, buf: &mut BufferInner) -> Self::Diff {
        let buf_prev = buf.clone();
        let offset = buf.integrate_insertion(self.inner(), self.text())?;
        let point = buf_prev.point_of_offset(offset);
        Some(point)
    }
}

#[inline(never)]
fn panic_couldnt_resolve_anchor(anchor: &Anchor) -> ! {
    panic!("{anchor:?} couldn't be resolved");
}
