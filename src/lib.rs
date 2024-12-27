use neovim::{Neovim, NeovimApi};
use nvimx2::{Plugin, PluginApiCtx, PluginName, neovim};

#[neovim::plugin]
fn mad() -> Mad {
    Mad
}

/// TODO: docs.
struct Mad;

impl Plugin<Neovim> for Mad {
    const NAME: &'static PluginName = PluginName::new("mad");

    type Docs = ();

    fn api(&self, _ctx: PluginApiCtx<'_, Self, Neovim>) -> NeovimApi<Self> {
        // ctx.with_module(auth::Auth::new())
        //     .with_module(collab::Collab::new())
        //     .with_module(version::Version::new())
        //     .into_api()
        todo!()
    }

    fn docs() {}
}
