pub(crate) type NeovimOpenFiles = neovim::OpenFiles;

mod neovim {
    use core::pin::Pin;
    use core::task::{Context, Poll};

    use futures_util::Stream;
    use nomad::neovim::{self, Neovim};
    use nomad::Subscription;

    pin_project_lite::pin_project! {
        /// TODO: docs.
        pub(crate) struct OpenFiles {
            #[pin]
            inner: Subscription<neovim::events::OpenBufferEvent, Neovim>,
        }
    }

    impl Stream for OpenFiles {
        type Item = neovim::BufferId;

        fn poll_next(
            self: Pin<&mut Self>,
            ctx: &mut Context,
        ) -> Poll<Option<Self::Item>> {
            self.project().inner.poll_next(ctx).map(|maybe_open| {
                maybe_open.map(|open| {
                    // TODO: check that the file exists.
                    open.id()
                })
            })
        }
    }
}
