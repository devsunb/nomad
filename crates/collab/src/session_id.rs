use nomad::diagnostics::DiagnosticMessage;
use nomad::CommandArgs;

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct SessionId(collab_server::SessionId);

impl SessionId {
    pub(crate) fn into_inner(self) -> collab_server::SessionId {
        self.0
    }
}

impl TryFrom<&mut CommandArgs> for SessionId {
    type Error = DiagnosticMessage;

    fn try_from(args: &mut CommandArgs) -> Result<Self, Self::Error> {
        let [id] = <&[String; 1]>::try_from(args)?;
        id.parse().map(Self).map_err(|err| {
            let mut msg = DiagnosticMessage::new();
            msg.push_str(err.to_string());
            msg
        })
    }
}
