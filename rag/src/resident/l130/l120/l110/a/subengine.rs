//! Facade-side handle of one concrete sub-engine: planner contract, feed steps, device kind and
//! routing of finished GPU lanes. Owns no threads, memory or queues.
use std::sync::Arc;
use crate::resident::l130::l120::l110::l100::l090::a::gpu_completion::FinishedLane;
use crate::resident::l130::l120::l110::a::worker_loop::FeedStep;
use crate::resident::l130::l120::l110::l100::a::planner::Subengine;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::DeviceKind;
/// Builds a fresh set of feed steps for one worker.
pub type FeedFactory = Arc<dyn Fn() -> Vec<Box<dyn FeedStep>> + Send + Sync>;
/// `Err(lane)`: the tag does not belong to this sub-engine and the lane is returned unchanged.
pub type FinishedRouter = Arc<
    dyn Fn(FinishedLane) -> Result<(), FinishedLane> + Send + Sync,
>;
pub struct SubengineHandle {
    subengine: Arc<dyn Subengine>,
    device: DeviceKind,
    feeds: FeedFactory,
    on_finished: FinishedRouter,
}
impl SubengineHandle {
    pub fn new(
        subengine: Arc<dyn Subengine>,
        device: DeviceKind,
        feeds: FeedFactory,
        on_finished: FinishedRouter,
    ) -> Self {
        Self {
            subengine,
            device,
            feeds,
            on_finished,
        }
    }
    pub fn subengine(&self) -> &Arc<dyn Subengine> {
        &self.subengine
    }
    pub fn device(&self) -> DeviceKind {
        self.device
    }
    /// Called once per worker; the returned steps share the sub-engine's loops.
    pub fn feeds_for_worker(&self) -> Vec<Box<dyn FeedStep>> {
        (self.feeds)()
    }
    /// `Ok(())`: the lane was handled. `Err(lane)`: it belongs to another sub-engine.
    pub fn on_finished(&self, lane: FinishedLane) -> Result<(), FinishedLane> {
        (self.on_finished)(lane)
    }
}
#[cfg(test)]
#[path = "test/subengine.rs"]
mod tests;
