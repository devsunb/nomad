//! TODO: docs.

mod access_token;
mod github_access_token;
mod github_client_id;
mod oauth_state;

pub use access_token::AccessToken;
pub use github_access_token::GitHubAccessToken;
pub use github_client_id::GitHubClientId;
pub use oauth_state::{OAuthState, OAuthStateFromStrError};

/// The [`GitHubClientId`] assigned to Nomad's OAuth app.
pub const NOMAD_GITHUB_CLIENT_ID: GitHubClientId =
    GitHubClientId("Ov23liDqf1zOSVXAoVJq");
