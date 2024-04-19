use smol_str::SmolStr;

/// An replacement edit on a buffer.
pub struct Replacement<Offset> {
    start: Offset,
    end: Offset,
    replacement: SmolStr,
}

impl<Offset: Copy> Replacement<Offset> {
    /// The end of the replaced range.
    #[inline]
    pub fn end(&self) -> Offset {
        self.end
    }

    #[inline]
    pub(crate) fn new(
        start: Offset,
        end: Offset,
        replacement: impl Into<SmolStr>,
    ) -> Self {
        Self { start, end, replacement: replacement.into() }
    }

    /// The text the range is replaced with.
    #[inline]
    pub fn replacement(&self) -> &str {
        &self.replacement
    }

    /// The end of the replaced range.
    #[inline]
    pub fn start(&self) -> Offset {
        self.start
    }
}
