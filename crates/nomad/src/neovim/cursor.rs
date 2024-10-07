use core::cmp::Ordering;

use super::{BufferId, Neovim};
use crate::{ActorId, Context, Emitter, Event, Shared};

/// TODO: docs.
#[derive(Clone)]
pub struct Cursor {}

/// TODO: docs.
pub struct CursorEvent {
    pub(super) id: BufferId,
    pub(super) next_cursor_moved_by: Shared<Option<ActorId>>,
}

impl PartialEq for CursorEvent {
    fn eq(&self, other: &Self) -> bool {
        self.id.cmp(&other.id) == Ordering::Equal
    }
}

impl Eq for CursorEvent {}

impl PartialOrd for CursorEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CursorEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Event<Neovim> for CursorEvent {
    type Payload = Cursor;
    type SubscribeCtx = ();

    fn subscribe(&mut self, _: Emitter<Self::Payload>, _: &Context<Neovim>) {}
}
