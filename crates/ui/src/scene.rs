use compact_str::CompactString;

use crate::{Bound, Cells, SceneFragment, Surface};

/// TODO: docs
pub(crate) struct Scene {
    lines: Vec<SceneLine>,
    diff: DiffTracker,
}

impl Scene {
    /// Turns the entire `Scene` into a `SceneFragment` which can be used in
    /// the [`paint`](crate::Render::paint) method of a
    /// [`Render`](crate::Render) implementation.
    #[inline]
    pub(crate) fn as_fragment(&mut self) -> SceneFragment<'_> {
        todo!()
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn diff(&mut self) -> SceneDiff<'_> {
        todo!();
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn new() -> Self {
        todo!()
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn resize(&mut self, new_size: Bound<Cells>) {
        let op = ResizeOp::new(self.size(), new_size);
        op.apply(self);
        self.diff.resize = op;
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn size(&self) -> Bound<Cells> {
        todo!();
    }
}

/// TODO: docs
struct SceneLine {
    runs: Vec<SceneRun>,
}

/// TODO: docs
struct SceneRun {
    /// TODO: docs.
    text: CompactString,
}

/// TODO: docs
#[derive(Debug, Default)]
struct DiffTracker {
    /// TODO: docs.
    resize: ResizeOp,

    /// TODO: docs
    paint: Vec<PaintOp>,
}

/// A `ResizeOp` is a collection of operations that resize a `Scene`.
#[derive(Debug, Copy, Clone, Default)]
struct ResizeOp {
    shrink: ShrinkOp,
    expand: ExpandOp,
}

impl ResizeOp {
    /// Applies the resize operations to a `Scene`.
    ///
    /// The [`size`](Scene::size) of the given scene is guaranteed to return
    /// `new_size` after this method is called, where `new_size` is the new
    /// size passed to [`ResizeOp::new`].
    #[inline]
    fn apply(&self, scene: &mut Scene) {
        self.shrink.apply(scene);
        self.expand.apply(scene);
    }

    #[inline]
    fn new(old_size: Bound<Cells>, new_size: Bound<Cells>) -> Self {
        let shrink = ShrinkOp::new(old_size, new_size);
        let expand = ExpandOp::new(old_size, new_size);
        Self { shrink, expand }
    }
}

/// A `ShrinkOp` shrinks a [`Scene`] by deleting lines and/or truncating lines.
#[derive(Debug, Copy, Clone, Default)]
struct ShrinkOp {
    delete_lines: Option<DeleteLinesOp>,
    truncate_lines: Option<TruncateLinesOp>,
}

impl ShrinkOp {
    #[inline]
    fn apply(&self, _scene: &mut Scene) {}

    #[inline]
    fn new(old_size: Bound<Cells>, new_size: Bound<Cells>) -> Self {
        let delete_lines = if new_size.height() < old_size.height() {
            Some(DeleteLinesOp((old_size.height() - new_size.height()).into()))
        } else {
            None
        };

        let truncate_lines = if new_size.width() < old_size.width() {
            Some(TruncateLinesOp((old_size.width() - new_size.width()).into()))
        } else {
            None
        };

        Self { delete_lines, truncate_lines }
    }
}

/// A `DeleteLinesOp(n)` shrinks a [`Scene`] vertically by keeping the first
/// `n` lines of a `Scene` and deletes the rest.
///
/// For example, a `DeleteLinesOp(1)` would transform the following scene:
///
/// ```txt
/// ┌────────────┐
/// │▒▒▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒▒3x12▒▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒▒▒│
/// └────────────┘
/// ```
///
/// into:
///
/// ```txt
/// ┌────────────┐
/// │▒▒▒▒1x12▒▒▒▒│
/// └────────────┘
/// ```
///
/// A `DeleteLinesOp(0)` deletes all the lines of a `Scene`.
#[derive(Debug, Clone, Copy)]
struct DeleteLinesOp(u32);

/// A `TruncateLinesOp(n)` shrinks a [`Scene`] horizontally by keeping the
/// first `n` cells of every line of a `Scene` and deleting the rest.
///
/// For example, a `TruncateLinesOp(10)` would transform the following scene:
///
/// ```txt
/// ┌────────────────────┐
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒▒▒▒▒▒3x20▒▒▒▒▒▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒│
/// └────────────────────┘
/// ```
///
/// into:
///
/// ```txt
/// ┌──────────┐
/// │▒▒▒▒▒▒▒▒▒▒│
/// │▒▒▒3x10▒▒▒│
/// │▒▒▒▒▒▒▒▒▒▒│
/// └──────────┘
/// ```
///
/// A `TruncateLinesOp(0)` deletes all the cells of a `Scene`.
#[derive(Debug, Clone, Copy)]
struct TruncateLinesOp(u32);

/// An `ExpandOp` expands a `Scene` by inserting lines and/or extending lines.
#[derive(Debug, Clone, Copy, Default)]
struct ExpandOp {}

impl ExpandOp {
    #[inline]
    fn apply(&self, _scene: &mut Scene) {}

    #[inline]
    fn new(old_size: Bound<Cells>, new_size: Bound<Cells>) -> Self {
        todo!();
    }
}

/// TODO: docs
#[derive(Debug)]
struct PaintOp {}

/// TODO: docs
pub(crate) struct SceneDiff<'a> {
    fragment: SceneFragment<'a>,
}

impl<'a> SceneDiff<'a> {
    /// TODO: docs
    #[inline]
    pub(crate) fn apply(self, _surface: &mut Surface) {
        todo!()
    }
}
