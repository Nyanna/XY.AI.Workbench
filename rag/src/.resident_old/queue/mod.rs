//! Queue & task primitives.
//!
//! Single singly-linked, lock-free, append-only list (CAS on `tail.next`).
//! Unlink is logical: finished nodes (`Done`/`Cancelled`) stay in the chain and are
//! skipped by `Queue::iter_live`; no mutex guards the chain, memory is reclaimed when the
//! queue drops. Order equals age. No version, no snapshot,
//! no lock on the chain itself. Priority/resource/model/mode are metadata in
//! `params`, opaque to the queue.
//!
//! Instantiated multiple times: one task queue, three op queues (CPU/GPU/
//! shared). This module provides the generic `Queue`/`Node` building block
//! plus the task-side `ClaimState`/`TaskState` state machine.

mod node;
mod queue;
mod state;
pub mod task;

pub use node::{Iter, Node};
pub use queue::Queue;
pub use state::{ClaimState, TaskState};
