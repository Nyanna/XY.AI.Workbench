use std::any::Any;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use super::error::TaskError;
use super::handle::{State, TaskHandle};
use super::params::{
    EmbeddingParams, InferenceParams, PairInput, SimilarityParams, TextInput, VectorInput,
};
use super::task::{Payload, TaskKind, TaskMeta, TaskQueue};
use crate::resident::error::{ConfigError, ResidentError};
use crate::resident::index::Index;
use crate::resident::op::{IndexId, ModelId};
use crate::resident::queue::{Queue, TaskState};
use crate::resident::telemetry::task_span;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct WorkerConfig {
    pub cpu: usize,
    pub gpu: usize,
}

impl WorkerConfig {
    pub fn new(cpu: usize, gpu: usize) -> Self {
        Self { cpu, gpu }
    }
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self::new(1, 0)
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ResidentConfig {
    model: ModelId,
    workers: WorkerConfig,
    min_op_buffer: usize,
}

impl ResidentConfig {
    pub fn model(&self) -> &ModelId {
        &self.model
    }

    pub fn workers(&self) -> WorkerConfig {
        self.workers
    }

    pub fn min_op_buffer(&self) -> usize {
        self.min_op_buffer
    }
}

#[derive(Debug, Default)]
pub struct ResidentBuilder {
    model: Option<ModelId>,
    workers: WorkerConfig,
    min_op_buffer: Option<usize>,
}

impl ResidentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn model(mut self, model: ModelId) -> Self {
        self.model = Some(model);
        self
    }

    pub fn worker_hint(mut self, workers: WorkerConfig) -> Self {
        self.workers = workers;
        self
    }

    pub fn min_op_buffer(mut self, n: usize) -> Self {
        self.min_op_buffer = Some(n);
        self
    }

    /// Acquires no model or device resources; those are created by the first task.
    pub fn init(self) -> Result<Resident, ConfigError> {
        let model = self.model.ok_or_else(|| ConfigError::MissingField {
            field: "model".into(),
        })?;
        let min_op_buffer = self.min_op_buffer.unwrap_or(1);
        if min_op_buffer == 0 {
            return Err(ConfigError::InvalidValue {
                field: "min_op_buffer".into(),
                reason: "must be at least 1".into(),
            });
        }
        Ok(Resident {
            config: ResidentConfig {
                model,
                workers: self.workers,
                min_op_buffer,
            },
            queue: Arc::new(Queue::new(Vec::new())),
            next_task: AtomicU64::new(0),
            next_index: AtomicU64::new(0),
            indexes: Mutex::new(HashMap::new()),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Residency {
    NotLoaded,
    Loaded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct IndexInfo {
    pub residency: Residency,
    pub leases: usize,
}

struct IndexEntry {
    info: IndexInfo,
    _index: Arc<dyn Any + Send + Sync>,
}

pub struct Resident {
    config: ResidentConfig,
    queue: Arc<TaskQueue>,
    next_task: AtomicU64,
    next_index: AtomicU64,
    indexes: Mutex<HashMap<IndexId, IndexEntry>>,
}

impl Resident {
    pub fn builder() -> ResidentBuilder {
        ResidentBuilder::new()
    }

    pub fn config(&self) -> &ResidentConfig {
        &self.config
    }

    pub fn queue(&self) -> &Arc<TaskQueue> {
        &self.queue
    }

    /// Registers the index without loading it; loading is triggered by a task.
    /// `Index` has return-position `impl Trait` and is not dyn-compatible, so the
    /// table keeps the concrete index type-erased.
    pub fn add_index<I>(&self, index: Arc<I>) -> IndexId
    where
        I: Index + Send + Sync + 'static,
    {
        let id = IndexId(format!("idx-{}", self.next_index.fetch_add(1, Ordering::Relaxed)));
        self.lock_indexes().insert(
            id.clone(),
            IndexEntry {
                info: IndexInfo {
                    residency: Residency::NotLoaded,
                    leases: 0,
                },
                _index: index,
            },
        );
        id
    }

    pub fn index_info(&self, id: &IndexId) -> Option<IndexInfo> {
        self.lock_indexes().get(id).map(|entry| entry.info)
    }

    pub fn submit_embedding(&self, input: TextInput, params: EmbeddingParams) -> TaskHandle {
        let precision = params.get_precision();
        self.submit(
            TaskKind::Embedding,
            precision,
            Payload::Embedding { input, params },
        )
    }

    pub fn submit_similarity(&self, input: VectorInput, params: SimilarityParams) -> TaskHandle {
        let precision = params.get_precision();
        self.submit(
            TaskKind::Similarity,
            precision,
            Payload::Similarity { input, params },
        )
    }

    pub fn submit_inference(&self, input: PairInput, params: InferenceParams) -> TaskHandle {
        let precision = params.get_precision();
        self.submit(
            TaskKind::Inference,
            precision,
            Payload::Inference { input, params },
        )
    }

    /// Cancels every task that no worker has claimed yet.
    pub fn shutdown(self) -> Result<(), ResidentError> {
        drop(self);
        Ok(())
    }

    fn submit(&self, kind: TaskKind, precision: f32, payload: Payload) -> TaskHandle {
        let id = self.next_task.fetch_add(1, Ordering::Relaxed);
        let state: Arc<State> = Arc::new(TaskState::new());
        let handle = TaskHandle::new(state.clone(), id);
        task_span(&id.to_string()).in_scope(|| {
            if precision.is_nan() {
                state.complete(Err(TaskError::InvalidPrecision));
            } else {
                self.queue.push(TaskMeta { id, kind }, state, payload);
            }
            tracing::debug!(?kind, "task submitted");
        });
        handle
    }

    fn lock_indexes(&self) -> std::sync::MutexGuard<'_, HashMap<IndexId, IndexEntry>> {
        self.indexes.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl Drop for Resident {
    fn drop(&mut self) {
        for node in self.queue.iter_live() {
            if let Some(state) = node.task_state() {
                state.cancel();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::resident::facade::Output;

    fn resident() -> Resident {
        Resident::builder()
            .model(ModelId("e5-small".into()))
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
}
