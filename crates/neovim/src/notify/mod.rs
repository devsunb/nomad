//! TODO: docs.

mod chunk;
mod chunks;
mod context_ext;
mod notify;
mod progress_reporter;

pub use chunk::Chunk;
pub use chunks::Chunks;
pub use context_ext::NotifyContextExt;
pub use notify::{
    NeovimEmitter,
    NvimNotify,
    VimNotify,
    VimNotifyProvider,
    detect,
};
pub use nvim_oxi::api::types::LogLevel as Level;
pub use progress_reporter::ProgressReporter;
