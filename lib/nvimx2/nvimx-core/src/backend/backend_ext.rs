use crate::backend::Backend;
use crate::notify::{self, Emitter};
use crate::plugin::Plugin;

/// TODO: docs.
pub(crate) trait BackendExt: Backend {
    #[inline]
    fn emit_err<P, Err>(&mut self, source: notify::Source, err: Err)
    where
        P: Plugin<Self>,
        Err: notify::Error<Self>,
    {
        let Some((level, message)) = err.to_message::<P>(source) else {
            return;
        };

        let notification = notify::Notification {
            level,
            source,
            message,
            updates_prev: None,
        };

        self.emitter().emit(notification);
    }
}

impl<B: Backend> BackendExt for B {}
