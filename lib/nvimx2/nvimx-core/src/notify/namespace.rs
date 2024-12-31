use smallvec::SmallVec;

use crate::ActionName;
use crate::module::ModuleName;

/// TODO: docs.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
    action: Option<&'static ActionName>,
    modules: SmallVec<[&'static ModuleName; 2]>,
}

impl Namespace {
    /// TODO: docs.
    #[inline]
    pub fn action(&self) -> Option<&'static ActionName> {
        self.action
    }

    /// TODO: docs.
    #[inline]
    pub fn components(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.modules()
            .map(|name| name.as_str())
            .chain(self.action.map(|name| name.as_str()))
    }

    /// TODO: docs.
    #[inline]
    pub fn modules(&self) -> impl Iterator<Item = &'static ModuleName> + '_ {
        self.modules.iter().copied()
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn pop(&mut self) {
        self.modules.pop();
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn push_module(&mut self, module_name: &'static ModuleName) {
        self.modules.push(module_name);
    }

    /// TODO: docs.
    #[inline]
    pub(crate) fn set_action(&mut self, action_name: &'static ActionName) {
        debug_assert!(self.action.is_none());
        self.action = Some(action_name);
    }
}
