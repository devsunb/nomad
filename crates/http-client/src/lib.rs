//! This crate provides the [`HttpClient`] trait which abstracts over different
//! HTTP clients, together with a few feature-gated implementations for
//! popular HTTP client libraries.

mod http_client;
#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "ureq")]
mod ureq;

pub use http_client::HttpClient;
