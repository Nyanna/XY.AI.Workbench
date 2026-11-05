use super::*;

type TestTask = Task<String, u32, String>;

fn new_task() -> TestTask {
    Task::new("data".to_string(), TaskParams::default())
}

#[test]
fn new_task_is_enqueued_without_result() {
    let params = TaskParams {
        priority: 3,
        resource: Some("gpu".into()),
        model: Some("m".into()),
        mode: None,
    };
    let task: TestTask = Task::new("x".into(), params.clone());
    assert_eq!(task.state(), TaskState::Enqueued);
    assert!(task.result().is_none());
    assert_eq!(task.params(), &params);
}

#[test]
fn claim_complete_flow() {
    let task = new_task();
    let other = task.clone();
    assert_eq!(task.claim().unwrap(), "data");
    assert_eq!(other.state(), TaskState::Claimed);
    assert!(other.result().is_none());
    assert!(!task.cancel());
    assert!(task.claim().is_err());
    task.complete(Ok(7)).unwrap();
    assert_eq!(other.result(), Some(Ok(&7)));
}

#[test]
fn failed_result() {
    let task = new_task();
    task.claim().unwrap();
    task.complete(Err("boom".to_string())).unwrap();
    assert_eq!(task.result(), Some(Err(TaskError::Failed(&"boom".to_string()))));
}

#[test]
fn cancel_flow() {
    let task = new_task();
    assert!(task.cancel());
    assert!(!task.cancel());
    assert_eq!(task.state(), TaskState::Cancelled);
    assert_eq!(task.result(), Some(Err(TaskError::Cancelled)));
    assert_eq!(task.claim(), Err(ClaimError::Cancelled));
}

#[test]
fn complete_without_claim_fails() {
    let task = new_task();
    assert_eq!(
        task.complete(Ok(1)),
        Err(CompleteError::NotClaimed(QueueState::Enqueued))
    );
    assert!(task.result().is_none());
    let cancelled = new_task();
    cancelled.cancel();
    assert_eq!(
        cancelled.complete(Ok(1)),
        Err(CompleteError::NotClaimed(QueueState::Cancelled))
    );
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
    assert_eq!(task.wait().await, Err(TaskError::Cancelled));
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
        assert_eq!(w.await.unwrap(), Ok(5));
    }
}

#[tokio::test]
async fn wait_is_woken_by_cancel() {
    let task = new_task();
    let t = task.clone();
    let waiter = tokio::spawn(async move { matches!(t.wait().await, Err(TaskError::Cancelled)) });
    tokio::task::yield_now().await;
    assert!(task.cancel());
    assert!(waiter.await.unwrap());
}
