//! TODO: docs

mod panic_hook;
mod tracing_subscriber;

pub use tracing::{debug, error, info, trace, warn};

/// Intializes the logging system.
#[inline]
pub(crate) fn init() {
    panic_hook::init();
    tracing_subscriber::init();
}
