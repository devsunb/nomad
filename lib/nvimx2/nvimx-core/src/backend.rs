use crate::api::Api;
use crate::executor::{BackgroundExecutor, LocalExecutor};
use crate::{Plugin, notify};

/// TODO: docs.
pub trait Backend: 'static + Sized {
    /// TODO: docs.
    type Api<P: Plugin<Self>>: Api<P, Self>;

    /// TODO: docs.
    type LocalExecutor: LocalExecutor;

    /// TODO: docs.
    type BackgroundExecutor: BackgroundExecutor;

    /// TODO: docs.
    type Emitter<'a>: notify::Emitter;

    /// TODO: docs.
    fn api_builder<P: Plugin<Self>>(
        &mut self,
    ) -> <Self::Api<P> as Api<P, Self>>::Builder<'_>;

    /// TODO: docs.
    fn init() -> Self;

    /// TODO: docs.
    fn emitter(&mut self) -> Self::Emitter<'_>;
}
