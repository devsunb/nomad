//! Contains the editor-specific implementations of [`VersionEditor`].

#[cfg(feature = "neovim")]
pub mod neovim;

use editor::context::Borrowed;
use editor::{Context, Editor};

use crate::Version;

/// An [`Editor`] subtrait defining additional capabilities needed by the
/// actions in this crate.
pub trait VersionEditor: Editor {
    /// Emits a notification about the given version through the editor's
    /// notification system.
    fn emit_version(version: Version, ctx: &mut Context<Self, Borrowed>);
}
