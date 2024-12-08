use core::ops::Deref;
use std::rc::Rc;

#[derive(Default, serde::Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// TODO: docs.
    pub(crate) server_socket: ServerSocket,
}

#[derive(Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ServerSocket {
    #[serde(flatten)]
    inner: Rc<str>,
}

impl ServerSocket {
    /// Returns the server's TCP socket as an opaque object that can be
    /// dereferenced to a `str`.
    pub(crate) fn as_deref_str(&self) -> impl Deref<Target = str> + '_ {
        self.inner.as_ref()
    }
}

impl Default for ServerSocket {
    #[inline]
    fn default() -> Self {
        Self { inner: "collab.nomad.foo:64420".to_owned().into() }
    }
}
