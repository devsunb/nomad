use core::fmt;

use collab_fs::{AbsUtf8Path, AbsUtf8PathBuf};
use collab_messaging::PeerId;
use collab_server::{JoinRequest, SessionId};
use nohash::IntSet as NoHashSet;
use nomad2::{Context, Editor};
use nomad_server::client::{Joined, Receiver, Sender};
use nomad_server::Io;
use root_finder::markers::Git;
use root_finder::Finder;

use crate::SessionError;

pub(crate) struct Session<E: Editor> {
    ctx: Context<E>,

    /// The session's ID.
    id: SessionId,

    /// The peers currently in the session, including the local peer but
    /// excluding the server.
    peers: NoHashSet<PeerId>,

    /// A receiver for receiving messages from the server.
    receiver: Receiver,

    /// The server's ID.
    server_id: PeerId,

    /// A sender for sending messages to the server.
    sender: Sender,
}

impl<E: Editor> Session<E> {
    pub(crate) async fn join(
        id: SessionId,
        ctx: Context<E>,
    ) -> Result<Self, SessionError> {
        todo!();
    }

    pub(crate) async fn run(self) -> Result<(), SessionError> {
        todo!();
    }

    pub(crate) async fn start(
        ctx: Context<E>,
    ) -> Result<Self, StartSessionError> {
        let Some(file) = ctx.buffer().file() else {
            return Err(StartSessionError::NotInFile);
        };

        let Some(root_candidate) =
            Finder::find_root(file.path(), &Git, ctx.fs()).await?
        else {
            return Err(StartSessionError::CouldntFindRoot(
                file.path().to_owned(),
            ));
        };

        let root = match ctx.ask_user(ConfirmStart(&root_candidate)).await {
            Ok(true) => root_candidate,
            Ok(false) => return Err(StartSessionError::UserCancelled),
            Err(err) => return Err(err.into()),
        };

        let joined = Io::connect()
            .await?
            .authenticate(())
            .await?
            .join(JoinRequest::StartNewSession)
            .await?;

        Ok(Self::new(ctx, joined))
    }

    fn new(ctx: Context<E>, joined: Joined) -> Self {
        let Joined { sender, receiver, join_response, peers } = joined;
        Self {
            ctx,
            id: join_response.session_id,
            peers,
            receiver,
            sender,
            server_id: join_response.server_id,
        }
    }
}

struct ConfirmStart<'path>(&'path AbsUtf8Path);

impl fmt::Display for ConfirmStart<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "found root of project at '{}'. Start session?", self.0)
    }
}

enum StartSessionError {
    /// The session was started in a non-file buffer.
    NotInFile,

    /// It was not possible to find the root of the project containing the file
    /// at the given path.
    CouldntFindRoot(AbsUtf8PathBuf),

    /// We asked the user for confirmation to start the session, but they
    /// cancelled.
    UserCancelled,
}

impl<E: Editor> Drop for Session<E> {
    fn drop(&mut self) {
        todo!();
    }
}
