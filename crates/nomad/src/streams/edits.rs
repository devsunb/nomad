use core::pin::Pin;
use core::task::{Context, Poll};

use async_broadcast::Receiver;
use futures::Stream;
use pin_project_lite::pin_project;

use crate::Edit;

pin_project! {
    /// A [`Stream`] that yields the [`Edit`]s that are applied to a
    /// [`Buffer`](crate::Buffer).
    pub struct Edits {
        #[pin]
        inner: Receiver<Edit>,
    }
}

impl Edits {
    #[inline]
    pub(crate) fn new(inner: Receiver<Edit>) -> Self {
        Self { inner }
    }
}

impl Stream for Edits {
    type Item = Edit;

    #[inline]
    fn poll_next(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_next(ctx)
    }
}
