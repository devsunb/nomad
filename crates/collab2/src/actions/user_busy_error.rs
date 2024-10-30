use nomad::diagnostics::DiagnosticMessage;
use nomad::Shared;

use crate::session::Project;
use crate::session_status::SessionStatus;

/// The type of error returned when a "busy" user tries to join/start a new
/// session.
pub(crate) enum UserBusyError {
    /// Another session is being started.
    Starting,

    /// Another session is being joined.
    Joining,

    /// The user is already in a session.
    InSession(Shared<Project>),
}

impl TryFrom<&SessionStatus> for UserBusyError {
    type Error = ();

    fn try_from(status: &SessionStatus) -> Result<Self, Self::Error> {
        match status {
            SessionStatus::Starting => Ok(Self::Starting),
            SessionStatus::Joining(_) => Ok(Self::Joining),
            SessionStatus::InSession(p) => Ok(Self::InSession(p.clone())),
            _ => Err(()),
        }
    }
}

impl From<UserBusyError> for DiagnosticMessage {
    fn from(_err: UserBusyError) -> Self {
        todo!();
    }
}
