use crate::Task;

/// TODO: docs.
pub trait LocalSpawner {
    /// TODO: docs.
    type Task<T>: Task<T>;

    /// TODO: docs.
    fn spawn<Fut>(&mut self, fut: Fut) -> Self::Task<Fut::Output>
    where
        Fut: Future + 'static,
        Fut::Output: 'static;
}
