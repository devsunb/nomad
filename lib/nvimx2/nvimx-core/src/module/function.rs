use serde::de::Deserialize;
use serde::ser::Serialize;

use crate::action::{Action, ActionCtx};
use crate::backend::Backend;
use crate::module::Module;
use crate::notify::{MaybeResult, Name};

/// TODO: docs.
pub trait Function<M, B>: 'static
where
    M: Module<B>,
    B: Backend,
{
    /// TODO: docs.
    const NAME: Name;

    /// TODO: docs.
    type Args<'args>: Deserialize<'args>;

    /// TODO: docs.
    type Return: Serialize + 'static;

    /// TODO: docs.
    fn call<'this, 'args>(
        &'this mut self,
        args: Self::Args<'args>,
        ctx: &mut ActionCtx<M, B>,
    ) -> impl MaybeResult<Self::Return> + use<'this, 'args, Self, M, B>;
}

impl<A, M, B> Function<M, B> for A
where
    A: Action<M, B>,
    for<'args> A::Args<'args>: Deserialize<'args>,
    A::Return: Serialize,
    M: Module<B>,
    B: Backend,
{
    const NAME: Name = A::NAME;

    type Args<'a> = A::Args<'a>;
    type Return = A::Return;

    #[inline]
    fn call<'this, 'args>(
        &'this mut self,
        args: A::Args<'args>,
        ctx: &mut ActionCtx<M, B>,
    ) -> impl MaybeResult<Self::Return> + use<'this, 'args, A, M, B> {
        A::call(self, args, ctx)
    }
}
