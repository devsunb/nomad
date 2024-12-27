use super::ModuleApi;
use crate::{Backend, Module};

/// TODO: docs.
pub trait ModuleApiBuilder<MA: ModuleApi<M, B>, M: Module<B>, B: Backend> {
    /// TODO: docs.
    fn build(self) -> MA;
}
