use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use std::time::Instant;

use neovim::nvim::libuv;
use pin_project_lite::pin_project;

/// TODO: docs.
#[inline]
pub fn sleep(duration: Duration) -> Sleep {
    Sleep::new(Instant::now() + duration)
}

pin_project! {
    /// TODO: docs
    pub struct Sleep {
        has_completed: bool,
        sleep_until: Instant,
    }
}

impl Sleep {
    #[inline]
    fn new(sleep_until: Instant) -> Self {
        Self { has_completed: false, sleep_until }
    }
}

impl Future for Sleep {
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<()> {
        let this = self.project();

        if *this.has_completed {
            panic!("Sleep future polled after completion");
        }

        let Some(left_to_sleep) =
            this.sleep_until.checked_duration_since(Instant::now())
        else {
            *this.has_completed = true;
            return Poll::Ready(());
        };

        let waker = ctx.waker().clone();

        let _ = libuv::TimerHandle::once(left_to_sleep, || {
            waker.wake();
            Ok::<_, core::convert::Infallible>(())
        })
        .unwrap();

        Poll::Pending
    }
}
