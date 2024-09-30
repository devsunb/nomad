use core::future::Future;
use core::pin::Pin;

use crate::{Context, Editor, JoinHandle, Module, Spawner};

/// TODO: docs.
pub struct Nomad<E> {
    api: E::Api,
    run: Vec<Pin<Box<dyn Future<Output = ()> + Send>>>,
    ctx: Context<E>,
}

impl<E: Editor> Nomad<E> {
    /// TODO: docs.
    #[inline]
    pub fn into_api(self) -> E::Api {
        self.api
    }

    /// TODO: docs.
    #[inline]
    pub fn new(editor: E) -> Self {
        Self {
            api: E::Api::default(),
            run: Vec::default(),
            ctx: Context::new(editor),
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn start(&mut self) {
        for fut in self.run.drain(..) {
            self.ctx.spawner().spawn(fut).detach();
        }
    }

    /// TODO: docs.
    #[inline]
    pub fn with_module<M: Module<E>>(mut self, module: M) -> Self {
        let module_api = M::init(&self.ctx);
        self.api += module_api;
        self.run.push({
            let ctx = self.ctx.clone();
            Box::pin(async move { module.run(&ctx).await })
        });
        self
    }
}
