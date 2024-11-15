use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;

use nvimx_common::{oxi, MaybeResult};
use nvimx_ctx::NeovimCtx;
use nvimx_diagnostics::{DiagnosticSource, Level};

use crate::command::Command;
use crate::config::Setup;
use crate::module::Module;
use crate::plugin::Plugin;

/// TODO: docs.
pub struct PluginCtx<P: Plugin> {
    api: oxi::Dictionary,
    command: Command,
    neovim_ctx: NeovimCtx<'static>,
    plugin: PhantomData<P>,
    run: Vec<Pin<Box<dyn Future<Output = ()>>>>,
    setup: Setup,
}

impl<P: Plugin> PluginCtx<P> {
    /// TODO: docs.
    pub fn new() -> Self {
        todo!();
    }

    /// TODO: docs.
    pub fn with_module<M>(mut self, module: M) -> Self
    where
        M: Module<Plugin = Self>,
    {
        let config_rx = self.setup.add_module::<M>();
        let module = M::from(config_rx);
        let module_api = module.init(self.neovim_ctx.reborrow());
        self.api.insert(M::NAME.as_str(), module_api.dictionary);
        self.command.add_module(module_api.commands);
        self.run.push({
            let neovim_ctx = self.neovim_ctx.clone();
            Box::pin(async move {
                if let Err(err) = module.run(neovim_ctx).await.into_result() {
                    let mut source = DiagnosticSource::new();
                    source.push_segment(M::NAME.as_str());
                    err.into().emit(Level::Error, source);
                }
            })
        });
        self
    }
}
