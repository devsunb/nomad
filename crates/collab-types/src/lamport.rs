//! Contains a [`LamportClock`] producing [`LamportTimestamp`]s.

use core::hash::{Hash, Hasher};

use crate::Counter;

/// A [Lamport clock][lamport-ts] producing [`LamportTimestamp`]s.
///
/// [lamport-ts]: https://en.wikipedia.org/wiki/Lamport_timestamp
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LamportClock {
    counter: Counter<u64>,
}

impl LamportClock {
    /// Sets the clock to the maximum of its current value and the given
    /// `LamportTimestamp`.
    #[inline]
    pub fn max_assign(&mut self, lamport_ts: LamportTimestamp) {
        self.counter.value = self.counter.value.max(lamport_ts.into_u64() + 1);
    }

    /// Creates a new [`LamportClock`] with the given initial value.
    #[inline]
    pub const fn new(initial_value: u64) -> Self {
        Self { counter: Counter::new(initial_value) }
    }

    /// Increments the clock and returns a new [`LamportTimestamp`].
    #[inline]
    pub fn tick(&mut self) -> LamportTimestamp {
        LamportTimestamp(self.counter.post_increment())
    }
}

/// A Lamport timestamp produced by a [`LamportClock`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct LamportTimestamp(u64);

impl LamportTimestamp {
    /// Converts this [`LamportTimestamp`] into the inner `u64` value.
    #[inline]
    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

impl Hash for LamportTimestamp {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_u64(self.into_u64());
    }
}

impl nohash::IsEnabled for LamportTimestamp {}
