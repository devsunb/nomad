use std::collections::HashMap;

use nvim_oxi::Dictionary as NvimDictionary;

use super::command::OnExecute;
use super::config::OnConfigChange;
use super::events::ConfigEvent;
use super::{CommandHandle, FunctionHandle, Neovim};
use crate::{Context, Module, Shared, Subscription};

/// TODO: docs.
pub fn module_api<M: Module>(
    ctx: &Context<Neovim>,
) -> (ModuleApi, Subscription<ConfigEvent<M>, Neovim>) {
    let buf = Shared::new(None);
    let event = ConfigEvent::<M>::new(buf.clone());
    let sub = ctx.subscribe(event);
    let api = ModuleApi {
        name: M::NAME.as_str(),
        commands: ModuleCommands::new(M::NAME.as_str()),
        on_config_change: buf
            .with_mut(Option::take)
            .expect("just set when subscribing"),
        inner: NvimDictionary::default(),
    };
    (api, sub)
}

/// TODO: docs.
pub struct ModuleApi {
    pub(super) name: &'static str,
    pub(super) commands: ModuleCommands,
    pub(super) inner: NvimDictionary,
    pub(super) on_config_change: OnConfigChange,
}

impl ModuleApi {
    /// TODO: docs.
    #[track_caller]
    pub fn with_command(mut self, command: CommandHandle) -> Self {
        self.commands.add_command(command);
        self
    }

    /// TODO: docs.
    #[track_caller]
    pub fn with_default_command(mut self, command: CommandHandle) -> Self {
        self.commands.add_default_command(command);
        self
    }

    /// TODO: docs.
    #[track_caller]
    pub fn with_function(mut self, function: FunctionHandle) -> Self {
        if self.name != function.module_name {
            panic!(
                "trying to register a function for module '{}' in the API \
                 for module '{}'",
                function.module_name, self.name
            );
        }

        if self.inner.get(function.name).is_some() {
            panic!(
                "a function with the name '{}' already exists in the API for \
                 modulle '{}'",
                function.name, self.name
            );
        }

        self.inner.insert(function.name, function.inner);
        self
    }
}
