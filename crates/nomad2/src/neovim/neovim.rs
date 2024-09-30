use collab_fs::OsFs;

use super::{ModuleApi, NeovimSpawner};
use crate::{Editor, Module};

/// TODO: docs.
#[derive(Default)]
pub struct Neovim {}

impl Editor for Neovim {
    type Fs = OsFs;
    type ModuleApi<M: Module<Self>> = ModuleApi<M>;
    type Spawner = NeovimSpawner;

    #[inline]
    fn fs(&self) -> Self::Fs {
        OsFs::new()
    }

    #[inline]
    fn spawner(&self) -> Self::Spawner {
        NeovimSpawner
    }
}
