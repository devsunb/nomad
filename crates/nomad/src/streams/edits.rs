use core::pin::Pin;
use core::task::{Context, Poll};

use futures::Stream;

/// A [`Stream`] that yields the [`Edit`]s that are applied to a
/// [`Buffer`](crate::editor::Buffer).
pub struct Edits {}

impl Stream for Edits {
    type Item = AppliedEdit;

    #[inline]
    fn poll_next(
        self: Pin<&mut Self>,
        _ctx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        todo!()
    }
}

/// A single edit to a [`Buffer`].
#[derive(Clone)]
pub enum AppliedEdit {
    /// TODO: docs
    Insertion(AppliedInsertion),

    /// TODO: docs
    Deletion(AppliedDeletion),
}

/// TODO: docs
#[derive(Clone)]
pub struct AppliedInsertion {
    inner: cola::Insertion,
    text: String,
}

impl AppliedInsertion {
    #[inline]
    pub(crate) fn anchor(&self) -> cola::Anchor {
        todo!();
    }

    #[inline]
    pub(crate) fn new(inner: cola::Insertion, text: String) -> Self {
        Self { inner, text }
    }

    #[inline]
    pub(crate) fn text(&self) -> &str {
        &self.text
    }
}

/// TODO: docs
#[derive(Clone)]
pub struct AppliedDeletion {
    inner: cola::Deletion,
}

impl AppliedDeletion {
    #[inline]
    pub(crate) fn end(&self) -> cola::Anchor {
        todo!();
    }

    #[inline]
    pub(crate) fn new(inner: cola::Deletion) -> Self {
        Self { inner }
    }

    #[inline]
    pub(crate) fn start(&self) -> cola::Anchor {
        todo!();
    }
}
