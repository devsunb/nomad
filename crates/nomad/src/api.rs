use nvim_oxi::Dictionary as NvimDictionary;

use crate::module_api::ModuleApi;
use crate::Module;

#[derive(Default)]
pub(crate) struct Api {
    /// A dictionary from [`ModuleName`] to the corresponding module's API
    /// dictionary.
    dictionary: NvimDictionary,
}

impl Api {
    #[track_caller]
    pub(crate) fn add_module<M: Module>(&mut self, module_api: ModuleApi<M>) {
        if self.dictionary.get(&M::NAME.as_str()).is_some() {
            panic!(
                "a module with the name '{}' has already been added to the \
                 API",
                M::NAME.as_str()
            );
        }
        self.dictionary.insert(M::NAME.as_str(), module_api.dictionary);
    }
}
