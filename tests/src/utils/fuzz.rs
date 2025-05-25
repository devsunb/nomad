use core::convert::Infallible;
use core::fmt;
use std::{env, panic};

use futures_util::FutureExt;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

#[track_caller]
pub(crate) fn run<T: FuzzResult>(fun: impl FnOnce(&mut ChaChaRng) -> T) {
    let mut rng = ChaChaRng::seed_from_u64(seed());

    match panic::catch_unwind(panic::AssertUnwindSafe(move || {
        fun(&mut rng).into_result()
    })) {
        Ok(Ok(())) => {},
        Ok(Err(_err)) => todo!(),
        Err(_panic_payload) => todo!(),
    }
}

#[track_caller]
pub(crate) async fn run_async<T: FuzzResult>(
    fun: impl AsyncFnOnce(&mut ChaChaRng) -> T,
) {
    let mut rng = ChaChaRng::seed_from_u64(seed());

    match panic::AssertUnwindSafe(fun(&mut rng))
        .catch_unwind()
        .await
        .map(FuzzResult::into_result)
    {
        Ok(Ok(())) => {},
        Ok(Err(_err)) => todo!(),
        Err(_panic_payload) => todo!(),
    }
}

#[track_caller]
fn seed() -> u64 {
    match env::var("SEED") {
        Ok(seed) => seed.parse().expect("couldn't parse $SEED"),
        Err(env::VarError::NotPresent) => rand::random(),
        Err(env::VarError::NotUnicode(seed)) => {
            panic!("$SEED contained invalid unicode: {seed:?}")
        },
    }
}

pub(crate) trait FuzzClosure {
    #[doc(hidden)]
    fn run(self, rng: &mut impl Rng) -> impl FuzzResult;
}

/// A trait for the result of a fuzz run.
///
/// It's only implemented for `()` and `Result<(), E>` where `E: Display`.
pub(crate) trait FuzzResult {
    #[doc(hidden)]
    type Error: fmt::Display;

    #[doc(hidden)]
    fn into_result(self) -> Result<(), Self::Error>;
}

impl FuzzResult for () {
    type Error = Infallible;

    fn into_result(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<E: fmt::Display> FuzzResult for Result<(), E> {
    type Error = E;

    fn into_result(self) -> Result<(), Self::Error> {
        self
    }
}
