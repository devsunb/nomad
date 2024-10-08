use nomad::ByteOffset;

use crate::CollabEditor;

pub(crate) type NeovimCursors = neovim::Cursors;

pub(crate) struct Cursor<E: CollabEditor> {
    pub(crate) action: CursorAction,
    pub(crate) cursor_id: E::CursorId,
    pub(crate) file_id: E::FileId,
}

#[derive(Clone)]
pub(crate) enum CursorAction {
    Created(ByteOffset),
    Moved(ByteOffset),
    Removed,
}

impl<E: CollabEditor> Clone for Cursor<E> {
    fn clone(&self) -> Self {
        Self {
            action: self.action.clone(),
            cursor_id: self.cursor_id.clone(),
            file_id: self.file_id.clone(),
        }
    }
}

mod neovim {
    use core::pin::Pin;
    use core::task::{Context, Poll};

    use futures_util::Stream;
    use nomad::neovim::{self, Neovim};
    use nomad::Subscription;

    pin_project_lite::pin_project! {
        /// TODO: docs.
        pub(crate) struct Cursors {
            buffer_id: neovim::BufferId,
            #[pin]
            inner: Subscription<neovim::events::CursorEvent, Neovim>,
        }
    }

    impl Stream for Cursors {
        type Item = super::Cursor<Neovim>;

        fn poll_next(
            self: Pin<&mut Self>,
            ctx: &mut Context,
        ) -> Poll<Option<Self::Item>> {
            self.project()
                .inner
                .poll_next(ctx)
                .map(|maybe_cursor| maybe_cursor.map(Into::into))
        }
    }

    impl From<neovim::events::Cursor> for super::Cursor<Neovim> {
        fn from(cursor: neovim::events::Cursor) -> Self {
            Self {
                action: cursor.action().into(),
                cursor_id: (),
                file_id: todo!(),
            }
        }
    }

    impl From<neovim::events::CursorAction> for super::CursorAction {
        fn from(action: neovim::events::CursorAction) -> Self {
            match action {
                neovim::events::CursorAction::Created(_point) => todo!(),
                neovim::events::CursorAction::Moved(_point) => todo!(),
                neovim::events::CursorAction::Removed => Self::Removed,
            }
        }
    }
}
