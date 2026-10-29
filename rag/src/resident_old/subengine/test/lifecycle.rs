use super::*;
use std::sync::atomic::AtomicUsize;
struct Dummy(u32);
impl Resource for Dummy {
    fn key(&self) -> ResourceKey {
        ResourceKey::new(format!("dummy-{}", self.0))
    }
}
fn cell(ttl_ms: u64, backoff_ms: u64) -> ResourceCell<Dummy> {
    ResourceCell::new(ResourceKey::new("dummy"), CellConfig { ttl_ms, backoff_ms })
}
fn init_err() -> SubengineError {
    SubengineError::ResourceInit {
        resource: "dummy".into(),
        reason: "boom".into(),
    }
}
#[tokio::test]
async fn parallel_callers_share_one_initialiser() {
    let c = cell(10, 10);
    let limiter = InitLimiter::new(4);
    let inits = AtomicUsize::new(0);
    let call = || {
        c.get_or_init(
            &limiter,
            0,
            || async {
                inits.fetch_add(1, Ordering::SeqCst);
                tokio::task::yield_now().await;
                Ok(Dummy(1))
            },
        )
    };
    let leases = futures_join(vec![call(), call(), call(), call()]).await;
    assert_eq!(inits.load(Ordering::SeqCst), 1);
    assert!(leases.iter().all(| l | l.as_ref().is_ok_and(| l | l.0 == 1)));
    assert_eq!(c.state(), ResourceState::Leased(4));
    drop(leases);
    assert_eq!(c.state(), ResourceState::Idle);
}
async fn futures_join<F: Future>(fs: Vec<F>) -> Vec<F::Output> {
    let mut pending: Vec<_> = fs.into_iter().map(|f| Some(Box::pin(f))).collect();
    let mut results: Vec<Option<F::Output>> = (0..pending.len())
        .map(|_| None)
        .collect();
    std::future::poll_fn(|cx| {
            let mut done = true;
            for (slot, result) in pending.iter_mut().zip(results.iter_mut()) {
                if let Some(f) = slot {
                    match f.as_mut().poll(cx) {
                        std::task::Poll::Ready(v) => {
                            *result = Some(v);
                            *slot = None;
                        }
                        std::task::Poll::Pending => done = false,
                    }
                }
            }
            if done { std::task::Poll::Ready(()) } else { std::task::Poll::Pending }
        })
        .await;
    results.into_iter().flatten().collect()
}
#[tokio::test]
async fn failed_init_leaves_cell_empty_and_backs_off() {
    let c = cell(10, 100);
    let limiter = InitLimiter::new(1);
    let err = c
        .get_or_init(&limiter, 0, || async { Err(init_err()) })
        .await
        .err()
        .unwrap();
    assert!(matches!(err, SubengineError::ResourceInit { .. }));
    assert_eq!(c.state(), ResourceState::Failed);
    let err = c
        .get_or_init(&limiter, 50, || async { Ok(Dummy(1)) })
        .await
        .err()
        .unwrap();
    assert!(matches!(err, SubengineError::Backoff { until_ms : 100, .. }));
    let lease = c
        .get_or_init(&limiter, 100, || async { Ok(Dummy(2)) })
        .await
        .unwrap();
    assert_eq!(lease.0, 2);
    assert_eq!(c.state(), ResourceState::Leased(1));
}
#[tokio::test]
async fn limiter_bounds_parallel_inits_across_cells() {
    let limiter = InitLimiter::new(1);
    let running = AtomicUsize::new(0);
    let peak = AtomicUsize::new(0);
    let cells: Vec<_> = (0..3).map(|_| cell(10, 10)).collect();
    let calls = cells
        .iter()
        .map(|c| {
            c.get_or_init(
                &limiter,
                0,
                || async {
                    let now = running.fetch_add(1, Ordering::SeqCst) + 1;
                    peak.fetch_max(now, Ordering::SeqCst);
                    tokio::task::yield_now().await;
                    running.fetch_sub(1, Ordering::SeqCst);
                    Ok(Dummy(0))
                },
            )
        });
    let leases = futures_join(calls.collect()).await;
    assert!(leases.iter().all(Result::is_ok));
    assert_eq!(peak.load(Ordering::SeqCst), 1);
}
#[tokio::test]
async fn eviction_waits_for_lease_op_and_ttl() {
    let c = cell(100, 10);
    let limiter = InitLimiter::new(1);
    let lease = c.get_or_init(&limiter, 0, || async { Ok(Dummy(1)) }).await.unwrap();
    assert!(! c.evict_idle(1_000));
    let op = lease.begin_op();
    drop(lease);
    assert!(! c.evict_idle(1_000));
    drop(op);
    assert!(! c.evict_idle(1_000), "first call only starts the idle clock");
    assert!(! c.evict_idle(1_050));
    assert!(c.evict_idle(1_100));
    assert_eq!(c.state(), ResourceState::Unloaded);
    let lease = c
        .get_or_init(&limiter, 1_200, || async { Ok(Dummy(2)) })
        .await
        .unwrap();
    assert_eq!(lease.0, 2);
}
#[tokio::test]
async fn lease_taken_after_idle_clock_blocks_eviction() {
    let c = cell(10, 10);
    let limiter = InitLimiter::new(1);
    drop(c.get_or_init(&limiter, 0, || async { Ok(Dummy(1)) }).await.unwrap());
    assert!(! c.evict_idle(0));
    let lease = c.get_or_init(&limiter, 5, || async { Ok(Dummy(9)) }).await.unwrap();
    assert_eq!(lease.0, 1);
    assert!(! c.evict_idle(1_000));
    drop(lease);
    assert!(! c.evict_idle(1_000));
    assert!(c.evict_idle(1_010));
}
#[test]
fn concurrent_leases_and_eviction_never_lose_a_resident_lease() {
    let c = Arc::new(cell(0, 1));
    let limiter = InitLimiter::new(1);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .unwrap();
    let evictor = {
        let c = Arc::clone(&c);
        std::thread::spawn(move || {
            for t in 0..5_000 {
                c.evict_idle(t);
            }
        })
    };
    rt.block_on(async {
        for _ in 0..2_000 {
            let lease = c
                .get_or_init(&limiter, 0, || async { Ok(Dummy(7)) })
                .await
                .unwrap();
            assert_eq!(lease.0, 7);
            tokio::task::yield_now().await;
            assert_eq!(lease.0, 7);
        }
    });
    evictor.join().unwrap();
}
