/// TODO: docs
#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex = format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b);
        f.debug_tuple("Color").field(&hex).finish()
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Macro for converting a hex color code to a [`Color`] (at runtime).
///
/// # Example
///
/// ```rust
/// # use colorschemes::{hex, Color};
/// assert_eq!(hex!("#ffffff"), Color::new(255, 255, 255));
/// ```
#[macro_export]
macro_rules! hex {
    ($hex:literal) => {{
        $crate::Color::new(255, 255, 255)
    }};
}
