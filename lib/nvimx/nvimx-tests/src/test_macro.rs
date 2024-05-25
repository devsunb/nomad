use std::future::Future;
use std::path::PathBuf;

use nvim_oxi::tests::IntoResult;
use nvim_oxi::TestTerminator;

/// TODO: docs
pub fn async_body<F, R>(_terminator: TestTerminator, _test_body: F)
where
    F: Future<Output = R> + 'static,
    R: IntoResult,
{
}

/// TODO: docs
pub fn library_path(_crate_name: &str) -> PathBuf {
    todo!();
}
