//! TODO: docs.

use crate::ByteOffset;
use crate::backend::{Backend, Value};
use crate::command::{CommandArgs, CommandCompletion};
use crate::module::Module;
use crate::notify::Name;
use crate::plugin::Plugin;

/// TODO: docs.
pub trait Api<B: Backend>: 'static + Sized {
    /// TODO: docs.
    type Value: Value<B>;

    /// TODO: docs.
    fn add_constant(&mut self, constant_name: Name, value: Self::Value);

    /// TODO: docs.
    fn add_function<Fun>(&mut self, function_name: Name, function: Fun)
    where
        Fun: FnMut(Self::Value) -> Option<Self::Value> + 'static;

    /// TODO: docs.
    fn add_submodule<M>(&mut self, module_api: B::Api)
    where
        M: Module<B>;

    /// TODO: docs.
    fn add_command<P, Command, CompletionFn, Completions>(
        &mut self,
        command: Command,
        completion_fn: CompletionFn,
    ) where
        Command: FnMut(CommandArgs) + 'static,
        CompletionFn: FnMut(CommandArgs, ByteOffset) -> Completions + 'static,
        Completions: IntoIterator<Item = CommandCompletion>,
        P: Plugin<B>;
}
