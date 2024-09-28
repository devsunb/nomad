use core::cmp::Ordering;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::NonNull;

use nvim_oxi::serde::Deserializer;
use nvim_oxi::{
    Function as NvimFunction,
    Object as NvimObject,
    String as NvimString,
};
use serde::de::{Deserialize, DeserializeOwned};

use super::Neovim;
use crate::{Context, Emitter, Event, Module, ModuleName, Subscription};

/// TODO: docs.
pub struct NeovimFunction {
    name: NvimString,
    inner: NvimFunction<NvimObject, ()>,
}

impl NeovimFunction {
    /// Constructs a new `NeovimFunctionBuilder`.
    #[inline]
    pub fn builder() -> NeovimFunctionBuilder<NoName, NoArgs> {
        NeovimFunctionBuilder { name: NoName, args: PhantomData }
    }
}

/// TODO: docs.
pub struct NeovimFunctionBuilder<Name, Args> {
    name: Name,
    args: PhantomData<Args>,
}

impl NeovimFunctionBuilder<NoName, NoArgs> {
    /// The name of the function.
    #[inline]
    pub fn name<Name: Into<NvimString>>(
        self,
        name: Name,
    ) -> NeovimFunctionBuilder<NvimString, NoArgs> {
        NeovimFunctionBuilder { name: name.into(), args: self.args }
    }
}

impl NeovimFunctionBuilder<NvimString, NoArgs> {
    /// The event to execute when the function is called.
    #[inline]
    pub fn args<Args: DeserializeOwned + 'static>(
        self,
    ) -> NeovimFunctionBuilder<NvimString, Args> {
        NeovimFunctionBuilder { name: self.name, args: PhantomData }
    }
}

impl<Args> NeovimFunctionBuilder<NvimString, Args>
where
    Args: DeserializeOwned + 'static,
{
    /// TODO: docs.
    #[inline]
    pub fn build<M: Module<Neovim>>(
        self,
        ctx: &Context<Neovim>,
    ) -> (NeovimFunction, Subscription<NeovimFunctionEvent<Args>, Neovim>)
    {
        let buf = Box::new_uninit();
        let ptr = NonNull::new(Box::into_raw(buf)).expect("just allocated");
        let event = NeovimFunctionEvent {
            module_name: M::NAME,
            function_name: self.name.to_string_lossy().into_owned(),
            ptr,
            args: PhantomData,
        };
        let sub = ctx.subscribe(event);
        // SAFETY: `NeovimFunctionEvent`'s subscribe impl didn't drop the `Box`
        // and it initialized the function.
        let inner = *(unsafe { Box::from_raw(ptr.as_ptr()).assume_init() });
        let fun = NeovimFunction { name: self.name, inner };
        (fun, sub)
    }
}

/// TODO: docs.
pub struct NeovimFunctionEvent<T> {
    module_name: ModuleName,
    function_name: String,
    ptr: NonNull<MaybeUninit<NvimFunction<NvimObject, ()>>>,
    args: PhantomData<T>,
}

impl<T> PartialEq for NeovimFunctionEvent<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> Eq for NeovimFunctionEvent<T> {}

impl<T> PartialOrd for NeovimFunctionEvent<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for NeovimFunctionEvent<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.module_name.cmp(&other.module_name) {
            Ordering::Equal => self.function_name.cmp(&other.function_name),
            ord => ord,
        }
    }
}

impl<T: DeserializeOwned + 'static> Event<Neovim> for NeovimFunctionEvent<T> {
    type Payload = T;
    type SubscribeCtx = ();

    #[inline]
    fn subscribe(&mut self, emitter: Emitter<T>, _: &Context<Neovim>) {
        let nvim_fun = NvimFunction::<NvimObject, ()>::from_fn(move |obj| {
            let payload =
                T::deserialize(Deserializer::new(obj)).expect("something");
            emitter.send(payload);
        });

        // SAFETY: the pointer is still valid (look at `build()`), and the
        // `subscribe()` method is only called once.
        let buf = unsafe { &mut *self.ptr.as_ptr() };

        buf.write(nvim_fun);
    }
}

pub struct NoName;

pub struct NoArgs;
