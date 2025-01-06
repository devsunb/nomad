#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

use std::env;
use std::fmt::Write;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut file = GeneratedFile::default();
    add_commit_infos(&mut file);
    add_version_infos(&mut file);
    file.create();
}

fn add_commit_infos(file: &mut GeneratedFile) {
    file.add_const("COMMIT_SHORT_HASH", "c26db43")
        .add_const("COMMIT_YEAR", 2025u16)
        .add_const("COMMIT_MONTH", 1u8)
        .add_const("COMMIT_DAY", 17u8);
}

fn add_version_infos(file: &mut GeneratedFile) {
    file.add_const("MAJOR", 0u8)
        .add_const("MINOR", 1u8)
        .add_const("PATCH", 0u8)
        .add_const("PRE_RELEASE_LABEL", Some("dev"));
}

#[derive(Default)]
struct GeneratedFile {
    contents: String,
}

impl GeneratedFile {
    const NAME: &'static str = "generated.rs";

    fn add_const<T: DisplayType>(
        &mut self,
        name: &str,
        value: T,
    ) -> &mut Self {
        write!(&mut self.contents, "pub(crate) const {name}: ").unwrap();
        T::display_type(&mut self.contents);
        self.contents.push_str(" = ");
        T::display_value(&value, &mut self.contents);
        self.contents.push_str(";\n");
        self
    }

    fn create(self) {
        use std::io::Write;
        write!(out_file(Self::NAME), "{}", self.contents).unwrap()
    }
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

trait DisplayType {
    fn display_type(buf: &mut String);
    fn display_value(&self, buf: &mut String);
}

impl DisplayType for u8 {
    fn display_type(buf: &mut String) {
        buf.push_str("u8");
    }

    fn display_value(&self, buf: &mut String) {
        write!(buf, "{self}").unwrap()
    }
}

impl DisplayType for u16 {
    fn display_type(buf: &mut String) {
        buf.push_str("u16");
    }

    fn display_value(&self, buf: &mut String) {
        write!(buf, "{self}").unwrap()
    }
}

impl DisplayType for &str {
    fn display_type(buf: &mut String) {
        buf.push_str("&str");
    }

    fn display_value(&self, buf: &mut String) {
        write!(buf, "\"{self}\"").unwrap()
    }
}

impl<T: DisplayType> DisplayType for Option<T> {
    fn display_type(buf: &mut String) {
        buf.push_str("Option<");
        T::display_type(buf);
        buf.push('>');
    }

    fn display_value(&self, buf: &mut String) {
        match self {
            Some(value) => {
                write!(buf, "Some(").unwrap();
                value.display_value(buf);
                write!(buf, ")").unwrap();
            },
            None => {
                buf.push_str("None");
            },
        }
    }
}
