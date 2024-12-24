use serde::de::DeserializeOwned;

use crate::{Backend, NeovimCtx, Plugin};

/// TODO: docs.
pub trait Module<B: Backend>: 'static {
    /// TODO: docs.
    const NAME: ModuleName;

    /// TODO: docs.
    type Plugin: Plugin<B>;

    /// TODO: docs.
    type Config: DeserializeOwned;

    /// TODO: docs.
    type Docs;

    /// TODO: docs.
    fn api(&self, ctx: ModuleCtx<'_, B>) -> ModuleApi<Self, B>;

    /// TODO: docs.
    fn on_config_changed(
        &mut self,
        new_config: Self::Config,
        ctx: NeovimCtx<'_, B>,
    );

    /// TODO: docs.
    fn docs() -> Self::Docs;
}
