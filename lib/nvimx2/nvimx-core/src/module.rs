use core::mem::ManuallyDrop;

use serde::de::DeserializeOwned;

use crate::api::{Api, ModuleApi};
use crate::{
    Backend,
    BackendHandle,
    Function,
    MaybeResult,
    NeovimCtx,
    Plugin,
    notify,
};

/// TODO: docs.
pub trait Module<B: Backend>: 'static + Sized {
    /// TODO: docs.
    const NAME: &'static ModuleName;

    /// TODO: docs.
    type Plugin: Plugin<B>;

    /// TODO: docs.
    type Config: DeserializeOwned;

    /// TODO: docs.
    type Docs;

    /// TODO: docs.
    fn api(&self, ctx: ModuleApiCtx<'_, Self, B>);

    /// TODO: docs.
    fn on_config_changed(
        &mut self,
        new_config: Self::Config,
        ctx: NeovimCtx<'_, B>,
    );

    /// TODO: docs.
    fn docs() -> Self::Docs;
}

/// TODO: docs.
pub struct ModuleApiCtx<'a, M: Module<B>, B: Backend> {
    #[allow(clippy::type_complexity)]
    api: ManuallyDrop<
        <B::Api<M::Plugin> as Api<M::Plugin, B>>::ModuleApi<'a, M>,
    >,
    backend: &'a BackendHandle<B>,
}

/// TODO: docs.
#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ModuleName(str);

impl<'a, M, B> ModuleApiCtx<'a, M, B>
where
    M: Module<B>,
    B: Backend,
{
    /// TODO: docs.
    #[track_caller]
    #[inline]
    pub fn with_function<Fun>(mut self, mut fun: Fun) -> Self
    where
        Fun: Function<B, Module = M>,
    {
        let backend = self.backend.clone();
        let callback = move |args| {
            let fun = &mut fun;
            backend.with_mut(move |backend| {
                fun.call(args, NeovimCtx::new(backend))
                    .into_result()
                    // Even though the error is bound to `notify::Error`
                    // (which itself is bound to `'static`), Rust thinks that
                    // the error captures some lifetime due to
                    // `Function::call()` returning an `impl MaybeResult`.
                    //
                    // Should be the same problem as
                    // https://github.com/rust-lang/rust/issues/42940
                    //
                    // FIXME: Is there a better way around this than boxing the
                    // error?
                    .map_err(|err| Box::new(err) as Box<dyn notify::Error>)
            })
        };
        self.api.add_function::<Fun, _, _>(callback);
        self
    }

    #[inline]
    pub(crate) fn new(
        api: &'a mut B::Api<M::Plugin>,
        backend: &'a BackendHandle<B>,
    ) -> Self {
        Self { api: ManuallyDrop::new(api.with_module::<M>()), backend }
    }
}

impl ModuleName {
    /// TODO: docs.
    #[inline]
    pub const fn as_str(&self) -> &str {
        &self.0
    }

    /// TODO: docs.
    #[inline]
    pub const fn new(name: &str) -> &Self {
        assert!(!name.is_empty());
        assert!(name.len() <= 24);
        // SAFETY: `ModuleName` is a `repr(transparent)` newtype around `str`.
        unsafe { &*(name as *const str as *const Self) }
    }
}

impl<M, B> Drop for ModuleApiCtx<'_, M, B>
where
    M: Module<B>,
    B: Backend,
{
    #[inline]
    fn drop(&mut self) {
        // SAFETY: We never use the `ManuallyDrop` again.
        let api = unsafe { ManuallyDrop::take(&mut self.api) };
        api.finish();
    }
}
