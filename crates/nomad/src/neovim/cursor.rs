use core::cmp::Ordering;

use nvim_oxi::api;

use super::point::Point;
use super::{BufferId, Neovim};
use crate::{ActorId, Context, Emitter, Event, Shared};

/// TODO: docs.
#[derive(Clone)]
pub struct Cursor {
    action: CursorAction,
    moved_by: ActorId,
}

/// TODO: docs.
#[derive(Clone, Copy)]
pub enum CursorAction {
    /// The cursor has been moved away from the buffer.
    Removed,

    /// The cursor has been moved into the buffer at the given point.
    Created(Point),

    /// The cursor has been moved to the given point.
    Moved(Point),
}

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
    type SubscribeCtx = u32;

    fn subscribe(
        &mut self,
        emitter: Emitter<Self::Payload>,
        _: &Context<Neovim>,
    ) -> Self::SubscribeCtx {
        let opts = api::opts::CreateAutocmdOpts::builder()
            .buffer(self.id.as_nvim().clone())
            .callback({
                let next_cursor_moved_by = self.next_cursor_moved_by.clone();
                move |_| {
                    let action = CursorAction::Moved(Point::zero());

                    let moved_by = next_cursor_moved_by
                        .with_mut(Option::take)
                        .unwrap_or(ActorId::unknown());

                    emitter.send(Cursor { action, moved_by });
                    false
                }
            })
            .build();

        api::create_autocmd(["CursorMoved", "CursorMovedI"], &opts)
            .expect("all arguments are valid")
    }

    fn unsubscribe(
        &mut self,
        autocmd_id: Self::SubscribeCtx,
        _: &Context<Neovim>,
    ) {
        // Will fail if someone else has already deleted the autocmd.
        let _ = api::del_autocmd(autocmd_id);
    }
}
