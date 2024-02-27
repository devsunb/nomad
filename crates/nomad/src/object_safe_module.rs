use alloc::rc::Rc;
use core::cell::RefCell;

use neovim::nvim::Dictionary;
use neovim::Ctx;

use crate::Module;

/// TODO: docs
pub(crate) trait ObjectSafeModule {
    /// TODO: docs
    fn api(this: &Rc<Self>, ctx: &Rc<RefCell<Ctx>>) -> Dictionary;
}

impl<M: Module> ObjectSafeModule for M {
    #[inline]
    fn api(this: &Rc<Self>, ctx: &Rc<RefCell<Ctx>>) -> Dictionary {
        let api = this.api();
        todo!();
    }
}
