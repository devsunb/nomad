use core::time::Duration;

use crate::clear::Clear;
use crate::{Emit, EmitMessage, Severity};

/// TODO: docs.
pub struct ClearAfter<T> {
    inner: T,
    duration: Duration,
}

impl<T> ClearAfter<T> {
    pub(crate) fn new(inner: T, duration: Duration) -> Self {
        Self { inner, duration }
    }
}

impl<T: Emit> Emit for ClearAfter<T> {
    const ADD_TO_MESSAGE_HISTORY: bool = T::ADD_TO_MESSAGE_HISTORY;
    type Action = T::Action;

    fn message(&self) -> EmitMessage {
        self.inner.message()
    }
    fn severity(&self) -> Severity {
        self.inner.severity()
    }
    fn emit(self) {
        let Self { inner, duration } = self;
        inner.emit();
        nvimx_executor::Executor::register()
            .spawn(async move {
                nvimx_executor::sleep(duration).await;
                // This will clear the message area even if other messages have
                // been emitted in the meantime (either by us or by other
                // plugins).
                //
                // I don't think Neovim keeps a message counter, so I'm not
                // sure how to avoid that.
                Clear.emit();
            })
            .detach();
    }
}
