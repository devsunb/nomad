use core::convert::Infallible;
use std::borrow::Cow;

#[derive(Default)]
pub struct MockClipboard {
    contents: Option<String>,
}

impl clipboard::Clipboard for MockClipboard {
    type GetError = Infallible;
    type SetError = Infallible;

    fn get(&mut self) -> Result<Option<Cow<'_, str>>, Self::GetError> {
        Ok(self.contents.as_deref().map(Cow::Borrowed))
    }

    fn set<T: AsRef<str>>(&mut self, value: T) -> Result<(), Self::SetError> {
        self.contents = Some(value.as_ref().to_owned());
        Ok(())
    }
}
