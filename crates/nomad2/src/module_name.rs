use core::fmt;

/// TODO: docs
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleName {
    name: &'static str,
}

impl ModuleName {
    /// TODO: docs
    #[doc(hidden)]
    pub const fn from_str(name: &'static str) -> Self {
        Self { name }
    }

    /// TODO: docs
    #[inline]
    pub(crate) const fn as_str(&self) -> &'static str {
        self.name
    }
}

impl fmt::Debug for ModuleName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("ModuleName").field(&self.name).finish()
    }
}

impl fmt::Display for ModuleName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name)
    }
}

impl AsRef<str> for ModuleName {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name
    }
}
