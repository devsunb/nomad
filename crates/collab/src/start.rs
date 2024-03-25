use nomad::prelude::*;

use crate::{Collab, Config, Session, SessionId, SessionState};

/// TODO: docs
#[derive(Clone)]
pub(crate) struct Start {
    config: Get<Config>,

    /// The current collab session, if there is one.
    state: Get<SessionState>,

    /// TODO: docs
    set_state: Set<SessionState>,
}

impl Start {
    pub(crate) fn new(config: Get<Config>) -> Self {
        let (state, set_state) = new_input(SessionState::Inactive);
        Self { config, state, set_state }
    }
}

#[async_action]
impl Action<Collab> for Start {
    const NAME: ActionName = action_name!("join");

    type Args = ();

    type Return = ();

    async fn execute(&self, _: ()) -> Result<(), StartError> {
        if let &SessionState::Active(session_id) = self.state.get() {
            return Err(StartError::ExistingSession(session_id));
        }

        let session = Session::start(self.config.clone()).await?;

        self.set_state.set(SessionState::Active(session.id()));

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StartError {
    #[error("there is already an active session with ID {0}")]
    ExistingSession(SessionId),

    #[error(transparent)]
    Start(#[from] crate::session::StartError),
}

impl From<StartError> for WarningMsg {
    fn from(_err: StartError) -> Self {
        todo!();
    }
}
