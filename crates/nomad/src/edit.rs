use smol_str::SmolStr;

use crate::ByteOffset;

/// An edit on a buffer.
pub struct Edit {
    start: ByteOffset,
    end: ByteOffset,
    replacement: SmolStr,
}

impl Edit {
    /// The end of the edited range.
    #[inline]
    pub fn end(&self) -> ByteOffset {
        self.end
    }

    #[inline]
    pub(crate) fn new(
        start: ByteOffset,
        end: ByteOffset,
        replacement: impl Into<SmolStr>,
    ) -> Self {
        Self { start, end, replacement: replacement.into() }
    }

    /// The replacement text.
    #[inline]
    pub fn replacement(&self) -> &str {
        &self.replacement
    }

    /// The start of the edited range.
    #[inline]
    pub fn start(&self) -> ByteOffset {
        self.start
    }
}

impl From<nvim::api::opts::OnBytesArgs> for Edit {
    #[inline]
    fn from(
        (
            _bytes,
            buf,
            _changedtick,
            start_row,
            start_col,
            start_offset,
            _old_end_row,
            _old_end_col,
            old_end_len,
            new_end_row,
            new_end_col,
            _new_end_len,
        ): nvim::api::opts::OnBytesArgs,
    ) -> Self {
        todo!();
        // let replacement_start = Point { row: start_row, col: start_col };
        //
        // let replacement_end = Point {
        //     row: start_row + new_end_row,
        //     col: start_col * (new_end_row == 0) as usize + new_end_col,
        // };
        //
        // let replacement = if replacement_start == replacement_end {
        //     String::new()
        // } else {
        //     nvim_buf_get_text(&buf, replacement_start..replacement_end)
        //         .expect("buffer must exist")
        // };
        //
        // Self {
        //     start: start_offset,
        //     end: start_offset + old_end_len,
        //     replacement,
        // }
    }
}
