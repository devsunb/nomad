use editor::Context;
use editor::command::ToCompletionFn;
use editor::context::Borrowed;
use editor::module::Action;

use crate::{VERSION, VersionEditor};

/// TODO: docs.
#[derive(Default)]
pub struct EmitVersion {}

impl EmitVersion {
    /// Creates a new [`EmitVersion`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl<Ed: VersionEditor> Action<Ed> for EmitVersion {
    const NAME: &str = "version";

    type Args<'args> = ();
    type Return = ();

    fn call(&mut self, _: Self::Args<'_>, ctx: &mut Context<Ed, Borrowed>) {
        Ed::emit_version(VERSION, ctx);
    }
}

impl<Ed: VersionEditor> ToCompletionFn<Ed> for EmitVersion {
    fn to_completion_fn(&self) {}
}
