use editor::Editor;
use mock::Mock;
use mock::fs::MockFs;

mod ed_cursor {
    //! Contains the editor-agnostic cursor tests.

    use super::*;
    use crate::editor::cursor;

    #[test]
    fn on_cursor_moved_1() {
        Mock::<MockFs>::default()
            .with_ctx(|ctx| ctx.block_on(cursor::on_cursor_moved_1));
    }
}
