use ed::backend::Backend;

/// TODO: docs.
pub trait AuthBackend: Backend {
    /// TODO: docs.
    fn credential_store(&self) -> Box<keyring::CredentialBuilder>;
}
