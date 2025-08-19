use core::pin::Pin;
use core::task::{Context, Poll};

use crate::{BackgroundSpawner, Executor, LocalSpawner};

/// TODO: docs.
pub trait Task<T>: Future<Output = T> {
    /// TODO: docs.
    fn detach(self);
}

pin_project_lite::pin_project! {
    /// TODO: docs.
    pub struct LocalTask<T, Ex: Executor> {
        #[pin]
        inner: <Ex::LocalSpawner as LocalSpawner>::Task<T>,
    }
}

pin_project_lite::pin_project! {
    /// TODO: docs.
    pub struct BackgroundTask<T, Ex: Executor> where T: 'static, T: Send {
        #[pin]
        inner: <Ex::BackgroundSpawner as BackgroundSpawner>::Task<T>,
    }
}

impl<T, Ex: Executor> LocalTask<T, Ex> {
    /// TODO: docs.
    #[inline]
    pub fn detach(self) {
        self.inner.detach();
    }

    /// TODO: docs.
    #[inline]
    pub fn new(inner: <Ex::LocalSpawner as LocalSpawner>::Task<T>) -> Self {
        Self { inner }
    }
}

impl<T: Send + 'static, Ex: Executor> BackgroundTask<T, Ex> {
    /// TODO: docs.
    #[inline]
    pub fn detach(self) {
        self.inner.detach();
    }

    /// TODO: docs.
    #[inline]
    pub fn new(
        inner: <Ex::BackgroundSpawner as BackgroundSpawner>::Task<T>,
    ) -> Self {
        Self { inner }
    }
}

impl<T, Ex: Executor> Future for LocalTask<T, Ex> {
    type Output = T;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        self.project().inner.poll(cx)
    }
}

impl<T: Send + 'static, Ex: Executor> Future for BackgroundTask<T, Ex> {
    type Output = T;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        self.project().inner.poll(cx)
    }
}

#[cfg(feature = "async-task")]
impl<T> Task<T> for async_task::Task<T> {
    #[inline]
    fn detach(self) {
        Self::detach(self);
    }
}
