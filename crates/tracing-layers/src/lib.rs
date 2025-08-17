//! This crate contains the implementations of various tracing
//! [`Layer`](tracing_subscriber::Layer)s.

#[cfg(feature = "file-appender")]
mod file_appender;

#[cfg(feature = "file-appender")]
pub use file_appender::FileAppender;
