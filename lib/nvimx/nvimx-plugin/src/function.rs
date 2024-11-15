use nvimx_common::MaybeResult;
use nvimx_ctx::NeovimCtx;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::action_name::ActionName;
use crate::{Action, Module};

/// TODO: docs
pub trait Function<M: Module>: 'static {
    /// TODO: docs
    const NAME: ActionName;

    /// TODO: docs
    type Args: DeserializeOwned;

    /// TODO: docs
    type Docs;

    /// TODO: docs
    type Return: Serialize;

    /// TODO: docs
    fn execute<'a>(
        &'a mut self,
        args: Self::Args,
        ctx: NeovimCtx<'a>,
    ) -> impl MaybeResult<Self::Return>;

    /// TODO: docs
    fn docs(&self) -> Self::Docs;
}

impl<A, M> Function<M> for A
where
    A: for<'a> Action<M, Ctx<'a> = NeovimCtx<'a>>,
    A::Args: DeserializeOwned,
    A::Return: Serialize,
    M: Module,
{
    const NAME: ActionName = A::NAME;
    type Args = A::Args;
    type Docs = A::Docs;
    type Return = A::Return;

    fn execute<'a>(
        &'a mut self,
        args: Self::Args,
        ctx: NeovimCtx<'a>,
    ) -> impl MaybeResult<Self::Return> {
        A::execute(self, args, ctx)
    }

    fn docs(&self) -> Self::Docs {
        A::docs(self)
    }
}
