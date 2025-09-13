//! This crate provides the [`HttpClient`] trait which abstracts over different
//! HTTP clients, together with a few feature-gated implementations for
//! popular HTTP client libraries.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod http_client;
#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "ureq")]
mod ureq;

pub use http_client::HttpClient;
#[cfg(feature = "mock")]
pub use mock::MockHttpClient;
#[cfg(feature = "ureq")]
pub use ureq::UreqClient;
