use core::future::Future;

/// TODO: docs.
pub trait Spawner {
    /// TODO: docs.
    type JoinHandle<T>: JoinHandle<T>;

    /// TODO: docs.
    fn spawn<F: Future<Output = ()> + 'static>(
        &self,
        fut: F,
    ) -> Self::JoinHandle<()>;

    /// TODO: docs.
    fn spawn_background<F: Future<Output = ()> + 'static + Send>(
        &self,
        fut: F,
    ) -> Self::JoinHandle<()>;
}

/// TODO: docs.
pub trait JoinHandle<T>: Future<Output = T> {
    /// TODO: docs.
    fn detach(self);
}
