//! Orchestration: task queue + op queues.
mod completion;
#[cfg(feature = "cuda")]
mod completion_cuda;
mod config;
mod error;
mod hub;
mod ops;
mod resource;
mod starved;
mod worker;
#[cfg(test)]
mod tests;
pub use completion::{
    Completion, LaneBoard, LaneId, LaneOutcome, ManualCompletion, PollState,
};
#[cfg(feature = "cuda")]
pub use completion_cuda::{HostFnCompletion, PolledCompletion};
pub use config::WorkerConfig;
pub use error::OrchestrationError;
pub use hub::{Engine, Expander, Expansion, Hub, RegionCheck, Step, Wakeups};
pub use ops::{Collect, OpJob, OpQueue, OpQueues, Side, TaskRun};
pub use resource::{ResourceSlot, ResourceState};
pub use starved::{StarvedSet, StarvedSnapshot};
pub use worker::Processor;
