use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn runs_job_and_returns_result() {
    let pool = BackgroundPool::new("t", 2).unwrap();
    assert_eq!(pool.name(), "t");
    assert_eq!(pool.thread_count(), 2);
    assert_eq!(pool.submit(|| 21 * 2).unwrap().wait(), Ok(42));
}

#[test]
fn zero_threads_raised_to_one() {
    let pool = BackgroundPool::new("z", 0).unwrap();
    assert_eq!(pool.thread_count(), 1);
}

#[test]
fn single_thread_preserves_order_and_names_thread() {
    let pool = BackgroundPool::new("ord", 1).unwrap();
    let log = Arc::new(Mutex::new(Vec::new()));
    let futs: Vec<_> = (0..20)
        .map(|i| {
            let l = Arc::clone(&log);
            pool.submit(move || l.lock().unwrap().push(i)).unwrap()
        })
        .collect();
    for f in futs {
        f.wait().unwrap();
    }
    assert_eq!(*log.lock().unwrap(), (0..20).collect::<Vec<_>>());
    let name = pool
        .submit(|| thread::current().name().map(String::from))
        .unwrap()
        .wait()
        .unwrap();
    assert_eq!(name.as_deref(), Some("ord-0"));
}

#[test]
fn panic_is_caught_and_worker_survives() {
    let pool = BackgroundPool::new("p", 1).unwrap();
    let a = pool.submit(|| -> i32 { panic!("boom") }).unwrap();
    let b = pool.submit(|| -> i32 { std::panic::panic_any(5u8) }).unwrap();
    assert_eq!(a.wait(), Err(JobError::Panicked("boom".into())));
    assert_eq!(b.wait(), Err(JobError::Panicked("unknown panic".into())));
    assert_eq!(pool.submit(|| 1).unwrap().wait(), Ok(1));
}

#[test]
fn try_get_yields_once() {
    let pool = BackgroundPool::new("g", 1).unwrap();
    let mut f = pool.submit(|| 7).unwrap();
    let r = loop {
        if let Some(r) = f.try_get() {
            break r;
        }
        thread::yield_now();
    };
    assert_eq!(r, Ok(7));
    assert!(f.is_done());
    assert!(f.try_get().is_none());
}

#[test]
fn wait_timeout_returns_future_then_succeeds() {
    let pool = BackgroundPool::new("w", 1).unwrap();
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    let f = pool
        .submit(move || {
            rx.recv().unwrap();
            9
        })
        .unwrap();
    let f = f.wait_timeout(Duration::from_millis(20)).err().unwrap();
    assert!(!f.is_done());
    tx.send(()).unwrap();
    assert_eq!(f.wait_timeout(Duration::from_secs(10)).ok().unwrap(), Ok(9));
}

#[test]
fn shutdown_drains_and_rejects() {
    let pool = BackgroundPool::new("s", 2).unwrap();
    let count = Arc::new(AtomicUsize::new(0));
    for _ in 0..50 {
        let c = Arc::clone(&count);
        let _ = pool.submit(move || {
            thread::sleep(Duration::from_millis(1));
            c.fetch_add(1, Ordering::SeqCst);
        });
    }
    pool.shutdown();
    pool.shutdown();
    assert_eq!(count.load(Ordering::SeqCst), 50);
    assert!(matches!(pool.submit(|| 1), Err(PoolError::ShutDown)));
}

#[test]
fn drop_runs_dropped_future_jobs() {
    let count = Arc::new(AtomicUsize::new(0));
    {
        let pool = BackgroundPool::new("d", 1).unwrap();
        for _ in 0..10 {
            let c = Arc::clone(&count);
            drop(pool.submit(move || c.fetch_add(1, Ordering::SeqCst)).unwrap());
        }
    }
    assert_eq!(count.load(Ordering::SeqCst), 10);
}

#[test]
fn pool_and_future_are_send() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    assert_send::<BackgroundPool>();
    assert_sync::<BackgroundPool>();
    assert_send::<JobFuture<i32>>();
}
