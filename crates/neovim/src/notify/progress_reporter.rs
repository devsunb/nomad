use editor::Context;
use editor::context::BorrowState;

use crate::Neovim;
use crate::notify::{self, NvimNotify, NvimNotifyProgressReporter};

/// TODO: docs.
pub enum ProgressReporter {
    /// TODO: docs.
    NvimNotify(NvimNotifyProgressReporter),
}

impl ProgressReporter {
    /// Creates a new progress reporter.
    pub fn new(ctx: &mut Context<Neovim, impl BorrowState>) -> Self {
        if NvimNotify::is_installed() {
            Self::NvimNotify(NvimNotifyProgressReporter::new(ctx))
        } else {
            todo!()
        }
    }

    /// TODO: docs.
    pub fn report_error(self, chunks: notify::Chunks) {
        match self {
            Self::NvimNotify(inner) => inner.report_error(chunks),
        }
    }

    /// TODO: docs.
    pub fn report_progress(&self, chunks: notify::Chunks) {
        match self {
            Self::NvimNotify(inner) => inner.report_progress(chunks),
        }
    }

    /// TODO: docs.
    pub fn report_success(self, chunks: notify::Chunks) {
        match self {
            Self::NvimNotify(inner) => inner.report_success(chunks),
        }
    }
}
