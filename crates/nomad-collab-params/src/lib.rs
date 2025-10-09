//! This crate contains the [`Params`][NomadParams] used by Nomad's collab
//! server running at `collab.nomad.foo`.

mod auth_error;
mod nomad_params;

pub use auth_error::AuthError;
pub use nomad_params::NomadParams;
pub use {auth_types, ulid};
