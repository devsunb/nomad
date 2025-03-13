use nvimx::diagnostics::DiagnosticMessage;
use nvimx::plugin::{SubCommandArg, SubCommandArgs};

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct SessionId(collab_server::SessionId);

impl SessionId {
    pub(crate) fn into_inner(self) -> collab_server::SessionId {
        self.0
    }
}

impl TryFrom<SubCommandArgs<'_>> for SessionId {
    type Error = DiagnosticMessage;

    fn try_from(args: SubCommandArgs) -> Result<Self, Self::Error> {
        <[Self; 1]>::try_from(args).map(|[id]| id).map_err(Into::into)
    }
}

impl TryFrom<SubCommandArg<'_>> for SessionId {
    type Error = DiagnosticMessage;

    fn try_from(arg: SubCommandArg) -> Result<Self, Self::Error> {
        arg.parse().map(Self).map_err(|err| {
            let mut msg = DiagnosticMessage::new();
            msg.push_str(err.to_string());
            msg
        })
    }
}
