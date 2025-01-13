//! TODO: docs.

use serde::Serialize;
use serde::de::Deserialize;

use crate::backend::{Api, ApiValue, BackgroundExecutor, LocalExecutor};
use crate::module::Module;
use crate::notify::{self, MaybeResult};

/// TODO: docs.
pub trait Backend: 'static + Sized {
    /// TODO: docs.
    type Api: Api<Self>;

    /// TODO: docs.
    type LocalExecutor: LocalExecutor;

    /// TODO: docs.
    type BackgroundExecutor: BackgroundExecutor;

    /// TODO: docs.
    type Emitter<'this>: notify::Emitter;

    /// TODO: docs.
    fn api<M: Module<Self>>(&mut self) -> Self::Api;

    /// TODO: docs.
    fn init() -> Self;

    /// TODO: docs.
    fn emitter(&mut self) -> Self::Emitter<'_>;

    /// TODO: docs.
    fn local_executor(&mut self) -> &mut Self::LocalExecutor;

    /// TODO: docs.
    fn background_executor(&mut self) -> &mut Self::BackgroundExecutor;

    /// TODO: docs.
    fn serialize<T>(
        &mut self,
        value: &T,
    ) -> impl MaybeResult<ApiValue<Self>> + use<Self, T>
    where
        T: ?Sized + Serialize;

    /// TODO: docs.
    fn deserialize<'de, T>(
        &mut self,
        value: ApiValue<Self>,
    ) -> impl MaybeResult<T> + use<Self, T>
    where
        T: Deserialize<'de>;
}
