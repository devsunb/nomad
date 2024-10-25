use nomad::ctx::NeovimCtx;
use nomad::{action_name, Action, ActionName};

use crate::Collab;

#[derive(Clone)]
pub(crate) struct Join {}

impl Join {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Action for Join {
    const NAME: ActionName = action_name!("join");
    type Args = ();
    type Docs = ();
    type Module = Collab;
    type Return = ();

    fn execute(&mut self, _args: Self::Args) {
        todo!()
    }

    fn docs(&self) -> Self::Docs {}
}
