/// Same as [`Into`], but for types defined in other crates (for which we
/// couldn't implement [`Into`] because of the orphan rule).
pub(crate) trait Convert<T> {
    fn convert(self) -> T;
}

impl Convert<crate::oxi::api::types::LogLevel> for ed::notify::Level {
    #[inline]
    fn convert(self) -> crate::oxi::api::types::LogLevel {
        match self {
            Self::Off => crate::oxi::api::types::LogLevel::Off,
            Self::Trace => crate::oxi::api::types::LogLevel::Trace,
            Self::Debug => crate::oxi::api::types::LogLevel::Debug,
            Self::Info => crate::oxi::api::types::LogLevel::Info,
            Self::Warn => crate::oxi::api::types::LogLevel::Warn,
            Self::Error => crate::oxi::api::types::LogLevel::Error,
        }
    }
}

impl<T> Convert<smallvec::SmallVec<[T; 1]>>
    for crate::oxi::api::types::OneOrMore<T>
{
    #[inline]
    fn convert(self) -> smallvec::SmallVec<[T; 1]> {
        match self {
            Self::One(item) => smallvec::smallvec_inline![item],
            Self::List(items) => items.into(),
        }
    }
}
