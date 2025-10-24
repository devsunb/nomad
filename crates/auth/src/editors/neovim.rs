#![allow(missing_docs)]

use auth_types::JsonWebToken;
use editor::context::Borrowed;
use editor::{Access, Context};
use neovim::Neovim;
use neovim::notify::{self, NotifyContextExt};

use crate::{AuthEditor, config, github, login, logout};

impl AuthEditor for Neovim {
    type LoginError = github::GitHubLoginError<Self::HttpClient>;

    #[allow(clippy::manual_async_fn)]
    fn credential_builder(
        _: &mut Context<Self, Borrowed>,
    ) -> impl Future<Output = Box<keyring::CredentialBuilder>> + Send + 'static
    {
        async { keyring::default_credential_builder() }
    }

    async fn login(
        config: impl Access<config::Config>,
        ctx: &mut Context<Self>,
    ) -> Result<JsonWebToken, Self::LoginError> {
        let jwt = github::login(config, ctx).await?;

        let mut chunks = notify::Chunks::default();

        chunks
            .push("Successfully logged in as '")
            .push_highlighted(jwt.claims().username.as_str(), "Identifier")
            .push("'");

        ctx.notify_info(chunks);

        Ok(jwt)
    }

    fn on_login_error(
        error: login::LoginError<Self>,
        ctx: &mut Context<Self>,
    ) {
        ctx.notify_error(error);
    }

    fn on_logout_error(error: logout::LogoutError, ctx: &mut Context<Self>) {
        ctx.notify_error(error);
    }
}

impl From<login::LoginError<Neovim>> for notify::Chunks {
    fn from(error: login::LoginError<Neovim>) -> Self {
        error.to_string().into()
    }
}

impl From<logout::LogoutError> for notify::Chunks {
    fn from(error: logout::LogoutError) -> Self {
        error.to_string().into()
    }
}
