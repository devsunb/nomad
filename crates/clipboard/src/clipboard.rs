use core::fmt;
use std::borrow::Cow;

/// A trait representing the system's clipboard.
pub trait Clipboard {
    /// The type of error returned when [`get`](Clipboard::get)ting text fails.
    type GetError: fmt::Debug;

    /// The type of error returned when [`set`](Clipboard::set)ting text fails.
    type SetError: fmt::Debug;

    /// Gets the current text contents of the clipboard.
    fn get(&mut self) -> Result<Option<Cow<'_, str>>, Self::GetError>;

    /// Sets the current text contents of the clipboard.
    fn set<T: AsRef<str>>(&mut self, value: T) -> Result<(), Self::SetError>;
}
