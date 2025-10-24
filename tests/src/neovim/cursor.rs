use core::mem;
use core::time::Duration;

use editor::{AgentId, Buffer, ByteOffset, Context, Cursor, Editor};
use futures_util::future::FutureExt as _;
use futures_util::stream::{FusedStream, StreamExt};
use neovim::Neovim;
use neovim::buffer::BufferId;
use neovim::tests::NeovimExt;

use crate::editor::cursor::{CursorCreation, CursorEvent, CursorRemoval};
use crate::utils::FutureExt as _;

#[neovim::test]
async fn normal_to_insert_with_i(ctx: &mut Context<Neovim>) {
    let buffer_id = ctx.create_and_focus_scratch_buffer();

    ctx.feedkeys("ihello<Esc>");

    let mut offsets = ByteOffset::new_stream(buffer_id, ctx);

    // The offset of a block cursor is set to its left-hand side, so entering
    // insert mode with "i" shouldn't change the offset.
    ctx.enter_insert_with_i();

    if let Some(offset) =
        offsets.select_next_some().timeout(Duration::from_millis(500)).await
    {
        panic!("expected no offsets, got {offset:?}");
    }
}

#[neovim::test]
#[ignore = "feedkeys(\"a<Esc>\") is not quite equivalent to the 'startinsert' \
            command, so no event is emitted and the test hangs"]
async fn normal_to_insert_with_a(ctx: &mut Context<Neovim>) {
    let buffer_id = ctx.create_and_focus_scratch_buffer();

    ctx.feedkeys("ihello<Esc>");

    let mut offsets = ByteOffset::new_stream(buffer_id, ctx);

    // The offset of a block cursor is set to its left-hand side, so entering
    // insert mode with "a" should move the offset to its right side.
    ctx.feedkeys("a<Esc>");

    assert_eq!(offsets.next().await.unwrap(), 5);
}

#[neovim::test]
async fn insert_to_normal(ctx: &mut Context<Neovim>) {
    let buffer_id = ctx.create_and_focus_scratch_buffer();

    ctx.feedkeys("ihello<Esc>");

    // The cursor is now between the second "l" and the "o".
    ctx.enter_insert_with_i();

    let mut offsets = ByteOffset::new_stream(buffer_id, ctx);

    // When we switch from insert to normal mode, the cursor is moved on top
    // of the second "l", which is at offset 3.
    ctx.feedkeys("<Esc>");

    assert_eq!(offsets.next().await.unwrap(), 3);
}

#[neovim::test]
async fn cursor_is_removed_from_old_and_created_in_new_when_switching_buffers(
    ctx: &mut Context<Neovim>,
) {
    // Create the first buffer and put some text in it.
    let first_buffer_id = ctx.create_and_focus_scratch_buffer();
    ctx.feedkeys("ihello<Esc>");

    let mut events = CursorEvent::new_stream(ctx);

    // Create and focus a second buffer.
    let second_buffer_id = ctx.create_and_focus_scratch_buffer();

    assert_eq!(
        events.next().await.unwrap(),
        CursorEvent::Removed(CursorRemoval {
            cursor_id: first_buffer_id,
            removed_by: AgentId::UNKNOWN
        })
    );

    assert_eq!(
        events.next().await.unwrap(),
        CursorEvent::Created(CursorCreation {
            buffer_id: second_buffer_id,
            byte_offset: 0,
            created_by: AgentId::UNKNOWN,
        })
    );
}

#[neovim::test]
async fn cursor_is_created_when_switching_focus_back_from_terminal(
    ctx: &mut Context<Neovim>,
) {
    // Create a focus a normal buffer (buftype=).
    let buffer_id = ctx.create_and_focus_scratch_buffer();

    let mut events = CursorEvent::new_stream(ctx);

    // Create a focus a terminal buffer (buftype=terminal).
    ctx.command("terminal");
    assert!(matches!(events.next().await.unwrap(), CursorEvent::Removed(_)));

    let agent_id = ctx.new_agent_id();

    // Switch focus back to the normal buffer.
    ctx.with_editor(|nvim| {
        nvim.buffer(buffer_id).unwrap().schedule_focus(agent_id).boxed_local()
    })
    .await;

    assert_eq!(
        events.next().await.unwrap(),
        CursorEvent::Created(CursorCreation {
            buffer_id,
            byte_offset: 0,
            created_by: agent_id,
        })
    );
}

trait ByteOffsetExt {
    /// Returns a never-ending stream of [`ByteOffset`]s on the buffer
    /// with the given ID corresponding to the cursor positions.
    fn new_stream(
        buffer_id: BufferId,
        ctx: &mut Context<Neovim>,
    ) -> impl FusedStream<Item = ByteOffset> + Unpin + use<Self> {
        let (tx, rx) = flume::unbounded();

        ctx.with_borrowed(|ctx| {
            ctx.buffer(buffer_id).unwrap().for_each_cursor(
                move |mut cursor| {
                    let tx2 = tx.clone();
                    mem::forget(cursor.on_moved(move |cursor, _moved_by| {
                        let _ = tx2.send(cursor.byte_offset());
                    }));
                },
            )
        });

        rx.into_stream()
    }
}

impl ByteOffsetExt for ByteOffset {}

mod ed_cursor {
    //! Contains the editor-agnostic cursor tests.

    use super::*;
    use crate::editor::cursor;

    #[neovim::test]
    async fn on_cursor_moved_1(ctx: &mut Context<Neovim>) {
        cursor::on_cursor_moved_1(ctx).await;
    }
}
