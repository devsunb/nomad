use core::any::type_name;

use collab_server::message::Message;
use nvimx::ctx::{BufferCtx, ShouldDetach};
use nvimx::event::{CursorArgs, CursorKind};
use nvimx::plugin::{action_name, Action, ActionName};
use nvimx::Shared;

use super::Project;
use crate::Collab;

#[derive(Clone)]
pub(super) struct SyncCursor {
    pub(super) message_tx: flume::Sender<Message>,
    pub(super) project: Shared<Project>,
    pub(super) should_detach: Shared<ShouldDetach>,
}

impl Action for SyncCursor {
    const NAME: ActionName = action_name!("synchronize-cursor");
    type Args = CursorArgs;
    type Ctx<'a> = BufferCtx<'a>;
    type Docs = ();
    type Module = Collab;
    type Return = ShouldDetach;

    fn execute<'a>(
        &'a mut self,
        cursor: Self::Args,
        _: Self::Ctx<'a>,
    ) -> Self::Return {
        let maybe_message = self.project.with_mut(|proj| {
            if cursor.moved_by == proj.actor_id {
                return None;
            }

            Some(match cursor.kind {
                CursorKind::Created(byte_offset) => {
                    let Some(mut file) =
                        proj.file_mut_of_buffer_id(cursor.buffer_id)
                    else {
                        panic!(
                            "couldn't convert BufferId to file in {}",
                            type_name::<Self>()
                        );
                    };
                    let (cursor_id, creation) =
                        file.sync_created_cursor(byte_offset.into_u64());
                    assert!(
                        proj.local_cursor_id.is_none(),
                        "creating a new cursor when another already exists, \
                         but Neovim only supports a single cursor"
                    );
                    proj.local_cursor_id = Some(cursor_id);
                    Message::CreatedCursor(creation)
                },
                CursorKind::Moved(byte_offset) => Message::MovedCursor(
                    proj.local_cursor().sync_relocated(byte_offset)?,
                ),
                CursorKind::Removed => {
                    Message::RemovedCursor(proj.local_cursor().sync_removed())
                },
            })
        });

        if let Some(message) = maybe_message {
            if self.message_tx.send(message).is_err() {
                self.should_detach.set(ShouldDetach::Yes);
            }
        }

        self.should_detach.get()
    }

    fn docs(&self) {}
}
