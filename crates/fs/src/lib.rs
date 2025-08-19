//! TODO: docs.

mod directory;
mod file;
mod fs;
mod metadata;
mod node;
mod node_kind;
#[cfg(feature = "os-fs")]
pub mod os;
mod symlink;

pub use directory::{
    Directory,
    DirectoryEvent,
    NodeCreation,
    NodeDeletion,
    NodeMove,
    ReadNodeError,
    ReplicateError,
};
pub use file::{File, FileEvent, FileIdChange, FileModification};
pub use fs::{
    DeleteNodeError,
    Fs,
    GetDirError,
    MoveNodeError,
    ReadFileError,
    ReadFileToStringError,
};
pub use metadata::{Metadata, MetadataNameError};
pub use node::{Node, NodeDeleteError, NodeMoveError};
pub use node_kind::NodeKind;
pub use symlink::Symlink;
