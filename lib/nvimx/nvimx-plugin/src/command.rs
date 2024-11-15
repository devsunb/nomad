use core::marker::PhantomData;

use fxhash::FxHashMap;
use nvimx_action::ActionNameStr;
use nvimx_common::oxi::api;
use nvimx_diagnostics::{
    DiagnosticMessage,
    DiagnosticSource,
    HighlightGroup,
    Level,
};

use crate::module_name::{ModuleName, ModuleNameStr};
use crate::module_subcommands::ModuleSubCommands;
use crate::plugin::Plugin;
use crate::subcommand_args::SubCommandArgs;

pub(crate) struct Command<P> {
    /// A map from module name to the subcommands for that module.
    subcommands: FxHashMap<ModuleNameStr, ModuleSubCommands>,
    plugin: PhantomData<P>,
}

impl<P: Plugin> Command<P> {
    pub(crate) fn add_module(&mut self, module_commands: ModuleSubCommands) {
        let module_name = module_commands.module_name.as_str();
        if self.subcommands.contains_key(&module_name) {
            panic!(
                "subcommands from a module named '{}' have already been added",
                module_name
            );
        }
        self.subcommands.insert(module_name, module_commands);
    }

    pub(crate) fn create(mut self) {
        let opts = api::opts::CreateCommandOpts::builder()
            .nargs(api::types::CommandNArgs::Any)
            .build();

        api::create_user_command(
            P::COMMAND_NAME,
            move |args| {
                let args = SubCommandArgs::from(args);
                if let Err(err) = self.call(args) {
                    err.emit()
                }
            },
            &opts,
        )
        .expect("all the arguments are valid");
    }

    fn call(&mut self, mut args: SubCommandArgs) -> Result<(), CommandError> {
        let Some(module_name) = args.pop_front() else {
            return Err(CommandError::MissingModule {
                valid: self.subcommands.keys().copied().collect(),
            });
        };

        let Some(module_commands) =
            self.subcommands.get_mut(module_name.as_str())
        else {
            return Err(CommandError::UnknownModule {
                module_name,
                valid: self.subcommands.keys().copied().collect(),
            });
        };

        let Some(command_name) = args.pop_front() else {
            return if let Some(default) = module_commands.default_subcommand()
            {
                default(args);
                Ok(())
            } else {
                Err(CommandError::MissingCommand {
                    module_name: module_commands.module_name,
                    valid: module_commands.subcommand_names().collect(),
                })
            };
        };

        match module_commands.subcommand(command_name.as_str()) {
            Some(command) => {
                command(args);
                Ok(())
            },
            None => Err(CommandError::UnknownCommand {
                module_name: module_commands.module_name,
                command_name,
                valid: module_commands.subcommand_names().collect(),
            }),
        }
    }
}

/// The type of error that can occur when [`call`](NomadCommand::call)ing the
/// [`NomadCommand`].
enum CommandError {
    /// TODO: docs.
    MissingCommand { module_name: ModuleName, valid: Vec<ActionNameStr> },

    /// TODO: docs.
    MissingModule { valid: Vec<ModuleNameStr> },

    /// TODO: docs.
    UnknownCommand {
        module_name: ModuleName,
        command_name: String,
        valid: Vec<ActionNameStr>,
    },

    /// TODO: docs.
    UnknownModule { module_name: String, valid: Vec<ModuleNameStr> },
}

impl CommandError {
    fn emit(self) {
        self.message().emit(Level::Warning, self.source());
    }

    fn message(&self) -> DiagnosticMessage {
        let mut message = DiagnosticMessage::new();
        match self {
            Self::MissingCommand { valid, .. } => {
                message
                    .push_str("missing command, the valid commands are: ")
                    .push_comma_separated(valid, HighlightGroup::special());
            },
            Self::MissingModule { valid } => {
                message
                    .push_str("missing module, the valid modules are: ")
                    .push_comma_separated(valid, HighlightGroup::special());
            },

            Self::UnknownCommand { command_name, valid, .. } => {
                message
                    .push_str("unknown command '")
                    .push_str_highlighted(
                        command_name,
                        HighlightGroup::warning(),
                    )
                    .push_str("', the valid commands are: ")
                    .push_comma_separated(valid, HighlightGroup::special());
            },
            Self::UnknownModule { module_name, valid } => {
                message
                    .push_str("unknown module '")
                    .push_str_highlighted(
                        module_name,
                        HighlightGroup::warning(),
                    )
                    .push_str("', the valid modules are: ")
                    .push_comma_separated(valid, HighlightGroup::special());
            },
        }
        message
    }

    fn source(&self) -> DiagnosticSource {
        let mut source = DiagnosticSource::new();
        match self {
            Self::UnknownCommand { module_name, .. }
            | Self::MissingCommand { module_name, .. } => {
                source.push_segment(module_name.as_str());
            },
            _ => (),
        }
        source
    }
}
