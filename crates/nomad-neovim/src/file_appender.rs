use core::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

use ed::{BorrowState, Context};
use neovim::Neovim;
use tracing::error;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt::{self, format, time};
use tracing_subscriber::registry::LookupSpan;

/// A [`tracing_subscriber::Layer`] implementation that appends logs to a file
/// that is rolled over daily.
#[derive(cauchy::Clone)]
pub(crate) struct FileAppender<S> {
    inner: Arc<OnceLock<FileAppenderInner<S>>>,
    creating_inner_has_failed: Arc<AtomicBool>,
}

struct FileAppenderInner<S> {
    inner: fmt::Layer<
        S,
        format::DefaultFields,
        format::Format<format::Full, time::ChronoUtc>,
        NonBlocking,
    >,

    /// We need to keep this guard around for the entire lifetime of the
    /// program to ensure that the logs are flushed properly.
    ///
    /// The `Drop` implementation of this guard will flush any remaining logs
    /// to the file in case the program is terminated abruptly, for example by
    /// a panic.
    _guard: WorkerGuard,
}

impl<S: 'static> FileAppender<S> {
    pub(crate) fn new(ctx: &mut Context<Neovim, impl BorrowState>) -> Self {
        let this = Self {
            inner: Arc::new(OnceLock::new()),
            creating_inner_has_failed: Arc::new(AtomicBool::new(false)),
        };

        // Creating the inner file appender does a bunch of blocking I/O, so we
        // do it in the background.
        ctx.spawn_background({
            let this = this.clone();
            async move {
                match FileAppenderInner::new() {
                    Ok(file_appender) => match this.inner.set(file_appender) {
                        Ok(_) => (),
                        Err(_) => unreachable!("only set once"),
                    },
                    Err(err) => {
                        error!(
                            "failed to create tracing file appender: {err}"
                        );
                        this.creating_inner_has_failed
                            .store(true, Ordering::Relaxed);
                    },
                }
            }
        })
        .detach();

        this
    }
}

impl<S> FileAppenderInner<S> {
    fn new() -> Result<Self, std::io::Error> {
        todo!();
    }
}

impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for FileAppender<S>
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_register_dispatch(&self, subscriber: &tracing::Dispatch) {
        if let Some(inner) = self.inner.get() {
            inner.on_register_dispatch(subscriber);
        }
    }

    fn enabled(
        &self,
        metadata: &tracing::Metadata<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        if let Some(inner) = self.inner.get() {
            inner.enabled(metadata, ctx)
        } else {
            !self.creating_inner_has_failed.load(Ordering::Relaxed)
        }
    }

    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(inner) = self.inner.get() {
            inner.on_new_span(attrs, id, ctx);
        }
    }

    fn on_record(
        &self,
        span: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(inner) = self.inner.get() {
            inner.on_record(span, values, ctx);
        }
    }

    fn on_follows_from(
        &self,
        span: &tracing::span::Id,
        follows: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(inner) = self.inner.get() {
            inner.on_follows_from(span, follows, ctx);
        }
    }

    fn event_enabled(
        &self,
        event: &tracing::Event<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        if let Some(inner) = self.inner.get() {
            inner.event_enabled(event, ctx)
        } else {
            !self.creating_inner_has_failed.load(Ordering::Relaxed)
        }
    }

    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(inner) = self.inner.get() {
            inner.on_event(event, ctx);
        }
    }

    fn on_enter(
        &self,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(inner) = self.inner.get() {
            inner.on_enter(id, ctx);
        }
    }

    fn on_exit(
        &self,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(inner) = self.inner.get() {
            inner.on_exit(id, ctx);
        }
    }

    fn on_close(
        &self,
        id: tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(inner) = self.inner.get() {
            inner.on_close(id, ctx);
        }
    }

    fn on_id_change(
        &self,
        old: &tracing::span::Id,
        new: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(inner) = self.inner.get() {
            inner.on_id_change(old, new, ctx);
        }
    }
}

impl<S: tracing::Subscriber> tracing_subscriber::Layer<S>
    for FileAppenderInner<S>
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_register_dispatch(&self, subscriber: &tracing::Dispatch) {
        self.inner.on_register_dispatch(subscriber);
    }

    fn register_callsite(
        &self,
        metadata: &'static tracing::Metadata<'static>,
    ) -> tracing::subscriber::Interest {
        self.inner.register_callsite(metadata)
    }

    fn enabled(
        &self,
        metadata: &tracing::Metadata<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        self.inner.enabled(metadata, ctx)
    }

    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_new_span(attrs, id, ctx);
    }

    fn on_record(
        &self,
        span: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_record(span, values, ctx);
    }

    fn on_follows_from(
        &self,
        span: &tracing::span::Id,
        follows: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_follows_from(span, follows, ctx);
    }

    fn event_enabled(
        &self,
        event: &tracing::Event<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        self.inner.event_enabled(event, ctx)
    }

    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_event(event, ctx);
    }

    fn on_enter(
        &self,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_enter(id, ctx);
    }

    fn on_exit(
        &self,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_exit(id, ctx);
    }

    fn on_close(
        &self,
        id: tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_close(id, ctx);
    }

    fn on_id_change(
        &self,
        old: &tracing::span::Id,
        new: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        self.inner.on_id_change(old, new, ctx);
    }
}
