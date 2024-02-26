use std::path::PathBuf;

use serde::Deserialize;
use url::Url;

/// TODO: docs
#[derive(Debug, Deserialize)]
pub struct CollabConfig {
    /// TODO: docs
    project_dir: PathBuf,

    /// TODO: docs
    server_address: Url,

    /// TODO: docs
    server_port: u16,
}

impl Default for CollabConfig {
    #[inline]
    fn default() -> Self {
        Self {
            // TODO: this should be a path relative to the `/nomad` path.
            project_dir: PathBuf::new(),
            server_address: Url::parse("collab.nomad.foo").unwrap(),
            server_port: 64420,
        }
    }
}
