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
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum HostData {
    Text(Vec<String>),
    Tokens(Vec<Vec<u32>>),
    F32(Vec<f32>),
    Hits(Vec<(u64, f32)>),
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
mod tests {
    use super::*;
    use crate::resident::error::PlannerError;
    use crate::resident::memory::region::ArenaId;
    use crate::resident::memory::Region;
    fn desc(kind: OpKind) -> Arc<dyn Op> {
        let req = Requirements::new(ResourceKind::Encoder, EngineSelector::Either);
        Arc::new(OpDesc::new(kind, OpParams::new(req)))
    }
    fn err() -> ResidentError {
        PlannerError::InvalidConfig {
            reason: "x".into(),
        }
            .into()
    }
    #[test]
    fn params_clone_hash_eq() {
        use std::collections::HashSet;
        let p = OpParams::new(
            Requirements::new(ResourceKind::Scorer, EngineSelector::Gpu),
        );
        let mut set = HashSet::new();
        set.insert(p.clone());
        assert!(set.contains(& p));
        let mut q = p.clone();
        q.top_k = Some(5);
        assert_ne!(p, q);
    }
    #[test]
    fn requires_comes_from_params() {
        let op = desc(OpKind::Embed);
        assert_eq!(op.requires(), op.params().requires);
    }
    #[test]
    fn task_done_only_at_zero() {
        let ctx = TaskCtx::new(1, 0, 3);
        assert!(! ctx.finish_op());
        assert!(! ctx.finish_op());
        assert!(ctx.finish_op());
        assert_eq!(ctx.open_ops(), 0);
    }
    #[test]
    fn first_error_cancels_and_is_kept() {
        let ctx = TaskCtx::new(1, 0, 2);
        assert!(ctx.fail(err()));
        assert!(! ctx.fail(err()));
        assert!(ctx.is_cancelled());
        assert!(ctx.error().is_some());
        ctx.finish_op();
        assert!(! ctx.finish_op());
    }
    #[test]
    fn output_set_once_and_device_value_holds_lease() {
        let ctx = TaskCtx::new(1, 0, 1);
        let node = OperationNode::new(
            desc(OpKind::Tokenize),
            Value::Host(HostData::Text(vec![])),
            true,
            ctx,
        );
        assert!(node.set_output(Value::Host(HostData::F32(vec![1.0]))).is_ok());
        assert!(node.set_output(Value::Host(HostData::F32(vec![]))).is_err());
        let pin = PinCount::new();
        let region = Region {
            arena: ArenaId(0),
            offset: 0,
            len: 8,
            gen: 1,
        };
        let v = Value::Device(Lease::new(region, pin.clone(), None));
        assert_eq!(pin.get(), 1);
        drop(v);
        assert!(pin.is_unpinned());
    }
    #[test]
    fn affinity_is_non_exhaustive() {
        let a = Affinity::Device(DeviceId(0));
        #[allow(unreachable_patterns)]
        let _ = match a {
            Affinity::Handle(_) | Affinity::Device(_) => {}
            _ => {}
        };
    }
}
