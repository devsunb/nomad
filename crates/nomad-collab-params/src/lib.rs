//! This crate contains the [`Params`][NomadParams] used by Nomad's collab
//! server running at `collab.nomad.foo`.

use auth_types::AccessToken;
use collab_server::Params;
pub use ulid;

/// The [`Params`] used by the Collab server deployed at `collab.nomad.foo`.
pub struct NomadParams;

/// The type of error that can occur when the server's authenticator tries to
/// authenticate a peer from the provided [`AccessToken`].
#[derive(Debug, derive_more::Display, cauchy::Error, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("{_0}")]
pub enum AuthError {
    /// Authentication via GitHub failed.
    GitHub(GitHubAuthError),
}

/// The type of error that can occur when the server's authenticator tries to
/// authenticate a peer using a
/// [`GitHubAccessToken`](crate::GitHubAccessToken).
#[derive(Debug, derive_more::Display, cauchy::Error, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("{_0}")]
pub enum GitHubAuthError {
    /// GitHub's API returned an error.
    ApiError(String),

    /// Deserializing the response's body into JSON failed.
    DeserializeResponse(String),

    /// The HTTP request to GitHub's API failed.
    HttpRequest(String),
}

impl Params for NomadParams {
    const MAX_FRAME_LEN: u32 = 2048;

    type AuthenticateInfos = AccessToken;
    type AuthenticateError = AuthError;
    type SessionId = ulid::Ulid;
}
