//! TODO: docs.

#![allow(missing_docs)]
#![feature(async_fn_traits)]
#![feature(unboxed_closures)]

pub mod api;
mod backend_ext;
pub mod buffer;
mod context_ext;
pub mod emitter;
pub mod executor;
pub mod fs;
mod mock;
pub mod serde;
pub mod value;

pub use backend_ext::BackendExt;
pub use context_ext::ContextExt;
pub use mock::Mock;
pub use mock_macros::fs;
