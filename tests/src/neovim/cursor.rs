use ed::EditorCtx;
use neovim::Neovim;

use crate::ed::cursor;

#[neovim::test]
fn on_cursor_created_1(ctx: &mut EditorCtx<Neovim>) {
    cursor::on_cursor_created_1(ctx);
}

#[neovim::test]
fn on_cursor_created_2(ctx: &mut EditorCtx<Neovim>) {
    cursor::on_cursor_created_2(ctx);
}

#[neovim::test]
fn on_cursor_moved_1(ctx: &mut EditorCtx<Neovim>) {
    cursor::on_cursor_moved_1(ctx);
}
