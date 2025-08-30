use core::time::Duration;

use futures_lite::future;

/// An extension trait for [`Future`]s.
pub(crate) trait FutureExt: Future {
    /// Returns `Some(output)` if the future completes before the given
    /// deadline, or `None` otherwise.
    fn timeout(
        self,
        deadline: Duration,
    ) -> impl Future<Output = Option<Self::Output>>
    where
        Self: Sized,
    {
        use futures_util::FutureExt;
        future::or(
            self.map(Some),
            async_io::Timer::after(deadline).map(|_now| None),
        )
    }
}

impl<T: Future> FutureExt for T {}
