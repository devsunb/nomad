use core::fmt;
use core::str::FromStr;

use smol_str::SmolStr;

/// A validated email address.
#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct EmailAddress {
    inner: SmolStr,
}

/// The type of error that can occur when parsing an [`EmailAddress`] from a
/// string.
#[derive(Debug, derive_more::Display, cauchy::Error, PartialEq, Eq)]
pub enum EmailAddressFromStrError {
    InvalidFormat,
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner)
    }
}

impl FromStr for EmailAddress {
    type Err = EmailAddressFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: do this properly.
        if s.contains('@') && !s.contains(' ') {
            Ok(Self { inner: SmolStr::new(s) })
        } else {
            Err(EmailAddressFromStrError::InvalidFormat)
        }
    }
}
