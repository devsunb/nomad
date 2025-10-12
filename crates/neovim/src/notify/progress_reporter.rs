use core::fmt;
use core::time::Duration;

use editor::Context;
use editor::context::BorrowState;
use flume::TrySendError;
use futures_util::{FutureExt, StreamExt, select_biased};

use crate::Neovim;
use crate::notify::{self, NotifyContextExt};

/// Frames for the spinner animation.
const SPINNER_FRAMES: &[&str] = &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

/// How many revolutions per minute the spinner should complete.
const SPINNER_RPM: u8 = 75;

/// How often the spinner should be updated to achieve the desired RPM.
const SPINNER_UPDATE_INTERVAL: Duration = Duration::from_millis({
    (60_000.0 / ((SPINNER_RPM as u16 * SPINNER_FRAMES.len() as u16) as f32))
        .round() as u64
});

/// TODO: docs.
pub struct ProgressReporter {
    updates_tx: flume::Sender<Update>,
}

struct Update {
    level: notify::Level,
    chunks: notify::Chunks,
    is_last: bool,
}

impl ProgressReporter {
    /// Creates a new progress reporter.
    pub fn new(ctx: &mut Context<Neovim, impl BorrowState>) -> Self {
        let (updates_tx, updates_rx) = flume::bounded::<Update>(4);

        ctx.spawn_and_detach(async move |ctx| {
            let namespace = ctx.namespace().clone();
            let mut spin = async_io::Timer::interval(SPINNER_UPDATE_INTERVAL);
            let mut updates = updates_rx.into_stream();

            let Some(mut update) = updates.next().await else { return };
            let mut spinner_frame_idx = 0;
            // let mut prev_id = None;

            loop {
                let prefix: &dyn fmt::Display = if !update.is_last {
                    &format_args!("{} ", SPINNER_FRAMES[spinner_frame_idx])
                } else if update.level == notify::Level::Error {
                    &""
                } else {
                    &"✔ "
                };

                // prev_id = ctx.with_editor(|nvim| {
                //     Some(nvim.emitter().emit(editor::notify::Notification {
                //         level: update.level,
                //         message: editor::notify::Message::from_display(
                //             format_args!("{prefix}{}", update.text),
                //         ),
                //         namespace: &namespace,
                //         updates_prev: prev_id,
                //     }))
                // });

                if update.is_last {
                    break;
                }

                select_biased! {
                    _ = spin.next().fuse() => {
                        spinner_frame_idx += 1;
                        spinner_frame_idx %= SPINNER_FRAMES.len();
                    },
                    next_update = updates.next() => {
                        match next_update {
                            Some(next_update) => update = next_update,
                            // The pipeline has been cancelled.
                            None => break,
                        }
                    },
                }
            }
        });

        Self { updates_tx }
    }

    /// TODO: docs.
    pub fn report_error(self, chunks: notify::Chunks) {
        self.send_update(Update {
            level: notify::Level::Error,
            chunks,
            is_last: true,
        });
    }

    /// TODO: docs.
    pub fn report_progress(&self, chunks: notify::Chunks) {
        self.send_update(Update {
            level: notify::Level::Info,
            chunks,
            is_last: false,
        });
    }

    /// TODO: docs.
    pub fn report_success(self, chunks: notify::Chunks) {
        self.send_update(Update {
            level: notify::Level::Info,
            chunks,
            is_last: true,
        });
    }

    fn send_update(&self, update: Update) {
        if let Err(err) = self.updates_tx.try_send(update) {
            match err {
                TrySendError::Disconnected(_) => unreachable!(),
                TrySendError::Full(_) => {},
            }
        }
    }
}
