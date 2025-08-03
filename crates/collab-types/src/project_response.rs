use crate::{PeerId, Peers};

/// TODO: docs.
#[derive(
    cauchy::Debug,
    Clone,
    cauchy::PartialEq,
    cauchy::Eq,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct ProjectResponse {
    /// TODO: docs.
    #[debug(skip)]
    #[partial_eq(skip)]
    pub encoded_project: Vec<u8>,

    /// TODO: docs.
    pub peers: Peers,

    /// TODO: docs.
    pub respond_to: PeerId,
}
