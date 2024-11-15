use core::future::Future;

use nvimx_common::MaybeResult;
use nvimx_ctx::NeovimCtx;
use nvimx_diagnostics::{DiagnosticSource, Level};

use crate::{Action, ActionName, Module};

/// TODO: docs
pub trait AsyncAction<M: Module>: 'static {
    /// TODO: docs
    const NAME: ActionName;

    /// TODO: docs
    type Args: 'static;

    /// TODO: docs
    type Docs;

    /// TODO: docs
    fn execute(
        &mut self,
        args: Self::Args,
        ctx: NeovimCtx<'_>,
    ) -> impl Future<Output = impl MaybeResult<()>>;

    /// TODO: docs
    fn docs(&self) -> Self::Docs;
}

impl<M, T> Action<M> for T
where
    M: Module,
    T: AsyncAction<M> + Clone,
{
    const NAME: ActionName = T::NAME;
    type Args = T::Args;
    type Ctx<'a> = NeovimCtx<'a>;
    type Docs = T::Docs;
    type Return = ();

    fn execute<'a>(&'a mut self, args: Self::Args, ctx: NeovimCtx<'a>) {
        let mut this = self.clone();
        ctx.spawn(|ctx| async move {
            if let Err(message) =
                this.execute(args, ctx).await.into_result().map_err(Into::into)
            {
                let mut source = DiagnosticSource::new();
                source
                    .push_segment(M::NAME.as_str())
                    .push_segment(Self::NAME.as_str());
                message.emit(Level::Warning, source);
            }
        })
        .detach();
    }

    fn docs(&self) -> Self::Docs {
        self.docs()
    }
}
