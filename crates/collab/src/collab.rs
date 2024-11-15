use nvimx::ctx::NeovimCtx;
use nvimx::plugin::{
    module_name,
    ConfigReceiver,
    Module,
    ModuleApi,
    ModuleName,
};
use nvimx::Shared;

use crate::actions::{Join, Start, Yank};
use crate::config::Config;
use crate::session_status::SessionStatus;

/// TODO: docs.
pub struct Collab {
    config: Config,
    config_rx: ConfigReceiver<Self>,
    session_status: Shared<SessionStatus>,
}

impl Module for Collab {
    const NAME: ModuleName = module_name!("collab");

    type Config = Config;
    type Plugin = nomad::Nomad;

    fn init(&self, ctx: NeovimCtx<'_>) -> ModuleApi<Self> {
        let join = Join::new(self.session_status.clone());
        let start = Start::new(self.session_status.clone());
        let yank = Yank::new(self.session_status.clone());

        ModuleApi::new(ctx.to_static())
            .subcommand(join.clone())
            .subcommand(start.clone())
            .subcommand(yank.clone())
            .function(join)
            .function(start)
            .function(yank)
    }

    async fn run(mut self, _: NeovimCtx<'static>) {
        loop {
            self.config = self.config_rx.recv().await;
        }
    }
}

impl From<ConfigReceiver<Self>> for Collab {
    fn from(config_rx: ConfigReceiver<Self>) -> Self {
        Self {
            config: Config::default(),
            config_rx,
            session_status: Shared::default(),
        }
    }
}
