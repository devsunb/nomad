use serde::Serialize;

/// TODO: docs.
pub trait Constant: Serialize + 'static {
    /// TODO: docs.
    const NAME: &str;
}
