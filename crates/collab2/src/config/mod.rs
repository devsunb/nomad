mod server_socket;

pub(crate) use server_socket::ServerSocket;

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// TODO: docs.
    pub(crate) server_socket: ServerSocket,
}
