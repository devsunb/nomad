use editor::{AccessMut, AgentId, Editor, Shared};

use crate::Neovim;
use crate::buffer::{BufferId, NeovimBuffer};
use crate::events::{AutocmdId, Callbacks, Event, EventKind, Events};
use crate::oxi::api;
use crate::utils::CallbackExt;

#[derive(Debug, Clone, Copy)]
pub(crate) struct CursorCreated;

impl Event for CursorCreated {
    type Args<'a> = (NeovimBuffer<'a>, AgentId);
    type Container<'ev> = &'ev mut Option<Callbacks<Self>>;
    type RegisterOutput = (AutocmdId, AutocmdId);

    #[inline]
    fn container<'ev>(&self, events: &'ev mut Events) -> Self::Container<'ev> {
        &mut events.on_cursor_created
    }

    #[inline]
    fn kind(&self) -> EventKind {
        EventKind::CursorCreated(*self)
    }

    #[inline]
    fn key(&self) {}

    #[inline]
    fn register(
        &self,
        events: &Events,
        mut nvim: impl AccessMut<Neovim> + 'static,
    ) -> Self::RegisterOutput {
        let has_left_buffer = Shared::<bool>::new(false);

        let on_buf_leave = {
            let has_left_buffer = has_left_buffer.clone();
            move |_: api::types::AutocmdCallbackArgs| {
                has_left_buffer.set(true);
                false
            }
        }
        .into_function();

        let buf_leave_autocmd_id = api::create_autocmd(
            ["BufLeave"],
            &api::opts::CreateAutocmdOpts::builder()
                .group(events.augroup_id)
                .callback(on_buf_leave)
                .build(),
        )
        .expect("couldn't create autocmd on BufLeave");

        let on_buf_enter = (move |args: api::types::AutocmdCallbackArgs| {
            // Some commands like ":edit" or ":split" can cause BufEnter to be
            // fired multiple times for the same buffer without any
            // intermediate BufLeave.
            //
            // When that happens we should ignore the BufEnter because the
            // buffer hasn't actually changed.
            if !has_left_buffer.take() {
                return false;
            }

            nvim.with_mut(|nvim| {
                let buffer_id = BufferId::from(args.buffer);

                let Some(mut buffer) = nvim.buffer(buffer_id) else {
                    return false;
                };

                let events = &mut buffer.nvim.events;

                let Some(callbacks) = &events.on_cursor_created else {
                    return true;
                };

                let created_by = events
                    .agent_ids
                    .created_cursor
                    .remove(&buffer_id)
                    .unwrap_or(AgentId::UNKNOWN);

                for callback in callbacks.cloned() {
                    callback((buffer.reborrow(), created_by));
                }

                false
            })
        })
        .catch_unwind()
        .map(|maybe_detach| maybe_detach.unwrap_or(true))
        .into_function();

        let buf_enter_autocmd_id = api::create_autocmd(
            ["BufEnter"],
            &api::opts::CreateAutocmdOpts::builder()
                .group(events.augroup_id)
                .callback(on_buf_enter)
                .build(),
        )
        .expect("couldn't create autocmd on BufEnter");

        (buf_leave_autocmd_id, buf_enter_autocmd_id)
    }

    #[inline]
    fn unregister(
        (buf_leave_autocmd_id, buf_enter_autocmd_id): Self::RegisterOutput,
    ) {
        let _ = api::del_autocmd(buf_leave_autocmd_id);
        let _ = api::del_autocmd(buf_enter_autocmd_id);
    }
}
