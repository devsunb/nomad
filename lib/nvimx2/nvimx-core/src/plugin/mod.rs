//! TODO: docs.

mod panic_infos;
mod plugin;

pub use panic_infos::{PanicInfo, PanicLocation};
pub(crate) use plugin::NO_COMMAND_NAME;
pub use plugin::Plugin;
