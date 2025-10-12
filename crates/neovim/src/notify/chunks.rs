use compact_str::CompactString;
use smallvec::SmallVec;

use crate::notify::Chunk;

/// The chunks of text forming a notification message.
#[derive(Default)]
pub struct Chunks {
    inner: SmallVec<[Chunk; 4]>,
}

impl Chunks {
    /// Appends a chunk with no highlight group.
    #[inline]
    pub fn push(&mut self, chunk_text: impl Into<CompactString>) -> &mut Self {
        self.inner.push(Chunk::new(chunk_text));
        self
    }

    /// Appends a chunk with the given highlight group.
    #[inline]
    pub fn push_highlighted(
        &mut self,
        text: impl Into<CompactString>,
        hl_group: impl Into<CompactString>,
    ) -> &mut Self {
        self.inner.push(Chunk::new_highlighted(text, hl_group));
        self
    }

    /// Appends a newline character to the previous chunk (creating a new one
    /// if necessary).
    #[inline]
    pub fn push_newline(&mut self) -> &mut Self {
        match self.inner.last_mut() {
            Some(last) => last.text_mut().push('\n'),
            None => self.inner.push(Chunk::new("\n")),
        }
        self
    }
}

impl From<&str> for Chunks {
    #[inline]
    fn from(s: &str) -> Self {
        Self { inner: smallvec::smallvec![Chunk::new(s)] }
    }
}

impl From<String> for Chunks {
    #[inline]
    fn from(s: String) -> Self {
        Self { inner: smallvec::smallvec![Chunk::new(s)] }
    }
}

impl From<core::fmt::Arguments<'_>> for Chunks {
    #[inline]
    fn from(args: core::fmt::Arguments<'_>) -> Self {
        Self {
            inner: smallvec::smallvec![Chunk::new(
                compact_str::format_compact!("{args}")
            )],
        }
    }
}
