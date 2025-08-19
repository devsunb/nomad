use auth_types::AuthInfos;
use editor::{Borrowed, Context};
use neovim::Neovim;

use crate::AuthEditor;

impl AuthEditor for Neovim {
    type LoginError = core::convert::Infallible;

    #[allow(clippy::manual_async_fn)]
    fn credential_builder(
        _: &mut Context<Self, Borrowed>,
    ) -> impl Future<Output = Box<keyring::CredentialBuilder>> + Send + 'static
    {
        async move { keyring::default_credential_builder() }
    }

    async fn login(
        _: &mut Context<Self>,
    ) -> Result<AuthInfos, Self::LoginError> {
        Ok(AuthInfos { github_handle: "noib3".parse().expect("valid") })
    }
}
