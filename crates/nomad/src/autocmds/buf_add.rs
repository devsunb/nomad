use core::fmt;

use crate::neovim::{Autocmd, BufferId, ShouldDetach};
use crate::Action;

/// TODO: docs.
pub struct BufAdd<A>(pub A);

impl<A> Autocmd<A::Module> for BufAdd<A>
where
    A: Action<Args = BufferId>,
    A::Docs: fmt::Display,
    A::Return: Into<ShouldDetach>,
{
    type Action = A;

    fn into_action(self) -> A {
        self.0
    }

    fn on_events(&self) -> impl IntoIterator<Item = &str> {
        ["BufAdd"]
    }
}
