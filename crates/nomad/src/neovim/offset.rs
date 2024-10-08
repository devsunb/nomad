use core::ops::Range;

use nvim_oxi::api;

use super::Point;
use crate::ByteOffset;

pub trait Offset: Sized + 'static {
    /// TODO: docs.
    fn deleted_range(args: &api::opts::OnBytesArgs) -> Range<Self>;
}

impl Offset for ByteOffset {
    fn deleted_range(args: &api::opts::OnBytesArgs) -> Range<Self> {
        let &(
            ref _bytes,
            ref _buf,
            _changedtick,
            _start_row,
            _start_col,
            start_offset,
            _old_end_row,
            _old_end_col,
            old_end_len,
            _new_end_row,
            _new_end_col,
            _new_end_len,
        ) = args;

        (start_offset).into()..(start_offset + old_end_len).into()
    }
}

impl Offset for Point {
    fn deleted_range(args: &api::opts::OnBytesArgs) -> Range<Self> {
        let &(
            ref _bytes,
            ref _buf,
            _changedtick,
            start_row,
            start_col,
            start_offset,
            old_end_row,
            old_end_col,
            _old_end_len,
            _new_end_row,
            _new_end_col,
            _new_end_len,
        ) = args;

        let start = Point {
            line_idx: start_row,
            byte_offset: ByteOffset::new(start_offset),
        };

        let end = Point {
            line_idx: start_row + old_end_row,
            byte_offset: (start_col * (old_end_row == 0) as usize
                + old_end_col)
                .into(),
        };

        start..end
    }
}
