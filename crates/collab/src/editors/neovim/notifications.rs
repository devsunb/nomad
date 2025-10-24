//! Contains several constants and utility functions used to display
//! user-facing notifications.

use core::cell::OnceCell;
use core::fmt;

use abs_path::{AbsPath, AbsPathBuf};
use compact_str::ToCompactString;
use editor::Context;
use editor::context::BorrowState;
use fs::{Directory, Fs};
use neovim::{Neovim, notify};

/// The highlight group used to highlight a peer's handle in notifications.
pub(super) const PEER_HANDLE_HL_GROUP: &str = "Identifier";

/// The highlight group used to highlight a project's name in notifications.
pub(super) const PROJ_NAME_HL_GROUP: &str = "Directory";

/// The highlight group used to highlight paths in notifications.
const PATH_HL_GROUP: &str = "String";

pub(super) fn on_init(ctx: &mut Context<Neovim, impl BorrowState>) {
    // Pre-fetch the home path so that it'll be ready when `path_chunk` is
    // called.
    with_home_path(|_| (), ctx);
}

pub(super) fn path_chunk(
    path: &AbsPath,
    ctx: &mut Context<Neovim>,
) -> notify::Chunk {
    with_home_path(
        |home_dir| {
            let text = TildePath { path, home_dir }.to_compact_string();
            notify::Chunk::new_highlighted(text, PATH_HL_GROUP)
        },
        ctx,
    )
}

/// An [`AbsPath`] wrapper whose `Display` impl replaces the path's home
/// directory with `~`.
struct TildePath<'a> {
    path: &'a AbsPath,
    home_dir: Option<&'a AbsPath>,
}

impl fmt::Display for TildePath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some(home_dir) = self.home_dir else {
            return fmt::Display::fmt(&self.path, f);
        };

        if self.path.starts_with(home_dir) && self.path != home_dir {
            write!(f, "~{}", &self.path[home_dir.len()..])
        } else {
            fmt::Display::fmt(&self.path, f)
        }
    }
}

/// Calls the given function with the path to the user's home directory, or
/// `None` if the function has never been called before or if getting the home
/// path returned an error.
fn with_home_path<R>(
    fun: impl FnOnce(Option<&AbsPath>) -> R,
    ctx: &mut Context<Neovim, impl BorrowState>,
) -> R {
    thread_local! {
        static HOME_DIR: OnceCell<AbsPathBuf> = const { OnceCell::new() };
    }

    let (out, was_set) = HOME_DIR.with(|cell| match cell.get() {
        Some(path) => (fun(Some(path)), true),
        None => (fun(None), false),
    });

    if !was_set {
        ctx.spawn_and_detach(async |ctx| {
            if let Ok(Some(home)) = ctx.fs().home().await {
                HOME_DIR.with(|cell| cell.set(home.path().to_owned())).ok();
            }
        });
    }

    out
}
