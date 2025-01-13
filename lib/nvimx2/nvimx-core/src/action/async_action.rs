use crate::AsyncCtx;
use crate::action::{Action, ActionCtx};
use crate::backend::{Backend, BackendExt};
use crate::module::Module;
use crate::notify::{self, MaybeResult, Name};

/// TODO: docs.
pub trait AsyncAction<M, B>: 'static
where
    M: Module<B>,
    B: Backend,
{
    /// TODO: docs.
    const NAME: Name;

    /// TODO: docs.
    type Args;

    /// TODO: docs.
    fn call<'this>(
        &'this mut self,
        args: Self::Args,
        ctx: &mut AsyncCtx<M, B>,
    ) -> impl Future<Output = impl MaybeResult<()> + 'this>;
}

impl<T, M, B> Action<M, B> for T
where
    T: AsyncAction<M, B> + Clone,
    M: Module<B>,
    B: Backend,
{
    const NAME: Name = T::NAME;
    type Args<'args> = T::Args;
    type Return = ();

    #[inline]
    fn call<'s: 's, 'a: 'a>(
        &mut self,
        args: Self::Args<'_>,
        ctx: &mut ActionCtx<M, B>,
    ) {
        let mut this = self.clone();
        let module_path = ctx.module_path().clone();
        ctx.spawn_local(async move |ctx| {
            if let Err(err) = this.call(args, ctx).await.into_result() {
                ctx.with_ctx(move |ctx| {
                    ctx.backend_mut().emit_err(
                        notify::Source {
                            module_path: &module_path,
                            action_name: Some(Self::NAME),
                        },
                        err,
                    );
                });
            }
        });
    }
}
