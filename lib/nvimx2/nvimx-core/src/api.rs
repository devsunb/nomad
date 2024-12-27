//! TODO: docs.

use crate::{Backend, Function, Module, Plugin, notify};

/// TODO: docs.
pub trait Api<P: Plugin<B>, B: Backend>: 'static + Sized {
    /// TODO: docs.
    type ModuleApi<'a, M: Module<B, Plugin = P>>: ModuleApi<M, B>;

    /// TODO: docs.
    fn with_module<M>(&mut self) -> Self::ModuleApi<'_, M>
    where
        M: Module<B, Plugin = P>;
}

/// TODO: docs.
pub trait ModuleApi<M: Module<B>, B: Backend>: Sized {
    /// TODO: docs.
    fn add_function<Fun, Cb, Err>(&mut self, callback: Cb)
    where
        Fun: Function<B, Module = M>,
        Cb: FnMut(Fun::Args) -> Result<Fun::Return, Err> + 'static,
        Err: notify::Error;

    /// TODO: docs.
    fn finish(self);
}
