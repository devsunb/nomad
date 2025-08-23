use abs_path::path;
use editor::{Context, Shared};
use fs::File;
use neovim::Neovim;
use neovim::tests::NeovimExt;
use real_fs::RealFs;

#[neovim::test]
async fn on_buffer_created_doesnt_fire_for_nameless_buffers(
    ctx: &mut Context<Neovim>,
) {
    let num_times_fired = Shared::<u8>::new(0);

    let _handle = ctx.on_buffer_created({
        let num_times_fired = num_times_fired.clone();
        move |_, _| num_times_fired.with_mut(|n| *n += 1)
    });

    // The event should fire when creating a named buffer.
    let tempfile = RealFs::default().tempfile().await.unwrap();
    ctx.command(format!("edit {}", tempfile.path()));
    assert_eq!(num_times_fired.take(), 1);

    // The event shouldn't fire when creating a nameless buffer.
    ctx.command("enew");
    assert_eq!(num_times_fired.take(), 0);
}

#[neovim::test]
async fn on_buffer_created_fires_when_creating_buffer_not_backed_by_a_file(
    ctx: &mut Context<Neovim>,
) {
    let num_times_fired = Shared::<u8>::new(0);

    let _handle = ctx.on_buffer_created({
        let num_times_fired = num_times_fired.clone();
        move |_, _| num_times_fired.with_mut(|n| *n += 1)
    });

    // The event should fire when creating a named buffer, even if it's not
    // backed by a file.
    ctx.command(format!("edit {}", path!("/foo/bar.txt")));
    assert_eq!(num_times_fired.take(), 1);
}

#[neovim::test]
async fn on_buffer_created_fires_when_nameless_buffer_is_renamed(
    ctx: &mut Context<Neovim>,
) {
    let num_times_fired = Shared::<u8>::new(0);

    let _handle = ctx.on_buffer_created({
        let num_times_fired = num_times_fired.clone();
        move |_, _| num_times_fired.with_mut(|n| *n += 1)
    });

    // The event shouldn't fire when creating a nameless buffer.
    ctx.command("enew");
    assert_eq!(num_times_fired.take(), 0);

    // The event should fire when the buffer is given a name.
    ctx.command("file foo.txt");
    assert_eq!(num_times_fired.take(), 1);
}
