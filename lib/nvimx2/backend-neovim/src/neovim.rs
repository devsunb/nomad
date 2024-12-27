use core::marker::PhantomData;

use nvimx_core::api::Api;
use nvimx_core::{Backend, Plugin};

use crate::{
    NeovimBackgroundExecutor,
    NeovimLocalExecutor,
    NeovimVersion,
    api,
    notify,
};

/// TODO: docs.
pub struct Neovim<V: NeovimVersion> {
    emitter: notify::NeovimEmitter,
    version: PhantomData<V>,
}

impl<V: NeovimVersion> Backend for Neovim<V> {
    type Api<P: Plugin<Self>> = api::NeovimApi<P, V>;
    type LocalExecutor = NeovimLocalExecutor;
    type BackgroundExecutor = NeovimBackgroundExecutor;
    type Emitter<'a> = &'a mut notify::NeovimEmitter;

    #[inline]
    fn init() -> Self {
        Self {
            emitter: notify::NeovimEmitter::default(),
            version: PhantomData,
        }
    }

    #[inline]
    fn api_builder<P: Plugin<Self>>(
        &mut self,
    ) -> <Self::Api<P> as Api<P, Self>>::Builder<'_> {
        api::NeovimApi::<P, V>::default()
    }

    #[inline]
    fn emitter(&mut self) -> Self::Emitter<'_> {
        &mut self.emitter
    }
}
