use crate::SessionId;

/// Tracks whether there is an active collaborative editing session.
#[derive(Copy, Clone, Debug, Default)]
pub(crate) enum Activity {
    /// There is an active session with the given ID.
    Active(SessionId),

    /// There is no active session.
    #[default]
    Inactive,

    /// There is no active session yet, but we're in the process of joining
    /// one.
    Joining,

    /// There is no active session yet, but we're in the process of starting
    /// one.
    Starting,
}
