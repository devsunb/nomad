use crate::action::Action;
use crate::notify::{MaybeResult, Name};
use crate::{Borrowed, Context, Editor};

/// TODO: docs.
pub trait AsyncAction<Ed: Editor>: 'static {
    /// TODO: docs.
    const NAME: Name;

    /// TODO: docs.
    type Args;

    /// TODO: docs.
    fn call<'this>(
        &'this mut self,
        args: Self::Args,
        ctx: &mut Context<Ed>,
    ) -> impl Future<Output = impl MaybeResult<()> + 'this>;
}

impl<T, Ed> Action<Ed> for T
where
    T: AsyncAction<Ed> + Clone,
    Ed: Editor,
{
    const NAME: Name = T::NAME;

    type Args<'args> = T::Args;
    type Return = ();

    #[inline]
    fn call<'s: 's, 'a: 'a>(
        &mut self,
        args: Self::Args<'_>,
        ctx: &mut Context<Ed, Borrowed<'_>>,
    ) {
        let mut this = self.clone();
        ctx.spawn_and_detach(async move |ctx| {
            if let Err(err) = this.call(args, ctx).await.into_result() {
                ctx.emit_err(err);
            }
        });
    }
}
