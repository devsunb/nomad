use nvim::api::{self, opts};

use crate::{ByteOffset, Replacement, Shared};

type OnEdit = Box<dyn FnMut(&Replacement<ByteOffset>) + 'static>;

/// A handle to a Neovim buffer.
#[cfg_attr(not(feature = "tests"), doc(hidden))]
#[derive(Clone)]
pub struct NvimBuffer {
    /// The buffer handle.
    inner: api::Buffer,

    /// The list of callbacks to be called every time the buffer is edited.
    on_edit_callbacks: Shared<Vec<OnEdit>>,
}

impl core::fmt::Debug for NvimBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NvimBuffer").field(&self.inner).finish()
    }
}

impl NvimBuffer {
    /// Creates a new buffer.
    #[inline]
    pub fn create() -> Self {
        let Ok(buf) = api::create_buf(true, false) else { unreachable!() };
        let Ok(buf) = Self::new(buf) else { unreachable!() };
        buf
    }

    /// Registers a callback to be called every time the buffer is edited.
    #[inline]
    pub fn on_edit<F: FnMut(&Replacement<ByteOffset>) + 'static>(
        &self,
        callback: F,
    ) {
        self.on_edit_callbacks
            .with_mut(|callbacks| callbacks.push(Box::new(callback)));
    }

    #[inline]
    fn new(buffer: api::Buffer) -> Result<Self, NvimBufferDoesntExistError> {
        let on_edit_callbacks = Shared::<Vec<OnEdit>>::default();

        let cbs = on_edit_callbacks.clone();

        let opts = opts::BufAttachOpts::builder()
            .on_bytes(move |args| {
                let edit = Replacement::from(args);
                cbs.with_mut(|cbs| cbs.iter_mut().for_each(|cb| cb(&edit)));
                Ok(false)
            })
            .build();

        buffer
            .attach(false, &opts)
            // All the arguments passed to `attach()` are valid, so if it fails
            // it must be because the buffer doesn't exist.
            .map_err(|_| NvimBufferDoesntExistError)?;

        Ok(Self { inner: buffer, on_edit_callbacks })
    }
}

impl From<nvim::api::opts::OnBytesArgs> for Replacement<ByteOffset> {
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

/// An error returned whenever a..
pub struct NvimBufferDoesntExistError;
