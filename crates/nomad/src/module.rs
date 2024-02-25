use serde::de::DeserializeOwned;

use crate::ModuleName;

/// TODO: docs
pub trait Module {
    /// TODO: docs
    const NAME: ModuleName;

    /// TODO: docs
    type Config: Default + DeserializeOwned;
}
