use core::marker::PhantomData;

use crate::NeovimVersion;

/// TODO: docs.
pub struct Neovim<V: NeovimVersion> {
    version: PhantomData<V>,
}
