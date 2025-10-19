//! TODO: docs.

use clipboard::Clipboard;
use compact_str::ToCompactString;
use editor::Context;
use editor::command::ToCompletionFn;
use editor::module::AsyncAction;

use crate::SessionId;
use crate::collab::Collab;
use crate::editors::{ActionForSelectedSession, CollabEditor};
use crate::session::{NoActiveSessionError, Sessions};

/// An `Action` that pastes the [`SessionId`](crate::editors::SessionId) of any
/// active session to the user's clipboard.
#[derive(cauchy::Clone)]
pub struct CopyId<Ed: CollabEditor> {
    sessions: Sessions<Ed>,
}

impl<Ed: CollabEditor> CopyId<Ed> {
    pub(crate) async fn call_inner(
        &self,
        ctx: &mut Context<Ed>,
    ) -> Result<Option<SessionId<Ed>>, CopyIdError<Ed>> {
        let Some((_, session_id)) = self
            .sessions
            .select(ActionForSelectedSession::CopySessionId, ctx)
            .await?
        else {
            return Ok(None);
        };

        match Self::copy_id(&session_id, ctx) {
            Ok(()) => Ok(Some(session_id)),
            Err(err) => Err(CopyIdError::CopySessionId(err, session_id)),
        }
    }

    pub(crate) fn copy_id(
        session_id: &SessionId<Ed>,
        ctx: &mut Context<Ed>,
    ) -> Result<(), <Ed::Clipboard as Clipboard>::SetError> {
        ctx.with_editor(|ed| {
            ed.clipboard().set(session_id.to_compact_string())
        })
    }
}

impl<Ed: CollabEditor> AsyncAction<Ed> for CopyId<Ed> {
    const NAME: &str = "copy-id";

    type Args = ();

    async fn call(&mut self, _: Self::Args, ctx: &mut Context<Ed>) {
        match self.call_inner(ctx).await {
            Ok(Some(session_id)) => Ed::on_copied_session_id(session_id, ctx),
            Ok(None) => {},
            Err(err) => Ed::on_copy_session_id_error(err, ctx),
        }
    }
}

/// The type of error that can occur when [`CopyId`] fails.
#[derive(
    cauchy::Debug, derive_more::Display, cauchy::Error, cauchy::PartialEq,
)]
pub enum CopyIdError<Ed: CollabEditor> {
    /// TODO: docs.
    #[display("Couldn't copy {_0} to clipboard: {_1}")]
    CopySessionId(<Ed::Clipboard as Clipboard>::SetError, SessionId<Ed>),

    /// TODO: docs.
    #[display("{}", NoActiveSessionError)]
    NoActiveSession,
}

impl<Ed: CollabEditor> From<&Collab<Ed>> for CopyId<Ed> {
    fn from(collab: &Collab<Ed>) -> Self {
        Self { sessions: collab.sessions.clone() }
    }
}

impl<Ed: CollabEditor> ToCompletionFn<Ed> for CopyId<Ed> {
    fn to_completion_fn(&self) {}
}

impl<Ed: CollabEditor> From<NoActiveSessionError> for CopyIdError<Ed> {
    fn from(_: NoActiveSessionError) -> Self {
        Self::NoActiveSession
    }
}
