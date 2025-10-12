use editor::context::{BorrowState, Context};
use nvim_oxi::api::types::LogLevel;

use crate::Neovim;
use crate::notify::{Chunks, ProgressReporter};

/// An extension trait for `Context<Neovim>` providing methods to emit
/// notifications via the `vim.notify()` API.
pub trait NotifyContextExt {
    /// Emits a notification with the given message and level.
    fn new_progress_reporter(&mut self) -> ProgressReporter;

    /// Emits a notification with the given message and level.
    fn notify(
        &mut self,
        notification_message: impl Into<Chunks>,
        notification_level: LogLevel,
    );

    /// Emits a notification at the `ERROR` level with the given message.
    fn notify_error(&mut self, notification_message: impl Into<Chunks>) {
        self.notify(notification_message, LogLevel::Error);
    }

    /// Emits a notification at the `INFO` level with the given message.
    fn notify_info(&mut self, notification_message: impl Into<Chunks>) {
        self.notify(notification_message, LogLevel::Info);
    }
}

impl<Bs: BorrowState> NotifyContextExt for Context<Neovim, Bs> {
    #[inline]
    fn new_progress_reporter(&mut self) -> ProgressReporter {
        ProgressReporter::new(self)
    }

    #[inline]
    fn notify(
        &mut self,
        _notification_message: impl Into<Chunks>,
        _notification_level: LogLevel,
    ) {
        todo!();
        // let namespace = self.namespace().clone();
        //
        // self.with_editor(|nvim| {
        //     nvim.emitter().emit(Notification {
        //         level: notification_level.convert(),
        //         message: Message::from_display(notification_message),
        //         namespace: &namespace,
        //         updates_prev: None,
        //     })
        // });
    }
}
