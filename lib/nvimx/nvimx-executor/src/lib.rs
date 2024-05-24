//! TODO: docs

#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

mod executor;
mod task;

pub use executor::Executor;
pub use task::Task;
