#![allow(missing_docs)]

use editor::Context;
use editor::context::Borrowed;
use neovim::Neovim;
use neovim::notify::{self, NotifyContextExt};

use crate::{Version, VersionEditor};

impl VersionEditor for Neovim {
    fn emit_version(version: Version, ctx: &mut Context<Self, Borrowed>) {
        ctx.notify_info(version);
    }
}

impl From<Version> for notify::Chunks {
    fn from(version: Version) -> Self {
        format_args!("{version}").into()
    }
}
