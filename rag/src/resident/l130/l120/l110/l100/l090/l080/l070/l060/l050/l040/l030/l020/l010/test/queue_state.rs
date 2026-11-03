use super::*;
use std::time::Duration;
type Entry = EntryState<String, u32, String>;
#[test]
fn new_entry_is_enqueued() {
    let e = Entry::new("p".into());
    assert_eq!(e.state(), QueueState::Enqueued);
    assert!(e.result().is_none());
}
#[test]
fn claim_moves_payload_once() {
    let e = Entry::new("p".into());
    assert_eq!(e.claim().unwrap(), "p");
    assert_eq!(e.state(), QueueState::Claimed);
    assert_eq!(e.claim(), Err(ClaimError::NotEnqueued(QueueState::Claimed)));
}
#[test]
fn sentinel_claim_fails_and_keeps_state() {
    let e = Entry::sentinel();
    assert_eq!(claim_entry(& e), Err(ClaimError::NoPayload));
    assert_eq!(e.state(), QueueState::Enqueued);
}
#[test]
fn busy_when_locked() {
    let e = Entry::new("p".into());
    let guard = e.payload.lock().unwrap();
    assert_eq!(e.claim(), Err(ClaimError::Busy));
    drop(guard);
    assert_eq!(e.state(), QueueState::Enqueued);
    assert!(e.claim().is_ok());
}
#[test]
fn poisoned_mutex_is_recovered() {
    let e = Entry::new("p".into());
    let e2 = e.clone();
    let _ = std::thread::spawn(move || {
            let _g = e2.payload.lock().unwrap();
            panic!("poison");
        })
        .join();
    assert_eq!(e.claim().unwrap(), "p");
}
#[test]
fn cancel_then_claim_fails() {
    let e = Entry::new("p".into());
    assert!(e.cancel());
    assert!(! e.cancel());
    assert_eq!(e.claim(), Err(ClaimError::Cancelled));
    assert_eq!(e.state(), QueueState::Cancelled);
    assert!(e.result().is_none());
}
#[test]
fn cancel_after_claim_fails() {
    let e = Entry::new("p".into());
    e.claim().unwrap();
    assert!(! e.cancel());
    assert_eq!(e.state(), QueueState::Claimed);
}
#[test]
fn complete_stores_result() {
    let e = Entry::new("p".into());
    e.claim().unwrap();
    e.complete(Ok(7)).unwrap();
    assert_eq!(e.state(), QueueState::Completed);
    assert_eq!(e.result(), Some(& Ok(7)));
}
#[test]
fn complete_on_enqueued_or_cancelled_fails() {
    let e = Entry::new("p".into());
    assert_eq!(e.complete(Ok(1)), Err(CompleteError::NotClaimed(QueueState::Enqueued)));
    assert!(e.cancel());
    assert_eq!(e.complete(Ok(1)), Err(CompleteError::NotClaimed(QueueState::Cancelled)));
    assert!(e.result().is_none());
}
#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn second_completion_panics_in_debug() {
    let e = Entry::new("p".into());
    e.claim().unwrap();
    e.complete(Ok(1)).unwrap();
    let _ = e.complete(Ok(2));
}
#[test]
#[cfg(not(debug_assertions))]
fn second_completion_keeps_first_result() {
    let e = Entry::new("p".into());
    e.claim().unwrap();
    e.complete(Ok(1)).unwrap();
    assert_eq!(e.complete(Ok(2)), Err(CompleteError::NotClaimed(QueueState::Completed)));
    assert_eq!(e.result(), Some(& Ok(1)));
}
#[test]
#[should_panic(expected = "invalid value")]
fn invalid_state_panics() {
    let e = Entry::new("p".into());
    e.state.store(200, Ordering::SeqCst);
    let _ = e.state();
}
#[test]
fn concurrent_claims_have_one_winner() {
    let e = Entry::new("p".into());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let e = e.clone();
            std::thread::spawn(move || e.claim().is_ok())
        })
        .collect();
    let wins = handles.into_iter().map(|h| h.join().unwrap()).filter(|w| *w).count();
    assert_eq!(wins, 1);
}
#[test]
fn concurrent_claim_and_cancel_are_exclusive() {
    for _ in 0..200 {
        let e = Entry::new("p".into());
        let a = e.clone();
        let b = e.clone();
        let c = std::thread::spawn(move || a.claim().is_ok());
        let d = std::thread::spawn(move || b.cancel());
        let claimed = c.join().unwrap();
        let cancelled = d.join().unwrap();
        assert_ne!(claimed, cancelled);
        let expected = if claimed { QueueState::Claimed } else { QueueState::Cancelled };
        assert_eq!(e.state(), expected);
    }
}
#[tokio::test]
async fn wait_returns_immediately_when_final() {
    let e = Entry::new("p".into());
    assert!(e.cancel());
    assert_eq!(e.wait_finished(). await, QueueState::Cancelled);
}
#[tokio::test]
async fn waiters_are_woken_on_completion() {
    let e = Entry::new("p".into());
    let waiters: Vec<_> = (0..4)
        .map(|_| {
            let e = e.clone();
            tokio::spawn(async move { e.wait_finished().await })
        })
        .collect();
    tokio::time::sleep(Duration::from_millis(20)).await;
    e.claim().unwrap();
    e.complete(Ok(3)).unwrap();
    for w in waiters {
        assert_eq!(w. await .unwrap(), QueueState::Completed);
    }
}
#[tokio::test]
async fn waiters_are_woken_on_cancel() {
    let e = Entry::new("p".into());
    let e2 = e.clone();
    let w = tokio::spawn(async move { e2.wait_finished().await });
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(e.cancel());
    assert_eq!(w. await .unwrap(), QueueState::Cancelled);
}
