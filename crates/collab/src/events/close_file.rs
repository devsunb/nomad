pub(crate) type NeovimCloseFiles = neovim::CloseFiles;

mod neovim {
    use core::pin::Pin;
    use core::task::{Context, Poll};

    use futures_util::Stream;
    use nomad::neovim::{self, Neovim};
    use nomad::Subscription;

    pin_project_lite::pin_project! {
        /// TODO: docs.
        pub(crate) struct CloseFiles {
            #[pin]
            inner: Subscription<neovim::events::CloseBufferEvent, Neovim>,
        }
    }

    impl Stream for CloseFiles {
        type Item = neovim::BufferId;

        fn poll_next(
            self: Pin<&mut Self>,
            ctx: &mut Context,
        ) -> Poll<Option<Self::Item>> {
            self.project()
                .inner
                .poll_next(ctx)
                .map(|maybe_close| maybe_close.map(|close| close.id()))
        }
    }
}
