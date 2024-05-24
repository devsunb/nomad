//! TODO: docs.

use core::cell::OnceCell;
use core::future::Future;
use core::marker::PhantomData;

use async_task::{Builder, Runnable};
use concurrent_queue::ConcurrentQueue;

use crate::oxi::{self, libuv};
pub use crate::task::Task;

/// A single-threaded executor integrated with the Neovim event loop.
///
/// See the [module-level](crate::executor) documentation for more information.
pub struct Executor<'a> {
    /// The executor state.
    state: OnceCell<ExecutorState>,

    /// A handle to the callback that ticks the executor.
    callback_handle: libuv::AsyncHandle,

    /// A fake lifetime to avoid having to require a `'static` lifetime for the
    /// futures given to [`spawn`](Self::spawn).
    _lifetime: PhantomData<&'a ()>,
}

struct ExecutorState {
    /// The queue of tasks that are ready to be polled.
    woken_queue: ConcurrentQueue<Runnable<()>>,
}

impl<'a> Executor<'a> {
    /// TODO: docs.
    #[inline]
    pub fn spawn<F>(&self, _fut: F) -> Task<F::Output>
    where
        F: Future<Output = ()> + 'a,
    {
        todo!();
    }
}

impl ExecutorState {
    /// Creates a new [`ExecutorState`].
    #[inline]
    fn new() -> Self {
        Self { woken_queue: ConcurrentQueue::unbounded() }
    }

    /// Polls all the tasks that have awoken since the last poll.
    ///
    /// This consumes the task queue in a FIFO manner.
    #[inline]
    fn poll_all_woken(&self) {
        while let Ok(runnable) = self.woken_queue.pop() {
            runnable.run();
        }
    }
}
