use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

use parking_lot::Mutex;
use tokio::sync::Notify;

use crate::resident::error::ResidentError;
use crate::resident::facade::{Output, TaskError};
use crate::resident::op::{EngineSelector, OpParams, OperationNode, TaskCtx};
use crate::resident::queue::{Queue, TaskState};

use super::error::OrchestrationError;

pub type Collect = Box<dyn FnOnce() -> Result<Output, TaskError> + Send>;

/// Per-task completion bookkeeping shared by all of the task's operations.
pub struct TaskRun {
    ctx: Arc<TaskCtx>,
    state: Arc<TaskState<Output, TaskError>>,
    collect: Mutex<Option<Collect>>,
    completed: AtomicBool,
    deadline: Option<Instant>,
}

impl TaskRun {
    pub fn new(
        ctx: Arc<TaskCtx>,
        state: Arc<TaskState<Output, TaskError>>,
        collect: Collect,
        deadline: Option<Instant>,
    ) -> Arc<Self> {
        Arc::new(Self {
            ctx,
            state,
            collect: Mutex::new(Some(collect)),
            completed: AtomicBool::new(false),
            deadline,
        })
    }

    pub fn ctx(&self) -> &Arc<TaskCtx> {
        &self.ctx
    }

    pub fn is_cancelled(&self) -> bool {
        self.ctx.is_cancelled()
    }

    /// Fails the task with a timeout once its deadline has passed; returns whether it has.
    pub fn expire_if_due(&self) -> bool {
        let due = self.deadline.is_some_and(|d| Instant::now() >= d);
        if due && self.ctx.fail(OrchestrationError::Timeout { task: self.ctx.id() }.into()) {
            self.complete_once(|| Err(TaskError::Timeout));
        }
        due
    }

    fn complete_once(&self, result: impl FnOnce() -> Result<Output, TaskError>) {
        if !self.completed.swap(true, Ordering::AcqRel) {
            self.state.complete(result());
        }
    }

    /// Completes the task with its collected result.
    pub fn finish(&self) {
        let collect = self.collect.lock().take();
        self.complete_once(|| match collect {
            Some(collect) => collect(),
            None => Err(TaskError::Cancelled),
        });
    }

    /// Closes one operation; the last one of a healthy task completes it.
    pub fn op_done(&self) {
        if self.ctx.finish_op() {
            self.finish();
        }
    }

    /// Closes an operation that is dropped unexecuted because the task already failed.
    pub fn skip(&self) {
        let _ = self.ctx.finish_op();
    }

    /// Records the first error and completes the task with it. Does not touch the op counter.
    pub fn fail(&self, err: ResidentError) {
        let err = Arc::new(err);
        let wrapped = OrchestrationError::OpFailed { task: self.ctx.id(), source: err.clone() };
        if self.ctx.fail(wrapped.into()) {
            self.complete_once(|| Err(TaskError::Engine(err)));
        }
    }
}

pub struct OpJob {
    pub node: Arc<OperationNode>,
    pub run: Arc<TaskRun>,
}

impl OpJob {
    pub fn new(node: Arc<OperationNode>, run: Arc<TaskRun>) -> Self {
        Self { node, run }
    }

    pub fn params(&self) -> &OpParams {
        self.node.op.params()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Cpu,
    Gpu,
}

pub type OpQueue = Queue<OpParams, OpJob, (), ()>;

const SHARED_SKIP_LIMIT: usize = 8;

struct Counted {
    queue: OpQueue,
    queued: AtomicUsize,
}

impl Counted {
    fn new(relays: Vec<Arc<Notify>>) -> Self {
        Self { queue: Queue::new(relays), queued: AtomicUsize::new(0) }
    }

    fn push(&self, job: OpJob) {
        let params = job.params().clone();
        self.queued.fetch_add(1, Ordering::AcqRel);
        self.queue.push(params, Arc::new(TaskState::new()), job);
    }

    /// Claims the oldest acceptable job, skipping at most `max_skip` foreign entries.
    fn claim(&self, accepts: &dyn Fn(&OpParams) -> bool, max_skip: usize) -> Option<OpJob> {
        let mut skipped = 0;
        for node in self.queue.iter_live() {
            let Some(params) = node.params() else { continue };
            if !accepts(params) {
                skipped += 1;
                if skipped >= max_skip {
                    break;
                }
                continue;
            }
            let Some(job) = self.queue.claim(&node) else { continue };
            self.queued.fetch_sub(1, Ordering::AcqRel);
            if let Some(state) = node.task_state() {
                state.complete(Ok(()));
            }
            if job.run.is_cancelled() || job.run.expire_if_due() {
                job.run.skip();
                continue;
            }
            return Some(job);
        }
        None
    }
}

/// CPU, GPU and shared operation queues. Shared is wired in but never filled by routing.
pub struct OpQueues {
    cpu: Counted,
    gpu: Counted,
    shared: Counted,
    has_gpu: bool,
}

impl OpQueues {
    pub fn new(cpu_wake: Arc<Notify>, gpu_wake: Arc<Notify>, has_gpu: bool) -> Self {
        Self {
            cpu: Counted::new(vec![cpu_wake.clone()]),
            gpu: Counted::new(vec![gpu_wake.clone()]),
            shared: Counted::new(vec![cpu_wake, gpu_wake]),
            has_gpu,
        }
    }

    pub fn route(&self, engine: EngineSelector) -> Result<Side, OrchestrationError> {
        match engine {
            EngineSelector::Gpu if !self.has_gpu => Err(OrchestrationError::EngineUnavailable { engine }),
            EngineSelector::Gpu => Ok(Side::Gpu),
            _ => Ok(Side::Cpu),
        }
    }

    pub fn push(&self, job: OpJob) -> Result<(), OrchestrationError> {
        let side = self.route(job.params().requires.engine)?;
        self.side(side).push(job);
        Ok(())
    }

    pub fn push_shared(&self, job: OpJob) {
        self.shared.push(job);
    }

    fn side(&self, side: Side) -> &Counted {
        match side {
            Side::Cpu => &self.cpu,
            Side::Gpu => &self.gpu,
        }
    }

    /// Own queue first, then shared.
    pub fn claim(&self, side: Side, accepts: &dyn Fn(&OpParams) -> bool) -> Option<OpJob> {
        self.side(side)
            .claim(accepts, usize::MAX)
            .or_else(|| self.shared.claim(accepts, SHARED_SKIP_LIMIT))
    }

    pub fn queued(&self, side: Side) -> usize {
        self.side(side).queued.load(Ordering::Acquire)
    }

    pub fn total_queued(&self) -> usize {
        self.cpu.queued.load(Ordering::Acquire)
            + self.gpu.queued.load(Ordering::Acquire)
            + self.shared.queued.load(Ordering::Acquire)
    }
}

#[cfg(test)]
pub(crate) mod testutil {
    use super::*;
    use crate::resident::op::{Op, OpDesc, OpKind, Requirements, ResourceKind, Value, HostData};

    pub fn node(task: &Arc<TaskCtx>, resource: ResourceKind, engine: EngineSelector) -> Arc<OperationNode> {
        let params = OpParams::new(Requirements::new(resource, engine));
        let op: Arc<dyn Op> = Arc::new(OpDesc::new(OpKind::Embed, params));
        Arc::new(OperationNode::new(op, Value::Host(HostData::F32(vec![])), true, task.clone()))
    }

    pub fn run(id: u64) -> (Arc<TaskRun>, Arc<TaskState<Output, TaskError>>) {
        let state = Arc::new(TaskState::new());
        let ctx = TaskCtx::new(id, 0, 0);
        let run = TaskRun::new(ctx, state.clone(), Box::new(|| Ok(Output::Scores(vec![1.0]))), None);
        (run, state)
    }
}

#[cfg(test)]
#[path = "test/ops.rs"]
mod tests;
