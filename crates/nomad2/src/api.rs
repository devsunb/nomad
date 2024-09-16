use crate::Module;

/// TODO: docs.
pub struct Api<M: Module> {
    pub(crate) module: M,
}
