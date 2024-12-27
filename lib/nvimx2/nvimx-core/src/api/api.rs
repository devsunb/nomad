use super::{ApiBuilder, ModuleApi};
use crate::{Backend, Module, Plugin};

/// TODO: docs.
pub trait Api<P: Plugin<B>, B: Backend>: 'static + Sized {
    /// TODO: docs.
    type Builder<'a>: ApiBuilder<Self, P, B>;

    /// TODO: docs.
    type ModuleApi<M: Module<B, Plugin = P>>: ModuleApi<M, B>;
}
