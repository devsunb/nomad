use core::marker::PhantomData;

use nvimx_action::{Action, IntoModuleName};
use nvimx_common::MaybeResult;
pub use nvimx_ctx::OnBytesArgs;
use nvimx_ctx::{RegisterOnBytesArgs, ShouldDetach, TextBufferCtx};

use crate::Event;

/// TODO: docs.
pub struct OnBytes<A, M> {
    action: A,
    module_name: PhantomData<M>,
}

impl<A, M> Event for OnBytes<A, M>
where
    A: for<'ctx> Action<
        M,
        Args = OnBytesArgs,
        Ctx<'ctx> = TextBufferCtx<'ctx>,
    >,
    A::Return: Into<ShouldDetach>,
    M: IntoModuleName + 'static,
{
    type Ctx<'ctx> = TextBufferCtx<'ctx>;

    fn register(mut self, ctx: Self::Ctx<'_>) {
        let callback = move |args, ctx: TextBufferCtx<'_>| {
            self.action
                .execute(args, ctx)
                .into_result()
                .map(Into::into)
                .map_err(Into::into)
        };
        let args = RegisterOnBytesArgs {
            callback,
            module_name: M::NAME,
            callback_name: Some(A::NAME.as_str()),
        };
        ctx.register_on_bytes(args);
    }
}
