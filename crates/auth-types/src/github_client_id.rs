/// A GitHub OAuth client ID.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct GitHubClientId(pub(crate) &'static str);

impl GitHubClientId {
    /// Returns the string representation of the client ID.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}
