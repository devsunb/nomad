use collab::messages::{
    Deletion as CollabDeletion,
    Insertion as CollabInsertion,
    OutboundMessage,
    Session,
};
use nomad::editor::{BufferSnapshot, RemoteDeletion, RemoteInsertion};
use nomad::streams::AppliedEdit;

/// Exactly the same as the [`Into`] trait, but it lets us convert `T -> U` even
/// when neither `T` nor `U` are defined in this crate.
pub(crate) trait Convert<T> {
    fn convert(self) -> T;
}

impl Convert<OutboundMessage> for AppliedEdit {
    fn convert(self) -> OutboundMessage {
        todo!();
    }
}

impl Convert<RemoteDeletion> for CollabDeletion {
    fn convert(self) -> RemoteDeletion {
        todo!();
    }
}

impl Convert<RemoteInsertion> for CollabInsertion {
    fn convert(self) -> RemoteInsertion {
        todo!();
    }
}

impl Convert<Session> for BufferSnapshot {
    fn convert(self) -> Session {
        todo!();
    }
}
