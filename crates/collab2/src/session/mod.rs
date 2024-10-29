mod detach_buffer_actions;
mod peer_selection;
mod peer_tooltip;
mod project;
mod register_buffer_actions;
mod sync_cursor;
mod sync_replacement;

use collab_server::message::Message;
use detach_buffer_actions::DetachBufferActions;
use futures_util::{
    pin_mut,
    select,
    FutureExt,
    Sink,
    SinkExt,
    Stream,
    StreamExt,
};
use nomad::autocmds::{BufAdd, BufUnload};
use nomad::ctx::NeovimCtx;
use nomad::{BufferId, Event, Shared};
use peer_selection::PeerSelection;
use peer_tooltip::PeerTooltip;
use project::Project;
use register_buffer_actions::RegisterBufferActions;
use sync_cursor::SyncCursor;
use sync_replacement::SyncReplacement;
use tracing::error;

/// TODO: docs.
pub(crate) struct Session {
    neovim_ctx: NeovimCtx<'static>,
    project: Shared<Project>,
}

impl Session {
    pub(crate) fn new() -> Self {
        todo!();
    }

    pub(crate) async fn run<Tx, Rx>(&mut self, remote_tx: Tx, remote_rx: Rx)
    where
        Tx: Sink<Message, Error = core::convert::Infallible>,
        Rx: Stream<Item = Message>,
    {
        let (local_tx, local_rx) = flume::unbounded();

        let mut register_buffer_actions = RegisterBufferActions {
            message_tx: local_tx.clone(),
            project: self.project.clone(),
        };

        let detach_buffer_actions = DetachBufferActions {
            message_tx: local_tx,
            project: self.project.clone(),
        };

        for buffer_id in BufferId::opened() {
            register_buffer_actions.register_actions(buffer_id);
        }

        BufAdd::new(register_buffer_actions)
            .register(self.neovim_ctx.reborrow());

        BufUnload::new(detach_buffer_actions)
            .register(self.neovim_ctx.reborrow());

        pin_mut!(remote_rx);
        pin_mut!(remote_tx);

        loop {
            select! {
                remote_message = remote_rx.next().fuse() => {
                    if let Some(remote_message) = remote_message {
                        self.integrate_message(remote_message);
                    }
                },
                local_message = local_rx.recv_async().fuse() => {
                    if let Ok(local_message) = local_message {
                        remote_tx
                            .send(local_message)
                            .await
                            .expect("Infallible");
                    }
                },
            }
        }
    }

    fn integrate_message(&self, message: Message) {
        use Message::*;
        match message {
            CreatedCursor(msg) => self.integrate_created_cursor(msg),
            CreatedDirectory(msg) => self.integrate_created_directory(msg),
            CreatedFile(msg) => self.integrate_created_file(msg),
            CreatedSelection(msg) => self.integrate_created_selection(msg),
            EditedBuffer(msg) => self.integrate_edited_buffer(msg),
            MovedCursor(msg) => self.integrate_moved_cursor(msg),
            MovedDirectory(msg) => self.integrate_moved_directory(msg),
            MovedFile(msg) => self.integrate_moved_file(msg),
            MovedSelection(msg) => self.integrate_moved_selection(msg),
            PeerDisconnected(msg) => self.integrate_peer_disconnected(msg),
            PeerJoined(msg) => self.integrate_peer_joined(msg),
            PeerLeft(msg) => self.integrate_peer_left(msg),
            ProjectRequest(msg) => self.integrate_project_request(msg),
            RemovedCursor(msg) => self.integrate_removed_cursor(msg),
            RemovedSelection(msg) => self.integrate_removed_selection(msg),
            RemovedFile(msg) => self.integrate_removed_file(msg),
            RemovedDirectory(msg) => self.integrate_removed_directory(msg),
            ProjectResponse(msg) => {
                error!("received unexpected ProjectResponse: {:?}", msg)
            },
        }
    }

    fn integrate_created_cursor(&self, cursor_creation: e31e::CursorCreation) {
        self.project
            .with_mut(|p| p.integrate_cursor_creation(cursor_creation));
    }

    fn integrate_created_directory(
        &self,
        directory_creation: e31e::DirectoryCreation,
    ) {
        self.project.with_mut(|p| {
            if let Some(_create_directory) =
                p.replica.integrate_directory_creation(directory_creation)
            {
                todo!();
            }
        });
    }

    fn integrate_created_file(&self, file_creation: e31e::FileCreation) {
        let Some((_file_path, _replacements)) = self.project.with_mut(|p| {
            p.replica.integrate_file_creation(file_creation).map(
                |create_file| (create_file.file.path(), create_file.hunks),
            )
        }) else {
            return;
        };
    }

    fn integrate_created_selection(
        &self,
        selection_creation: e31e::SelectionCreation,
    ) {
        self.project
            .with_mut(|p| p.integrate_selection_creation(selection_creation));
    }

    fn integrate_edited_buffer(&self, edit: e31e::Edit) {
        if let Some((_file_path, _replacements)) =
            self.project.with_mut(|p| p.integrate_edit(edit))
        {
            todo!();
        }
    }

    fn integrate_moved_cursor(
        &self,
        cursor_relocation: e31e::CursorRelocation,
    ) {
        self.project
            .with_mut(|p| p.integrate_cursor_relocation(cursor_relocation));
    }

    fn integrate_moved_directory(
        &self,
        directory_relocation: e31e::DirectoryRelocation,
    ) {
        if let Some((_old_path, _new_path)) = self.project.with_mut(|p| {
            p.replica.integrate_directory_relocation(directory_relocation).map(
                |relocate_dir| {
                    let dir = &relocate_dir.directory;
                    let mut new_path = dir.path();
                    new_path.push(dir.name().expect("can't be root"));
                    (relocate_dir.old_path, new_path)
                },
            )
        }) {
            todo!();
        }
    }

    fn integrate_moved_file(&self, file_relocation: e31e::FileRelocation) {
        if let Some((_old_path, _new_path)) = self.project.with_mut(|p| {
            p.replica.integrate_file_relocation(file_relocation).map(
                |relocate_file| {
                    let file = &relocate_file.file;
                    let mut new_path = file.path();
                    new_path.push(file.name());
                    (relocate_file.old_path, new_path)
                },
            )
        }) {
            todo!();
        }
    }

    fn integrate_moved_selection(
        &self,
        selection_relocation: e31e::SelectionRelocation,
    ) {
        self.project.with_mut(|p| {
            p.integrate_selection_relocation(selection_relocation)
        });
    }

    fn integrate_peer_disconnected(&self, peer_id: e31e::PeerId) {
        self.project.with_mut(|p| p.integrate_peer_left(peer_id));
    }

    fn integrate_peer_joined(&self, peer: collab_server::message::Peer) {
        self.project.with_mut(|p| p.integrate_peer_joined(peer));
    }

    fn integrate_peer_left(&self, peer_id: e31e::PeerId) {
        self.project.with_mut(|p| p.integrate_peer_left(peer_id));
    }

    fn integrate_project_request(
        &self,
        project_request: collab_server::message::ProjectRequest,
    ) {
        todo!();
    }

    fn integrate_removed_cursor(&self, cursor_removal: e31e::CursorRemoval) {
        self.project.with_mut(|p| p.integrate_cursor_removal(cursor_removal));
    }

    fn integrate_removed_selection(
        &self,
        selection_removal: e31e::SelectionRemoval,
    ) {
        self.project
            .with_mut(|p| p.integrate_selection_removal(selection_removal));
    }

    fn integrate_removed_file(&self, file_removal: e31e::FileRemoval) {
        if let Some(_remove_file) = self
            .project
            .with_mut(|p| p.replica.integrate_file_removal(file_removal))
        {
            todo!();
        }
    }

    fn integrate_removed_directory(
        &self,
        directory_removal: e31e::DirectoryRemoval,
    ) {
        if let Some(_remove_directory) = self.project.with_mut(|p| {
            p.replica.integrate_directory_removal(directory_removal)
        }) {
            todo!();
        }
    }
}
