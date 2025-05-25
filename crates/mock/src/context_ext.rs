use ed::executor::Executor;
use ed::{Backend, BorrowState, Context};

/// TODO: docs.
pub trait ContextExt {
    /// TODO: docs.
    fn block_on<T>(&mut self, fun: impl AsyncFnOnce(&mut Self) -> T) -> T;
}

impl<Ed, S: BorrowState> ContextExt for Context<Ed, S>
where
    Ed: Backend<Executor: Executor<Runner = crate::executor::Runner>>,
{
    #[inline]
    fn block_on<T>(&mut self, fun: impl AsyncFnOnce(&mut Self) -> T) -> T {
        futures_lite::future::block_on(self.run(fun))
    }
}
