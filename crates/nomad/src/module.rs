use core::future::Future;
use std::error::Error;

use neovim::{Get, Neovim};
use serde::de::DeserializeOwned;

use crate::{DefaultEnable, EnableConfig, ModuleName};

/// TODO: docs
pub trait Module: DefaultEnable + Sized {
    /// TODO: docs
    const NAME: ModuleName;

    /// TODO: docs
    type Config: Default + DeserializeOwned;

    /// TODO: docs
    type InitError: Error;

    /// TODO: docs
    fn init(
        config: Get<EnableConfig<Self>>,
        nvim: &Neovim,
    ) -> impl Future<Output = Result<Self, Self::InitError>>;
}
