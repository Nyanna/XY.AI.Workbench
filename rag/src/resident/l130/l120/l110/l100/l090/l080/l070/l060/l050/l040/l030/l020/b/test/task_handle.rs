use super::*;

type Entry = Arc<EntryState<u32, u32, String>>;

fn entry() -> Entry {
    EntryState::new(7)
}

#[test]
fn try_result_states() {
    let e = entry();
    let h = TaskHandle::new(e.clone());
    assert_eq!(h.try_result().unwrap_err(), HandleError::NotReady);
    e.claim().unwrap();
    assert_eq!(h.try_result().unwrap_err(), HandleError::NotReady);
    e.complete(Ok(5)).unwrap();
    assert_eq!(h.try_result().unwrap(), &Ok(5));
    assert_eq!(h.try_result().unwrap(), &Ok(5));
}

#[test]
fn drop_cancels_enqueued_only() {
    let e = entry();
    drop(TaskHandle::new(e.clone()));
    assert_eq!(e.state(), QueueState::Cancelled);

    let e = entry();
    e.claim().unwrap();
    drop(TaskHandle::new(e.clone()));
    assert_eq!(e.state(), QueueState::Claimed);
}

#[test]
fn display_texts() {
    assert_eq!(HandleError::Cancelled.to_string(), "cancelled");
    assert_eq!(HandleError::NotReady.to_string(), "noch kein Ergebnis");
    assert_eq!(HandleError::Timeout.to_string(), "timeout");
}

#[tokio::test(start_paused = true)]
async fn wait_gets_worker_result() {
    let e = entry();
    let h = TaskHandle::new(e.clone());
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(250)).await;
        e.claim().unwrap();
        e.complete(Err("boom".to_string())).unwrap();
    });
    let out = h.wait().await.unwrap();
    assert_eq!(out.result(), &Err("boom".to_string()));
}

#[tokio::test(start_paused = true)]
async fn wait_on_cancelled() {
    let e = entry();
    let h = TaskHandle::new(e.clone());
    assert!(e.cancel());
    assert_eq!(h.wait().await.err(), Some(HandleError::Cancelled));
}

#[tokio::test(start_paused = true)]
async fn timeout_cancels_unclaimed() {
    let e = entry();
    let h = TaskHandle::new(e.clone());
    let r = h.wait_timeout(Duration::from_millis(350)).await;
    assert_eq!(r.err(), Some(HandleError::Timeout));
    assert_eq!(e.state(), QueueState::Cancelled);
}

#[tokio::test(start_paused = true)]
async fn timeout_keeps_claimed_running() {
    let e = entry();
    e.claim().unwrap();
    let h = TaskHandle::new(e.clone());
    let r = h.wait_timeout(Duration::from_millis(50)).await;
    assert_eq!(r.err(), Some(HandleError::Timeout));
    assert_eq!(e.state(), QueueState::Claimed);
}

#[tokio::test(start_paused = true)]
async fn zero_timeout() {
    let e = entry();
    let h = TaskHandle::new(e.clone());
    assert_eq!(
        h.wait_timeout(Duration::ZERO).await.err(),
        Some(HandleError::Timeout)
    );
    let e = entry();
    e.claim().unwrap();
    e.complete(Ok(1)).unwrap();
    let out = TaskHandle::new(e).wait_timeout(Duration::ZERO).await.unwrap();
    assert_eq!(out.result(), &Ok(1));
}

fn assert_send_sync<X: Send + Sync>() {}

#[test]
fn auto_traits() {
    assert_send_sync::<TaskHandle<u32, u32, String>>();
    assert_send_sync::<TaskOutcome<u32, u32, String>>();
}
