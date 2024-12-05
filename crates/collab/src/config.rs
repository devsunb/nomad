#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// TODO: docs.
    server_address: String,

    /// TODO: docs.
    server_port: String,
}
