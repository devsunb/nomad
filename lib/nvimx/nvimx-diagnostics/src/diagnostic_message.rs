use core::iter;

use nvim_oxi::api;

use crate::diagnostic_source::DiagnosticSource;
use crate::highlight_group::HighlightGroup;
use crate::level::Level;

/// TODO: docs.
#[derive(Default)]
pub struct DiagnosticMessage {
    #[doc(hidden)]
    pub chunks: Vec<(nvim_oxi::String, Option<HighlightGroup>)>,
}

impl DiagnosticMessage {
    /// TODO: docs.
    pub fn emit(self, level: Level, source: DiagnosticSource) {
        let source_chunk = (source.to_string().into(), Some(level.into()));
        let space_chunk = (" ".into(), None);
        let chunks = iter::once(source_chunk)
            .chain(iter::once(space_chunk))
            .chain(self.chunks);
        let opts = api::opts::EchoOpts::default();
        api::echo(chunks, true, &opts).expect("all parameters are valid");
    }

    /// Creates a new, empty [`DiagnosticMessage`].
    pub fn new() -> Self {
        Self::default()
    }

    /// TODO: docs.
    pub fn push(&mut self, c: char) -> &mut Self {
        self.push_chunk(c.into(), None)
    }

    /// TODO: docs.
    pub fn push_comma_separated<T, I>(
        &mut self,
        iter: I,
        hl: HighlightGroup,
    ) -> &mut Self
    where
        T: AsRef<str>,
        I: IntoIterator<Item = T>,
    {
        self.push_separated(iter, hl, ", ")
    }

    /// TODO: docs.
    pub fn push_dot_separated<T, I>(
        &mut self,
        iter: I,
        hl: HighlightGroup,
    ) -> &mut Self
    where
        T: AsRef<str>,
        I: IntoIterator<Item = T>,
    {
        self.push_separated(iter, hl, ".")
    }

    /// TODO: docs.
    pub fn push_str<T: AsRef<str>>(&mut self, s: T) -> &mut Self {
        self.push_chunk(s.as_ref().into(), None)
    }

    /// TODO: docs.
    pub fn push_str_highlighted<T: AsRef<str>>(
        &mut self,
        s: T,
        hl: HighlightGroup,
    ) -> &mut Self {
        self.push_chunk(s.as_ref().into(), Some(hl))
    }

    fn push_chunk(
        &mut self,
        s: nvim_oxi::String,
        hl: Option<HighlightGroup>,
    ) -> &mut Self {
        self.chunks.push((s, hl));
        self
    }

    fn push_separated<T, I>(
        &mut self,
        iter: I,
        hl: HighlightGroup,
        separator: &str,
    ) -> &mut Self
    where
        T: AsRef<str>,
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter().peekable();
        loop {
            let Some(text) = iter.next() else {
                break;
            };
            self.push_str_highlighted(text.as_ref(), hl.clone());
            if iter.peek().is_some() {
                self.push_str(separator);
            }
        }
        self
    }
}

impl From<core::convert::Infallible> for DiagnosticMessage {
    fn from(_: core::convert::Infallible) -> Self {
        unreachable!()
    }
}
