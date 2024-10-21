use fxhash::FxHashMap;
use nvim_oxi::api;

use crate::command_args::CommandArgs;
use crate::module_commands::ModuleCommands;
use crate::ModuleName;

#[derive(Default)]
pub(crate) struct NomadCommand {
    /// A map from module name to the commands for that module.
    commands: FxHashMap<ModuleName, ModuleCommands>,
}

impl NomadCommand {
    const NAME: &'static str = "Mad";

    #[track_caller]
    pub(crate) fn add_module(&mut self, module_commands: ModuleCommands) {
        if self.commands.contains_key(&module_commands.module_name) {
            panic!(
                "commands from a module named '{}' have already been added",
                module_commands.module_name
            );
        }
        self.commands.insert(module_commands.module_name, module_commands);
    }

    pub(crate) fn create(self) {
        let opts = api::opts::CreateCommandOpts::builder()
            .nargs(api::types::CommandNArgs::Any)
            .build();

        api::create_user_command(
            Self::NAME,
            move |args| {
                let args = CommandArgs::from(args);
                if let Err(err) = self.call(args) {
                    err.emit()
                }
            },
            &opts,
        )
        .expect("all the arguments are valid");
    }

    fn call(&self, args: CommandArgs) -> Result<(), NomadCommandError> {
        todo!();
    }
}

/// The type of error that can occur when [`call`](NomadCommand::call)ing the
/// [`NomadCommand`].
enum NomadCommandError {}

impl NomadCommandError {
    fn emit(self) {}
}
