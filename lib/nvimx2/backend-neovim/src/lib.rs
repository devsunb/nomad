//! TODO: docs.

mod neovim;
mod version;

pub use neovim::Neovim;
pub use version::NeovimVersion;
#[cfg(feature = "neovim-nightly")]
pub use version::Nightly;
#[cfg(feature = "neovim-0-10")]
pub use version::ZeroDotTen;
