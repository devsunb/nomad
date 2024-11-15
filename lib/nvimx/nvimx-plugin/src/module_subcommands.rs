use fxhash::FxHashMap;
use nvimx_action::ActionNameStr;
use nvimx_ctx::NeovimCtx;

use crate::module::Module;
use crate::module_name::ModuleName;
use crate::subcommand::SubCommand;
use crate::subcommand_args::SubCommandArgs;

pub(super) struct ModuleSubCommands {
    /// The name of the module these commands belong to.
    pub(super) module_name: ModuleName,

    /// The command to run when no command is specified.
    pub(super) default_command: Option<Box<dyn FnMut(SubCommandArgs)>>,

    /// Map from command name to the corresponding [`Command`].
    pub(super) subcommands:
        FxHashMap<ActionNameStr, Box<dyn FnMut(SubCommandArgs)>>,

    pub(super) neovim_ctx: NeovimCtx<'static>,
}

impl ModuleSubCommands {
    #[track_caller]
    pub(crate) fn add_subcommand<T: SubCommand>(&mut self, command: T) {
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
        let mut callback = command.into_callback();
        let ctx = self.neovim_ctx.clone();
        self.subcommands.insert(
            T::NAME.as_str(),
            Box::new(move |args| callback(args, ctx.reborrow())),
        );
    }

    #[track_caller]
    pub(crate) fn add_default_subcommand<T: SubCommand>(
        &mut self,
        command: T,
    ) {
        if self.module_name != T::Module::NAME {
            panic!(
                "trying to register a command for module '{}' in the API for \
                 module '{}'",
                T::Module::NAME,
                self.module_name
            );
        }
        if self.default_command.is_some() {
            panic!(
                "a default command has already been set for module '{}'",
                self.module_name
            );
        }
        let mut callback = command.into_callback();
        let ctx = self.neovim_ctx.clone();
        self.default_command =
            Some(Box::new(move |args| callback(args, ctx.reborrow())));
    }

    pub(crate) fn default_subcommand(
        &mut self,
    ) -> Option<&mut impl FnMut(SubCommandArgs)> {
        self.default_command.as_mut()
    }

    pub(crate) fn subcommand<'a>(
        &'a mut self,
        subcommand_name: &'a str,
    ) -> Option<&'a mut impl FnMut(SubCommandArgs)> {
        self.subcommands.get_mut(subcommand_name)
    }

    pub(crate) fn subcommand_names(
        &self,
    ) -> impl Iterator<Item = ActionNameStr> + '_ {
        self.subcommands.keys().copied()
    }

    pub(crate) fn new<M: Module>(neovim_ctx: NeovimCtx<'static>) -> Self {
        Self {
            module_name: M::NAME,
            default_command: None,
            subcommands: FxHashMap::default(),
            neovim_ctx,
        }
    }
}
