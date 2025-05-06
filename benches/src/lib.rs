#![allow(missing_docs)]

mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}
#[cfg(feature = "collab")]
mod collab;

#[cfg(feature = "collab")]
pub fn collab(c: &mut criterion::Criterion) {
    collab::benches(c);
}
