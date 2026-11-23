//! Synchronous scheduling core of a worker: results, GPU feed, CPU feed, expansion.

use std::sync::Arc;
use std::time::Instant;

use crate::resident::l130::l120::l110::a::gpu_completion::{FinishedLane, GpuCompletion};
use crate::resident::l130::l120::l110::l100::l090::l080::subengine_loop::{
    IterationOutcome, ResourceExec, SubengineLoop,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::resource_requirements::HasResourceRequirements;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::queue::Claim;

/// One feed pass of a sub-engine. Non-blocking.
pub trait FeedStep: Send {
    fn feed(&mut self, now: Instant) -> IterationOutcome;
}

impl<M, P, H> FeedStep for Arc<SubengineLoop<M, P, H>>
where
    M: HasResourceRequirements + Send + Sync + 'static,
    P: Send + 'static,
    H: ResourceExec<Entry = Claim<M, P, (), ()>>,
{
    fn feed(&mut self, now: Instant) -> IterationOutcome {
        self.run_iteration(now)
    }
}

/// Expansion of at most one new task per call. Implemented by the engine facade.
pub trait ExpandStep: Send {
    /// `true`: one task was taken and handled, whether its expansion succeeded or failed.
    /// `false`: no task available. May block only inside the planner (sub-engine activation).
    fn expand_next(&mut self) -> bool;
}

/// Handler for a finished GPU lane. Called on the worker thread, in poll order.
pub type FinishedHandler = Box<dyn FnMut(FinishedLane) + Send>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    GpuResults,
    GpuFeed,
    CpuFeed,
    Expand,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WorkerReport {
    /// Number of operations executed (each one stage run with progress).
    pub operations: usize,
    /// Finished lanes handled.
    pub gpu_results: usize,
    /// Sum of `IterationOutcome::claimed` over all GPU and CPU feed calls.
    pub claimed: usize,
    /// Tasks taken by `expand_next`.
    pub expanded: usize,
}

impl WorkerReport {
    pub fn progress(&self) -> bool {
        self.operations > 0
    }

    fn absorb(&mut self, other: &WorkerReport) {
        self.operations += other.operations;
        self.gpu_results += other.gpu_results;
        self.claimed += other.claimed;
        self.expanded += other.expanded;
    }
}

pub struct WorkerLoop {
    completion: Arc<GpuCompletion>,
    on_finished: FinishedHandler,
    gpu_feeds: Vec<Box<dyn FeedStep>>,
    cpu_feeds: Vec<Box<dyn FeedStep>>,
    expand: Box<dyn ExpandStep>,
}

impl WorkerLoop {
    pub fn new(
        completion: Arc<GpuCompletion>,
        on_finished: FinishedHandler,
        gpu_feeds: Vec<Box<dyn FeedStep>>,
        cpu_feeds: Vec<Box<dyn FeedStep>>,
        expand: Box<dyn ExpandStep>,
    ) -> Self {
        Self {
            completion,
            on_finished,
            gpu_feeds,
            cpu_feeds,
            expand,
        }
    }

    /// Executes exactly one operation: the first stage in priority order that makes progress.
    /// `None`: no stage made progress.
    pub fn run_once(&mut self, now: Instant) -> Option<Stage> {
        self.step(now).map(|(stage, _)| stage)
    }

    /// Repeats `run_once` until it returns `None`.
    pub fn run_until_idle(&mut self) -> WorkerReport {
        let mut total = WorkerReport::default();
        while let Some((_, delta)) = self.step(Instant::now()) {
            total.absorb(&delta);
        }
        total
    }

    fn step(&mut self, now: Instant) -> Option<(Stage, WorkerReport)> {
        let handled = self.process_results();
        if handled > 0 {
            log::trace!("worker loop: gpu results, {handled} lane(s)");
            return Some((
                Stage::GpuResults,
                WorkerReport {
                    operations: 1,
                    gpu_results: handled,
                    ..WorkerReport::default()
                },
            ));
        }

        for (stage, feeds) in [
            (Stage::GpuFeed, &mut self.gpu_feeds),
            (Stage::CpuFeed, &mut self.cpu_feeds),
        ] {
            let (claimed, progress) = feed_until_progress(feeds, now);
            if progress {
                log::trace!("worker loop: {stage:?}, claimed {claimed}");
                return Some((
                    stage,
                    WorkerReport {
                        operations: 1,
                        claimed,
                        ..WorkerReport::default()
                    },
                ));
            }
        }

        if self.expand.expand_next() {
            log::trace!("worker loop: expanded one task");
            return Some((
                Stage::Expand,
                WorkerReport {
                    operations: 1,
                    expanded: 1,
                    ..WorkerReport::default()
                },
            ));
        }

        log::trace!("worker loop: idle");
        None
    }

    fn process_results(&mut self) -> usize {
        let finished = self.completion.poll();
        let count = finished.len();
        for lane in finished {
            (self.on_finished)(lane);
        }
        count
    }
}

/// Calls the feeds in order and stops after the first one that made progress.
/// Returns the claimed total over the calls made and whether progress was made.
fn feed_until_progress(feeds: &mut [Box<dyn FeedStep>], now: Instant) -> (usize, bool) {
    let mut claimed = 0;
    for feed in feeds.iter_mut() {
        let outcome = feed.feed(now);
        claimed += outcome.claimed;
        if outcome.progress() {
            return (claimed, true);
        }
    }
    (claimed, false)
}

#[cfg(test)]
#[path = "test/worker_loop.rs"]
mod tests;
