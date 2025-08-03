//! TODO: docs.

mod directory;
mod file;
mod node;
mod op;
mod sync;

use collab_types::puff;
pub(crate) use directory::DirectoryContents;
pub use directory::{Children, Directory, DirectoryMut};
pub(crate) use file::FileContents;
pub use file::{File, FileMut};
pub use node::{Node, NodeMut};
pub use op::FsOp;
pub use sync::{
    Create,
    CreateAndResolve,
    Delete,
    Move,
    MoveAndResolve,
    Rename,
    RenameAndResolve,
    ResolveConflict,
    SyncAction,
    SyncActions,
};

pub(crate) type ProjectTree = puff::Fs<DirectoryContents, FileContents>;

pub(crate) type ProjectTreeBuilder =
    puff::builder::FsBuilder<DirectoryContents, FileContents>;

pub(crate) type PuffDirectory<'a, S> =
    puff::directory::Directory<'a, DirectoryContents, FileContents, S>;

pub(crate) type PuffDirectoryMut<'a, S> =
    puff::directory::DirectoryMut<'a, DirectoryContents, FileContents, S>;

pub(crate) type PuffFile<'a, S> =
    puff::file::File<'a, DirectoryContents, FileContents, S>;

pub(crate) type PuffFileMut<'a, S> =
    puff::file::FileMut<'a, DirectoryContents, FileContents, S>;

pub(crate) type PuffFileState<'a> =
    puff::file::FileState<'a, DirectoryContents, FileContents>;

pub(crate) type PuffFileStateMut<'a> =
    puff::file::FileMutState<'a, DirectoryContents, FileContents>;

pub(crate) type PuffChildren<'a, S> =
    puff::directory::Children<'a, DirectoryContents, FileContents, S>;

pub(crate) type PuffNode<'a, S> =
    puff::node::Node<'a, DirectoryContents, FileContents, S>;

pub(crate) type PuffNodeMut<'a, S> =
    puff::node::NodeMut<'a, DirectoryContents, FileContents, S>;

pub(crate) type PuffSyncActions<'a> =
    puff::diff::FsDiff<'a, DirectoryContents, FileContents>;

pub(crate) type PuffSyncAction<'a> =
    puff::diff::DiffOp<'a, DirectoryContents, FileContents>;

pub(crate) type PuffCreate<'a> =
    puff::diff::Create<'a, DirectoryContents, FileContents>;

pub(crate) type PuffCreateAndResolve<'a> =
    puff::diff::CreateAndResolve<'a, DirectoryContents, FileContents>;

pub(crate) type PuffDelete<'a> =
    puff::diff::Delete<'a, DirectoryContents, FileContents>;

pub(crate) type PuffMove<'a> =
    puff::diff::Move<'a, DirectoryContents, FileContents>;

pub(crate) type PuffMoveAndResolve<'a> =
    puff::diff::MoveAndResolve<'a, DirectoryContents, FileContents>;

pub(crate) type PuffRename<'a> =
    puff::diff::Rename<'a, DirectoryContents, FileContents>;

pub(crate) type PuffRenameAndResolve<'a> =
    puff::diff::RenameAndResolve<'a, DirectoryContents, FileContents>;

pub(crate) type PuffResolveConflict<'a> =
    puff::diff::ResolveConflict<'a, DirectoryContents, FileContents>;
