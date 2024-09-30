use nvim_oxi::Dictionary as NvimDictionary;

use super::{FunctionHandle, Neovim};
use crate::Module;

/// TODO: docs.
pub struct ModuleApi<M: Module<Neovim>> {
    pub(super) dict: NvimDictionary,
    pub(super) module: M,
}

impl<M: Module<Neovim>> ModuleApi<M> {
    /// TODO: docs.
    #[inline]
    pub fn new(module: M) -> Self {
        Self { dict: NvimDictionary::default(), module }
    }

    /// TODO: docs.
    #[inline]
    pub fn with_function(mut self, function: FunctionHandle) -> Self {
        self.dict.insert(function.name, function.inner);
        self
    }
}
