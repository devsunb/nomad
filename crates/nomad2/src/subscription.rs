use core::any::Any;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::sync::Arc;

use futures_util::Stream;

use crate::{Editor, Event};

/// TODO: docs.
pub struct Subscription<T: Event<E>, E: Editor> {
    /// TODO: docs.
    event: Arc<dyn Any>,

    /// TODO: docs.
    rx: Receiver<T::Payload>,

    /// TODO: docs.
    ctx: crate::Context<E>,
}

impl<T: Event<E>, E: Editor> Subscription<T, E> {
    pub(crate) fn new(
        event: Arc<dyn Any>,
        rx: Receiver<T::Payload>,
        ctx: crate::Context<E>,
    ) -> Self {
        Self { event, rx, ctx }
    }
}

impl<T: Event<E>, E: Editor> Stream for Subscription<T, E> {
    type Item = T::Payload;

    #[inline]
    fn poll_next(
        self: Pin<&mut Self>,
        _ctx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        todo!();
    }
}

impl<T: Event<E>, E: Editor> Drop for Subscription<T, E> {
    #[inline]
    fn drop(&mut self) {
        let count = Arc::strong_count(&self.event);

        // The `Context` owns another instance of the event, so if the ref
        // count reaches 2, it means this is the last subscription.
        if count == 2 {
            let event = self.event.downcast_ref::<T>().unwrap();
            let state = self.ctx.remove_subscription(event).unwrap();
            let sub_ctx = state.sub_ctx.downcast::<T::SubscribeCtx>().unwrap();
            event.unsubscribe(*sub_ctx, &self.ctx);
        }
    }
}

pub(crate) fn channel<T>() -> (Emitter<T>, Receiver<T>) {
    todo!();
}

/// TODO: docs.
pub struct Emitter<T> {
    item: T,
}

impl<T> Emitter<T> {
    /// TODO: docs.
    #[inline]
    pub fn send(&self, _: T) {
        todo!();
    }
}

pub(crate) struct Receiver<T> {
    inner: T,
}

impl<T> Receiver<T> {
    pub(crate) fn deactivate(self) -> InactiveReceiver<T> {
        todo!();
    }
}

impl<T> Clone for Receiver<T> {
    #[inline]
    fn clone(&self) -> Self {
        todo!();
    }
}

pub(crate) struct InactiveReceiver<T> {
    inner: T,
}

impl<T> InactiveReceiver<T> {
    pub(crate) fn reactivate(self) -> Receiver<T> {
        todo!();
    }

    pub(crate) fn into_any(self) -> AnyReceiver {
        todo!();
    }
}

impl<T> Clone for InactiveReceiver<T> {
    #[inline]
    fn clone(&self) -> Self {
        todo!();
    }
}

pub(crate) struct AnyReceiver {
    inner: InactiveReceiver<()>,
}

impl AnyReceiver {
    pub(crate) unsafe fn downcast_ref_unchecked<T>(
        &self,
    ) -> &InactiveReceiver<T> {
        todo!();
    }
}
