use api::opts::OptionOpts;
use nvim_oxi::api::{self, Buffer};
use nvimx_common::Apply;

/// TODO: docs.
pub struct TextBuffer {
    _inner: Buffer,
}

impl TextBuffer {
    /// TODO: docs.
    #[inline]
    pub fn current() -> Result<Self, NotTextBufferError> {
        let buffer = Buffer::current();

        let buftype: String = {
            let opts = OptionOpts::builder().buffer(buffer.clone()).build();
            api::get_option_value("buftype", &opts).expect("always set")
        };

        match buftype.as_ref() {
            "" => Ok(Self { _inner: buffer }),
            "help" => Err(NotTextBufferError::Help),
            "quickfix" => Err(NotTextBufferError::Quickfix),
            "terminal" => Err(NotTextBufferError::Terminal),
            _ => panic!("unknown buftype: {}", buftype),
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn edit<E>(&mut self, edit: E) -> <Self as Apply<E>>::Diff
    where
        Self: Apply<E>,
    {
        self.apply(edit)
    }
}

/// Error type returned by [`TextBuffer::current`] when the current buffer
/// is not a text buffer.
#[derive(Debug)]
pub enum NotTextBufferError {
    /// The current buffer is a help file.
    Help,

    /// The current buffer is a quickfix list.
    Quickfix,

    /// The current buffer house a terminal emulator.
    Terminal,
}
