//! TODO: docs.

mod executor;
mod join_handle;
mod neovim;
mod spawner;

pub use join_handle::NeovimJoinHandle;
pub use neovim::Neovim;
pub use spawner::NeovimSpawner;
