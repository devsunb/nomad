use core::fmt;
use core::num::NonZeroU64;

/// TODO: docs.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AgentId(u64);

impl AgentId {
    /// TODO: docs.
    pub const UNKNOWN: Self = Self(0);

    /// TODO: docs.
    #[inline]
    pub fn is_unknown(self) -> bool {
        self.0 == Self::UNKNOWN.0
    }

    #[inline]
    pub(crate) fn new(id: NonZeroU64) -> Self {
        Self(id.into())
    }

    #[inline]
    pub(crate) fn post_inc(&mut self) -> Self {
        let id = self.0;
        self.0 += 1;
        Self(id)
    }
}

impl fmt::Debug for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_unknown() {
            f.write_str("AgentId::UNKNOWN")
        } else {
            f.debug_tuple("AgentId").field(&self.0).finish()
        }
    }
}

impl Default for AgentId {
    #[inline]
    fn default() -> Self {
        Self::UNKNOWN
    }
}
