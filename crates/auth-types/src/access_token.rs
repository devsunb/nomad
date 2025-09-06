use crate::GitHubAccessToken;

/// TODO: docs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccessToken {
    /// TODO: docs.
    GitHub(GitHubAccessToken),
}
