//! TODO: docs.

use core::marker::PhantomData;

use nvimx_core::api::{Api, ApiBuilder, ModuleApi, ModuleApiBuilder};
use nvimx_core::{Module, Plugin};

use crate::{Neovim, NeovimVersion};

/// TODO: docs.
pub struct NeovimApi<P, V> {
    _phantom: PhantomData<(P, V)>,
}

/// TODO: docs.
pub struct NeovimModuleApi<M, V> {
    _phantom: PhantomData<(M, V)>,
}

impl<P, V> Api<P, Neovim<V>> for NeovimApi<P, V>
where
    P: Plugin<Neovim<V>>,
    V: NeovimVersion,
{
    type Builder<'a> = Self;
    type ModuleApi<M: Module<Neovim<V>, Plugin = P>> = NeovimModuleApi<M, V>;
}

impl<P, V> ApiBuilder<NeovimApi<P, V>, P, Neovim<V>> for NeovimApi<P, V>
where
    P: Plugin<Neovim<V>>,
    V: NeovimVersion,
{
    #[inline]
    fn add_module<M>(&mut self, _module_api: NeovimModuleApi<M, V>)
    where
        M: Module<Neovim<V>, Plugin = P>,
    {
        todo!();
    }

    #[inline]
    fn module_builder<M>(&mut self) -> &mut NeovimModuleApi<M, V>
    where
        M: Module<Neovim<V>, Plugin = P>,
    {
        todo!();
    }

    #[inline]
    fn build(self) -> NeovimApi<P, V> {
        self
    }
}

impl<M, V> ModuleApi<M, Neovim<V>> for NeovimModuleApi<M, V>
where
    M: Module<Neovim<V>>,
    V: NeovimVersion,
{
    type Builder<'a> = &'a mut Self;
}

impl<M, V> ModuleApiBuilder<NeovimModuleApi<M, V>, M, Neovim<V>>
    for &mut NeovimModuleApi<M, V>
where
    M: Module<Neovim<V>>,
    V: NeovimVersion,
{
    #[inline]
    fn build(self) -> NeovimModuleApi<M, V> {
        todo!();
    }
}

impl<P, V> Default for NeovimApi<P, V>
where
    P: Plugin<Neovim<V>>,
    V: NeovimVersion,
{
    #[inline]
    fn default() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<P, V> From<NeovimApi<P, V>> for crate::oxi::Dictionary
where
    P: Plugin<Neovim<V>>,
    V: NeovimVersion,
{
    #[inline]
    fn from(_api: NeovimApi<P, V>) -> Self {
        todo!();
    }
}
