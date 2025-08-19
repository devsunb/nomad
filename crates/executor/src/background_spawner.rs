use crate::Task;

/// TODO: docs.
pub trait BackgroundSpawner: Clone + Send + 'static {
    /// TODO: docs.
    type Task<T: Send + 'static>: Task<T> + Send;

    /// TODO: docs.
    fn spawn<Fut>(&mut self, fut: Fut) -> Self::Task<Fut::Output>
    where
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static;
}
