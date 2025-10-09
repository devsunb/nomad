use core::fmt;
use core::str::FromStr;

use smol_str::{SmolStr, format_smolstr};

use crate::{EmailAddress, PeerHandle};

type UnixEpoch = u64;

/// The claims contained in the JWT returned by Nomad's authentication server
/// after a successful login.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    /// The JWT's `aud` claim.
    #[serde(rename = "aud")]
    pub audience: SmolStr,

    /// The JWT's `exp` claim.
    #[serde(rename = "exp")]
    pub expires_at: UnixEpoch,

    /// The JWT's `iat` claim.
    #[serde(rename = "iat")]
    pub issued_at: UnixEpoch,

    /// The JWT's `iss` claim.
    #[serde(rename = "iss")]
    pub issuer: SmolStr,

    /// The JWT's `sub` claim.
    #[serde(rename = "sub")]
    pub subject: Subject,

    /// The user's email address.
    pub email: EmailAddress,

    /// The user's name.
    pub name: Option<SmolStr>,

    /// The user's username.
    pub username: PeerHandle,
}

/// The possible values for the `sub` claim in a Nomad JWT.
#[derive(Debug, Copy, Clone)]
pub enum Subject {
    GitHubUserId(u64),
}

/// The type of error that can occur when parsing a [`Subject`] from a string.
#[derive(Debug, derive_more::Display, cauchy::Error, PartialEq, Eq)]
pub enum SubjectFromStrError {
    #[display("invalid format")]
    InvalidFormat,

    #[display("couldn't parse GitHub user ID: {_0}")]
    InvalidGitHubUserId(core::num::ParseIntError),
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GitHubUserId(id) => write!(f, "github:{}", id),
        }
    }
}

impl FromStr for Subject {
    type Err = SubjectFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (service, id) =
            s.split_once(':').ok_or(SubjectFromStrError::InvalidFormat)?;

        match service {
            "github" => id
                .parse::<u64>()
                .map(Subject::GitHubUserId)
                .map_err(SubjectFromStrError::InvalidGitHubUserId),
            _ => Err(SubjectFromStrError::InvalidFormat),
        }
    }
}

impl serde::Serialize for Subject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format_smolstr!("{}", self))
    }
}

impl<'de> serde::Deserialize<'de> for Subject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = SmolStr::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}
