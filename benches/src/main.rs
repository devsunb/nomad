#![allow(missing_docs)]

use criterion::{Criterion, criterion_main};

fn benches() {
    let mut criterion = Criterion::default().configure_from_args();

    #[cfg(feature = "collab")]
    benches::collab(&mut criterion);
}

criterion_main!(benches);
