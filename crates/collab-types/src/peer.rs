use crate::{GitHubHandle, PeerId};

/// TODO: docs.
#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct Peer {
    /// The peer's ID.
    pub id: PeerId,

    /// The peer's GitHub handle.
    pub github_handle: GitHubHandle,
}
