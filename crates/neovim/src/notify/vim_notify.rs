use core::any;

use executor::LocalSpawner;
use flume::TrySendError;
use futures_util::{FutureExt, StreamExt, select_biased};
use nvim_oxi::mlua;

use crate::executor::NeovimLocalSpawner;
use crate::notify;
use crate::notify::nvim_notify::{SPINNER_FRAMES, SPINNER_UPDATE_INTERVAL};
use crate::notify::progress_reporter::{ProgressNotification, ProgressStatus};

/// Uses vim.notify (which can be overridden by plugins like Snacks.nvim)
pub struct VimNotify;

/// Progress reporter using vim.notify
pub struct VimNotifyProgressReporter {
    notif_tx: flume::Sender<ProgressNotification>,
}

impl VimNotify {
    pub(crate) fn notify(
        namespace: &editor::notify::Namespace,
        message_chunks: notify::Chunks,
        level: notify::Level,
        _namespace_id: u32,
    ) {
        let lua = mlua::lua();

        let vim_notify = lua
            .globals()
            .get::<mlua::Table>("vim")
            .expect("'vim' table not found")
            .get::<mlua::Function>("notify")
            .expect("'vim.notify' function not found");

        let title = namespace.dot_separated().to_string();
        let message = message_chunks.concat_text();
        let full_message = format!("[{}] {}", title, message);

        vim_notify
            .call::<mlua::Value>((full_message, level as u8))
            .expect("couldn't call 'vim.notify'");
    }
}

impl VimNotifyProgressReporter {
    /// Creates a new progress reporter.
    pub fn new(
        namespace: editor::notify::Namespace,
        spawner: &mut NeovimLocalSpawner,
    ) -> Self {
        let (notif_tx, notif_rx) = flume::unbounded();

        spawner
            .spawn(async move {
                Self::event_loop(notif_rx, &namespace, &mlua::lua()).await;
            })
            .detach();

        Self { notif_tx }
    }

    /// TODO: docs.
    pub fn report_error(self, chunks: notify::Chunks) {
        self.send_notification(ProgressNotification {
            chunks,
            status: ProgressStatus::Error,
        });
    }

    /// TODO: docs.
    pub fn report_progress(
        &self,
        chunks: notify::Chunks,
        perc: Option<notify::Percentage>,
    ) {
        self.send_notification(ProgressNotification {
            chunks,
            status: ProgressStatus::Progress(perc),
        });
    }

    /// TODO: docs.
    pub fn report_success(self, chunks: notify::Chunks) {
        self.send_notification(ProgressNotification {
            chunks,
            status: ProgressStatus::Success,
        });
    }

    pub(super) fn send_notification(&self, notif: ProgressNotification) {
        if let Err(err) = self.notif_tx.try_send(notif) {
            match err {
                TrySendError::Disconnected(_) => tracing::error!(
                    "{}'s event loop panicked",
                    any::type_name::<Self>()
                ),
                TrySendError::Full(_) => unreachable!("channel is unbounded"),
            }
        }
    }

    async fn event_loop(
        notif_rx: flume::Receiver<ProgressNotification>,
        namespace: &editor::notify::Namespace,
        mlua: &mlua::Lua,
    ) {
        let Ok(first_notif) = notif_rx.recv_async().await else { return };

        let mut spin = async_io::Timer::interval(SPINNER_UPDATE_INTERVAL);
        let mut spinner_frame_idx = 0;

        let vim_notify = mlua
            .globals()
            .get::<mlua::Table>("vim")
            .expect("'vim' table not found")
            .get::<mlua::Function>("notify")
            .expect("'vim.notify' function not found");

        let title = namespace.dot_separated().to_string();
        let mut notif = first_notif;

        loop {
            let icon_char = match notif.status {
                ProgressStatus::Progress(Some(perc)) => format!("{}%", perc),
                ProgressStatus::Progress(None) => {
                    SPINNER_FRAMES[spinner_frame_idx].to_string()
                },
                ProgressStatus::Success => "✔".to_string(),
                ProgressStatus::Error => "✘".to_string(),
            };

            let message = format!(
                "[{}] {} {}",
                title,
                icon_char,
                notif.chunks.concat_text()
            );

            let level = notify::Level::from(notif.status) as u8;

            let opts =
                mlua.create_table().expect("couldn't create options table");
            opts.set("id", title.replace('.', "_"))
                .expect("couldn't set notification id");

            vim_notify
                .call::<mlua::Value>((message, level, &opts))
                .expect("couldn't call 'vim.notify'");

            if !matches!(notif.status, ProgressStatus::Progress(_)) {
                break;
            }

            'spin: loop {
                select_biased! {
                    _ = spin.next().fuse() => {
                        spinner_frame_idx += 1;
                        spinner_frame_idx %= SPINNER_FRAMES.len();
                        let icon_char = SPINNER_FRAMES[spinner_frame_idx];
                        let message = format!(
                            "[{}] {} {}",
                            title,
                            icon_char,
                            notif.chunks.concat_text()
                        );
                        vim_notify
                            .call::<mlua::Value>((message, level, &opts))
                            .expect("couldn't call 'vim.notify'");
                        continue 'spin;
                    },

                    maybe_notif = notif_rx.recv_async() => {
                        let Ok(new_notif) = maybe_notif else { return; };

                        match (notif.status, new_notif.status) {
                            // Stop spinning if we've started showing percentages.
                            (
                                ProgressStatus::Progress(None),
                                ProgressStatus::Progress(Some(_)),
                            ) => spin.clear(),

                            // Start spinning if we've stopped showing percentages.
                            (
                                ProgressStatus::Progress(Some(_)),
                                ProgressStatus::Progress(None),
                            ) => spin.set_interval(SPINNER_UPDATE_INTERVAL),

                            _ => {},
                        }

                        notif = new_notif;
                        break 'spin;
                    },
                }
            }
        }
    }
}
