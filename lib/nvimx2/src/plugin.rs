use crate::{Backend, PluginApi};

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
pub struct PluginName(str);
