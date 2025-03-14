//! TODO: docs.

use collab_server::message::GitHubHandle;
use ed::action::AsyncAction;
use ed::command::ToCompletionFn;
use ed::notify::{self, Name};
use ed::{AsyncCtx, Shared};

use crate::{Auth, AuthBackend, AuthInfos};

/// TODO: docs.
#[derive(Clone, Default)]
pub struct Login {
    _infos: Shared<Option<AuthInfos>>,
}

impl<B: AuthBackend> AsyncAction<B> for Login {
    const NAME: Name = "login";

    type Args = ();

    async fn call(
        &mut self,
        _: Self::Args,
        _: &mut AsyncCtx<'_, B>,
    ) -> Result<(), LoginError<B>> {
        todo!();
    }
}

/// TODO: docs.
pub enum LoginError<B: AuthBackend> {
    /// TODO: docs.
    AlreadyLoggedIn(GitHubHandle),

    /// TODO: docs.
    BuildCredentialEntry(keyring::Error),

    /// TODO: docs.
    Login(B::LoginError),

    /// TODO: docs.
    StoreAuthInfos(keyring::Error),
}

impl From<&Auth> for Login {
    fn from(auth: &Auth) -> Self {
        Self { _infos: auth.infos().clone() }
    }
}

impl<B: AuthBackend> ToCompletionFn<B> for Login {
    fn to_completion_fn(&self) {}
}

impl<B: AuthBackend> notify::Error for LoginError<B> {
    fn to_message(&self) -> (notify::Level, notify::Message) {
        todo!();
    }
}
