use nvimx2::NeovimCtx;
use nvimx2::backend::Backend;
use nvimx2::module::{ApiCtx, Module};
use nvimx2::notify::Name;

/// TODO: docs.
#[derive(Default)]
pub struct Collab {}

impl<B: Backend> Module<B> for Collab {
    const NAME: Name = "collab";

    type Config = crate::config::Config;

    fn api(&self, _: &mut ApiCtx<B>) {}

    fn on_new_config(&self, _: Self::Config, _: &mut NeovimCtx<B>) {}
}
