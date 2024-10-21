use crate::command_args::CommandArgs;
use crate::diagnostics::{DiagnosticMessage, DiagnosticSource, Level};
use crate::maybe_result::MaybeResult;
use crate::{Action, Module};

/// TODO: docs.
pub trait Command:
    Action<
    Args: Clone
              + for<'a> TryFrom<
        &'a mut CommandArgs,
        Error: Into<DiagnosticMessage>,
    >,
    Return = (),
>
{
    fn into_function(self) -> Box<dyn Fn(CommandArgs)>;
}

impl<T> Command for T
where
    T: Action<
        Args: Clone
                  + for<'a> TryFrom<
            &'a mut CommandArgs,
            Error: Into<DiagnosticMessage>,
        >,
        Return = (),
    >,
{
    fn into_function(self) -> Box<dyn Fn(CommandArgs)> {
        Box::new(move |mut args| {
            let args = match T::Args::try_from(&mut args) {
                Ok(args) => args,
                Err(err) => {
                    let mut source = DiagnosticSource::new();
                    source.push_segment(T::Module::NAME).push_segment(T::NAME);
                    err.into().emit(Level::Error, source);
                    return;
                },
            };
            if let Err(err) = self.execute(args).into_result() {
                let mut source = DiagnosticSource::new();
                source.push_segment(T::Module::NAME).push_segment(T::NAME);
                err.into().emit(Level::Error, source);
            }
        })
    }
}
