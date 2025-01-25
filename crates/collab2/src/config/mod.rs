mod server_address;

pub(crate) use server_address::ServerAddress;

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The address of the server to connect to when starting or joining an
    /// editing session.
    pub(crate) server_address: ServerAddress,
}
