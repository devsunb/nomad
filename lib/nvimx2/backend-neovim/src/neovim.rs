use core::marker::PhantomData;

use nvimx_core::{Backend, Plugin, PluginApi};

use crate::{
    oxi,
    NeovimBackgroundExecutor,
    NeovimLocalExecutor,
    NeovimVersion,
};

/// TODO: docs.
pub struct Neovim<V: NeovimVersion> {
    version: PhantomData<V>,
}

impl<V: NeovimVersion> Backend for Neovim<V> {
    type Api<P: Plugin<Self>> = oxi::Dictionary;
    type LocalExecutor = NeovimLocalExecutor;
    type BackgroundExecutor = NeovimBackgroundExecutor;

    #[inline]
    fn init() -> Self {
        todo!();
    }

    #[inline]
    fn to_backend_api<P>(
        &mut self,
        _plugin_api: PluginApi<P, Self>,
    ) -> Self::Api<P>
    where
        P: Plugin<Self>,
    {
        todo!();
    }
}
