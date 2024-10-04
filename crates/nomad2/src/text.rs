use smol_str::SmolStr;

/// TODO: docs.
#[derive(Default)]
pub struct Text {
    inner: SmolStr,
}

impl Text {
    /// Creates a new empty `Text`.
    pub fn new() -> Self {
        Self::default()
    }

    /// TODO: docs.
    pub fn push(&mut self, ch: char) {
        todo!();
    }

    /// TODO: docs.
    pub fn push_str(&mut self, s: &str) {
        todo!();
    }
}
