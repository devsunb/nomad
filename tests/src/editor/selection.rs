use core::mem;
use core::ops::Range;

use editor::{AccessMut, Context, Editor, Selection};
use futures_util::stream::FusedStream;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SelectionEvent {
    Created(Range<usize>),
    Moved(Range<usize>),
    Removed,
}

impl SelectionEvent {
    /// Returns a never-ending [`Stream`] of [`SelectionEvent`]s.
    #[track_caller]
    pub(crate) fn new_stream<Ed: Editor>(
        ctx: &mut Context<Ed>,
    ) -> impl FusedStream<Item = Self> + Unpin + use<Ed> {
        let (tx, rx) = flume::unbounded();
        let editor = ctx.editor();

        // ctx.for_each_buffer(|mut buf| {
        //     buf.for_each_selection(|mut selection| {
        //         // Subscribe to events on each existing selection.
        //         subscribe(&mut selection, tx.clone(), editor.clone());
        //     });
        // });

        mem::forget(ctx.on_selection_created(
            move |selection, _created_by| {
                let event = Self::Created(selection.byte_range());
                let _ = tx.send(event);

                // Subscribe to events on the newly created selection.
                subscribe(selection, tx.clone(), editor.clone());
            },
        ));

        rx.into_stream()
    }
}

fn subscribe<Ed: Editor>(
    selection: &mut Ed::Selection<'_>,
    tx: flume::Sender<SelectionEvent>,
    editor: impl AccessMut<Ed> + Clone + 'static,
) {
    let tx2 = tx.clone();
    mem::forget(selection.on_moved(
        move |selection, _moved_by| {
            let byte_range = selection.byte_range();
            let _ = tx2.send(SelectionEvent::Moved(byte_range));
        },
        editor.clone(),
    ));

    let tx2 = tx.clone();
    mem::forget(selection.on_removed(
        move |_selection_id, _removed_by| {
            let _ = tx2.send(SelectionEvent::Removed);
        },
        editor.clone(),
    ));
}
