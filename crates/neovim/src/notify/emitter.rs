use editor::notify::{Emitter, Notification, NotificationId};

use crate::convert::Convert;
use crate::notify::{NvimNotify, VimNotify};

/// TODO: docs.
pub struct NeovimEmitter {
    namespace_id: u32,
}

impl NeovimEmitter {
    pub(crate) fn new(namespace_id: u32) -> Self {
        Self { namespace_id }
    }
}

impl Emitter for NeovimEmitter {
    fn emit(&mut self, notification: Notification) -> NotificationId {
        let namespace = notification.namespace;
        let chunks = notification.message.into();
        let level = notification.level.convert();
        if NvimNotify::is_installed() {
            NvimNotify::notify(namespace, chunks, level, self.namespace_id);
        } else {
            // Use vim.notify (which can be overridden by Snacks.nvim)
            VimNotify::notify(namespace, chunks, level, self.namespace_id);
        }
        NotificationId::new(0)
    }
}
