use crate::Backend;

/// TODO: docs.
pub trait Plugin<B: Backend>: 'static {
    /// TODO: docs.
    const NAME: PluginName;

    /// TODO: docs.
    type Docs;

    /// TODO: docs.
    fn api(&self, ctx: PluginCtx<'_, B>) -> PluginApi<Self, B>;

    /// TODO: docs.
    fn docs() -> Self::Docs;
}
