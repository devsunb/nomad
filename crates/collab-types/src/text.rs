//! Contains the message types related to text files.

use smallvec::SmallVec;

use crate::{Counter, annotation, fs};

/// The message representing a cursor creation in a text file.
pub type CursorCreation = annotation::AnnotationCreation<Cursor>;

/// The message representing the movement of a cursor in a text file.
pub type CursorMove = annotation::AnnotationModification<Cursor>;

/// The message representing a cursor being removed from a text file.
pub type CursorRemoval = annotation::AnnotationDeletion;

/// The message representing a selection creation in a text file.
pub type SelectionCreation = annotation::AnnotationCreation<Selection>;

/// The message representing the movement of a selection in a text file.
pub type SelectionMove = annotation::AnnotationModification<Selection>;

/// The message representing a selection being removed from a text file.
pub type SelectionRemoval = annotation::AnnotationDeletion;

/// The type representing a cursor in a text file.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cursor {
    /// The cursor's anchor in the file.
    pub anchor: cola::Anchor,

    /// The cursor's sequence number, used to track which [`CursorMove`] is the
    /// most recent.
    pub sequence_num: Counter<u32>,
}

/// The type representing a selection in a text file.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Selection {
    /// The selection's start anchor in the file.
    pub start: cola::Anchor,

    /// The selection's end anchor in the file.
    pub end: cola::Anchor,

    /// The selection's sequence number, used to track which [`SelectionMove`]
    /// is the most recent.
    pub sequence_num: Counter<u32>,
}

/// The message representing an edit to a text file.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextEdit {
    /// The ID of the text file being edited.
    pub file_id: fs::GlobalFileId,

    /// The edit's deletions.
    ///
    /// If the edit only inserts text, this will be empty and
    /// [`insertions`](Self::insertions) will contains at least one element.
    ///
    /// If the edit deletes or replaces text, this will contain at least one
    /// element.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "SmallVec::is_empty")
    )]
    pub deletions: SmallVec<[cola::Deletion; 1]>,

    /// The edit's insertions.
    ///
    /// If the edit only deletes text, this will be empty and
    /// [`deletions`](Self::deletions) will contains at least one element.
    ///
    /// If the edit inserts or replaces text, this will contain at least one
    /// element.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "SmallVec::is_empty")
    )]
    pub insertions: SmallVec<[(cola::Insertion, smol_str::SmolStr); 1]>,
}
