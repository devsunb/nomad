use nvimx_core::{Backend, Plugin};

use crate::{api, executor, notify, oxi, serde};

/// TODO: docs.
pub struct Neovim {
    emitter: notify::NeovimEmitter,
}

impl Backend for Neovim {
    type Api<P: Plugin<Self>> = api::NeovimApi<P>;
    type ApiValue = oxi::Object;
    type LocalExecutor = executor::NeovimLocalExecutor;
    type BackgroundExecutor = executor::NeovimBackgroundExecutor;
    type Emitter<'a> = &'a mut notify::NeovimEmitter;
    type Serializer = serde::NeovimSerializer;
    type Deserializer<'de> = serde::NeovimDeserializer;

    #[inline]
    fn init() -> Self {
        Self { emitter: notify::NeovimEmitter::default() }
    }

    #[inline]
    fn api<P: Plugin<Self>>(&mut self) -> Self::Api<P> {
        api::NeovimApi::default()
    }

    #[inline]
    fn emitter(&mut self) -> Self::Emitter<'_> {
        &mut self.emitter
    }

    #[inline]
    fn serializer(&mut self) -> Self::Serializer {
        serde::NeovimSerializer::default()
    }

    #[inline]
    fn deserializer<'de>(
        &mut self,
        value: oxi::Object,
    ) -> Self::Deserializer<'de> {
        serde::NeovimDeserializer::new(value)
    }
}
