use core::convert::Infallible;

use nomad::prelude::*;

use crate::CollabConfig;

/// TODO: docs.
pub struct Collab {}

impl DefaultEnable for Collab {
    const ENABLE: bool = false;
}

impl Module for Collab {
    const NAME: ModuleName = module_name!("collab");

    type Config = CollabConfig;

    type InitError = Infallible;

    async fn init(
        _config: Get<EnableConfig<Self>>,
        _nvim: &Neovim,
    ) -> Result<Self, Self::InitError> {
        todo!();
    }
}
