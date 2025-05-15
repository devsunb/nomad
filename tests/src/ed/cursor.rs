use ed::backend::Backend;
use ed::{EditorCtx, Shared};
use futures_lite::future;

pub(crate) fn on_cursor_created<Ed: Backend>(ctx: &mut EditorCtx<'_, Ed>) {
    let agent_id = ctx.new_agent_id();

    let num_called = Shared::new(0);

    let _handle = ctx.on_cursor_created({
        let num_called = num_called.clone();
        move |_cursor, created_by| {
            assert_eq!(created_by, agent_id);
            num_called.with_mut(|count| *count += 1);
        }
    });

    future::block_on(ctx.spawn_local(async |ctx| {
        // Focusing the buffer should create a cursor.
        ctx.create_and_focus(path!("/foo.txt"), agent_id).await.unwrap();
    }));

    assert_eq!(num_called.copied(), 1);
}
