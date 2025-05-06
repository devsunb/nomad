#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

#[cfg(all(test, feature = "collab"))]
mod collab;
#[cfg(all(test, feature = "walkdir"))]
mod walkdir;
