use std::path::PathBuf;

use common::nvim::{self, api::opts::*};

use crate::*;

pub type ViewId = nvim::api::Window;

/// TODO: docs.
pub(crate) struct View {
    /// TODO: docs.
    buffer: nvim::api::Buffer,

    /// TODO: docs.
    id: ViewId,
}

impl View {
    /// TODO: docs.
    pub fn close(self) {
        self.id.close(true).unwrap();

        self.buffer
            .delete(&BufDeleteOpts::builder().force(true).build())
            .unwrap();
    }

    /// TODO: docs.
    pub fn id(&self) -> ViewId {
        self.id.clone()
    }

    /// TODO: docs.
    pub fn new(at_path: PathBuf, with_config: &WindowConfig) -> Self {
        let mut buffer = nvim::api::create_buf(false, true).unwrap();

        buffer
            .set_lines(
                0..0,
                true,
                std::iter::once(at_path.display().to_string().as_str()),
            )
            .unwrap();

        let window = nvim::api::open_win(&buffer, true, &(with_config.into()))
            .expect("the config is valid");

        Self { buffer, id: window }
    }
}
