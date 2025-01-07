use core::convert::Infallible;

use super::{Level, Message, Notification};
use crate::action_ctx::ModulePath;
use crate::{Backend, Name, Plugin};

/// TODO: docs.
pub trait Error {
    /// TODO: docs.
    fn to_notification<P, B>(
        &self,
        module_path: &ModulePath,
        action_name: Option<Name>,
    ) -> Option<(Level, Message)>
    where
        P: Plugin<B>,
        B: Backend;
}

impl Error for Infallible {
    fn to_notification<P, B>(
        &self,
        _: &ModulePath,
        _: Option<Name>,
    ) -> Option<(Level, Message)>
    where
        P: Plugin<B>,
        B: Backend,
    {
        unreachable!()
    }
}

impl<T: Error> Error for &T {
    #[inline]
    fn to_notification<P, B>(
        &self,
        module_path: &ModulePath,
        action_name: Option<Name>,
    ) -> Option<(Level, Message)>
    where
        P: Plugin<B>,
        B: Backend,
    {
        (&**self).to_notification::<P, B>(module_path, action_name)
    }
}
//
// impl Error for Box<dyn Error> {
//     #[inline]
//     fn to_notification<P, B>(
//         &self,
//         module_path: &ModulePath,
//         action_name: Option<Name>,
//     ) -> Option<Notification>
//     where
//         P: Plugin<B>,
//         B: Backend,
//     {
//         (&**self).to_notification(module_path, action_name)
//     }
// }
