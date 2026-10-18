use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::sync::Notify;

use crate::resident::error::ResidentError;
use crate::resident::facade::{Payload, TaskError, TaskMeta, TaskQueue};
use crate::resident::op::{OpParams, OperationNode, ResourceKind, TaskCtx};
use crate::resident::queue::ClaimState;
use crate::resident::telemetry;

use super::config::ProcessorConfig;
use super::error::OrchestrationError;
use super::ops::{Collect, OpJob, OpQueues, Side, TaskRun};
use super::starved::{StarvedSet, StarvedSnapshot};

pub struct Expansion {
    pub ops: Vec<Arc<OperationNode>>,
    pub collect: Collect,
}

/// Task-to-operation expansion; implemented by the planner.
pub trait Expander: Send + Sync {
    /// Resource the task's first operation needs, used to serve starved hints first.
    fn first_resource(&self, _meta: &TaskMeta) -> Option<ResourceKind> {
        None
    }

    fn expand(&self, meta: &TaskMeta, payload: Payload, ctx: &Arc<TaskCtx>) -> Result<Expansion, ResidentError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Worked,
    /// Nothing claimable; carries the resource kind that sat free without work.
    Idle(Option<ResourceKind>),
}

/// Subengine side of the pull model. Each method does at most one unit of work.
pub trait Engine: Send + Sync {
    /// Takes over finished device work and returns results to their tasks.
    fn harvest(&self, _hub: &Hub) -> bool {
        false
    }

    /// Synchronises indexes and launches work onto a free lane.
    fn feed(&self, _hub: &Hub) -> bool {
        false
    }

    /// Claims and runs operations.
    fn step(&self, hub: &Hub) -> Step;
}

pub struct Wakeups {
    pub cpu: Arc<Notify>,
    pub gpu: Arc<Notify>,
}

impl Wakeups {
    pub fn new() -> Self {
        Self { cpu: Arc::new(Notify::new()), gpu: Arc::new(Notify::new()) }
    }

    /// Relays for the task queue so every submit wakes both worker groups.
    pub fn relays(&self) -> Vec<Arc<Notify>> {
        vec![self.cpu.clone(), self.gpu.clone()]
    }

    pub fn side(&self, side: Side) -> &Arc<Notify> {
        match side {
            Side::Cpu => &self.cpu,
            Side::Gpu => &self.gpu,
        }
    }
}

impl Default for Wakeups {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Hub {
    tasks: Arc<TaskQueue>,
    queues: OpQueues,
    starved_cpu: StarvedSet,
    starved_gpu: StarvedSet,
    wakeups: Wakeups,
    expander: Arc<dyn Expander>,
    cfg: ProcessorConfig,
    shutdown: AtomicBool,
}

impl Hub {
    pub(super) fn new(
        cfg: ProcessorConfig,
        tasks: Arc<TaskQueue>,
        wakeups: Wakeups,
        expander: Arc<dyn Expander>,
    ) -> Self {
        Self {
            tasks,
            queues: OpQueues::new(wakeups.cpu.clone(), wakeups.gpu.clone(), cfg.gpu_workers > 0),
            starved_cpu: StarvedSet::new(cfg.starved_ttl),
            starved_gpu: StarvedSet::new(cfg.starved_ttl),
            wakeups,
            expander,
            cfg,
            shutdown: AtomicBool::new(false),
        }
    }

    pub fn config(&self) -> &ProcessorConfig {
        &self.cfg
    }

    pub fn queues(&self) -> &OpQueues {
        &self.queues
    }

    pub fn wakeups(&self) -> &Wakeups {
        &self.wakeups
    }

    pub fn starved(&self, side: Side) -> &StarvedSet {
        match side {
            Side::Cpu => &self.starved_cpu,
            Side::Gpu => &self.starved_gpu,
        }
    }

    pub(super) fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::Acquire)
    }

    pub(super) fn begin_shutdown(&self) {
        self.shutdown.store(true, Ordering::Release);
    }

    /// A pick of a resource kind satisfies that kind's starved hint.
    pub fn claim(&self, side: Side, accepts: &dyn Fn(&OpParams) -> bool) -> Option<OpJob> {
        let job = self.queues.claim(side, accepts)?;
        self.starved(side).clear(job.params().requires.resource);
        Some(job)
    }

    /// Closes `job` successfully after registering and enqueueing its follow-up operations.
    pub fn complete_op(&self, job: OpJob, followups: Vec<Arc<OperationNode>>) {
        let OpJob { run, .. } = job;
        if !followups.is_empty() {
            if let Err(err) = self.enqueue(&run, followups) {
                run.fail(err.into());
                run.skip();
                return;
            }
        }
        run.op_done();
    }

    pub fn fail_op(&self, job: OpJob, err: ResidentError) {
        tracing::debug!(task = job.run.ctx().id(), error = %err, "operation failed");
        job.run.fail(err);
        job.run.skip();
    }

    /// Counts first, then enqueues, so the task's open-op counter never dips to zero early.
    fn enqueue(&self, run: &Arc<TaskRun>, ops: Vec<Arc<OperationNode>>) -> Result<(), OrchestrationError> {
        for node in &ops {
            self.queues.route(node.op.params().requires.engine)?;
        }
        run.ctx().add_ops(ops.len());
        for node in ops {
            self.queues.push(OpJob::new(node, run.clone()))?;
        }
        Ok(())
    }

    /// Expands one queued task when hinted or when the op buffer ran below its waterline.
    pub(super) fn expand_if_needed(&self, side: Side) -> bool {
        let hint = self.starved(side).snapshot();
        if hint.is_empty() && self.queues.total_queued() >= self.cfg.min_op_buffer {
            return false;
        }
        let expanded = self.expand_one(hint);
        if expanded {
            self.starved(side).clear_all(hint);
        }
        expanded
    }

    fn expand_one(&self, hint: StarvedSnapshot) -> bool {
        let mut chosen = None;
        for node in self.tasks.iter_live() {
            let Some(state) = node.task_state() else { continue };
            if state.claim_state() != ClaimState::Queued {
                continue;
            }
            let Some(meta) = node.params() else { continue };
            let preferred = self
                .expander
                .first_resource(meta)
                .is_some_and(|k| hint.contains(k));
            let first = chosen.is_none();
            if first || preferred {
                chosen = Some(node.clone());
            }
            if preferred {
                break;
            }
        }
        let Some(node) = chosen else { return false };
        let (Some(meta), Some(state)) = (node.params().copied(), node.task_state().cloned()) else {
            return false;
        };
        let Some(payload) = self.tasks.claim(&node) else {
            return true;
        };
        telemetry::task_span(&meta.id.to_string()).in_scope(|| {
            let ctx = TaskCtx::new(meta.id, 0, 0);
            match self.expander.expand(&meta, payload, &ctx) {
                Ok(Expansion { ops, collect }) => {
                    let run = TaskRun::new(ctx, state, collect);
                    if ops.is_empty() {
                        run.finish();
                    } else if let Err(err) = self.enqueue(&run, ops) {
                        run.fail(err.into());
                    }
                }
                Err(err) => {
                    tracing::debug!(error = %err, "expansion failed");
                    state.complete(Err(TaskError::Engine(Arc::new(err))));
                }
            }
        });
        true
    }
}
