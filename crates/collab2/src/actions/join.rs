use collab_server::message::Message;
use futures_util::StreamExt;
use nomad::ctx::NeovimCtx;
use nomad::diagnostics::DiagnosticMessage;
use nomad::{action_name, ActionName, AsyncAction, Shared};

use crate::session::{Project, Session};
use crate::session_id::SessionId;
use crate::session_status::SessionStatus;
use crate::Collab;

#[derive(Clone)]
pub(crate) struct Join {
    session_status: Shared<SessionStatus>,
}

impl Join {
    pub(crate) fn new(session_status: Shared<SessionStatus>) -> Self {
        Self { session_status }
    }
}

impl AsyncAction for Join {
    const NAME: ActionName = action_name!("join");
    type Args = SessionId;
    type Docs = ();
    type Module = Collab;

    async fn execute(
        &mut self,
        session_id: Self::Args,
        ctx: NeovimCtx<'_>,
    ) -> Result<(), JoinError> {
        let maybe_err = self.session_status.with_mut(|status| match status {
            SessionStatus::NotInSession => {
                *status = SessionStatus::Joining(session_id);
                None
            },
            SessionStatus::Starting => Some(JoinError::Starting),
            SessionStatus::Joining(_) => Some(JoinError::Joining),
            SessionStatus::InSession(project) => {
                Some(JoinError::InSession(project.clone()))
            },
        });

        if let Some(err) = maybe_err {
            return Err(err);
        }

        let mut session = Session::join().await;
        self.session_status.set(SessionStatus::InSession(session.project()));
        ctx.spawn(async move {
            let (tx, rx) = flume::unbounded::<Message>();
            let tx = tx.into_sink::<'static>();
            let rx = rx
                .into_stream::<'static>()
                .map(|msg| Ok::<_, core::convert::Infallible>(msg));
            let _err = session.run(tx, rx).await;
        });

        Ok(())
    }

    fn docs(&self) {}
}

pub(crate) enum JoinError {
    /// Couldn't join because another session is being started.
    Starting,

    /// Couldn't join because another session is being joined starting.
    Joining,

    /// Couldn't join because the user is already in a session.
    InSession(Shared<Project>),
}

impl From<JoinError> for DiagnosticMessage {
    fn from(_err: JoinError) -> Self {
        todo!();
    }
}
