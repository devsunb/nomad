use core::{fmt, str};

/// A GitHub access token.
#[derive(Clone, PartialEq, Eq)]
pub struct GitHubAccessToken {
    /// The token is never modified, so we can use a boxed slice instead of a
    /// `String`.
    ///
    /// GitHub doesn't provide any schema for their tokens apart from them
    /// being "strings", so we don't perform any validation on this inner
    /// value.
    inner: Box<str>,
}

impl GitHubAccessToken {
    /// Creates a new [`GitHubAccessToken`] from the given string.
    #[inline]
    pub fn new(token: impl AsRef<str>) -> Self {
        Self { inner: token.as_ref().into() }
    }

    #[inline]
    fn as_str(&self) -> &str {
        &self.inner
    }
}

impl fmt::Debug for GitHubAccessToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for GitHubAccessToken {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl str::FromStr for GitHubAccessToken {
    type Err = core::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl AsRef<str> for GitHubAccessToken {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::*;

    impl serde::Serialize for GitHubAccessToken {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }

    impl<'de> serde::Deserialize<'de> for GitHubAccessToken {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            <&str>::deserialize(deserializer)
                .map(|inner| Self { inner: inner.into() })
        }
    }
}
