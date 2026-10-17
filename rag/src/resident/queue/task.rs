//! Task-queue instantiation notes for C2/B3 (package plan B1 §5).
//!
//! The task queue is `Queue<P, D, O, E>` where `P`/`D` are the task's
//! params/payload types and `O`/`E` are its output/error types, all supplied
//! by the caller (concrete types land in later packages). `TaskState<O, E>`
//! carries exactly one `Result<O, E>`, set once by the claim winner.
//!
//! Op-queues (CPU/GPU/shared) reuse the same `Queue`/`Node` with their own
//! `OpParams`/value payload; completion is tracked via the task's op counter
//! (C2/B3), not through `TaskState::result`. The shared op-queue is created
//! empty and carries no scheduling logic yet; a subengine checks its own
//! queue before falling back to shared.
