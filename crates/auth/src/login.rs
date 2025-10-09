//! TODO: docs.

use editor::command::ToCompletionFn;
use editor::module::AsyncAction;
use editor::{Access, Context, Shared};
use peer_handle::PeerHandle;

use crate::credential_store::CredentialStore;
use crate::{Auth, AuthEditor, AuthState, Config};

/// TODO: docs.
#[derive(Clone, Default)]
pub struct Login {
    config: Shared<Config>,
    credential_store: CredentialStore,
    state: AuthState,
}

impl Login {
    pub(crate) async fn call_inner<Ed: AuthEditor>(
        &self,
        ctx: &mut Context<Ed>,
    ) -> Result<(), LoginError<Ed>> {
        if let Some(peer_handle) = self.state.with(|maybe_jwt| {
            maybe_jwt.as_ref().map(|jwt| jwt.claims().username.clone())
        }) {
            return Err(LoginError::AlreadyLoggedIn(peer_handle));
        }

        let jwt = Ed::login(self.config.clone(), ctx)
            .await
            .map_err(LoginError::Login)?;

        self.state.set_logged_in(jwt.clone());

        // Persisting the credentials blocks, so do it in the background.
        let credential_store = self.credential_store.clone();
        ctx.spawn_background(
            async move { credential_store.persist(&jwt).await },
        )
        .await
        .map_err(Into::into)
    }
}

impl<Ed: AuthEditor> AsyncAction<Ed> for Login {
    const NAME: &str = "login";

    type Args = ();

    async fn call(&mut self, _: Self::Args, ctx: &mut Context<Ed>) {
        if let Err(err) = self.call_inner(ctx).await {
            Ed::on_login_error(err, ctx);
        }
    }
}

/// TODO: docs.
#[derive(cauchy::Debug, derive_more::Display, cauchy::Error, cauchy::From)]
pub enum LoginError<Ed: AuthEditor> {
    /// TODO: docs.
    #[display("Already logged in as {_0}")]
    AlreadyLoggedIn(PeerHandle),

    /// TODO: docs.
    #[display("{_0}")]
    Login(Ed::LoginError),

    /// TODO: docs.
    #[display("Couldn't persist credentials: {_0}")]
    PersistCredentials(#[from] keyring::Error),
}

impl From<&Auth> for Login {
    fn from(auth: &Auth) -> Self {
        Self {
            config: auth.config.clone(),
            credential_store: auth.credential_store.clone(),
            state: auth.state(),
        }
    }
}

impl<Ed: AuthEditor> ToCompletionFn<Ed> for Login {
    fn to_completion_fn(&self) {}
}
