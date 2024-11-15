use nvimx_fs::AbsPathBuf;

/// TODO: docs.
pub trait Plugin {
    /// TODO: docs.
    const AUGROUP_NAME: &'static str;

    /// TODO: docs.
    const COMMAND_NAME: &'static str;

    /// TODO: docs.
    const DIAGNOSTIC_NAME: &'static str;

    /// TODO: docs.
    const NAMESPACE_NAME: &'static str;

    /// TODO: docs.
    fn log_dir(&self) -> AbsPathBuf;
}
