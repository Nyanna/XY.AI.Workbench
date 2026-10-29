use std::sync::atomic::{AtomicUsize, Ordering};
use futures::future::join_all;
use super::*;
async fn settle() {
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
}
#[tokio::test]
async fn concurrent_requests_share_one_load() {
    let flight = SingleFlight::<u32, u32>::new();
    let starts = AtomicUsize::new(0);
    let runs = (0..5)
        .map(|_| {
            flight
                .run(
                    1,
                    || {
                        starts.fetch_add(1, Ordering::SeqCst);
                        async {
                            settle().await;
                            Ok::<_, Arc<ManagerError>>(Arc::new(7))
                        }
                    },
                )
        });
    let results = join_all(runs).await;
    assert_eq!(starts.load(Ordering::SeqCst), 1);
    let first = results[0].as_ref().unwrap();
    for r in &results {
        assert!(Arc::ptr_eq(first, r.as_ref().unwrap()));
    }
    assert_eq!(flight.inflight(), 0);
}
#[tokio::test]
async fn error_reaches_every_waiter_and_next_call_retries() {
    let flight = SingleFlight::<u32, u32>::new();
    let starts = AtomicUsize::new(0);
    let failing = || {
        starts.fetch_add(1, Ordering::SeqCst);
        async {
            settle().await;
            Err::<
                Arc<u32>,
                _,
            >(
                Arc::new(ManagerError::ArenaExhausted {
                    requested: 8,
                    available: 0,
                }),
            )
        }
    };
    let results = join_all((0..3).map(|_| flight.run(1, &failing))).await;
    assert_eq!(starts.load(Ordering::SeqCst), 1);
    let first = results[0].as_ref().unwrap_err();
    for r in &results {
        let e = r.as_ref().unwrap_err();
        assert!(Arc::ptr_eq(first, e));
        assert!(matches!(** e, ManagerError::ArenaExhausted { requested : 8, .. }));
    }
    assert_eq!(flight.inflight(), 0);
    let again = flight.run(1, &failing).await;
    assert!(again.is_err());
    assert_eq!(starts.load(Ordering::SeqCst), 2);
}
#[tokio::test]
async fn distinct_keys_do_not_share() {
    let flight = SingleFlight::<u32, u32>::new();
    let starts = AtomicUsize::new(0);
    let load = |v: u32| {
        starts.fetch_add(1, Ordering::SeqCst);
        async move { Ok::<_, Arc<ManagerError>>(Arc::new(v)) }
    };
    let (a, b) = futures::join!(flight.run(1, || load(10)), flight.run(2, || load(20)));
    assert_eq!((* a.unwrap(), * b.unwrap()), (10, 20));
    assert_eq!(starts.load(Ordering::SeqCst), 2);
}
#[tokio::test]
async fn finished_entry_is_removed_before_a_new_load() {
    let flight = SingleFlight::<u32, u32>::new();
    let starts = AtomicUsize::new(0);
    for _ in 0..2 {
        let v = flight
            .run(
                1,
                || {
                    let n = starts.fetch_add(1, Ordering::SeqCst) as u32;
                    async move { Ok::<_, Arc<ManagerError>>(Arc::new(n)) }
                },
            )
            .await
            .unwrap();
        assert_eq!(flight.inflight(), 0);
        assert!(* v < 2);
    }
    assert_eq!(starts.load(Ordering::SeqCst), 2);
}
