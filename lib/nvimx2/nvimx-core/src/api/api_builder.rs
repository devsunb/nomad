use super::{Api, ModuleApi};
use crate::{Backend, Module, Plugin};

/// TODO: docs.
pub trait ApiBuilder<A: Api<P, B>, P: Plugin<B>, B: Backend> {
    /// TODO: docs.
    fn add_module<M>(&mut self, module_api: A::ModuleApi<M>)
    where
        M: Module<B, Plugin = P>;

    /// TODO: docs.
    fn module_builder<M>(
        &mut self,
    ) -> <A::ModuleApi<M> as ModuleApi<M, B>>::Builder<'_>
    where
        M: Module<B, Plugin = P>;

    /// TODO: docs.
    fn build(self) -> A;
}
