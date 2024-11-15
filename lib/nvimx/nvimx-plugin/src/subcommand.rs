use crate::ctx::NeovimCtx;
use crate::diagnostics::{DiagnosticMessage, DiagnosticSource, Level};
use crate::maybe_result::MaybeResult;
use crate::subcommand_args::SubommandArgs;
use crate::{Action, Module};

/// TODO: docs.
pub trait SubCommand:
    for<'ctx> Action<
    NeovimCtx<'ctx>,
    Args: Clone
              + for<'args> TryFrom<
        &'args mut SubommandArgs,
        Error: Into<DiagnosticMessage>,
    >,
    Return = (),
>
{
    /// TODO: docs.
    fn into_callback(
        self,
    ) -> impl for<'ctx> FnMut(SubommandArgs, NeovimCtx<'ctx>) + 'static;
}

impl<T> SubCommand for T
where
    T: for<'ctx> Action<
        NeovimCtx<'ctx>,
        Args: Clone
                  + for<'args> TryFrom<
            &'args mut SubommandArgs,
            Error: Into<DiagnosticMessage>,
        >,
        Return = (),
    >,
{
    fn into_callback(
        mut self,
    ) -> impl for<'ctx> FnMut(SubommandArgs, NeovimCtx<'ctx>) + 'static {
        Box::new(move |mut args, ctx: NeovimCtx<'_>| {
            let args = match T::Args::try_from(&mut args) {
                Ok(args) => args,
                Err(err) => {
                    let mut source = DiagnosticSource::new();
                    source
                        .push_segment(T::Module::NAME.as_str())
                        .push_segment(T::NAME.as_str());
                    err.into().emit(Level::Error, source);
                    return;
                },
            };
            if let Err(err) = self.execute(args, ctx).into_result() {
                let mut source = DiagnosticSource::new();
                source
                    .push_segment(T::Module::NAME.as_str())
                    .push_segment(T::NAME.as_str());
                err.into().emit(Level::Error, source);
            }
        })
    }
}
