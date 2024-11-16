use nvimx_diagnostics::HighlightGroup;

/// TODO: docs.
pub enum Severity {
    /// TODO: docs.
    Debug,

    /// TODO: docs.
    Error,

    /// TODO: docs.
    Info,

    /// TODO: docs.
    Warning,
}

impl Severity {
    pub(crate) fn hl_group(&self) -> HighlightGroup {
        match self {
            Self::Debug => HighlightGroup::special(),
            Self::Error => HighlightGroup::error(),
            Self::Info => HighlightGroup::special(),
            Self::Warning => HighlightGroup::warning(),
        }
    }
}
