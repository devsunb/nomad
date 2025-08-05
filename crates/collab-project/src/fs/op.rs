use collab_types::PeerId;
use collab_types::fs::{
    DirectoryCreation,
    DirectoryDeletion,
    DirectoryMove,
    FileCreation,
    FileDeletion,
    FileMove,
    NewFileContents,
    Rename,
};

use crate::Project;
use crate::binary::BinaryContents;
use crate::fs::{FileContents, SyncActions};
use crate::symlink::SymlinkContents;
use crate::text::TextContents;

/// TODO: docs.
pub trait FsOp: Sized + private::Sealed {
    #[doc(hidden)]
    fn integrate_into(self, proj: &mut Project) -> SyncActions<'_>;
}

impl FsOp for FileCreation {
    #[inline]
    fn integrate_into(self, proj: &mut Project) -> SyncActions<'_> {
        let created_by = PeerId::new(self.performed_by());

        let creation = self.map_metadata(|contents| match contents {
            NewFileContents::Binary(bytes) => FileContents::Binary(
                BinaryContents::new_remote(bytes, created_by),
            ),
            NewFileContents::Symlink(target_path) => {
                FileContents::Symlink(SymlinkContents::new(target_path))
            },
            NewFileContents::Text(text) => FileContents::Text(Box::new(
                TextContents::new(created_by, text),
            )),
        });

        SyncActions::new(
            proj.peer_id(),
            proj.tree.integrate_file_creation(creation),
            &mut proj.backlogs,
            &mut proj.contexts,
        )
    }
}

impl FsOp for DirectoryCreation {
    #[doc(hidden)]
    fn integrate_into(self, proj: &mut Project) -> SyncActions<'_> {
        SyncActions::new(
            proj.peer_id(),
            proj.tree.integrate_directory_creation(self),
            &mut proj.backlogs,
            &mut proj.contexts,
        )
    }
}

impl FsOp for DirectoryDeletion {
    #[doc(hidden)]
    fn integrate_into(self, proj: &mut Project) -> SyncActions<'_> {
        SyncActions::new(
            proj.peer_id(),
            proj.tree.integrate_directory_deletion(self),
            &mut proj.backlogs,
            &mut proj.contexts,
        )
    }
}

impl FsOp for DirectoryMove {
    #[doc(hidden)]
    fn integrate_into(self, proj: &mut Project) -> SyncActions<'_> {
        SyncActions::new(
            proj.peer_id(),
            proj.tree.integrate_directory_move(self),
            &mut proj.backlogs,
            &mut proj.contexts,
        )
    }
}

impl FsOp for FileDeletion {
    #[doc(hidden)]
    fn integrate_into(self, proj: &mut Project) -> SyncActions<'_> {
        SyncActions::new(
            proj.peer_id(),
            proj.tree.integrate_file_deletion(self),
            &mut proj.backlogs,
            &mut proj.contexts,
        )
    }
}

impl FsOp for FileMove {
    #[doc(hidden)]
    fn integrate_into(self, proj: &mut Project) -> SyncActions<'_> {
        SyncActions::new(
            proj.peer_id(),
            proj.tree.integrate_file_move(self),
            &mut proj.backlogs,
            &mut proj.contexts,
        )
    }
}

impl FsOp for Rename {
    #[doc(hidden)]
    fn integrate_into(self, proj: &mut Project) -> SyncActions<'_> {
        SyncActions::new(
            proj.peer_id(),
            proj.tree.integrate_rename(self),
            &mut proj.backlogs,
            &mut proj.contexts,
        )
    }
}

mod private {
    use super::*;

    pub trait Sealed {}

    impl Sealed for FileCreation {}
    impl Sealed for DirectoryCreation {}
    impl Sealed for DirectoryDeletion {}
    impl Sealed for DirectoryMove {}
    impl Sealed for FileDeletion {}
    impl Sealed for FileMove {}
    impl Sealed for Rename {}
}
