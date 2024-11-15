use core::marker::PhantomData;

use nvimx_common::oxi;
use nvimx_ctx::NeovimCtx;

use crate::module_subcommands::ModuleSubCommands;
use crate::{Function, Module, SubCommand};

/// TODO: docs.
pub struct ModuleApi<M: Module> {
    pub(crate) dictionary: oxi::Dictionary,
    pub(crate) commands: ModuleSubCommands,
    ty: PhantomData<M>,
}

impl<M: Module> ModuleApi<M> {
    /// TODO: docs.
    pub fn subcommand<T>(mut self, command: T) -> Self
    where
        T: SubCommand<Module = M>,
    {
        self.commands.add_command(command);
        self
    }

    /// TODO: docs.
    pub fn default_command<T>(mut self, command: T) -> Self
    where
        T: SubCommand<Module = M>,
    {
        self.commands.add_default_command(command);
        self
    }

    /// TODO: docs.
    pub fn function<T>(mut self, function: T) -> Self
    where
        T: Function<Module = M>,
    {
        if self.dictionary.get(T::NAME.as_str()).is_some() {
            panic!(
                "a function with the name '{}' has already been added to the \
                 API for module '{}'",
                T::NAME,
                M::NAME,
            );
        }
        let ctx = self.neovim_ctx().to_static();
        let mut callback = function.into_callback();
        self.dictionary.insert(
            T::NAME.as_str(),
            oxi::Function::from_fn_mut(move |obj| {
                callback(obj, ctx.reborrow())
            }),
        );
        self
    }

    /// Creates a new [`ModuleApi`].
    pub fn new(neovim_ctx: NeovimCtx<'static>) -> Self {
        Self {
            dictionary: oxi::Dictionary::default(),
            commands: ModuleSubCommands::new::<M>(neovim_ctx),
            ty: PhantomData,
        }
    }

    fn neovim_ctx(&self) -> NeovimCtx<'_> {
        self.commands.neovim_ctx.reborrow()
    }
}
