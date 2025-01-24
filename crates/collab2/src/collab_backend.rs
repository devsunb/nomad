use nvimx2::backend::{Backend, BufferId};
use nvimx2::fs::AbsPathBuf;
use nvimx2::{AsyncCtx, notify};

/// TODO: docs.
pub trait CollabBackend: Backend {
    /// TODO: docs.
    type FindProjectRootError: notify::Error;

    /// TODO: docs.
    fn project_root(
        buffer_id: BufferId<Self>,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> impl Future<Output = Result<AbsPathBuf, Self::FindProjectRootError>>;
}

#[cfg(feature = "neovim")]
mod neovim {
    use mlua::{Function, Table};
    use nvimx2::fs;
    use nvimx2::neovim::{Neovim, NeovimBuffer, mlua};

    use super::*;

    pub enum NeovimFindProjectRootError {
        LspRootDirNotAbsolute(fs::AbsPathNotAbsoluteError),
    }

    impl CollabBackend for Neovim {
        type FindProjectRootError = NeovimFindProjectRootError;

        async fn project_root(
            buffer: NeovimBuffer,
            _ctx: &mut AsyncCtx<'_, Self>,
        ) -> Result<AbsPathBuf, Self::FindProjectRootError> {
            if let Some(root) = lsp_rootdir(buffer) {
                return root.as_str().try_into().map_err(
                    NeovimFindProjectRootError::LspRootDirNotAbsolute,
                );
            }

            todo!();
        }
    }

    /// Returns the root directory of the first language server attached to the
    /// given buffer, if any.
    fn lsp_rootdir(buffer: NeovimBuffer) -> Option<String> {
        let lua = mlua::lua();

        let get_clients = lua
            .globals()
            .get::<Table>("vim")
            .ok()?
            .get::<Table>("lsp")
            .ok()?
            .get::<Function>("get_clients")
            .ok()?;

        let opts = lua.create_table().ok()?;
        opts.set("bufnr", buffer).ok()?;

        get_clients
            .call::<Table>(opts)
            .ok()?
            .get::<Table>(1)
            .ok()?
            .get::<Table>("config")
            .ok()?
            .get::<String>("root_dir")
            .ok()
    }

    impl notify::Error for NeovimFindProjectRootError {
        fn to_message(&self) -> (notify::Level, notify::Message) {
            todo!()
        }
    }
}
