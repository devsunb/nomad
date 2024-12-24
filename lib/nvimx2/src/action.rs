use crate::{Backend, MaybeResult, Module};

/// TODO: docs.
pub trait Action<B: Backend>: 'static {
    /// TODO: docs.
    const NAME: &'static ActionName;

    /// TODO: docs.
    type Module: Module<B>;

    /// TODO: docs.
    type Args;

    /// TODO: docs.
    type Ctx<'a>;

    /// TODO: docs.
    type Return;

    /// TODO: docs.
    type Docs;

    /// TODO: docs.
    fn call(
        &mut self,
        args: Self::Args,
        ctx: Self::Ctx<'_>,
    ) -> impl MaybeResult<Self::Return>;

    /// TODO: docs.
    fn docs() -> Self::Docs;
}

/// TODO: docs.
pub struct ActionName(str);
