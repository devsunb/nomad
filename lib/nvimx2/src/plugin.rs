use crate::{Backend, NeovimCtx, PluginApi};

/// TODO: docs.
pub trait Plugin<B: Backend>: 'static + Sized {
    /// TODO: docs.
    const NAME: &'static PluginName;

    /// TODO: docs.
    type Docs;

    /// TODO: docs.
    fn api(&self, ctx: PluginCtx<'_, B>) -> PluginApi<Self, B>;

    /// TODO: docs.
    fn docs() -> Self::Docs;
}

/// TODO: docs.
pub struct PluginCtx<'a, B> {
    neovim_ctx: NeovimCtx<'a, B>,
}

/// TODO: docs.
pub struct PluginName(str);
