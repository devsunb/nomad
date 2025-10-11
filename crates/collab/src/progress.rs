//! Contains types and traits related to reporting the progress of long-running
//! operations to the user.

use std::borrow::Cow;

use abs_path::{AbsPath, NodeName};
use editor::Context;

use crate::{CollabEditor, config};

/// A trait for types that can report the progress of long-running operations
/// to the user.
///
/// Editors that don't support progress reporting can set their
/// [`ProgressReporter`](CollabEditor::ProgressReporter) to `()`, which
/// implements this trait for all [`CollabEditor`]s by simply doing nothing.
pub trait ProgressReporter<Ed: CollabEditor> {
    /// Returns a new instance of the reporter.
    fn new(ctx: &mut Context<Ed>) -> Self;

    /// Reports a progress update for the [`Join`](crate::join::Join) action.
    fn report_join_progress(
        &mut self,
        state: JoinState<'_>,
        ctx: &mut Context<Ed>,
    );

    /// Reports a progress update for the [`Start`](crate::start::Start)
    /// action.
    fn report_start_progress(
        &mut self,
        state: StartState<'_>,
        ctx: &mut Context<Ed>,
    );
}

/// An enum representing the different progress states of the
/// [`Join`](crate::join::Join) action.
///
/// The variants form a linear sequence, and each variant is guaranteed to be
/// followed by either another instance of the same variant, or the next
/// variant in the sequence.
pub enum JoinState<'a> {
    /// The client is connecting to the server.
    ConnectingToServer {
        /// The address of the server to which the client is connecting.
        server_addr: config::ServerAddress<'a>,
    },

    /// The client has connected to the server, and is now waiting for it to
    /// respond with a [`Welcome`](collab_server::client::Welcome) message.
    JoiningSession,

    /// We've received the [`Welcome`](collab_server::client::Welcome) message,
    /// and are now waiting to receive the project from another peer in the
    /// session.
    ReceivingProject {
        /// The name of the project.
        project_name: Cow<'a, NodeName>,
    },

    /// We've received the project, and are now writing it to disk.
    WritingProject {
        /// The path to the root directory under which the project is being
        /// written.
        root_path: Cow<'a, AbsPath>,
    },

    /// The project has been written, and we're done.
    Done,
}

/// An enum representing the different progress states of the
/// [`Start`](crate::start::Start) action.
///
/// The variants form a linear sequence, and each variant is guaranteed to be
/// followed by either another instance of the same variant, or the next
/// variant in the sequence.
pub enum StartState<'a> {
    /// The client is connecting to the server.
    ConnectingToServer {
        /// The address of the server to which the client is connecting.
        server_addr: config::ServerAddress<'a>,
    },

    /// The client has connected to the server, and is now waiting for it to
    /// respond with a [`Welcome`](collab_server::client::Welcome) message.
    StartingSession,

    /// We've received the [`Welcome`](collab_server::client::Welcome) message,
    /// and are now reading the project rooted at the given path.
    ReadingProject {
        /// The path to the root of the project being read.
        root_path: Cow<'a, AbsPath>,
    },

    /// The project has been read, and we're done.
    Done,
}

impl JoinState<'_> {
    /// Returns a `'static` version of this [`JoinState`].
    pub fn to_owned(&self) -> JoinState<'static> {
        match self {
            Self::ConnectingToServer { server_addr } => {
                JoinState::ConnectingToServer {
                    server_addr: server_addr.to_owned(),
                }
            },
            Self::JoiningSession => JoinState::JoiningSession,
            Self::ReceivingProject { project_name } => {
                JoinState::ReceivingProject {
                    project_name: Cow::Owned(
                        project_name.clone().into_owned(),
                    ),
                }
            },
            Self::WritingProject { root_path } => JoinState::WritingProject {
                root_path: Cow::Owned(root_path.clone().into_owned()),
            },
            Self::Done => JoinState::Done,
        }
    }
}

impl StartState<'_> {
    /// Returns a `'static` version of this [`StartState`].
    pub fn to_owned(&self) -> StartState<'static> {
        match self {
            Self::ConnectingToServer { server_addr } => {
                StartState::ConnectingToServer {
                    server_addr: server_addr.to_owned(),
                }
            },
            Self::StartingSession => StartState::StartingSession,
            Self::ReadingProject { root_path } => StartState::ReadingProject {
                root_path: Cow::Owned(root_path.clone().into_owned()),
            },
            Self::Done => StartState::Done,
        }
    }
}

impl<Ed: CollabEditor> ProgressReporter<Ed> for () {
    fn new(_: &mut Context<Ed>) -> Self {}
    fn report_join_progress(&mut self, _: JoinState, _: &mut Context<Ed>) {}
    fn report_start_progress(&mut self, _: StartState, _: &mut Context<Ed>) {}
}
