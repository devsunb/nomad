use crate::Color;

/// TODO: docs
#[derive(Default)]
pub struct HighlightGroup {
    link: Option<&'static str>,
    foreground: Option<Color>,
    background: Option<Color>,
}

impl HighlightGroup {
    pub fn into_some(self) -> Option<Self> {
        Some(self)
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn with_foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }
}
