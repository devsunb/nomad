/// A point in a text buffer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Point<Offset> {
    /// The index of the line the point is on.
    line_idx: usize,

    /// The offset of the point within the line.
    line_offset: Offset,
}

impl<Offset: Copy> Point<Offset> {
    /// The index of the line the point is on.
    #[inline]
    pub fn line(&self) -> usize {
        self.line_idx
    }

    /// Creates a new [`Point`].
    #[inline]
    pub fn new(line_idx: usize, line_offset: Offset) -> Self {
        Self { line_idx, line_offset }
    }

    /// The offset of the point within the line.
    #[inline]
    pub fn offset(&self) -> Offset {
        self.line_offset
    }
}
