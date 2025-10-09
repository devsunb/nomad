use auth_types::JsonWebToken;
use editor::{Access, Shared};

/// TODO: docs.
#[derive(Clone, Default)]
pub struct AuthState {
    inner: Shared<Option<JsonWebToken>>,
}

impl AuthState {
    pub(crate) fn set_logged_in(&self, jwt: JsonWebToken) {
        self.inner.set(Some(jwt));
    }

    /// Sets the state to logged out, returning whether it was logged in
    /// before.
    pub(crate) fn set_logged_out(&self) -> bool {
        self.inner.take().is_some()
    }
}

impl Access<Option<JsonWebToken>> for AuthState {
    fn with<R>(&self, fun: impl FnOnce(&Option<JsonWebToken>) -> R) -> R {
        self.inner.with(fun)
    }
}
