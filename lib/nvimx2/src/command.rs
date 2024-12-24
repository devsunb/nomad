use crate::{
    Action,
    ActionName,
    Backend,
    CommandArgs,
    MaybeResult,
    Module,
    NeovimCtx,
};

/// TODO: docs.
pub trait Command<B: Backend>: 'static {
    /// TODO: docs.
    const NAME: &'static ActionName;

    /// TODO: docs.
    type Module: Module<B>;

    /// TODO: docs.
    type Args: for<'a> TryFrom<CommandArgs<'a>>;

    /// TODO: docs.
    type Docs;

    /// TODO: docs.
    fn call(
        &mut self,
        args: Self::Args,
        ctx: NeovimCtx<'_, B>,
    ) -> impl MaybeResult<()>;

    /// TODO: docs.
    fn docs() -> Self::Docs;
}

impl<Cmd, B> Action<B> for Cmd
where
    Cmd: Command<B>,
    B: Backend,
{
    const NAME: &'static ActionName = Cmd::NAME;

    type Module = Cmd::Module;
    type Args = Cmd::Args;
    type Ctx<'a> = NeovimCtx<'a, B>;
    type Return = ();
    type Docs = Cmd::Docs;

    #[inline]
    fn call(
        &mut self,
        args: Self::Args,
        ctx: Self::Ctx<'_>,
    ) -> impl MaybeResult<Self::Return> {
        Cmd::call(self, args, ctx)
    }

    #[inline]
    fn docs() -> Self::Docs {
        Cmd::docs()
    }
}
