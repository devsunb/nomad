//! TODO: docs

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "arboard")]
mod arboard_impl;
mod clipboard;
#[cfg(feature = "fallible-init")]
mod fallible_init;

#[cfg(feature = "arboard")]
pub use arboard;
pub use clipboard::Clipboard;
#[cfg(feature = "fallible-init")]
pub use fallible_init::{FallibleInit, FallibleInitClipboard};
