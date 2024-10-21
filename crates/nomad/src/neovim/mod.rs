//! TODO: docs.

mod api;
mod buffer;
mod command;
mod config;
pub mod events;
mod function;
mod module_api;
mod neovim;
mod offset;
mod spawner;

pub use api::Api;
pub use buffer::{Buffer, BufferId};
pub use command::{command, Command, CommandHandle};
pub use function::{function, Function, FunctionHandle};
pub use module_api::{module_api, ModuleApi};
pub use neovim::Neovim;
use offset::Offset;
pub use spawner::NeovimSpawner;
