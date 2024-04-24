use crate::EditorId;

/// TODO: docs
#[derive(Debug, Clone, Default)]
pub struct Edit {}

impl Edit {
    /// TODO: docs
    #[inline]
    pub fn applied_by(&self) -> EditorId {
        todo!();
    }

    #[inline]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub(crate) fn with_editor(self, id: EditorId) -> Self {
        self
    }
}
