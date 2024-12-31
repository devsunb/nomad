use core::fmt;

/// TODO: docs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {}

impl Message {
    /// TODO: docs.
    #[inline]
    pub fn as_str(&self) -> &str {
        "TODO: Message"
    }
}

impl fmt::Display for Message {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
