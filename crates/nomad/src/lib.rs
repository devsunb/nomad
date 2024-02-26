//! # Nomad
//!
//! TODO: docs

mod enable;
mod module;
mod module_name;

pub use enable::{DefaultEnable, EnableConfig};
pub use macros::module_name;
pub use module::Module;
pub use module_name::ModuleName;

pub mod prelude {
    //! TODO: docs

    pub use neovim::*;

    pub use super::*;
}
