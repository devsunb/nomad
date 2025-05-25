use ed::Context;
use neovim::Neovim;

mod ed_buffer {
    //! Contains the editor-agnostic buffer tests.

    use super::*;
    use crate::ed::buffer;

    #[neovim::test]
    async fn fuzz_edits_10(ctx: &mut Context<Neovim>) {
        buffer::fuzz_edits_10(ctx).await;
    }

    #[neovim::test]
    async fn fuzz_edits_100(ctx: &mut Context<Neovim>) {
        buffer::fuzz_edits_100(ctx).await;
    }

    #[neovim::test]
    async fn fuzz_edits_1_000(ctx: &mut Context<Neovim>) {
        buffer::fuzz_edits_1_000(ctx).await;
    }

    #[neovim::test]
    async fn fuzz_edits_10_000(ctx: &mut Context<Neovim>) {
        buffer::fuzz_edits_10_000(ctx).await;
    }
}
