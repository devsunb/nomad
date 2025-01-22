use std::borrow::Cow;
use std::path::PathBuf;

use nvimx_core::backend::Buffer;

use crate::Neovim;

/// TODO: docs.
#[derive(Clone, PartialEq, Eq)]
pub struct NeovimBuffer(crate::oxi::api::Buffer);

impl NeovimBuffer {
    #[inline]
    pub(crate) fn current() -> Self {
        Self(crate::oxi::api::Buffer::current())
    }

    #[inline]
    pub(crate) fn exists(&self) -> bool {
        self.0.is_valid()
    }

    #[inline]
    pub(crate) fn get_name(&self) -> PathBuf {
        debug_assert!(self.exists());
        self.0.get_name().expect("buffer exists")
    }
}

impl Buffer<Neovim> for NeovimBuffer {
    type Id = Self;

    fn id(&self) -> Self::Id {
        self.clone()
    }

    fn name(&self) -> Cow<'_, str> {
        self.get_name().to_string_lossy().into_owned().into()
    }
}
