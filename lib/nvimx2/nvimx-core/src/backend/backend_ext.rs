use crate::backend::Backend;
use crate::notify::{self, Emitter};

/// TODO: docs.
pub(crate) trait BackendExt: Backend {
    #[inline]
    fn emit_err<Err>(&mut self, source: notify::Source, err: Err)
    where
        Err: notify::Error,
    {
        let Some((level, message)) = err.to_message(source) else {
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
