use collab_types::{PeerId, puff};
use puff::node::{Deleted, Visible};

use crate::abs_path::{AbsPathBuf, NodeName};
use crate::fs::{
    self,
    Directory,
    FileContents,
    Node,
    NodeMut,
    PuffFile,
    PuffFileMut,
    PuffNodeMut,
};
use crate::project::{Backlogs, Contexts};

/// TODO: docs.
pub struct SyncActions<'a> {
    inner: fs::PuffSyncActions<'a>,
    backlog: &'a mut Backlogs,
    ctxs: &'a mut Contexts,
    peer_id: PeerId,
}

/// TODO: docs.
pub enum SyncAction<'a> {
    /// TODO: docs.
    Create(Create<'a>),

    /// TODO: docs.
    CreateAndResolve(CreateAndResolve<'a>),

    /// TODO: docs.
    Delete(Delete<'a>),

    /// TODO: docs.
    Move(Move<'a>),

    /// TODO: docs.
    MoveAndResolve(MoveAndResolve<'a>),

    /// TODO: docs.
    Rename(Rename<'a>),

    /// TODO: docs.
    RenameAndResolve(RenameAndResolve<'a>),
}

/// TODO: docs.
pub struct Create<'a> {
    inner: fs::PuffCreate<'a>,
    ctxs: &'a Contexts,
}

/// TODO: docs.
pub struct CreateAndResolve<'a> {
    inner: fs::PuffCreateAndResolve<'a>,
    ctxs: &'a mut Contexts,
}

/// TODO: docs.
pub struct Delete<'a> {
    inner: fs::PuffDelete<'a>,
    ctxs: &'a Contexts,
}

/// TODO: docs.
pub struct Move<'a> {
    inner: fs::PuffMove<'a>,
    ctxs: &'a Contexts,
}

/// TODO: docs.
pub struct MoveAndResolve<'a> {
    inner: fs::PuffMoveAndResolve<'a>,
    ctxs: &'a mut Contexts,
}

/// TODO: docs.
pub struct Rename<'a> {
    inner: fs::PuffRename<'a>,
    ctxs: &'a Contexts,
}

/// TODO: docs.
pub struct RenameAndResolve<'a> {
    inner: fs::PuffRenameAndResolve<'a>,
    ctxs: &'a mut Contexts,
}

/// TODO: docs.
pub struct ResolveConflict<'a> {
    inner: fs::PuffResolveConflict<'a>,
    ctxs: &'a mut Contexts,
}

impl<'a> SyncActions<'a> {
    /// TODO: docs.
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn next(&mut self) -> Option<SyncAction<'_>> {
        self.inner.next().map(|action| match action {
            fs::PuffSyncAction::Create(mut inner) => {
                if let PuffNodeMut::File(mut file_mut) = inner.node_mut() {
                    integrate_backlogged_edits(
                        &mut file_mut,
                        self.backlog,
                        self.ctxs,
                        self.peer_id,
                    );
                    integrate_backlogged_annotations(
                        file_mut.as_file(),
                        self.ctxs,
                    );
                }
                SyncAction::Create(Create { inner, ctxs: self.ctxs })
            },
            fs::PuffSyncAction::CreateAndResolve(mut inner) => {
                if let PuffNodeMut::File(mut file_mut) =
                    inner.create().node_mut()
                {
                    integrate_backlogged_edits(
                        &mut file_mut,
                        self.backlog,
                        self.ctxs,
                        self.peer_id,
                    );
                    integrate_backlogged_annotations(
                        file_mut.as_file(),
                        self.ctxs,
                    );
                }
                SyncAction::CreateAndResolve(CreateAndResolve {
                    inner,
                    ctxs: self.ctxs,
                })
            },
            fs::PuffSyncAction::Delete(inner) => {
                SyncAction::Delete(Delete { inner, ctxs: self.ctxs })
            },
            fs::PuffSyncAction::Move(inner) => {
                SyncAction::Move(Move { inner, ctxs: self.ctxs })
            },
            fs::PuffSyncAction::MoveAndResolve(inner) => {
                SyncAction::MoveAndResolve(MoveAndResolve {
                    inner,
                    ctxs: self.ctxs,
                })
            },
            fs::PuffSyncAction::Rename(inner) => {
                SyncAction::Rename(Rename { inner, ctxs: self.ctxs })
            },
            fs::PuffSyncAction::RenameAndResolve(inner) => {
                SyncAction::RenameAndResolve(RenameAndResolve {
                    inner,
                    ctxs: self.ctxs,
                })
            },
        })
    }

    #[inline]
    pub(crate) fn new(
        peer_id: PeerId,
        inner: fs::PuffSyncActions<'a>,
        backlog: &'a mut Backlogs,
        ctxs: &'a mut Contexts,
    ) -> Self {
        Self { inner, backlog, ctxs, peer_id }
    }
}

fn integrate_backlogged_edits(
    file: &mut PuffFileMut<'_, Visible>,
    backlog: &mut Backlogs,
    ctxs: &mut Contexts,
    peer_id: PeerId,
) {
    let global_id = file.global_id();

    match file.metadata_mut() {
        FileContents::Binary(contents) => {
            if let Some(edit) = backlog.binary.take(global_id) {
                contents.integrate_edit(edit, &mut ctxs.binary);
            }
        },
        FileContents::Symlink(_) => {},
        FileContents::Text(contents) => {
            contents.decode(peer_id);
            for edit in backlog.text.take(global_id) {
                contents.integrate_edit(edit);
            }
        },
    }
}

fn integrate_backlogged_annotations(
    file: PuffFile<'_, Visible>,
    ctxs: &mut Contexts,
) {
    let local_id = file.local_id();
    let global_id = file.global_id();
    ctxs.text.cursors.integrate_file_creation(local_id, global_id);
    ctxs.text.selections.integrate_file_creation(local_id, global_id);
}

impl<'a> Create<'a> {
    /// TODO: docs.
    #[inline]
    pub fn node(&self) -> Node<'_> {
        Node::new(self.inner.node(), self.ctxs)
    }
}

impl<'a> CreateAndResolve<'a> {
    /// TODO: docs.
    #[inline]
    pub fn create(&mut self) -> Create<'_> {
        Create { inner: self.inner.create(), ctxs: self.ctxs }
    }

    /// TODO: docs.
    #[inline]
    pub fn into_resolve(self) -> ResolveConflict<'a> {
        ResolveConflict { inner: self.inner.into_resolve(), ctxs: self.ctxs }
    }
}

impl<'a> Delete<'a> {
    /// TODO: docs.
    #[inline]
    pub fn node(&self) -> Node<'_, Deleted> {
        Node::new(self.inner.node(), self.ctxs)
    }

    /// TODO: docs.
    #[inline]
    pub fn old_parent(&self) -> Directory<'_> {
        Directory::new(self.inner.old_parent(), self.ctxs)
    }

    /// TODO: docs.
    #[inline]
    pub fn old_path(&self) -> AbsPathBuf {
        self.inner.old_path()
    }
}

impl<'a> Move<'a> {
    /// TODO: docs.
    #[inline]
    pub fn new_path(&self) -> AbsPathBuf {
        self.inner.new_path()
    }

    /// TODO: docs.
    #[inline]
    pub fn node(&self) -> Node<'_> {
        Node::new(self.inner.node(), self.ctxs)
    }

    /// TODO: docs.
    #[inline]
    pub fn old_parent(&self) -> Directory<'_> {
        Directory::new(self.inner.old_parent(), self.ctxs)
    }

    /// TODO: docs.
    #[inline]
    pub fn old_path(&self) -> AbsPathBuf {
        self.inner.old_path()
    }
}

impl<'a> MoveAndResolve<'a> {
    /// TODO: docs.
    #[inline]
    pub fn r#move(&mut self) -> Move<'_> {
        Move { inner: self.inner.r#move(), ctxs: self.ctxs }
    }

    /// TODO: docs.
    #[inline]
    pub fn into_resolve(self) -> ResolveConflict<'a> {
        ResolveConflict { inner: self.inner.into_resolve(), ctxs: self.ctxs }
    }
}

impl<'a> Rename<'a> {
    /// TODO: docs.
    #[inline]
    pub fn new_path(&self) -> AbsPathBuf {
        self.inner.new_path()
    }

    /// TODO: docs.
    #[inline]
    pub fn node(&self) -> Node<'_> {
        Node::new(self.inner.node(), self.ctxs)
    }

    /// TODO: docs.
    #[inline]
    pub fn old_name(&self) -> &NodeName {
        self.inner.old_name()
    }

    /// TODO: docs.
    #[inline]
    pub fn old_path(&self) -> AbsPathBuf {
        self.inner.old_path()
    }

    /// TODO: docs.
    #[inline]
    pub fn parent(&self) -> Directory<'_> {
        Directory::new(self.inner.parent(), self.ctxs)
    }
}

impl<'a> RenameAndResolve<'a> {
    /// TODO: docs.
    #[inline]
    pub fn rename(&mut self) -> Rename<'_> {
        Rename { inner: self.inner.rename(), ctxs: self.ctxs }
    }

    /// TODO: docs.
    #[inline]
    pub fn into_resolve(self) -> ResolveConflict<'a> {
        ResolveConflict { inner: self.inner.into_resolve(), ctxs: self.ctxs }
    }
}

impl<'a> ResolveConflict<'a> {
    /// TODO: docs.
    #[inline]
    pub fn assume_resolved(self) -> Result<(), Self> {
        self.inner
            .assume_resolved()
            .map_err(|inner| Self { inner, ctxs: self.ctxs })
    }

    /// TODO: docs.
    #[inline]
    pub fn conflicting_node(&self) -> Node<'_> {
        Node::new(self.inner.conflicting_node(), self.ctxs)
    }

    /// TODO: docs.
    #[inline]
    pub fn conflicting_node_mut(&mut self) -> NodeMut<'_, Visible> {
        NodeMut::new(self.inner.conflicting_node_mut(), self.ctxs)
    }

    /// TODO: docs.
    #[inline]
    pub fn existing_node(&self) -> Node<'_> {
        Node::new(self.inner.existing_node(), self.ctxs)
    }

    /// TODO: docs.
    #[inline]
    pub fn existing_node_mut(&mut self) -> NodeMut<'_, Visible> {
        NodeMut::new(self.inner.existing_node_mut(), self.ctxs)
    }
}
