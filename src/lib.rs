use nvimx2::module::{ApiCtx, Module, ModuleName};
use nvimx2::neovim::{self, Neovim};
use nvimx2::{ActionName, NeovimCtx, Plugin};

#[neovim::plugin]
fn mad() -> Mad {
    Mad
}

/// TODO: docs.
struct Mad;

impl Plugin<Neovim> for Mad {
    const COMMAND_NAME: ActionName = ActionName::new("Mad");
}

impl Module<Self, Neovim> for Mad {
    const NAME: ModuleName = ModuleName::new("mad");

    type Config = ();

    fn api(&self, _ctx: &mut ApiCtx<Self, Self, Neovim>) {
        // ctx.with_module(auth::Auth::new())
        //     .with_module(collab::Collab::new())
        //     .with_constant(version::VERSION)
        //     .with_command(version::PrintVersion::new())
    }

    fn on_new_config(&mut self, _: (), _: &mut NeovimCtx<Neovim>) {}
}
