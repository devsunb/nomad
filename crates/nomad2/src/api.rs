use crate::{Editor, Module};

/// TODO: docs.
pub struct Api<M: Module<E>, E: Editor> {
    pub(crate) module: M,
    pub(crate) editor: E,
}
