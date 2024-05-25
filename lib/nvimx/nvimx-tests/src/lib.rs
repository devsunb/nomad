//! TODO: docs

#![cfg_attr(docsrs, feature(doc_cfg))]

mod build;
#[doc(hidden)]
pub mod test_macro;

pub use build::build;

type TestError = Box<dyn std::error::Error>;
type TestResult = Result<(), TestError>;
