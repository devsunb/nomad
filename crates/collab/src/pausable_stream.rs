use core::cell::Cell;
use core::pin::Pin;
use core::task::{Context, Poll, Waker, ready};
use std::collections::VecDeque;
use std::rc::Rc;

use futures_util::Stream;
use futures_util::stream::FusedStream;

pin_project_lite::pin_project! {
    /// A [`Stream`] adapter that can be [`pause`](Remote::pause) and
    /// [`resume`](Remote::resume)d via its [`Remote`].
    pub(crate) struct PausableStream<S: Stream> {
        buffer: VecDeque<S::Item>,
        #[pin]
        inner: S,
        remote: Remote,
    }
}

#[derive(Clone)]
pub(crate) struct Remote {
    inner: Rc<RemoteInner>,
}

#[derive(Default)]
struct RemoteInner {
    is_paused: Cell<bool>,
    waker: Cell<Option<Waker>>,
}

impl<S: Stream> PausableStream<S> {
    /// Returns whether the stream is currently paused.
    pub(crate) fn is_paused(&self) -> bool {
        self.remote.is_paused()
    }

    /// Creates a new [`PausableStream`], which starts unpaused.
    pub(crate) fn new(stream: S) -> Self {
        Self {
            buffer: Default::default(),
            inner: stream,
            remote: Remote::new(),
        }
    }

    /// Returns a [`Remote`] that can be used to pause and resume this
    /// [`PausableStream`].
    pub(crate) fn remote(&self) -> Remote {
        self.remote.clone()
    }
}

impl Remote {
    /// Pauses the associated [`PausableStream`], returning `true` if it was
    /// not already paused.
    pub(crate) fn pause(&self) -> bool {
        !self.inner.is_paused.replace(true)
    }

    /// Resumes the associated [`PausableStream`], returning `true` if the
    /// stream was paused.
    pub(crate) fn resume(&self) -> bool {
        let was_paused = self.inner.is_paused.replace(false);

        if was_paused && let Some(waker) = self.inner.waker.take() {
            waker.wake();
        }

        was_paused
    }

    fn is_paused(&self) -> bool {
        self.inner.is_paused.get()
    }

    fn new() -> Self {
        Self { inner: Rc::default() }
    }
}

impl<S: FusedStream> Stream for PausableStream<S> {
    type Item = S::Item;

    fn poll_next(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        let is_paused = this.remote.is_paused();

        if !is_paused && let Some(item) = this.buffer.pop_front() {
            return Poll::Ready(Some(item));
        }

        if is_paused {
            this.remote.inner.waker.set(Some(ctx.waker().clone()));
        }

        // Even if paused, we still want to poll the inner stream to avoid
        // creating backpressure.
        loop {
            match ready!(this.inner.as_mut().poll_next(ctx)) {
                Some(item) if !is_paused => return Poll::Ready(Some(item)),
                Some(item) => this.buffer.push_back(item),
                None if !is_paused => return Poll::Ready(None),
                None => return Poll::Pending,
            }
        }
    }
}

impl<S: FusedStream> FusedStream for PausableStream<S> {
    fn is_terminated(&self) -> bool {
        self.inner.is_terminated() && self.buffer.is_empty()
    }
}
