use core::ops::{Deref, DerefMut};

/// TODO: docs.
pub trait Access<T: ?Sized> {
    /// TODO: docs.
    fn with<R>(&self, fun: impl FnOnce(&T) -> R) -> R;
}

/// TODO: docs.
pub trait AccessMut<T: ?Sized>: Access<T> {
    /// TODO: docs.
    fn with_mut<R>(&mut self, fun: impl FnOnce(&mut T) -> R) -> R;
}

impl<T: Deref> Access<T::Target> for T {
    #[inline]
    fn with<R>(&self, fun: impl FnOnce(&T::Target) -> R) -> R {
        fun(self.deref())
    }
}

impl<T: DerefMut> AccessMut<T::Target> for T {
    #[inline]
    fn with_mut<R>(&mut self, fun: impl FnOnce(&mut T::Target) -> R) -> R {
        fun(self.deref_mut())
    }
}
