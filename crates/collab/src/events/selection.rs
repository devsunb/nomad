use nomad::ByteOffset;

use crate::CollabEditor;

pub(crate) type NeovimSelections = neovim::Selections;

pub(crate) struct Selection<E: CollabEditor> {
    pub(crate) action: SelectionAction,
    pub(crate) selection_id: E::SelectionId,
    pub(crate) file_id: E::FileId,
}

#[derive(Clone)]
pub(crate) enum SelectionAction {
    Created { head: ByteOffset, tail: ByteOffset },
    Moved { head: ByteOffset, tail: ByteOffset },
    Removed,
}

impl<E: CollabEditor> Clone for Selection<E> {
    fn clone(&self) -> Self {
        Self {
            action: self.action.clone(),
            selection_id: self.selection_id.clone(),
            file_id: self.file_id.clone(),
        }
    }
}

mod neovim {
    use core::pin::Pin;
    use core::task::{Context, Poll};

    use futures_util::Stream;
    use nomad::neovim::Neovim;

    /// TODO: docs.
    pub(crate) struct Selections {}

    impl Stream for Selections {
        type Item = super::Selection<Neovim>;

        fn poll_next(
            self: Pin<&mut Self>,
            _: &mut Context,
        ) -> Poll<Option<Self::Item>> {
            Poll::Pending
        }
    }
}
