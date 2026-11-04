//! Operation, requires(), Value::Host|Device.
mod lease;
mod requirements;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};
pub use lease::{DeviceHandle, Lease, PinCount};
pub use requirements::{
    Affinity, DeviceId, EngineSelector, IndexId, ModelId, Requirements, ResourceKind,
};
use crate::resident::error::ResidentError;
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OpKind {
    Tokenize,
    Embed,
    Pool,
    L2Norm,
    Similarity,
    TopK,
    WriteIndex,
    Infer,
    /// Explicit device-to-host transfer.
    IntoHost,
}
/// Immutable parameters; usable as cache/graph key.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OpParams {
    pub requires: Requirements,
    pub top_k: Option<u32>,
    pub dim: Option<u32>,
}
impl OpParams {
    pub fn new(requires: Requirements) -> Self {
        Self {
            requires,
            top_k: None,
            dim: None,
        }
    }
}
pub trait Op: Send + Sync {
    fn kind(&self) -> OpKind;
    fn params(&self) -> &OpParams;
    fn requires(&self) -> Requirements {
        self.params().requires.clone()
    }
}
/// Plain-data operation description.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OpDesc {
    kind: OpKind,
    params: OpParams,
}
impl OpDesc {
    pub fn new(kind: OpKind, params: OpParams) -> Self {
        Self { kind, params }
    }
}
impl Op for OpDesc {
    fn kind(&self) -> OpKind {
        self.kind
    }
    fn params(&self) -> &OpParams {
        &self.params
    }
}
/// One resolved hit: manager-level entry, its score, staleness and the source back-reference.
#[derive(Clone, Debug, PartialEq)]
pub struct ScoredHit {
    pub entry: u64,
    pub score: f32,
    pub outdated: bool,
    pub source: Option<crate::resident::index::payload::Payload>,
}
/// Result of a similarity chain before the facade ranks and summarises it.
#[derive(Clone, Debug, PartialEq)]
pub struct ScoredHits {
    /// Entries of the resolved scope that were scored.
    pub candidates: u64,
    /// Hits already dropped while resolving (entry removed).
    pub discarded: usize,
    pub hits: Vec<ScoredHit>,
}
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum HostData {
    Text(Vec<String>),
    Tokens(Vec<Vec<u32>>),
    F32(Vec<f32>),
    Hits(Vec<(u64, f32)>),
    Scored(ScoredHits),
}
/// Op input/output; never carries a tensor or raw pointer.
#[derive(Debug)]
pub enum Value {
    Host(HostData),
    Device(DeviceHandle),
}
/// Per-task shared state: open-op counter, first error, cancellation.
#[derive(Debug)]
pub struct TaskCtx {
    id: u64,
    priority: u8,
    open: AtomicUsize,
    cancelled: AtomicBool,
    error: OnceLock<ResidentError>,
}
impl TaskCtx {
    pub fn new(id: u64, priority: u8, ops: usize) -> Arc<Self> {
        Arc::new(Self {
            id,
            priority,
            open: AtomicUsize::new(ops),
            cancelled: AtomicBool::new(false),
            error: OnceLock::new(),
        })
    }
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn priority(&self) -> u8 {
        self.priority
    }
    /// Registers `n` follow-up ops. Must run before they are enqueued and before the
    /// producing op is finished, so the counter never reaches zero early.
    pub fn add_ops(&self, n: usize) {
        self.open.fetch_add(n, Ordering::Relaxed);
    }
    pub fn open_ops(&self) -> usize {
        self.open.load(Ordering::Acquire)
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
    pub fn error(&self) -> Option<&ResidentError> {
        self.error.get()
    }
    /// Returns `true` when this call closed the last open op of a non-failed task.
    pub fn finish_op(&self) -> bool {
        let prev = self.open.fetch_sub(1, Ordering::AcqRel);
        debug_assert!(prev > 0, "finish_op without open op");
        prev == 1 && !self.is_cancelled()
    }
    /// Cancels the task; only the first error is kept. Returns `true` if this call was first.
    pub fn fail(&self, err: ResidentError) -> bool {
        let first = self.error.set(err).is_ok();
        self.cancelled.store(true, Ordering::Release);
        if first {
            tracing::debug!(task = self.id, "task cancelled by first error");
        }
        first
    }
}
/// Builds the operation that consumes a finished operation's output.
pub trait Successor: Send + Sync {
    /// Empty when the chain ends; the implementation then keeps `output` as the task result.
    fn follow(&self, output: Value, task: &Arc<TaskCtx>) -> Vec<Arc<OperationNode>>;
}
pub struct OperationNode {
    pub op: Arc<dyn Op>,
    pub input: Value,
    pub output: OnceLock<Value>,
    /// `true`: result returns to host; `false`: device handle stays resident.
    pub materialize: bool,
    pub task: Arc<TaskCtx>,
    pub successor: Option<Arc<dyn Successor>>,
}
impl OperationNode {
    pub fn new(
        op: Arc<dyn Op>,
        input: Value,
        materialize: bool,
        task: Arc<TaskCtx>,
    ) -> Self {
        tracing::debug!(
            task = task.id(), kind = ? op.kind(), resource = ? op.params().requires
            .resource, materialize, "op created"
        );
        Self {
            op,
            input,
            output: OnceLock::new(),
            materialize,
            task,
            successor: None,
        }
    }
    pub fn with_successor(mut self, successor: Arc<dyn Successor>) -> Self {
        self.successor = Some(successor);
        self
    }
    /// Sets the output once; a second call returns the rejected value.
    pub fn set_output(&self, value: Value) -> Result<(), Value> {
        self.output.set(value)
    }
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
