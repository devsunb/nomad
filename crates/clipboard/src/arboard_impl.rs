use std::borrow::Cow;

use crate::Clipboard;

impl Clipboard for arboard::Clipboard {
    type GetError = arboard::Error;
    type SetError = arboard::Error;

    fn get(&mut self) -> Result<Option<Cow<'_, str>>, Self::GetError> {
        self.get_text().map(Cow::Owned).map(Some)
    }

    fn set<T: AsRef<str>>(&mut self, value: T) -> Result<(), Self::SetError> {
        self.set_text(value.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(
        all(headless, target_os = "linux"),
        ignore = "fails on headless X11"
    )]
    fn clipboard_set_get_cycle() {
        let mut arboard = arboard();

        for text in (0..10).map(|n| n.to_string()) {
            arboard.set(&text).unwrap();
            assert_eq!(arboard.get().unwrap().as_deref(), Some(&*text));
        }
    }

    fn arboard() -> impl Clipboard {
        arboard::Clipboard::new().unwrap()
    }
}
