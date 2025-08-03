use crate::{Peer, PeerId};

/// TODO: docs.
#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub struct ProjectRequest {
    /// The [`Peer`] that send the request.
    pub requested_by: Peer,

    /// The [`PeerId`] of the peer the request is directed to.
    pub request_from: PeerId,
}
