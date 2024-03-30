use futures::Stream;

use crate::editor::BufferId;

/// A [`Stream`] that yields the [`BufferId`] of the currently focused buffer
/// every time it changes.
pub struct FocusedBuffer {}
