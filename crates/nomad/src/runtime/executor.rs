use core::cell::{OnceCell, UnsafeCell};
use core::future::Future;
use std::sync::Arc;

use async_task::{Builder, Runnable};
use neovim::nvim::libuv;

use super::JoinHandle;

thread_local! {
    static EXECUTOR: OnceCell<UnsafeCell<LocalExecutor>>
        = const { OnceCell::new() };
}

/// TODO: doc
#[inline]
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    with_executor(move |executor| executor.spawn(future))
}

/// TODO: docs
#[inline]
fn with_executor<F, R>(fun: F) -> R
where
    F: FnOnce(&mut LocalExecutor) -> R,
{
    let executor_ptr = EXECUTOR
        .with(|executor| executor.get_or_init(UnsafeCell::default).get());

    // SAFETY: we never give out references to the executor, but can we prove
    // that the function is not reentrant?
    let executor = unsafe { &mut *executor_ptr };

    fun(executor)
}

/// TODO: docs
#[derive(Default)]
struct LocalExecutor {
    inner: OnceCell<LocalExecutorInner>,
}

impl LocalExecutor {
    /// TODO: docs
    #[inline]
    fn inner(&self) -> &LocalExecutorInner {
        self.inner.get_or_init(LocalExecutorInner::new)
    }

    /// TODO: docs
    const fn new() -> Self {
        Self { inner: OnceCell::new() }
    }

    /// TODO: docs
    #[inline]
    fn spawn<F: Future>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        self.inner().spawn(future)
    }
}

/// TODO: docs
struct LocalExecutorInner {
    /// TODO: docs
    async_handle: libuv::AsyncHandle,

    /// TODO: docs
    state: Arc<LocalExecutorState>,
}

impl LocalExecutorInner {
    /// TODO: docs
    #[inline]
    fn new() -> Self {
        let state = Arc::new(LocalExecutorState::new());

        let also_state = Arc::clone(&state);

        // This callback will be registered to be executed on the next tick of
        // the libuv event loop everytime a future calls `Waker::wake()`.
        let async_handle = libuv::AsyncHandle::new(move || {
            state.tick_all();
            Ok::<_, core::convert::Infallible>(())
        })
        .unwrap();

        Self { async_handle, state: also_state }
    }

    /// TODO: docs
    #[inline]
    fn spawn<F: Future>(&self, _future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        todo!();
    }
}

/// TODO: docs
struct LocalExecutorState {
    woken_queue: TaskQueue,
}

impl LocalExecutorState {
    /// TODO: docs
    #[inline]
    fn new() -> Self {
        Self { woken_queue: TaskQueue::new() }
    }

    /// TODO: docs
    #[inline]
    fn tick_all(&self) {
        for _ in 0..self.woken_queue.len() {
            self.woken_queue.pop_front().expect("checked queue length").poll();
        }
    }
}

/// TODO: docs
struct TaskQueue {}

impl TaskQueue {
    /// TODO: docs
    #[inline]
    fn len(&self) -> usize {
        todo!();
    }

    /// TODO: docs
    #[inline]
    fn new() -> Self {
        todo!();
    }

    /// TODO: docs
    #[inline]
    fn pop_front(&self) -> Option<Task> {
        todo!();
    }

    /// TODO: docs
    #[inline]
    fn push_back(&self, _task: Task) {
        todo!();
    }
}

/// TODO: docs
struct Task {
    runnable: Runnable<()>,
}

impl Task {
    #[inline(always)]
    fn poll(self) {
        self.runnable.run();
    }
}
