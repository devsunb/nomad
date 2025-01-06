use nvimx2::module::{ApiCtx, Module};
use nvimx2::neovim::{self, Neovim};
use nvimx2::{Name, NeovimCtx, Plugin};

#[neovim::plugin]
fn mad() -> Mad {
    Mad
}

/// TODO: docs.
struct Mad;

impl Plugin<Neovim> for Mad {
    const COMMAND_NAME: Name = "Mad";
}

impl Module<Self, Neovim> for Mad {
    const NAME: Name = "mad";

    type Config = ();

    fn api(&self, ctx: &mut ApiCtx<Self, Self, Neovim>) {
        ctx.with_command(auth::Login::new())
            .with_command(auth::Logout::new())
            .with_command(version::EmitVersion::new())
            .with_constant(version::VERSION);
        // .with_module(collab::Collab::new());
    }

    fn on_new_config(&mut self, _: (), _: &mut NeovimCtx<Neovim>) {}
}
