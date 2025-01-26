use collab_server::SessionId;
use nvimx2::{Shared, fs, notify};
use smallvec::SmallVec;
use smol_str::ToSmolStr;

/// TODO: docs.
#[derive(Default, Clone)]
pub(crate) struct Sessions {
    inner: Shared<SessionsInner>,
}

/// TODO: docs.
pub(crate) struct SessionGuard {
    root: fs::AbsPathBuf,
    sessions: Sessions,
}

/// TODO: docs.
pub struct OverlappingSessionError {
    pub(crate) existing_root: fs::AbsPathBuf,
    pub(crate) new_root: fs::AbsPathBuf,
}

#[derive(Default)]
struct SessionsInner {
    sessions: SmallVec<[(fs::AbsPathBuf, SessionState); 2]>,
}

/// TODO: docs.
enum SessionState {
    Active(SessionId),
    Joining,
    Starting,
}

impl Sessions {
    pub(crate) fn start_guard(
        &self,
        _root: fs::AbsPathBuf,
    ) -> Result<SessionGuard, OverlappingSessionError> {
        todo!();
    }
}

impl SessionGuard {
    pub(crate) fn set_to_active(&self, _session_id: SessionId) {
        todo!();
    }
}

impl SessionsInner {
    fn ancestor_or_descendant_of(
        &self,
        _path: &fs::AbsPath,
    ) -> Option<&fs::AbsPath> {
        todo!();
    }
}

impl notify::Error for OverlappingSessionError {
    fn to_message(&self) -> (notify::Level, notify::Message) {
        let mut msg = notify::Message::new();
        msg.push_str("cannot start a new session at ")
            .push_info(self.new_root.to_smolstr())
            .push_str(", another one is already running at ")
            .push_info(self.existing_root.to_smolstr())
            .push_str(" (sessions cannot overlap)");
        (notify::Level::Error, msg)
    }
}
