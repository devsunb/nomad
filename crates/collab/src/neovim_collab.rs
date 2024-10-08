use std::borrow::Cow;

use collab_fs::AbsUtf8Path;
use futures_util::stream::{select, Select};
use nomad::neovim::events::{CommandEvent, ConfigEvent, FunctionEvent};
use nomad::neovim::{self, command, function, module_api, ModuleApi, Neovim};
use nomad::{module_name, Buffer, Context, Module, ModuleName, Subscription};

use crate::collab_editor::CollabEditor;
use crate::events::{self, JoinSession, StartSession};
use crate::{Collab, Config};

/// TODO: docs.
pub struct NeovimCollab(Collab<Neovim>);

impl Module<Neovim> for NeovimCollab {
    const NAME: ModuleName = module_name!("collab");

    type Config = Config;

    fn init(ctx: &Context<Neovim>) -> (Self, ModuleApi) {
        let (api, config_stream) = module_api::<Self>(ctx);

        let (join_cmd, join_cmd_sub) = command::<JoinSession>(ctx);
        let (start_cmd, start_cmd_sub) = command::<StartSession>(ctx);

        let (join_fn, join_fn_sub) = function::<JoinSession>(ctx);
        let (start_fn, start_fn_sub) = function::<StartSession>(ctx);

        let collab = Self(Collab {
            ctx: ctx.clone(),
            config: Config::default(),
            config_stream,
            join_stream: select(join_cmd_sub, join_fn_sub),
            start_stream: select(start_cmd_sub, start_fn_sub),
        });

        let api = api
            .with_command(join_cmd)
            .with_command(start_cmd)
            .with_function(join_fn)
            .with_function(start_fn);

        (collab, api)
    }

    async fn run(&mut self, _: &Context<Neovim>) {
        self.0.run().await;
    }
}

impl CollabEditor for Neovim {
    type CursorId = ();
    type FileId = neovim::BufferId;
    type SelectionId = ();

    type OpenFiles = events::open_file::NeovimOpenFiles;
    type CloseFiles = events::close_file::NeovimCloseFiles;
    type Cursors = events::cursor::NeovimCursors;
    type Edits = events::edit::NeovimEdits;
    type Selections = events::selection::NeovimSelections;

    fn open_files(&mut self) -> Self::OpenFiles {
        todo!();
    }

    fn close_files(&mut self) -> Self::CloseFiles {
        todo!();
    }

    fn cursors(&mut self, _file_id: &Self::FileId) -> Self::Cursors {
        todo!();
    }

    fn edits(&mut self, _file_id: &Self::FileId) -> Self::Edits {
        todo!();
    }

    fn selections(&mut self, _file_id: &Self::FileId) -> Self::Selections {
        todo!();
    }

    fn is_text_file(&mut self, _file_id: &Self::FileId) -> bool {
        todo!();
    }

    fn path(&mut self, file_id: &Self::FileId) -> Cow<AbsUtf8Path> {
        Cow::Owned(
            self.get_buffer(file_id.clone())
                .expect("already checked")
                .path()
                .expect("already checked")
                .into_owned(),
        )
    }

    type ConfigStream = Subscription<ConfigEvent<NeovimCollab>, Neovim>;
    type JoinStream = Select<
        Subscription<CommandEvent<JoinSession>, Neovim>,
        Subscription<FunctionEvent<JoinSession>, Neovim>,
    >;
    type StartStream = Select<
        Subscription<CommandEvent<StartSession>, Neovim>,
        Subscription<FunctionEvent<StartSession>, Neovim>,
    >;
}
