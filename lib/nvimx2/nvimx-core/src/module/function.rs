use serde::de::Deserialize;
use serde::ser::Serialize;

use crate::action::{Action, ActionCtx};
use crate::backend::Backend;
use crate::notify::{MaybeResult, Name};
use crate::plugin::Plugin;

/// TODO: docs.
pub trait Function<P, B>: 'static
where
    P: Plugin<B>,
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
        ctx: &mut ActionCtx<P, B>,
    ) -> impl MaybeResult<Self::Return, B> + use<'this, 'args, Self, P, B>;
}

impl<A, P, B> Function<P, B> for A
where
    A: Action<P, B>,
    for<'args> A::Args<'args>: Deserialize<'args>,
    A::Return: Serialize,
    P: Plugin<B>,
    B: Backend,
{
    const NAME: Name = A::NAME;

    type Args<'a> = A::Args<'a>;
    type Return = A::Return;

    #[inline]
    fn call<'this, 'args>(
        &'this mut self,
        args: A::Args<'args>,
        ctx: &mut ActionCtx<P, B>,
    ) -> impl MaybeResult<Self::Return, B> + use<'this, 'args, A, P, B> {
        A::call(self, args, ctx)
    }
}
