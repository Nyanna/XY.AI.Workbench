use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LaneId(pub u32);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaneOutcome {
    Success,
    Failed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PollState {
    Pending,
    Ready(LaneOutcome),
}
/// Backend-specific completion detection. The push path (host callback) calls
/// [`LaneBoard::signal`]; `poll` is the fallback for callbacks that never fire.
pub trait Completion: Send + Sync {
    /// Called after the lane's work has been enqueued on its stream.
    fn arm(&self, lane: LaneId);
    fn poll(&self, lane: LaneId) -> PollState;
}
const IDLE: u8 = 0;
const RESERVED: u8 = 1;
const LAUNCHED: u8 = 2;
const COMPLETING: u8 = 3;
/// Lane lifecycle `Idle -> Reserved -> Launched -> Completing -> Idle`. Callback and
/// poller race on `Launched -> Completing`; only the winner processes the lane.
pub struct LaneBoard {
    states: Box<[AtomicU8]>,
    signalled: Box<[AtomicBool]>,
    wake: Arc<Notify>,
}
impl LaneBoard {
    pub fn new(lanes: usize, wake: Arc<Notify>) -> Self {
        Self {
            states: (0..lanes).map(|_| AtomicU8::new(IDLE)).collect(),
            signalled: (0..lanes).map(|_| AtomicBool::new(false)).collect(),
            wake,
        }
    }
    pub fn lanes(&self) -> usize {
        self.states.len()
    }
    pub fn reserve(&self) -> Option<LaneId> {
        self.states
            .iter()
            .enumerate()
            .find_map(|(i, s)| {
                s.compare_exchange(IDLE, RESERVED, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
                    .then_some(LaneId(i as u32))
            })
    }
    /// Returns a reserved lane that was never launched.
    pub fn abort(&self, lane: LaneId) {
        let _ = self
            .states[lane.0 as usize]
            .compare_exchange(RESERVED, IDLE, Ordering::AcqRel, Ordering::Relaxed);
    }
    pub fn launch(&self, lane: LaneId, completion: &dyn Completion) {
        let i = lane.0 as usize;
        debug_assert_eq!(self.states[i].load(Ordering::Acquire), RESERVED);
        self.signalled[i].store(false, Ordering::Release);
        completion.arm(lane);
        self.states[i].store(LAUNCHED, Ordering::Release);
        tracing::trace!(lane = lane.0, "lane launched");
    }
    /// Callback entry point: no CUDA calls, no blocking.
    pub fn signal(&self, lane: LaneId) {
        self.signalled[lane.0 as usize].store(true, Ordering::Release);
        self.wake.notify_one();
    }
    pub fn is_idle(&self, lane: LaneId) -> bool {
        self.states[lane.0 as usize].load(Ordering::Acquire) == IDLE
    }
    pub fn in_flight(&self) -> usize {
        self.states
            .iter()
            .filter(|s| matches!(s.load(Ordering::Acquire), LAUNCHED | COMPLETING))
            .count()
    }
    /// Takes over every finished lane exactly once and frees it after `on_done` returns.
    pub fn reap(
        &self,
        completion: &dyn Completion,
        mut on_done: impl FnMut(LaneId, LaneOutcome),
    ) -> usize {
        let mut reaped = 0;
        for (i, state) in self.states.iter().enumerate() {
            if state.load(Ordering::Acquire) != LAUNCHED {
                continue;
            }
            let lane = LaneId(i as u32);
            let outcome = if self.signalled[i].swap(false, Ordering::AcqRel) {
                LaneOutcome::Success
            } else {
                match completion.poll(lane) {
                    PollState::Pending => continue,
                    PollState::Ready(o) => o,
                }
            };
            if state
                .compare_exchange(
                    LAUNCHED,
                    COMPLETING,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .is_err()
            {
                continue;
            }
            on_done(lane, outcome);
            self.signalled[i].store(false, Ordering::Release);
            state.store(IDLE, Ordering::Release);
            reaped += 1;
        }
        reaped
    }
}
/// Test double whose lanes finish only when told to.
pub struct ManualCompletion {
    cells: Box<[AtomicU8]>,
}
impl ManualCompletion {
    pub fn new(lanes: usize) -> Self {
        Self {
            cells: (0..lanes).map(|_| AtomicU8::new(0)).collect(),
        }
    }
    pub fn finish(&self, lane: LaneId, outcome: LaneOutcome) {
        let v = match outcome {
            LaneOutcome::Success => 1,
            LaneOutcome::Failed => 2,
        };
        self.cells[lane.0 as usize].store(v, Ordering::Release);
    }
}
impl Completion for ManualCompletion {
    fn arm(&self, lane: LaneId) {
        self.cells[lane.0 as usize].store(0, Ordering::Release);
    }
    fn poll(&self, lane: LaneId) -> PollState {
        match self.cells[lane.0 as usize].load(Ordering::Acquire) {
            1 => PollState::Ready(LaneOutcome::Success),
            2 => PollState::Ready(LaneOutcome::Failed),
            _ => PollState::Pending,
        }
    }
}
#[cfg(test)]
#[path = "test/completion.rs"]
mod tests;
