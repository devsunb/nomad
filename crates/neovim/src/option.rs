use core::marker::PhantomData;

use crate::buffer::{BufferId, NeovimBuffer};
use crate::events::{Callbacks, Event, EventKind, Events, EventsBorrow};
use crate::oxi::{self, api};

/// TODO: docs.
pub(crate) trait NeovimOption: 'static + Sized {
    /// TODO: docs.
    const LONG_NAME: &'static str;

    /// TODO: docs.
    type Value: oxi::conversion::FromObject;

    /// TODO: docs.
    type LocalCtx: ?Sized;

    /// TODO: docs.
    fn get(&self, ctx: &Self::LocalCtx) -> Self::Value;

    /// TODO: docs.
    fn set(&mut self, value: Self::Value, ctx: &Self::LocalCtx);
}

/// TODO: docs.
pub(crate) trait WatchedOption: NeovimOption {
    fn callbacks(
        events: &mut Events,
    ) -> &mut Option<Callbacks<OptionSet<Self>>>;

    fn event_kind() -> EventKind;
}

/// TODO: docs.
pub(crate) struct OptionSet<T: NeovimOption>(PhantomData<T>);

impl<T: WatchedOption> Event for OptionSet<T> {
    /// A tuple of `(buffer, old_value, new_value)`, where `buffer` is only
    /// present for buffer-local options.
    type Args<'a> = (Option<NeovimBuffer<'a>>, &'a T::Value, &'a T::Value);
    type Container<'ev> = &'ev mut Option<Callbacks<Self>>;
    type RegisterOutput = u32;

    #[inline]
    fn container<'ev>(&self, events: &'ev mut Events) -> Self::Container<'ev> {
        T::callbacks(events)
    }

    #[inline]
    fn key(&self) {}

    #[inline]
    fn kind(&self) -> EventKind {
        T::event_kind()
    }

    #[inline]
    fn register(&self, events: EventsBorrow) -> Self::RegisterOutput {
        let augroup_id = events.augroup_id;

        let buf_fields = events.borrow.buffer_fields.clone();
        let events = events.handle;

        let opts = api::opts::CreateAutocmdOpts::builder()
            .group(augroup_id)
            .patterns([T::LONG_NAME])
            .callback(move |_: api::types::AutocmdCallbackArgs| {
                let is_local = api::get_vvar::<oxi::String>("option_type")
                    .expect("couldn't get option_type")
                    == "local";

                let buffer = is_local.then(|| {
                    Events::buffer(
                        BufferId::of_focused(),
                        &events,
                        &buf_fields,
                    )
                });

                let old_value = api::get_vvar::<T::Value>("option_old")
                    .expect("couldn't get option_old");

                let new_value = api::get_vvar::<T::Value>("option_new")
                    .expect("couldn't get option_new");

                let Some(callbacks) = events.with_mut(|ev| {
                    T::callbacks(ev).as_ref().map(Callbacks::cloned)
                }) else {
                    return true;
                };

                for callback in callbacks {
                    callback((buffer, &old_value, &new_value));
                }

                false
            })
            .build();

        api::create_autocmd(["OptionSet"], &opts)
            .expect("couldn't create autocmd on OptionSet")
    }

    #[inline]
    fn unregister(autocmd_id: Self::RegisterOutput) {
        let _ = api::del_autocmd(autocmd_id);
    }
}
