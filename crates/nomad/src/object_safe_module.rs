use alloc::rc::Rc;
use core::cell::RefCell;

use neovim::nvim::{Dictionary, Function};
use neovim::{Ctx, SetCtx};

use crate::Module;

/// TODO: docs
pub(crate) trait ObjectSafeModule {
    /// TODO: docs
    fn api(&self, ctx: &Rc<RefCell<Ctx>>) -> Dictionary;

    /// TODO: docs
    fn load(&self, ctx: &mut SetCtx);
}

impl<M: Module> ObjectSafeModule for M {
    #[inline]
    fn api(&self, ctx: &Rc<RefCell<Ctx>>) -> Dictionary {
        let mut dict = Dictionary::new();

        for (action_name, action) in self.api().into_iter() {
            let ctx = Rc::clone(ctx);

            let function = move |object| {
                let ctx = &mut *ctx.borrow_mut();
                action(object, ctx.as_set());
                Ok::<_, core::convert::Infallible>(())
            };

            dict.insert(action_name.as_str(), Function::from_fn(function));
        }

        dict
    }

    #[inline]
    fn load(&self, ctx: &mut SetCtx) {
        futures::executor::block_on(async {
            let _ = self.load(ctx).await;
        });
    }
}
