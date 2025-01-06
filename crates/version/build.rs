#![allow(missing_docs)]

use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let commit_hash = "c26db43";
    let commit_year = "2025";
    let commit_month = "1";
    let commit_day = "17";
    let major = "0";
    let minor = "1";
    let patch = "0";
    let pre_release_label = Some("dev");

    write!(
        out_file("generated.rs"),
        r#"
pub(crate) const COMMIT_SHORT_HASH: &'static str = "{commit_hash}";
pub(crate) const COMMIT_YEAR: u16 = {commit_year};
pub(crate) const COMMIT_MONTH: u8 = {commit_month};
pub(crate) const COMMIT_DAY: u8 = {commit_day};
pub(crate) const MAJOR: u8 = {major};
pub(crate) const MINOR: u8 = {minor};
pub(crate) const PATCH: u8 = {patch};
pub(crate) const PRE_RELEASE_LABEL: Option<&'static str> = {pre};
        "#,
        pre = pre_release_label
            .map(|pre| Cow::Owned(format!("Some(\"{pre}\")")))
            .unwrap_or(Cow::Borrowed("None")),
    )
    .expect("couldn't write to file");
}

/// Opens the file with the given name in the `OUT_DIR`, or creates a new one
/// if it doesn't exist.
fn out_file(file_name: &str) -> File {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is set in build script");
    let out_path = Path::new(&out_dir).join(file_name);
    File::create(&out_path).unwrap_or_else(|err| {
        panic!("couldn't create file at {out_path:?}: {err}")
    })
}
