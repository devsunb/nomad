//! TODO: docs.

use core::ptr::NonNull;

use crate::oxi;

/// TODO: docs.
pub struct NeovimSerializer {
    inner: oxi::serde::Serializer,
}

/// TODO: docs.
pub struct NeovimDeserializer {
    inner: serde_path_to_error::Deserializer<
        'static,
        'static,
        oxi::serde::Deserializer,
    >,
    track: NonNull<serde_path_to_error::Track>,
}

/// TODO: docs.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct NeovimSerializeError(oxi::serde::SerializeError);

/// TODO: docs.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct NeovimDeserializeError(oxi::serde::DeserializeError);

impl NeovimDeserializer {
    #[inline]
    pub(crate) fn new(value: oxi::Object) -> Self {
        let track = Box::leak(Box::new(serde_path_to_error::Track::new()));
        Self {
            track: NonNull::from(&*track),
            inner: serde_path_to_error::Deserializer::new(
                oxi::serde::Deserializer::new(value),
                track,
            ),
        }
    }
}

impl Default for NeovimSerializer {
    #[inline]
    fn default() -> Self {
        Self { inner: oxi::serde::Serializer::new() }
    }
}

impl Drop for NeovimDeserializer {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: the pointer was created by `Box::leak()`, and it's the first
        // and only time we're reclaiming the `Box`.
        let _ = unsafe { Box::from_raw(self.track.as_ptr()) };
    }
}
