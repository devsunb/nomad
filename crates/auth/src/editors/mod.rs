#[cfg(feature = "mock")]
pub mod mock;
#[cfg(feature = "neovim")]
mod neovim;

use core::fmt::Debug;

use auth_types::AuthInfos;
use editor::{Borrowed, Context, Editor, notify};

/// TODO: docs.
pub trait AuthEditor: Editor {
    /// TODO: docs.
    type LoginError: Debug + notify::Error;

    /// TODO: docs.
    fn credential_builder(
        ctx: &mut Context<Self, Borrowed>,
    ) -> impl Future<Output = Box<keyring::CredentialBuilder>> + Send + 'static;

    /// TODO: docs.
    fn login(
        ctx: &mut Context<Self>,
    ) -> impl Future<Output = Result<AuthInfos, Self::LoginError>>;
}
