use super::{Level, Message, Namespace, NotificationId};

/// TODO: docs.
pub struct Notification<'ns> {
    /// TODO: docs.
    pub level: Level,

    /// TODO: docs.
    pub namespace: &'ns Namespace,

    /// TODO: docs.
    pub message: Message,

    /// TODO: docs.
    pub updates_prev: Option<NotificationId>,
}
