use crate::Render;

/// TODO: docs
pub trait Component {
    /// TODO: docs
    fn compose(&self) -> impl Render;
}
