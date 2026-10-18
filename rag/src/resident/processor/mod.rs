//! Orchestration: task queue + op queues.
mod completion;
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
pub use config::ProcessorConfig;
pub use error::OrchestrationError;
pub use hub::{Engine, Expander, Expansion, Hub, Step, Wakeups};
pub use ops::{Collect, OpJob, OpQueue, OpQueues, Side, TaskRun};
pub use resource::{ResourceSlot, ResourceState};
pub use starved::{StarvedSet, StarvedSnapshot};
pub use worker::Processor;
