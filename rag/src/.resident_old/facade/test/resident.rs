use std::time::Duration;

use super::*;
use crate::resident::facade::Output;

fn resident() -> Resident {
    Resident::builder()
        .model(ModelId("e5-small".into()))
        .worker_hint(WorkerConfig::new(0, 0))
        .init()
        .unwrap()
}

fn serve_one(r: &Resident, out: Output) -> bool {
    for node in r.queue().iter_live() {
        if node.claim().is_some() {
            node.task_state().unwrap().complete(Ok(out));
            return true;
        }
    }
    false
}

#[test]
fn builder_requires_model() {
    let err = Resident::builder().init().err().unwrap();
    assert!(matches!(err, ConfigError::MissingField { .. }));
}

#[test]
fn builder_rejects_zero_op_buffer() {
    let err = Resident::builder()
        .model(ModelId("m".into()))
        .min_op_buffer(0)
        .init()
        .err()
        .unwrap();
    assert!(matches!(err, ConfigError::InvalidValue { .. }));
}

#[tokio::test]
async fn submitted_task_resolves_with_output() {
    let r = resident();
    let handle = r.submit_embedding(TextInput::single("hi"), EmbeddingParams::default());
    assert!(handle.try_get().is_none());
    assert!(serve_one(&r, Output::Embeddings(vec![vec![1.0]])));
    assert_eq!(handle.await.unwrap(), Output::Embeddings(vec![vec![1.0]]));
}

#[tokio::test]
async fn nan_precision_fails_without_enqueue() {
    let r = resident();
    let params = EmbeddingParams::default().precision(f32::NAN);
    let handle = r.submit_embedding(TextInput::single("hi"), params);
    assert!(matches!(handle.await, Err(TaskError::InvalidPrecision)));
    assert!(!serve_one(&r, Output::Scores(vec![])));
}

#[test]
fn dropping_handle_cancels_queued_task() {
    let r = resident();
    let handle = r.submit_inference(PairInput::new("q", "c"), InferenceParams::default());
    drop(handle);
    assert!(!serve_one(&r, Output::Scores(vec![])));
}

#[test]
fn dropping_handle_after_claim_keeps_result() {
    let r = resident();
    let handle = r.submit_inference(PairInput::new("q", "c"), InferenceParams::default());
    assert!(serve_one(&r, Output::Scores(vec![0.5])));
    drop(handle);
}

#[tokio::test(start_paused = true)]
async fn wait_timeout_reports_timeout_and_cancels() {
    let r = resident();
    let handle = r.submit_similarity(
        VectorInput::new(vec![vec![0.0]]),
        SimilarityParams::new(IndexId("x".into())),
    );
    let res = handle.wait_timeout(Duration::from_millis(10)).await;
    assert!(matches!(res, Err(TaskError::Timeout)));
    assert!(!serve_one(&r, Output::Scores(vec![])));
}

#[test]
fn shutdown_cancels_pending_tasks() {
    let r = resident();
    let handle = r.submit_embedding(TextInput::single("a"), EmbeddingParams::default());
    r.shutdown().unwrap();
    assert!(matches!(handle.try_get(), Some(Err(TaskError::Cancelled))));
}

#[tokio::test]
async fn submit_wakes_workers_and_unplanned_task_fails() {
    let r = Resident::builder()
        .model(ModelId("m".into()))
        .init()
        .unwrap();
    let handle = r.submit_embedding(TextInput::single("hi"), EmbeddingParams::default());
    let res = handle.wait_timeout(Duration::from_secs(5)).await;
    assert!(matches!(res, Err(TaskError::Engine(_))), "{res:?}");
}

#[test]
fn blocking_wrapper_returns_task_result() {
    let r = Resident::builder()
        .model(ModelId("m".into()))
        .init()
        .unwrap();
    let blocking = crate::resident::facade::ResidentBlocking::new(r).unwrap();
    let res = blocking.embedding(TextInput::single("hi"), EmbeddingParams::default());
    assert!(matches!(res, Err(TaskError::Engine(_))));
}

#[test]
fn add_index_registers_without_loading() {
    let r = resident();
    let id = r.add_index(Arc::new(crate::resident::index::memory::InMemoryIndex::new()));
    let info = r.index_info(&id).unwrap();
    assert_eq!(info.residency, Residency::NotLoaded);
    assert_eq!(info.leases, 0);
    assert!(r.index_info(&IndexId("missing".into())).is_none());
}

#[test]
fn builder_min_op_buffer_overrides_worker_hint() {
    let r = Resident::builder()
        .model(ModelId("m".into()))
        .worker_hint(WorkerConfig::new(0, 0).with_min_op_buffer(7))
        .min_op_buffer(3)
        .init()
        .unwrap();
    assert_eq!(r.config().min_op_buffer(), 3);
}
