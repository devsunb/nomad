use core::ops::{Deref, DerefMut};
use core::{any, mem};

use ed::Shared;
use ed::backend::{AgentId, Buffer, Cursor, Edit};
use nohash::IntMap as NoHashMap;
use slotmap::{DefaultKey, SlotMap};
use smallvec::smallvec_inline;

use crate::buffer::{BufferId, NeovimBuffer};
use crate::cursor::NeovimCursor;
use crate::oxi::api::{self, opts, types};

type AugroupId = u32;
type AutocmdId = u32;

/// TODO: docs.
pub struct EventHandle {
    event_key: DefaultKey,
    event_kind: EventKind,
    events: Shared<Events>,
}

pub(crate) trait Event: Clone + Into<EventKind> {
    /// The type of arguments given to the callbacks registered for this
    /// event.
    type Args<'a>;

    /// The output of [`register()`](Event::register)ing the event.
    type RegisterOutput;

    /// TODO: docs.
    fn get_or_insert_callbacks<'ev>(
        &self,
        events: &'ev mut Events,
    ) -> &'ev mut EventCallbacks<Self>;

    /// TODO: docs.
    fn register(&self, events: EventsBorrow) -> Self::RegisterOutput;

    /// TODO: docs.
    fn unregister(out: Self::RegisterOutput);

    /// TODO: docs.
    fn cleanup(&self, event_key: DefaultKey, events: &mut Events);
}

pub(crate) struct EventsBorrow<'a> {
    pub(crate) borrow: &'a mut Events,
    pub(crate) handle: Shared<Events>,
}

pub(crate) struct Events {
    pub(crate) agent_ids: AgentIds,
    augroup_id: AugroupId,
    on_buffer_created: EventCallbacks<BufReadPost>,
    on_buffer_edited: NoHashMap<BufferId, EventCallbacks<OnBytes>>,
    on_buffer_focused: EventCallbacks<BufEnter>,
    on_buffer_removed: NoHashMap<BufferId, EventCallbacks<BufUnload>>,
    on_buffer_saved: NoHashMap<BufferId, EventCallbacks<BufWritePost>>,
    on_buffer_unfocused: NoHashMap<BufferId, EventCallbacks<BufLeave>>,
    on_cursor_moved: NoHashMap<BufferId, EventCallbacks<CursorMoved>>,
}

#[derive(Default)]
pub(crate) struct AgentIds {
    pub(crate) created_buffer: NoHashMap<BufferId, AgentId>,
    pub(crate) edited_buffer: NoHashMap<BufferId, AgentId>,
    pub(crate) removed_buffer: NoHashMap<BufferId, AgentId>,
    pub(crate) saved_buffer: NoHashMap<BufferId, AgentId>,
}

#[derive(Default)]
#[doc(hidden)]
pub(crate) enum EventCallbacks<T: Event> {
    Registered {
        #[allow(clippy::type_complexity)]
        callbacks: SlotMap<DefaultKey, Box<dyn FnMut(T::Args<'_>) + 'static>>,
        output: T::RegisterOutput,
    },
    #[default]
    Unregistered,
}

#[derive(Clone, Copy)]
pub(crate) struct BufEnter;

#[derive(Clone, Copy)]
pub(crate) struct BufLeave(pub(crate) BufferId);

#[derive(Clone, Copy)]
pub(crate) struct BufReadPost;

#[derive(Clone, Copy)]
pub(crate) struct BufUnload(pub(crate) BufferId);

#[derive(Clone, Copy)]
pub(crate) struct BufWritePost(pub(crate) BufferId);

#[derive(Clone, Copy)]
pub(crate) struct CursorMoved(pub(crate) BufferId);

#[derive(Clone, Copy)]
pub(crate) struct OnBytes(pub(crate) BufferId);

#[derive(cauchy::From)]
pub(crate) enum EventKind {
    BufEnter(#[from] BufEnter),
    BufLeave(#[from] BufLeave),
    BufReadPost(#[from] BufReadPost),
    BufUnload(#[from] BufUnload),
    BufWritePost(#[from] BufWritePost),
    CursorMoved(#[from] CursorMoved),
    OnBytes(#[from] OnBytes),
}

impl Events {
    pub(crate) fn new(augroup_name: &str) -> Self {
        let augroup_id = api::create_augroup(
            augroup_name,
            &opts::CreateAugroupOpts::builder().clear(true).build(),
        )
        .expect("couldn't create augroup");

        Self {
            augroup_id,
            agent_ids: Default::default(),
            on_buffer_created: Default::default(),
            on_buffer_edited: Default::default(),
            on_buffer_focused: Default::default(),
            on_buffer_removed: Default::default(),
            on_buffer_saved: Default::default(),
            on_buffer_unfocused: Default::default(),
            on_cursor_moved: Default::default(),
        }
    }

    pub(crate) fn insert<T: Event>(
        events: Shared<Self>,
        event: T,
        fun: impl FnMut(T::Args<'_>) + 'static,
    ) -> EventHandle {
        let event_kind = event.clone().into();

        let event_key = events.with_mut(|this| {
            if let EventCallbacks::Registered { callbacks, .. } =
                event.get_or_insert_callbacks(this)
            {
                return callbacks.insert(Box::new(fun));
            }

            let output = event.register(EventsBorrow {
                borrow: this,
                handle: events.clone(),
            });

            let mut callbacks = SlotMap::new();

            let event_key = callbacks.insert(Box::new(fun) as Box<_>);

            *event.get_or_insert_callbacks(this) =
                EventCallbacks::Registered { callbacks, output };

            event_key
        });

        EventHandle { event_key, event_kind, events }
    }
}

impl<T: Event> EventCallbacks<T> {
    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            Self::Unregistered => true,
            Self::Registered { callbacks, .. } => callbacks.is_empty(),
        }
    }

    #[track_caller]
    #[inline]
    fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut impl FnMut(T::Args<'_>)> + '_ {
        match self {
            Self::Unregistered => panic!(
                "the autocommand for {} has not been registered",
                any::type_name::<T>()
            ),
            Self::Registered { callbacks, .. } => callbacks.values_mut(),
        }
    }

    #[inline]
    fn remove(&mut self, callback_key: DefaultKey) {
        if let Self::Registered { callbacks, .. } = self {
            callbacks.remove(callback_key);

            // If all the EventHandles have been dropped that means no one
            // cares about the event anymore, and we can unregister it.
            if callbacks.is_empty() {
                match mem::replace(self, Self::Unregistered) {
                    Self::Registered { output, .. } => T::unregister(output),
                    Self::Unregistered => unreachable!("just checked"),
                }
            }
        }
    }
}

impl Deref for EventsBorrow<'_> {
    type Target = Events;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.borrow
    }
}

impl DerefMut for EventsBorrow<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow
    }
}

impl Event for BufEnter {
    type Args<'a> = (&'a NeovimBuffer<'a>, AgentId);
    type RegisterOutput = AutocmdId;

    #[inline]
    fn get_or_insert_callbacks<'ev>(
        &self,
        events: &'ev mut Events,
    ) -> &'ev mut EventCallbacks<Self> {
        &mut events.on_buffer_focused
    }

    #[inline]
    fn register(&self, events: EventsBorrow) -> AutocmdId {
        let opts = opts::CreateAutocmdOpts::builder()
            .group(events.augroup_id)
            .callback({
                let events = events.handle;
                move |args: types::AutocmdCallbackArgs| {
                    events.with_mut(|inner| {
                        let buffer = NeovimBuffer::new(
                            BufferId::new(args.buffer),
                            &events,
                        );

                        for callback in inner.on_buffer_focused.iter_mut() {
                            callback((&buffer, AgentId::UNKNOWN));
                        }

                        false
                    })
                }
            })
            .build();

        api::create_autocmd(["BufEnter"], &opts)
            .expect("couldn't create autocmd")
    }

    #[inline]
    fn unregister(autocmd_id: Self::RegisterOutput) {
        let _ = api::del_autocmd(autocmd_id);
    }

    #[inline]
    fn cleanup(&self, event_key: DefaultKey, events: &mut Events) {
        events.on_buffer_focused.remove(event_key);
    }
}

impl Event for BufLeave {
    type Args<'a> = (&'a NeovimBuffer<'a>, AgentId);
    type RegisterOutput = AutocmdId;

    #[inline]
    fn get_or_insert_callbacks<'ev>(
        &self,
        events: &'ev mut Events,
    ) -> &'ev mut EventCallbacks<Self> {
        events.on_buffer_unfocused.entry(self.0).or_default()
    }

    #[inline]
    fn register(&self, events: EventsBorrow) -> AutocmdId {
        let opts = opts::CreateAutocmdOpts::builder()
            .group(events.augroup_id)
            .buffer(self.0.into())
            .callback({
                let events = events.handle;
                move |args: types::AutocmdCallbackArgs| {
                    events.with_mut(|inner| {
                        let buffer = NeovimBuffer::new(
                            BufferId::new(args.buffer),
                            &events,
                        );

                        let Some(callbacks) =
                            inner.on_buffer_unfocused.get_mut(&buffer.id())
                        else {
                            return true;
                        };

                        for callback in callbacks.iter_mut() {
                            callback((&buffer, AgentId::UNKNOWN));
                        }

                        false
                    })
                }
            })
            .build();

        api::create_autocmd(["BufLeave"], &opts)
            .expect("couldn't create autocmd")
    }

    #[inline]
    fn unregister(autocmd_id: Self::RegisterOutput) {
        let _ = api::del_autocmd(autocmd_id);
    }

    #[inline]
    fn cleanup(&self, event_key: DefaultKey, events: &mut Events) {
        if let Some(callbacks) = events.on_buffer_unfocused.get_mut(&self.0) {
            callbacks.remove(event_key);
            if callbacks.is_empty() {
                events.on_buffer_unfocused.remove(&self.0);
            }
        }
    }
}

impl Event for BufReadPost {
    type Args<'a> = (&'a NeovimBuffer<'a>, AgentId);
    type RegisterOutput = AutocmdId;

    #[inline]
    fn get_or_insert_callbacks<'ev>(
        &self,
        events: &'ev mut Events,
    ) -> &'ev mut EventCallbacks<Self> {
        &mut events.on_buffer_created
    }

    #[inline]
    fn register(&self, events: EventsBorrow) -> AutocmdId {
        let opts = opts::CreateAutocmdOpts::builder()
            .group(events.augroup_id)
            .callback({
                let events = events.handle;
                move |args: types::AutocmdCallbackArgs| {
                    events.with_mut(|inner| {
                        let buffer = NeovimBuffer::new(
                            BufferId::new(args.buffer),
                            &events,
                        );

                        let created_by = inner
                            .agent_ids
                            .created_buffer
                            .remove(&buffer.id())
                            .unwrap_or(AgentId::UNKNOWN);

                        for callback in inner.on_buffer_created.iter_mut() {
                            callback((&buffer, created_by));
                        }

                        false
                    })
                }
            })
            .build();

        api::create_autocmd(["BufReadPost"], &opts)
            .expect("couldn't create autocmd")
    }

    #[inline]
    fn unregister(autocmd_id: Self::RegisterOutput) {
        let _ = api::del_autocmd(autocmd_id);
    }

    #[inline]
    fn cleanup(&self, event_key: DefaultKey, events: &mut Events) {
        events.on_buffer_created.remove(event_key);
    }
}

impl Event for BufUnload {
    type Args<'a> = (&'a NeovimBuffer<'a>, AgentId);
    type RegisterOutput = AutocmdId;

    #[inline]
    fn get_or_insert_callbacks<'ev>(
        &self,
        events: &'ev mut Events,
    ) -> &'ev mut EventCallbacks<Self> {
        events.on_buffer_removed.entry(self.0).or_default()
    }

    #[inline]
    fn register(&self, events: EventsBorrow) -> AutocmdId {
        let opts = opts::CreateAutocmdOpts::builder()
            .group(events.augroup_id)
            .buffer(self.0.into())
            .callback({
                let events = events.handle;
                move |args: types::AutocmdCallbackArgs| {
                    events.with_mut(|inner| {
                        let buffer = NeovimBuffer::new(
                            BufferId::new(args.buffer),
                            &events,
                        );

                        let Some(callbacks) =
                            inner.on_buffer_saved.get_mut(&buffer.id())
                        else {
                            return true;
                        };

                        let removed_by = inner
                            .agent_ids
                            .removed_buffer
                            .remove(&buffer.id())
                            .unwrap_or(AgentId::UNKNOWN);

                        for callback in callbacks.iter_mut() {
                            callback((&buffer, removed_by));
                        }

                        false
                    })
                }
            })
            .build();

        api::create_autocmd(["BufWritePost"], &opts)
            .expect("couldn't create autocmd")
    }

    #[inline]
    fn unregister(autocmd_id: Self::RegisterOutput) {
        let _ = api::del_autocmd(autocmd_id);
    }

    #[inline]
    fn cleanup(&self, event_key: DefaultKey, events: &mut Events) {
        if let Some(callbacks) = events.on_buffer_removed.get_mut(&self.0) {
            callbacks.remove(event_key);
            if callbacks.is_empty() {
                events.on_buffer_removed.remove(&self.0);
            }
        }
    }
}

impl Event for BufWritePost {
    type Args<'a> = (&'a NeovimBuffer<'a>, AgentId);
    type RegisterOutput = AutocmdId;

    #[inline]
    fn get_or_insert_callbacks<'ev>(
        &self,
        events: &'ev mut Events,
    ) -> &'ev mut EventCallbacks<Self> {
        events.on_buffer_saved.entry(self.0).or_default()
    }

    #[inline]
    fn register(&self, events: EventsBorrow) -> AutocmdId {
        let opts = opts::CreateAutocmdOpts::builder()
            .group(events.augroup_id)
            .buffer(self.0.into())
            .callback({
                let events = events.handle;
                move |args: types::AutocmdCallbackArgs| {
                    events.with_mut(|inner| {
                        let buffer = NeovimBuffer::new(
                            BufferId::new(args.buffer),
                            &events,
                        );

                        let Some(callbacks) =
                            inner.on_buffer_saved.get_mut(&buffer.id())
                        else {
                            return true;
                        };

                        let saved_by = inner
                            .agent_ids
                            .saved_buffer
                            .remove(&buffer.id())
                            .unwrap_or(AgentId::UNKNOWN);

                        for callback in callbacks.iter_mut() {
                            callback((&buffer, saved_by));
                        }

                        false
                    })
                }
            })
            .build();

        api::create_autocmd(["BufWritePost"], &opts)
            .expect("couldn't create autocmd")
    }

    #[inline]
    fn unregister(autocmd_id: Self::RegisterOutput) {
        let _ = api::del_autocmd(autocmd_id);
    }

    #[inline]
    fn cleanup(&self, event_key: DefaultKey, events: &mut Events) {
        if let Some(callbacks) = events.on_buffer_saved.get_mut(&self.0) {
            callbacks.remove(event_key);
            if callbacks.is_empty() {
                events.on_buffer_saved.remove(&self.0);
            }
        }
    }
}

impl Event for CursorMoved {
    type Args<'a> = (&'a NeovimCursor<'a>, AgentId);
    type RegisterOutput = (AutocmdId, AutocmdId);

    #[inline]
    fn get_or_insert_callbacks<'ev>(
        &self,
        events: &'ev mut Events,
    ) -> &'ev mut EventCallbacks<Self> {
        events.on_cursor_moved.entry(self.0).or_default()
    }

    #[inline]
    fn register(&self, events: EventsBorrow) -> Self::RegisterOutput {
        let opts = opts::CreateAutocmdOpts::builder()
            .group(events.augroup_id)
            .buffer(self.0.into())
            .callback({
                let events = events.handle;
                move |args: types::AutocmdCallbackArgs| {
                    events.with_mut(|inner| {
                        let cursor = NeovimCursor::new(NeovimBuffer::new(
                            BufferId::new(args.buffer),
                            &events,
                        ));

                        let Some(callbacks) =
                            inner.on_cursor_moved.get_mut(&cursor.buffer_id())
                        else {
                            return true;
                        };

                        for callback in callbacks.iter_mut() {
                            callback((&cursor, AgentId::UNKNOWN));
                        }

                        false
                    })
                }
            })
            .build();

        // Neovim has 3 separate cursor-move-related autocommand events --
        // CursorMoved, CursorMovedI and CursorMovedC -- which are triggered
        // when the cursor is moved in Normal/Visual mode, Insert mode and in
        // the command line, respectively.
        //
        // Since ed has no concept of modes, we register the callback on both
        // CursorMoved and CursorMovedI.

        let cursor_moved_id = api::create_autocmd(["CursorMoved"], &opts)
            .expect("couldn't create autocmd");

        let cursor_moved_i_id = api::create_autocmd(["CursorMovedI"], &opts)
            .expect("couldn't create autocmd");

        (cursor_moved_id, cursor_moved_i_id)
    }

    #[inline]
    fn unregister((cursor_moved_id, cursor_moved_i_id): Self::RegisterOutput) {
        let _ = api::del_autocmd(cursor_moved_id);
        let _ = api::del_autocmd(cursor_moved_i_id);
    }

    #[inline]
    fn cleanup(&self, event_key: DefaultKey, events: &mut Events) {
        if let Some(callbacks) = events.on_cursor_moved.get_mut(&self.0) {
            callbacks.remove(event_key);
            if callbacks.is_empty() {
                events.on_cursor_moved.remove(&self.0);
            }
        }
    }
}

impl Event for OnBytes {
    type Args<'a> = (&'a NeovimBuffer<'a>, &'a Edit);
    type RegisterOutput = ();

    #[inline]
    fn get_or_insert_callbacks<'ev>(
        &self,
        events: &'ev mut Events,
    ) -> &'ev mut EventCallbacks<Self> {
        events.on_buffer_edited.entry(self.0).or_default()
    }

    #[inline]
    fn register(&self, events: EventsBorrow) {
        let buffer_id = self.0;

        let opts = opts::BufAttachOpts::builder()
            .on_bytes({
                let events = events.handle;
                move |args: opts::OnBytesArgs| {
                    events.with_mut(|inner| {
                        let buffer = NeovimBuffer::new(buffer_id, &events);

                        let Some(callbacks) =
                            inner.on_buffer_edited.get_mut(&buffer.id())
                        else {
                            return true;
                        };

                        let edited_by = inner
                            .agent_ids
                            .edited_buffer
                            .remove(&buffer.id())
                            .unwrap_or(AgentId::UNKNOWN);

                        let edit = Edit {
                            made_by: edited_by,
                            replacements: smallvec_inline![
                                buffer.replacement_of_on_bytes(args)
                            ],
                        };

                        for callback in callbacks.iter_mut() {
                            callback((&buffer, &edit));
                        }

                        false
                    })
                }
            })
            .build();

        api::Buffer::from(buffer_id)
            .attach(false, &opts)
            .expect("couldn't attach to buffer");
    }

    #[inline]
    fn unregister((): Self::RegisterOutput) {}

    #[inline]
    fn cleanup(&self, event_key: DefaultKey, events: &mut Events) {
        if let Some(callbacks) = events.on_buffer_edited.get_mut(&self.0) {
            callbacks.remove(event_key);
            if callbacks.is_empty() {
                events.on_buffer_edited.remove(&self.0);
            }
        }
    }
}

impl Drop for EventHandle {
    #[inline]
    fn drop(&mut self) {
        let key = self.event_key;
        self.events.with_mut(|events| match self.event_kind {
            EventKind::BufEnter(event) => event.cleanup(key, events),
            EventKind::BufLeave(event) => event.cleanup(key, events),
            EventKind::BufReadPost(event) => event.cleanup(key, events),
            EventKind::BufUnload(event) => event.cleanup(key, events),
            EventKind::BufWritePost(event) => event.cleanup(key, events),
            EventKind::CursorMoved(event) => event.cleanup(key, events),
            EventKind::OnBytes(event) => event.cleanup(key, events),
        })
    }
}
