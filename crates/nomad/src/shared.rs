//! TODO: docs

use alloc::rc::Rc;
use core::cell::{Cell, UnsafeCell};
use core::panic::Location;

/// TODO: docs
#[derive(Default)]
pub struct Shared<T: ?Sized> {
    inner: Rc<WithCell<T>>,
}

impl<T: ?Sized> Clone for Shared<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self { inner: Rc::clone(&self.inner) }
    }
}

#[derive(Default)]
struct WithCell<T: ?Sized> {
    borrow: Cell<Borrow>,
    value: UnsafeCell<T>,
}

#[derive(Copy, Clone, Default)]
enum Borrow {
    #[default]
    None,

    #[cfg(not(debug_assertions))]
    Shared,
    #[cfg(debug_assertions)]
    Shared(&'static Location<'static>),

    #[cfg(not(debug_assertions))]
    Exclusive,
    #[cfg(debug_assertions)]
    Exclusive(&'static Location<'static>),
}
