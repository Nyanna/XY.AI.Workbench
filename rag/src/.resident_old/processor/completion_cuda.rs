use std::ffi::c_void;
use std::sync::Arc;

use cudarc::driver::{result, sys, CudaEvent, CudaStream};
use parking_lot::Mutex;

use super::completion::{Completion, LaneBoard, LaneId, LaneOutcome, PollState};

/// Detects completion by querying a timing-free event recorded behind the lane's work.
pub struct PolledCompletion {
    stream: Arc<CudaStream>,
    events: Box<[Mutex<Option<CudaEvent>>]>,
}

impl PolledCompletion {
    pub fn new(stream: Arc<CudaStream>, lanes: usize) -> Self {
        Self {
            stream,
            events: (0..lanes).map(|_| Mutex::new(None)).collect(),
        }
    }
}

impl Completion for PolledCompletion {
    fn arm(&self, lane: LaneId) {
        let mut slot = self.events[lane.0 as usize].lock();
        *slot = None;
        match self.stream.record_event(Some(sys::CUevent_flags::CU_EVENT_DISABLE_TIMING)) {
            Ok(event) => *slot = Some(event),
            Err(error) => tracing::error!(lane = lane.0, %error, "could not record completion event"),
        }
    }

    fn poll(&self, lane: LaneId) -> PollState {
        let slot = self.events[lane.0 as usize].lock();
        let Some(event) = slot.as_ref() else {
            return PollState::Ready(LaneOutcome::Failed);
        };
        // `CudaEvent::is_complete` folds errors into `false`, so query the driver directly.
        match unsafe { result::event::query(event.cu_event()) } {
            Ok(()) => PollState::Ready(LaneOutcome::Success),
            Err(e) if e.0 == sys::CUresult::CUDA_ERROR_NOT_READY => PollState::Pending,
            Err(error) => {
                tracing::error!(lane = lane.0, %error, "lane failed on device");
                PollState::Ready(LaneOutcome::Failed)
            }
        }
    }
}

struct Callback {
    board: Arc<LaneBoard>,
    lane: LaneId,
}

unsafe extern "C" fn trampoline(arg: *mut c_void) {
    // SAFETY: `arg` is the `Box<Callback>` leaked by `HostFnCompletion::arm`, consumed once here.
    let cb = unsafe { Box::from_raw(arg.cast::<Callback>()) };
    cb.board.signal(cb.lane);
}

/// Push path: a host function at the end of the stream signals the lane board. The embedded
/// event poll covers callbacks that never fire, which happens when the stream errors.
pub struct HostFnCompletion {
    polled: PolledCompletion,
    board: Arc<LaneBoard>,
}

impl HostFnCompletion {
    pub fn new(stream: Arc<CudaStream>, board: Arc<LaneBoard>) -> Self {
        let polled = PolledCompletion::new(stream, board.lanes());
        Self { polled, board }
    }
}

impl Completion for HostFnCompletion {
    fn arm(&self, lane: LaneId) {
        self.polled.arm(lane);
        let arg = Box::into_raw(Box::new(Callback {
            board: self.board.clone(),
            lane,
        }));
        // SAFETY: the stream outlives the call and the trampoline only touches `arg`.
        let launched = unsafe {
            result::stream::launch_host_function(
                self.polled.stream.cu_stream(),
                trampoline,
                arg.cast::<c_void>(),
            )
        };
        if let Err(error) = launched {
            tracing::error!(lane = lane.0, %error, "could not enqueue completion callback");
            // SAFETY: the driver rejected the call, so ownership never left this function.
            drop(unsafe { Box::from_raw(arg) });
        }
    }

    fn poll(&self, lane: LaneId) -> PollState {
        self.polled.poll(lane)
    }
}

#[cfg(test)]
#[path = "test/completion_cuda.rs"]
mod tests;
