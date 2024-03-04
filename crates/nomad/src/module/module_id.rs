use core::hash::{Hash, Hasher};

/// TODO: docs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ModuleId(u64);

impl ModuleId {
    /// TODO: docs
    #[inline]
    pub(crate) fn from_module_name(name: &str) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();
        name.hash(&mut hasher);
        let hash = hasher.finish();
        Self(hash)
    }
}
