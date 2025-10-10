use core::error::Error;
use core::fmt;
use std::io;

pub(crate) trait IoErrorExt: Sized {
    fn with_context<Ctx>(self, fun: impl FnOnce() -> Ctx) -> Self
    where
        Ctx: fmt::Debug + fmt::Display + Send + Sync + 'static;
}

#[derive(Debug)]
struct IoErrorContext<Ctx> {
    context: Ctx,
    source: io::Error,
}

impl IoErrorExt for io::Error {
    fn with_context<Ctx>(self, fun: impl FnOnce() -> Ctx) -> Self
    where
        Ctx: fmt::Debug + fmt::Display + Send + Sync + 'static,
    {
        Self::new(self.kind(), IoErrorContext { context: fun(), source: self })
    }
}

impl<T> IoErrorExt for Result<T, io::Error> {
    fn with_context<Ctx>(self, fun: impl FnOnce() -> Ctx) -> Self
    where
        Ctx: fmt::Debug + fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|err| err.with_context(fun))
    }
}

impl<Ctx: fmt::Display> fmt::Display for IoErrorContext<Ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.source)
    }
}

impl<Ctx: fmt::Debug + fmt::Display> Error for IoErrorContext<Ctx> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
