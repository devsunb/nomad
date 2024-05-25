//! Tests async tests annotated with `#[nvimx::test]`.

use std::time::Duration;

use async_io::Timer;
use futures::FutureExt;

#[nvimx::test]
async fn async_timer() {
    let mut one_sec = Timer::after(Duration::from_secs(1));
    assert!((&mut one_sec).now_or_never().is_none());

    Timer::after(Duration::from_millis(500)).await;
    assert!((&mut one_sec).now_or_never().is_none());

    Timer::after(Duration::from_millis(500)).await;
    assert!(one_sec.now_or_never().is_some());
}
