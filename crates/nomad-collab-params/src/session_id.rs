use core::fmt;
use core::str::FromStr;

/// The unique identifier for a collaboration session used by Nomad's collab
/// server.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct SessionId {
    inner: ulid::Ulid,
}

impl SessionId {
    /// Creates a new [`SessionId`] from the given RNG.
    pub fn from_rng(rng: &mut impl rand::RngCore) -> Self {
        Self { inner: ulid::Ulid::with_source(rng) }
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; 26];
        let base32_encoded = self.inner.array_to_str(&mut buf);
        let (timestamp, random) = base32_encoded.split_at_mut(10);
        timestamp.make_ascii_lowercase();
        random.make_ascii_lowercase();
        write!(f, "{timestamp}-{random}")
    }
}

impl FromStr for SessionId {
    type Err = ulid::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 27 || s.as_bytes()[10] != b'-' {
            return Err(ulid::DecodeError::InvalidLength);
        }
        let mut buf = [0u8; 26];
        buf[..10].copy_from_slice(&s.as_bytes()[..10]);
        buf[10..].copy_from_slice(&s.as_bytes()[11..]);
        buf.make_ascii_uppercase();
        ulid::Ulid::from_str(str::from_utf8(&buf).expect("valid ASCII"))
            .map(|inner| Self { inner })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_id_display_from_str_roundtrip() {
        let session_id = SessionId::from_rng(&mut rand::rng());
        let session_id_str = session_id.to_string();
        let parsed_session_id = SessionId::from_str(&session_id_str).unwrap();
        assert_eq!(session_id, parsed_session_id);
    }
}
