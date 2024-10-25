use nomad::ctx::NeovimCtx;
use nomad::{action_name, Action, ActionName};

use crate::Collab;

#[derive(Clone)]
pub(crate) struct Start {}

impl Start {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Action for Start {
    const NAME: ActionName = action_name!("start");
    type Args = ();
    type Docs = ();
    type Module = Collab;
    type Return = ();

    fn execute(&mut self, _args: Self::Args) {
        todo!()
    }

    fn docs(&self) -> Self::Docs {}
}
