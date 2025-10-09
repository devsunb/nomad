use core::str::FromStr;
use std::borrow::Cow;
use std::sync::{Arc, LazyLock};

use jsonwebtoken::Algorithm;

use crate::Claims;

static AUTH_SERVER_JWT_SIGNING_PUBLIC_KEY: LazyLock<
    jsonwebtoken::DecodingKey,
> = LazyLock::new(|| {
    let contents = include_bytes!("../auth_server_jwt_signing_public_key.pem");
    jsonwebtoken::DecodingKey::from_ec_pem(contents)
        .expect("public key is valid")
});

/// The JWT returned by Nomad's authentication server, along with its parsed
/// [`Claims`].
#[derive(Clone)]
pub struct JsonWebToken {
    contents: Arc<str>,
    claims: Claims,
}

impl JsonWebToken {
    /// Returns the token's contents.
    pub fn as_str(&self) -> &str {
        &self.contents
    }

    /// Returns the token's claims.
    pub fn claims(&self) -> &Claims {
        &self.claims
    }
}

impl FromStr for JsonWebToken {
    type Err = jsonwebtoken::errors::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let token_data = jsonwebtoken::decode::<Claims>(
            str,
            &AUTH_SERVER_JWT_SIGNING_PUBLIC_KEY,
            &jsonwebtoken::Validation::new(Algorithm::ES256),
        )?;

        Ok(Self { contents: str.into(), claims: token_data.claims })
    }
}

impl serde::Serialize for JsonWebToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.contents)
    }
}

impl<'de> serde::Deserialize<'de> for JsonWebToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <Cow<str>>::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl From<JsonWebToken> for peer_handle::PeerHandle {
    fn from(jwt: JsonWebToken) -> Self {
        jwt.claims().username.clone()
    }
}
