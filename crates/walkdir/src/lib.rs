//! TODO: docs.

mod accumulate;
mod filter;
mod walkdir;

pub use accumulate::{AccumulateError, Accumulator};
pub use filter::{Filter, Filtered};
pub use walkdir::WalkDir;
