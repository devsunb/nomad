use core::future::Future;

use serde::de::DeserializeOwned;

use crate::{Api, Context, Editor, ModuleName};

/// TODO: docs.
pub trait Module<E: Editor>: Sized {
    /// TODO: docs.
    const NAME: ModuleName;

    /// TODO: docs.
    type Config: Default + DeserializeOwned;

    /// TODO: docs.
    fn init(ctx: &Context<E>) -> Api<Self, E>;

    /// TODO: docs.
    fn run(&mut self, ctx: &Context<E>) -> impl Future<Output = ()>;
}
