use core::future::Future;

use serde::de::DeserializeOwned;

use crate::{Api, Context, Editor, ModuleName};

/// TODO: docs.
pub trait Module: Sized {
    /// TODO: docs.
    const NAME: ModuleName;

    /// TODO: docs.
    type Config: Default + DeserializeOwned;

    /// TODO: docs.
    fn init<E: Editor>(ctx: &Context<E>) -> Api<Self>;

    /// TODO: docs.
    fn run<E: Editor>(&mut self, ctx: &Context<E>)
        -> impl Future<Output = ()>;
}
