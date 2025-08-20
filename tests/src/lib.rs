//! This crate contains the integration tests for all the crates in the
//! workspace.

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]
#![cfg_attr(not(test), allow(dead_code, unused_imports))]

mod collab;
mod editor;
mod fs;
mod gitignore;
mod mock;
#[cfg(feature = "neovim")]
mod neovim;
mod thread_pool;
mod utils;
