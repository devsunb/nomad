use ed::executor::BackgroundSpawner;
use futures_lite::future;
use thread_pool::ThreadPool;

#[test]
fn task_resolves_to_future_output() {
    let value = 42;
    let mut thread_pool = ThreadPool::default();
    let task = thread_pool.spawn(async move { value });
    assert_eq!(future::block_on(task), value);
}

#[test]
fn task_resolves_if_thread_pool_is_dropped() {
    let value = 42;
    let mut thread_pool = ThreadPool::default();
    let task = thread_pool.spawn(async move { value });
    drop(thread_pool);
    assert_eq!(future::block_on(task), value);
}

#[test]
#[should_panic]
fn panic_is_propagated_if_task_is_awaited() {
    let mut thread_pool = ThreadPool::default();
    let task = thread_pool.spawn(async { panic!() });
    future::block_on(task);
}
