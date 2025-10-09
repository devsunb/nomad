/// The type of error that can occur when the server's authenticator tries to
/// authenticate a peer from the provided JWT.
#[derive(Debug, derive_more::Display, cauchy::Error, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("{_0}")]
pub enum AuthError {
    /// The JWT was invalid.
    Jwt(String),
}
