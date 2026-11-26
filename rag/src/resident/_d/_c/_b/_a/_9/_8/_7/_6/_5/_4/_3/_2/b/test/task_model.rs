use super::*;
type TestTask = Task<String, u32, String>;
fn new_task() -> TestTask {
    Task::new("data".to_string(), TaskParams::default())
}
#[test]
fn new_task_is_enqueued_without_result() {
    let params = TaskParams { priority: 3 };
    let task: TestTask = Task::new("x".into(), params.clone());
    assert_eq!(task.state(), TaskState::Enqueued);
    assert!(task.result().is_none());
    assert_eq!(task.params(), & params);
}
#[test]
fn claim_complete_flow() {
    let task = new_task();
    let other = task.clone();
    assert_eq!(task.claim().unwrap(), "data");
    assert_eq!(other.state(), TaskState::Claimed);
    assert!(other.result().is_none());
    assert!(! task.cancel());
    assert!(task.claim().is_err());
    task.complete(Ok(7)).unwrap();
    assert_eq!(other.result(), Some(Ok(& 7)));
}
#[test]
fn claim_moves_data_once() {
    let task = new_task();
    assert_eq!(task.claim().unwrap(), "data");
    assert_eq!(task.claim(), Err(ClaimError2::NotEnqueued(TaskState::Claimed)));
}
#[test]
fn failed_result() {
    let task = new_task();
    task.claim().unwrap();
    task.complete(Err("boom".to_string())).unwrap();
    assert_eq!(task.result(), Some(Err(TaskError::Failed(& "boom".to_string()))));
}
#[test]
fn cancel_flow() {
    let task = new_task();
    assert!(task.cancel());
    assert!(! task.cancel());
    assert_eq!(task.state(), TaskState::Cancelled);
    assert_eq!(task.result(), Some(Err(TaskError::Cancelled)));
    assert_eq!(task.claim(), Err(ClaimError2::Cancelled));
}
#[test]
fn cancel_after_claim_fails() {
    let task = new_task();
    task.claim().unwrap();
    assert!(! task.cancel());
    assert_eq!(task.state(), TaskState::Claimed);
}
#[test]
fn complete_without_claim_fails() {
    let task = new_task();
    assert_eq!(
        task.complete(Ok(1)), Err(CompleteError::NotClaimed(TaskState::Enqueued))
    );
    assert!(task.result().is_none());
    let cancelled = new_task();
    cancelled.cancel();
    assert_eq!(
        cancelled.complete(Ok(1)), Err(CompleteError::NotClaimed(TaskState::Cancelled))
    );
    assert!(cancelled.result().is_some());
}
#[test]
#[cfg(debug_assertions)]
#[should_panic]
fn second_completion_panics_in_debug() {
    let task = new_task();
    task.claim().unwrap();
    task.complete(Ok(1)).unwrap();
    let _ = task.complete(Ok(2));
}
#[test]
#[cfg(not(debug_assertions))]
fn second_completion_keeps_first_result() {
    let task = new_task();
    task.claim().unwrap();
    task.complete(Ok(1)).unwrap();
    assert_eq!(
        task.complete(Ok(2)), Err(CompleteError::NotClaimed(TaskState::Completed))
    );
    assert_eq!(task.result(), Some(Ok(& 1)));
}
#[test]
fn busy_when_payload_locked() {
    let task = new_task();
    let guard = task.inner.payload.lock().unwrap();
    assert_eq!(task.claim(), Err(ClaimError2::Busy));
    drop(guard);
    assert_eq!(task.state(), TaskState::Enqueued);
    assert!(task.claim().is_ok());
}
#[test]
fn poisoned_payload_is_recovered() {
    let task = new_task();
    let poisoner = task.clone();
    let _ = std::thread::spawn(move || {
            let _guard = poisoner.inner.payload.lock().unwrap();
            panic!("poison");
        })
        .join();
    assert_eq!(task.claim().unwrap(), "data");
}
#[test]
#[should_panic(expected = "invalid value")]
fn invalid_state_panics() {
    let task = new_task();
    task.inner.state.store(200, Ordering::SeqCst);
    let _ = task.state();
}
#[test]
fn concurrent_claims_have_one_winner() {
    let task = new_task();
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let task = task.clone();
            std::thread::spawn(move || task.claim().is_ok())
        })
        .collect();
    let wins = handles.into_iter().map(|h| h.join().unwrap()).filter(|w| *w).count();
    assert_eq!(wins, 1);
}
#[test]
fn concurrent_claim_and_cancel_are_exclusive() {
    for _ in 0..200 {
        let task = new_task();
        let a = task.clone();
        let b = task.clone();
        let claimer = std::thread::spawn(move || a.claim().is_ok());
        let canceller = std::thread::spawn(move || b.cancel());
        let claimed = claimer.join().unwrap();
        let cancelled = canceller.join().unwrap();
        assert_ne!(claimed, cancelled);
        let expected = if claimed { TaskState::Claimed } else { TaskState::Cancelled };
        assert_eq!(task.state(), expected);
    }
}
#[test]
fn handle_is_send_sync() {
    fn check<X: Send + Sync>() {}
    check::<Task<std::cell::Cell<u8>, u32, String>>();
}
#[tokio::test]
async fn wait_returns_immediately_when_terminal() {
    let task = new_task();
    task.cancel();
    assert_eq!(task.wait(). await, Err(TaskError::Cancelled));
}
#[tokio::test]
async fn wait_is_woken_by_complete() {
    let task = new_task();
    let waiters: Vec<_> = (0..4)
        .map(|_| {
            let t = task.clone();
            tokio::spawn(async move { t.wait().await.map(|v| *v).map_err(|_| ()) })
        })
        .collect();
    tokio::task::yield_now().await;
    task.claim().unwrap();
    task.complete(Ok(5)).unwrap();
    for w in waiters {
        assert_eq!(w. await .unwrap(), Ok(5));
    }
}
#[tokio::test]
async fn wait_is_woken_by_cancel() {
    let task = new_task();
    let t = task.clone();
    let waiter = tokio::spawn(async move {
        matches!(t.wait(). await, Err(TaskError::Cancelled))
    });
    tokio::task::yield_now().await;
    assert!(task.cancel());
    assert!(waiter. await .unwrap());
}
