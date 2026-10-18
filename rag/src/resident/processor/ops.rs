use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

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
}

impl TaskRun {
    pub fn new(ctx: Arc<TaskCtx>, state: Arc<TaskState<Output, TaskError>>, collect: Collect) -> Arc<Self> {
        Arc::new(Self { ctx, state, collect: Mutex::new(Some(collect)), completed: AtomicBool::new(false) })
    }

    pub fn ctx(&self) -> &Arc<TaskCtx> {
        &self.ctx
    }

    pub fn is_cancelled(&self) -> bool {
        self.ctx.is_cancelled()
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
            if job.run.is_cancelled() {
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
        let run = TaskRun::new(ctx, state.clone(), Box::new(|| Ok(Output::Scores(vec![1.0]))));
        (run, state)
    }
}

#[cfg(test)]
mod tests {
    use super::testutil::*;
    use super::*;
    use crate::resident::op::ResourceKind;

    fn queues(has_gpu: bool) -> OpQueues {
        OpQueues::new(Arc::new(Notify::new()), Arc::new(Notify::new()), has_gpu)
    }

    fn job(run: &Arc<TaskRun>, resource: ResourceKind, engine: EngineSelector) -> OpJob {
        OpJob::new(node(run.ctx(), resource, engine), run.clone())
    }

    fn any(_: &OpParams) -> bool {
        true
    }

    #[test]
    fn routes_by_engine_and_rejects_missing_gpu() {
        let q = queues(false);
        let (run, _) = run(1);
        run.ctx().add_ops(2);
        q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Either)).unwrap();
        let err = q.push(job(&run, ResourceKind::Encoder, EngineSelector::Gpu)).unwrap_err();
        assert!(matches!(err, OrchestrationError::EngineUnavailable { .. }));
        assert_eq!(q.queued(Side::Cpu), 1);
        assert_eq!(q.queued(Side::Gpu), 0);
    }

    #[test]
    fn own_queue_precedes_shared_and_fifo_holds() {
        let q = queues(true);
        let (run, _) = run(1);
        run.ctx().add_ops(3);
        q.push_shared(job(&run, ResourceKind::Scorer, EngineSelector::Either));
        q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu)).unwrap();
        q.push(job(&run, ResourceKind::Normalizer, EngineSelector::Cpu)).unwrap();
        let kinds: Vec<_> = (0..3)
            .map(|_| q.claim(Side::Cpu, &any).unwrap().params().requires.resource)
            .collect();
        assert_eq!(kinds, [ResourceKind::Tokenizer, ResourceKind::Normalizer, ResourceKind::Scorer]);
        assert_eq!(q.total_queued(), 0);
    }

    #[test]
    fn foreign_head_does_not_block_shared_queue() {
        let q = queues(true);
        let (run, _) = run(1);
        run.ctx().add_ops(3);
        q.push_shared(job(&run, ResourceKind::Encoder, EngineSelector::Gpu));
        q.push_shared(job(&run, ResourceKind::Encoder, EngineSelector::Gpu));
        q.push_shared(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu));
        let cpu_only = |p: &OpParams| p.requires.engine == EngineSelector::Cpu;
        let got = q.claim(Side::Cpu, &cpu_only).unwrap();
        assert_eq!(got.params().requires.resource, ResourceKind::Tokenizer);
        assert!(q.claim(Side::Cpu, &cpu_only).is_none());
        assert_eq!(q.total_queued(), 2);
    }

    #[test]
    fn cancelled_task_ops_are_dropped_at_claim() {
        let q = queues(false);
        let (run, _) = run(1);
        run.ctx().add_ops(2);
        q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu)).unwrap();
        q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu)).unwrap();
        run.fail(crate::resident::error::PlannerError::UnknownModel { model: "m".into() }.into());
        assert!(q.claim(Side::Cpu, &any).is_none());
        assert_eq!(run.ctx().open_ops(), 0);
    }

    #[test]
    fn last_op_completes_task_once() {
        let (run, state) = run(7);
        run.ctx().add_ops(2);
        run.op_done();
        assert!(state.result().is_none());
        run.op_done();
        assert_eq!(state.result().unwrap().as_ref().unwrap(), &Output::Scores(vec![1.0]));
    }

    #[test]
    fn followups_counted_before_predecessor_closes() {
        let (run, state) = run(7);
        run.ctx().add_ops(1);
        run.ctx().add_ops(2);
        run.op_done();
        assert!(state.result().is_none());
        run.op_done();
        assert!(state.result().is_none());
        run.op_done();
        assert!(state.result().is_some());
    }

    #[test]
    fn first_error_wins_and_completes_exactly_once() {
        let (run, state) = run(7);
        run.ctx().add_ops(3);
        run.fail(crate::resident::error::PlannerError::UnknownModel { model: "a".into() }.into());
        run.fail(crate::resident::error::PlannerError::UnknownModel { model: "b".into() }.into());
        run.skip();
        run.skip();
        run.skip();
        let msg = format!("{:?}", state.result().unwrap());
        assert!(msg.contains("Engine") && msg.contains("\"a\""), "{msg}");
        assert!(run.ctx().is_cancelled());
    }
}
