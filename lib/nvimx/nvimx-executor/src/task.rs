/// TODO: docs
pub struct Task<T> {
    inner: async_task::Task<T>,
}

impl<T> Task<T> {
    /// TODO: docs
    #[inline]
    pub fn detach(self) {
        self.inner.detach()
    }

    /// Creates a new [`Task`].
    #[inline]
    pub(crate) fn new(inner: async_task::Task<T>) -> Self {
        Self { inner }
    }
}
