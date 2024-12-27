use super::ModuleApiBuilder;
use crate::{Backend, Module};

/// TODO: docs.
pub trait ModuleApi<M: Module<B>, B: Backend>: 'static + Sized {
    /// TODO: docs.
    type Builder<'a>: ModuleApiBuilder<Self, M, B>;
}
