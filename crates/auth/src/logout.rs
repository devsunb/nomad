//! TODO: docs.

use editor::command::ToCompletionFn;
use editor::module::AsyncAction;
use editor::{Context, Editor};

use crate::credential_store::CredentialStore;
use crate::{Auth, AuthEditor, AuthState};

/// TODO: docs.
#[derive(Clone, Default)]
pub struct Logout {
    credential_store: CredentialStore,
    state: AuthState,
}

impl Logout {
    pub(crate) async fn call_inner<Ed: Editor>(
        &self,
        ctx: &mut Context<Ed>,
    ) -> Result<(), LogoutError> {
        if !self.state.set_logged_out() {
            return Err(LogoutError::NotLoggedIn);
        }

        // Deleting the credentials blocks, so do it in the background.
        let credential_store = self.credential_store.clone();
        ctx.spawn_background(async move { credential_store.delete().await })
            .await
            .map_err(Into::into)
    }
}

impl<Ed: AuthEditor> AsyncAction<Ed> for Logout {
    const NAME: &str = "logout";

    type Args = ();

    async fn call(&mut self, _: Self::Args, ctx: &mut Context<Ed>) {
        if let Err(err) = self.call_inner(ctx).await {
            Ed::on_logout_error(err, ctx);
        }
    }
}

/// TODO: docs.
#[derive(Debug, derive_more::Display, cauchy::Error, cauchy::From)]
pub enum LogoutError {
    /// TODO: docs.
    #[display("Couldn't delete credentials from keyring: {_0}")]
    DeleteCredential(#[from] keyring::Error),

    /// TODO: docs.
    #[display("Not logged in")]
    NotLoggedIn,
}

impl From<&Auth> for Logout {
    fn from(auth: &Auth) -> Self {
        Self {
            credential_store: auth.credential_store.clone(),
            state: auth.state(),
        }
    }
}

impl<Ed: Editor> ToCompletionFn<Ed> for Logout {
    fn to_completion_fn(&self) {}
}
