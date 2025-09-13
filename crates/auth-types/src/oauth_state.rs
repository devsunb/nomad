use core::{fmt, str};

use base64::Engine as _;

/// A random value used in OAuth flows to map requests to responses and to
/// prevent CSRF attacks.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct OAuthState([u8; 16]);

/// The type of error that can occur when parsing an [`OAuthState`] from a
/// string.
#[derive(Debug)]
pub struct OAuthStateFromStrError {
    inner: base64::DecodeSliceError,
}

impl OAuthState {
    /// Creates a new [`OAuthState`] from the given bytes.
    #[inline]
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    fn to_str(&self) -> impl AsRef<str> {
        struct Base64Encoded {
            bytes: [u8; 22],
        }

        impl Base64Encoded {
            #[inline]
            fn new(bytes: [u8; 22]) -> Self {
                debug_assert!(str::from_utf8(&bytes[..]).is_ok());
                Self { bytes }
            }
        }

        impl AsRef<str> for Base64Encoded {
            #[inline]
            fn as_ref(&self) -> &str {
                // SAFETY: Base64 encoding always produces valid UTF-8.
                unsafe { str::from_utf8_unchecked(&self.bytes[..]) }
            }
        }

        let mut bytes = [0u8; 22];
        let num_written = base64_engine()
            .encode_slice(self, &mut bytes)
            .expect("22 bytes are enough to encode 16 bytes in base64");
        debug_assert_eq!(num_written, bytes.len());
        Base64Encoded::new(bytes)
    }
}

impl fmt::Debug for OAuthState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for OAuthState {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_str().as_ref().fmt(f)
    }
}

impl str::FromStr for OAuthState {
    type Err = OAuthStateFromStrError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0u8; _];
        let num_written = base64_engine()
            .decode_slice(s, &mut bytes)
            .map_err(|inner| OAuthStateFromStrError { inner })?;
        debug_assert_eq!(num_written, bytes.len());
        Ok(Self(bytes))
    }
}

impl AsRef<[u8]> for OAuthState {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for OAuthStateFromStrError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl core::error::Error for OAuthStateFromStrError {
    #[inline]
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        self.inner.source()
    }
}

const fn base64_engine() -> impl base64::Engine {
    base64::engine::general_purpose::URL_SAFE_NO_PAD
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::*;

    impl serde::Serialize for OAuthState {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.to_str().as_ref())
        }
    }

    impl<'de> serde::Deserialize<'de> for OAuthState {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            <&str>::deserialize(deserializer)?
                .parse()
                .map_err(serde::de::Error::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};

    use super::*;

    #[test]
    fn roundtrip_from_zeros() {
        let original = OAuthState([0; 16]);
        let str = original.to_string();
        let parsed: OAuthState = str.parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn roundtrip_from_rng() {
        for _ in 0..100 {
            let seed = rand::random::<u64>();
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
            let original = OAuthState::from_bytes(rng.random());
            let str = original.to_string();
            let parsed: OAuthState = str.parse().unwrap();
            if original != parsed {
                panic!("roundtrip failed for seed {seed}");
            }
        }
    }
}
