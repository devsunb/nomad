use fxhash::FxHashMap;
use nvimx_common::MaybeResult;
use nvimx_ctx::NeovimCtx;
use nvimx_diagnostics::{DiagnosticSource, Level};

use crate::action_name::ActionNameStr;
use crate::module::Module;
use crate::module_name::ModuleName;
use crate::subcommand::{CompletionFunc, SubCommand};
use crate::subcommand_args::{SubCommandArgs, SubCommandCursor};

pub(crate) type SubCommandCallback = Box<dyn FnMut(SubCommandArgs)>;

pub(crate) type SubCommandCompletionFunc =
    Box<dyn FnMut(SubCommandArgs, SubCommandCursor) -> Vec<String>>;

pub(crate) struct ModuleSubCommands {
    /// The name of the module these commands belong to.
    pub(crate) module_name: ModuleName,

    /// The command to run when no command is specified.
    pub(crate) default_subcommand: Option<SubCommandCallback>,

    /// Map from command name to the corresponding [`SubCommandCallback`].
    pub(crate) subcommands: FxHashMap<ActionNameStr, SubCommandCallback>,

    /// Map from command name to the corresponding [`SubCommandCompletionFunc`].
    pub(crate) completion_funcs:
        FxHashMap<ActionNameStr, SubCommandCompletionFunc>,

    pub(crate) neovim_ctx: NeovimCtx<'static>,
}

impl ModuleSubCommands {
    #[track_caller]
    pub(crate) fn add_default_subcommand<T>(&mut self, subcommand: T)
    where
        T: SubCommand,
    {
        if self.module_name != T::Module::NAME {
            panic!(
                "trying to register a command for module '{}' in the API for \
                 module '{}'",
                T::Module::NAME,
                self.module_name
            );
        }
        if self.default_subcommand.is_some() {
            panic!(
                "a default command has already been set for module '{}'",
                self.module_name
            );
        }
        self.default_subcommand =
            Some(callback_of_subcommand(subcommand, self.neovim_ctx.clone()));
    }

    #[track_caller]
    pub(crate) fn add_subcommand<T>(&mut self, subcommand: T)
    where
        T: SubCommand,
    {
        if self.module_name != T::Module::NAME {
            panic!(
                "trying to register a command for module '{}' in the API for \
                 module '{}'",
                T::Module::NAME,
                self.module_name
            );
        }
        if self.subcommands.contains_key(&T::NAME.as_str()) {
            panic!(
                "a command with the name '{}' already exists in the API for \
                 module '{}'",
                T::NAME,
                self.module_name
            );
        }
        self.completion_funcs.insert(
            T::NAME.as_str(),
            Box::new({
                let mut func = subcommand.completion_func();
                move |args: SubCommandArgs, cursor: SubCommandCursor| {
                    func.call(args, cursor)
                }
            }),
        );
        self.subcommands.insert(
            T::NAME.as_str(),
            callback_of_subcommand(subcommand, self.neovim_ctx.clone()),
        );
    }

    pub(crate) fn new<M: Module>(neovim_ctx: NeovimCtx<'static>) -> Self {
        Self {
            module_name: M::NAME,
            default_subcommand: None,
            subcommands: FxHashMap::default(),
            completion_funcs: FxHashMap::default(),
            neovim_ctx,
        }
    }
}

fn callback_of_subcommand<T: SubCommand>(
    mut subcommand: T,
    ctx: NeovimCtx<'static>,
) -> SubCommandCallback {
    Box::new(move |args| {
        let args = match T::Args::try_from(args) {
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
        if let Err(err) =
            subcommand.execute(args, ctx.reborrow()).into_result()
        {
            let mut source = DiagnosticSource::new();
            source
                .push_segment(T::Module::NAME.as_str())
                .push_segment(T::NAME.as_str());
            err.into().emit(Level::Error, source);
        }
    })
}
