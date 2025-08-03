//! TODO: docs.

use crate::{PeerId, fs};

/// The message representing the creation of an annotation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationCreation<T> {
    /// The annotation's ID.
    pub annotation_id: AnnotationId,

    /// The ID of the file the annotation is associated with.
    pub file_id: fs::GlobalFileId,

    /// The data associated with the annotation.
    pub data: T,
}

/// The message representing the deletion of an annotation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationDeletion {
    /// The annotation's ID.
    pub annotation_id: AnnotationId,
}

/// The message representing the modification of an annotation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationModification<T> {
    /// The annotation's ID.
    pub annotation_id: AnnotationId,

    /// The data associated with the annotation's modification.
    pub data: T,
}

/// A unique identifier for an annotation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnnotationId {
    /// The ID of the peer that created the annotation.
    pub created_by: PeerId,

    /// The annotation's sequence number.
    ///
    /// Together with `created_by`, this uniquely identifies the annotation.
    pub sequence_num: u64,
}

impl PartialOrd for AnnotationId {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnnotationId {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.created_by
            .into_u64()
            .cmp(&other.created_by.into_u64())
            .then(self.sequence_num.cmp(&other.sequence_num))
    }
}
