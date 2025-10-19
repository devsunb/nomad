use core::fmt;
use std::borrow::Cow;

use either::Either;

use crate::Clipboard;

/// A trait for types whose initialization can fail.
pub trait FallibleInit: Sized {
    /// The type of error returned if initialization fails.
    type Error: fmt::Debug;

    /// Attempts to create a new instance of `Self`.
    fn new() -> Result<Self, Self::Error>;
}

/// A [`Clipboard`] adapter for clipboards whose initialization can fail.
pub struct FallibleInitClipboard<C> {
    inner: Option<C>,
}

impl<C: FallibleInit> FallibleInitClipboard<C> {
    /// Creates a new `FallibleInitClipboard`, attempting to initialize the
    /// inner clipboard.
    pub fn new() -> Self {
        Self::default()
    }

    fn inner(&mut self) -> Result<&mut C, C::Error> {
        match &mut self.inner {
            Some(inner) => Ok(inner),
            none @ None => match C::new() {
                Ok(inner) => Ok(none.insert(inner)),
                Err(err) => Err(err),
            },
        }
    }
}

impl<C: FallibleInit> Default for FallibleInitClipboard<C> {
    fn default() -> Self {
        Self { inner: C::new().ok() }
    }
}

impl<C: FallibleInit + Clipboard> Clipboard for FallibleInitClipboard<C> {
    type GetError = Either<C::Error, C::GetError>;
    type SetError = Either<C::Error, C::SetError>;

    fn get(&mut self) -> Result<Option<Cow<'_, str>>, Self::GetError> {
        self.inner().map_err(Either::Left)?.get().map_err(Either::Right)
    }

    fn set<T: AsRef<str>>(&mut self, value: T) -> Result<(), Self::SetError> {
        self.inner().map_err(Either::Left)?.set(value).map_err(Either::Right)
    }
}

#[cfg(feature = "arboard")]
impl FallibleInit for arboard::Clipboard {
    type Error = arboard::Error;

    fn new() -> Result<Self, Self::Error> {
        Self::new()
    }
}
