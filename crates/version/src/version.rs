use nomad::config::ConfigReceiver;
use nomad::ctx::NeovimCtx;
use nomad::{
    action_name,
    module_name,
    Action,
    ActionName,
    Module,
    ModuleApi,
    ModuleName,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// TODO: docs.
#[derive(Copy, Clone)]
pub struct Version;

impl Module for Version {
    const NAME: ModuleName = module_name!("version");

    type Config = ();

    fn init(&self, ctx: NeovimCtx<'_>) -> ModuleApi<Self> {
        ModuleApi::new(ctx.to_static()).default_command(Self)
    }

    async fn run(self, _: NeovimCtx<'static>) {}
}

impl<'a> Action<NeovimCtx<'a>> for Version {
    const NAME: ActionName = action_name!("version");

    type Args = ();
    type Docs = ();
    type Module = Self;
    type Return = ();

    fn execute(&mut self, _: Self::Args, _: NeovimCtx<'a>) {
        nomad::nvim_oxi::print!("Nomad v{VERSION}");
    }

    fn docs(&self) -> Self::Docs {}
}

impl From<ConfigReceiver<Self>> for Version {
    fn from(_: ConfigReceiver<Self>) -> Self {
        Self {}
    }
}
