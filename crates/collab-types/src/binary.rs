//! Contains the message types related to binary files.

use crate::lamport::LamportTimestamp;
use crate::{PeerId, fs};

/// The message representing an edit to a binary file.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryEdit {
    /// The ID of the file being edited.
    pub file_id: fs::GlobalFileId,

    /// The file's new contents after the edit.
    pub new_contents: bytes::Bytes,

    /// The edit's timestamp.
    pub timestamp: BinaryEditTimestamp,
}

/// The timestamp of a [`BinaryEdit`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryEditTimestamp {
    /// The Lamport timestamp of the peer that performed the edit at the time
    /// of the edit.
    pub edited_at: LamportTimestamp,

    /// The ID of the peer that performed the edit.
    pub edited_by: PeerId,
}

impl PartialOrd for BinaryEditTimestamp {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BinaryEditTimestamp {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.edited_at
            .cmp(&other.edited_at)
            .then_with(|| self.edited_by.cmp(&other.edited_by))
    }
}
