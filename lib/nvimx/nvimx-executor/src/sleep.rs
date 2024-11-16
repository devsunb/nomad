use core::cell::UnsafeCell;
use core::future::Future;
use core::marker::{PhantomData, PhantomPinned};
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

use nvim_oxi::libuv;

/// TODO: docs.
pub fn sleep(duration: Duration) -> Sleep {
    Sleep {
        has_completed: UnsafeCell::new(false),
        state: State::Starting { duration: Some(duration) },
        _not_send: PhantomData,
        _not_unpin: PhantomPinned,
    }
}

/// TODO: docs.
pub struct Sleep {
    has_completed: UnsafeCell<bool>,
    state: State,
    _not_send: PhantomData<*mut ()>,
    _not_unpin: PhantomPinned,
}

enum State {
    Starting { duration: Option<Duration> },
    Sleeping { timer_handle: libuv::TimerHandle },
    Done,
}

impl Sleep {
    fn state_mut(self: Pin<&mut Self>) -> &mut State {
        // SAFETY: `state` is not structurally pinned and we never create
        // pinned references to it.
        unsafe { &mut self.get_unchecked_mut().state }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        ctx: &mut Context,
    ) -> Poll<Self::Output> {
        match self.as_mut().state_mut() {
            State::Starting { duration } => {
                let duration = duration.take().expect("first poll");
                let has_completed_ptr = self.has_completed.get();
                let waker = ctx.waker().clone();
                let timer_handle =
                    libuv::TimerHandle::once(duration, move || {
                        // SAFETY:
                        //
                        // - `State`'s `Drop` impl cancels this callback, so if
                        // we're here, the future is still alive;
                        //
                        // - `Sleep` is `!Unpin` so the pointer still points to
                        // valid memory;
                        let has_completed = unsafe { &mut *has_completed_ptr };
                        *has_completed = true;
                        waker.wake();
                    })
                    .expect("never fails(?)");
                *self.state_mut() = State::Sleeping { timer_handle };
                Poll::Pending
            },
            State::Sleeping { .. } => {
                // SAFETY: There arent't any active exclusive references.
                if unsafe { *self.has_completed.get() } {
                    *self.state_mut() = State::Done;
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            },
            State::Done => panic!("polled after completion"),
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        if let Self::Sleeping { timer_handle, .. } = self {
            timer_handle.stop().expect("never fails(?)");
        }
    }
}
