//! TODO: docs.

use nvimx_core::{KeyValuePair, MapAccess, Value, notify};

use crate::oxi::{self, Dictionary, Object, ObjectKind, lua};

/// TODO: docs.
#[derive(Default)]
pub struct NeovimValue {
    object: Object,
}

/// TODO: docs.
pub struct NeovimMapAccess<'a> {
    dict: &'a mut Dictionary,
    dict_idx: usize,
}

/// TODO: docs.
pub struct NeovimMapPair<'a> {
    key: NeovimMapKey<'a>,
}

/// TODO: docs.
#[derive(Copy, Clone)]
pub struct NeovimMapKey<'a> {
    dict_key: &'a oxi::String,
    dict_idx: usize,
}

/// TODO: docs.
pub struct NeovimMapAccessError(ObjectKind);

impl NeovimValue {
    #[inline]
    pub(crate) fn into_inner(self) -> Object {
        self.object
    }

    #[inline]
    pub(crate) fn new(object: Object) -> Self {
        Self { object }
    }
}

impl Value for NeovimValue {
    type MapAccess<'a> = NeovimMapAccess<'a>;
    type MapAccessError<'a> = NeovimMapAccessError;

    #[inline]
    fn map_access(
        &mut self,
    ) -> Result<Self::MapAccess<'_>, Self::MapAccessError<'_>> {
        match self.object.kind() {
            ObjectKind::Dictionary => Ok(NeovimMapAccess {
                // SAFETY: the object's kind is a `Dictionary`.
                dict: unsafe { self.object.as_dictionary_unchecked_mut() },
                dict_idx: 0,
            }),
            other => Err(NeovimMapAccessError(other)),
        }
    }
}

impl lua::Poppable for NeovimValue {
    #[inline]
    unsafe fn pop(
        lua_state: *mut lua::ffi::State,
    ) -> Result<Self, lua::Error> {
        unsafe { Object::pop(lua_state).map(|object| Self { object }) }
    }
}

impl lua::Pushable for NeovimValue {
    #[inline]
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::State,
    ) -> Result<std::ffi::c_int, lua::Error> {
        unsafe { self.object.push(lstate) }
    }
}

impl MapAccess for NeovimMapAccess<'_> {
    type Pair<'a>
        = NeovimMapPair<'a>
    where
        Self: 'a;

    fn next_pair(&mut self) -> Option<Self::Pair<'_>> {
        todo!()
    }
}

impl KeyValuePair for NeovimMapPair<'_> {
    type Key<'a>
        = NeovimMapKey<'a>
    where
        Self: 'a;

    type Value = NeovimValue;

    fn key(&self) -> Self::Key<'_> {
        todo!();
    }

    fn take_value(self) -> Self::Value {
        todo!()
    }
}

impl PartialEq<str> for NeovimMapKey<'_> {
    #[inline]
    fn eq(&self, s: &str) -> bool {
        self.dict_key == s
    }
}

impl notify::Error for NeovimMapAccessError {
    fn to_level(&self) -> Option<notify::Level> {
        todo!()
    }

    fn to_message(&self) -> notify::Message {
        todo!()
    }
}
