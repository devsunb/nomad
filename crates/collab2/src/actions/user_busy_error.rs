use nomad::diagnostics::DiagnosticMessage;
use nomad::Shared;

use crate::session::Project;
use crate::session_status::SessionStatus;

/// The type of error returned when a "busy" user tries to start/join a new
/// session.
///
/// The generic parameter represents whether the user was trying to start or
/// join a session when the error occurred.
pub(crate) enum UserBusyError<const WAS_STARTING: bool> {
    /// Another session is being started.
    Starting,

    /// Another session is being joined.
    Joining,

    /// The user is already in a session.
    InSession(Shared<Project>),
}

impl<const WAS_STARTING: bool> TryFrom<&SessionStatus>
    for UserBusyError<WAS_STARTING>
{
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

impl<const WAS_STARTING: bool> From<UserBusyError<WAS_STARTING>>
    for DiagnosticMessage
{
    fn from(_err: UserBusyError<WAS_STARTING>) -> Self {
        todo!();
    }
}
