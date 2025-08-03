use core::hash::{Hash, Hasher};

/// TODO: docs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct PeerId(puff::PeerId);

impl PeerId {
    /// TODO: docs.
    #[inline]
    pub const fn into_u64(self) -> u64 {
        self.0
    }

    /// TODO: docs.
    #[inline]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

impl From<puff::PeerId> for PeerId {
    #[inline]
    fn from(id: puff::PeerId) -> Self {
        Self(id)
    }
}

impl From<PeerId> for puff::PeerId {
    #[inline]
    fn from(id: PeerId) -> Self {
        id.0
    }
}

impl Hash for PeerId {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u64(self.into_u64());
    }
}

impl nohash::IsEnabled for PeerId {}
