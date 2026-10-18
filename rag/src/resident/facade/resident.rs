use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use super::error::TaskError;
use super::handle::{State, TaskHandle};
use super::params::{
    EmbeddingParams, InferenceParams, PairInput, SimilarityParams, TextInput, VectorInput,
};
use super::task::{Payload, TaskKind, TaskMeta, TaskQueue};
use crate::resident::error::{ConfigError, PlannerError, ResidentError};
use crate::resident::index::Index;
use crate::resident::op::{IndexId, ModelId, TaskCtx};
use crate::resident::processor::{
    Engine, Expander, Expansion, Hub, OrchestrationError, Processor, Step, Wakeups, WorkerConfig,
};
use crate::resident::queue::{Queue, TaskState};
use crate::resident::telemetry::task_span;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ResidentConfig {
    model: ModelId,
    workers: WorkerConfig,
}

impl ResidentConfig {
    pub fn model(&self) -> &ModelId {
        &self.model
    }

    pub fn workers(&self) -> &WorkerConfig {
        &self.workers
    }

    pub fn min_op_buffer(&self) -> usize {
        self.workers.min_op_buffer
    }
}

/// Stands in until a planner is supplied: tasks fail instead of waiting forever.
struct Unplanned;

impl Expander for Unplanned {
    fn expand(
        &self,
        _meta: &TaskMeta,
        _payload: Payload,
        _ctx: &Arc<TaskCtx>,
    ) -> Result<Expansion, ResidentError> {
        Err(PlannerError::MissingCapability {
            capability: "planner".into(),
        }
        .into())
    }
}

struct NoEngine;

impl Engine for NoEngine {
    fn step(&self, _hub: &Hub) -> Step {
        Step::Idle(None)
    }
}

#[derive(Default)]
pub struct ResidentBuilder {
    model: Option<ModelId>,
    workers: WorkerConfig,
    min_op_buffer: Option<usize>,
    expander: Option<Arc<dyn Expander>>,
    cpu: Option<Arc<dyn Engine>>,
    gpu: Option<Arc<dyn Engine>>,
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

    pub fn expander(mut self, expander: Arc<dyn Expander>) -> Self {
        self.expander = Some(expander);
        self
    }

    /// GPU workers are only started when a GPU engine is supplied.
    pub fn engines(mut self, cpu: Arc<dyn Engine>, gpu: Option<Arc<dyn Engine>>) -> Self {
        self.cpu = Some(cpu);
        self.gpu = gpu;
        self
    }

    /// Starts the processor and its worker threads; model and device resources are
    /// created by the first task, not here.
    pub fn init(self) -> Result<Resident, ConfigError> {
        let model = self.model.ok_or_else(|| ConfigError::MissingField {
            field: "model".into(),
        })?;
        let mut workers = self.workers;
        if let Some(n) = self.min_op_buffer {
            workers.min_op_buffer = n;
        }
        if workers.min_op_buffer == 0 {
            return Err(ConfigError::InvalidValue {
                field: "min_op_buffer".into(),
                reason: "must be at least 1".into(),
            });
        }
        let wakeups = Wakeups::new();
        let queue: Arc<TaskQueue> = Arc::new(Queue::new(wakeups.relays()));
        let processor = Processor::start(
            workers.clone(),
            queue.clone(),
            wakeups,
            self.expander.unwrap_or_else(|| Arc::new(Unplanned)),
            self.cpu.unwrap_or_else(|| Arc::new(NoEngine)),
            self.gpu,
        )
        .map_err(|e| ConfigError::InvalidValue {
            field: "workers".into(),
            reason: e.to_string(),
        })?;
        Ok(Resident {
            config: ResidentConfig { model, workers },
            queue,
            processor,
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
    _index: Arc<dyn Index + Send + Sync>,
}

pub struct Resident {
    config: ResidentConfig,
    queue: Arc<TaskQueue>,
    processor: Processor,
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

    pub fn processor(&self) -> &Processor {
        &self.processor
    }

    /// Registers the index without loading it; loading is triggered by a task.
    pub fn add_index(&self, index: Arc<dyn Index + Send + Sync>) -> IndexId {
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

    /// Joins the workers, then cancels every task that no worker has claimed.
    pub fn shutdown(self) -> Result<(), OrchestrationError> {
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
        self.processor.shutdown();
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
}
