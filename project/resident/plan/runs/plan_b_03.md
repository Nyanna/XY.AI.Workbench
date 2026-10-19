SystemInit:  id=eb74b449-aa62-41c4-8a82-0783f7fcf37d, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 50
Control Request:
```yaml
id: 78c8-1
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag
    pattern: \.(rs|toml)$
  reason: Inspect existing Rust workspace to place B3 code (B1/B2 dependencies).
```
/answer 78c8-1 allow
Control Request:
```yaml
id: 78c8-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag
      entries:
      - ".: (1 file)"
      - Cargo.toml
      - "./.bin/debug/build/libsqlite3-sys-5732091bfd9cbcf1/out: (1 file)"
      - bindgen.rs
      - "./.bin/debug/build/serde-045d6d86c89f5c13/out: (1 file)"
      - private.rs
      - "./.bin/debug/build/serde-882976cf9f9bdd13/out: (1 file)"
      - private.rs
      - "./.bin/debug/build/serde_core-4954989e5d8a5802/out: (1 file)"
      - private.rs
      - "./.bin/debug/build/serde_core-7bded54902a606b3/out: (1 file)"
      - private.rs
      - "./.bin/release/build/libsqlite3-sys-da0791f2ea192201/out: (1 file)"
      - bindgen.rs
      - "./.bin/release/build/serde-206768b153b78fd3/out: (1 file)"
      - private.rs
      - "./.bin/release/build/serde-423d3e60c1cd2ea8/out: (1 file)"
      - private.rs
      - "./.bin/release/build/serde_core-81444f961b76707e/out: (1 file)"
      - private.rs
      - "./.bin/release/build/serde_core-c1bd3ded7cc1e06e/out: (1 file)"
      - private.rs
      - "./.cargo: (1 file)"
      - config.toml
      - "./.xyrag/layers/trigram/mirror: (1 file)"
      - Cargo.toml
      - "./.xyrag/layers/trigram/mirror/benches: (1 file)"
      - trie_bench.rs
      - "./.xyrag/layers/trigram/mirror/src: (1 file)"
      - lib.rs
      - "./.xyrag/layers/trigram/mirror/src/bin: (1 file)"
      - xyrag.rs
      - "./.xyrag/layers/trigram/mirror/src/core: (9 files)"
      - engine.rs
      - "./.xyrag/layers/trigram/mirror/src/layers: (5 files)"
      - dir_cache.rs
      - "./.xyrag/layers/trigram/mirror/src/layers/trigram: (7 files)"
      - index.rs
      - "./.xyrag/layers/trigram/mirror/tests: (1 file)"
      - trigram_e2e.rs
      - "./benches: (1 file)"
      - trie_bench.rs
      - "./src: (1 file)"
      - lib.rs
      - "./src/bin: (1 file)"
      - xyrag.rs
      - "./src/core: (9 files)"
      - engine.rs
      - "./src/layers: (5 files)"
      - dir_cache.rs
      - "./src/layers/trigram: (7 files)"
      - index.rs
      - "./src/resident: (5 files)"
      - backend.rs
      - "./src/resident/facade: (1 file)"
      - mod.rs
      - "./src/resident/index: (1 file)"
      - mod.rs
      - "./src/resident/kernel: (2 files)"
      - cuda.rs
      - "./src/resident/manager: (1 file)"
      - mod.rs
      - "./src/resident/memory: (6 files)"
      - dtype.rs
      - "./src/resident/op: (1 file)"
      - mod.rs
      - "./src/resident/planner: (1 file)"
      - mod.rs
      - "./src/resident/precision: (1 file)"
      - mod.rs
      - "./src/resident/processor: (1 file)"
      - mod.rs
      - "./src/resident/queue: (5 files)"
      - mod.rs
      - "./src/resident/subengine: (2 files)"
      - device.rs
      - "./tests: (1 file)"
      - trigram_e2e.rs
      warning: Result reduced because the total number of entries (90) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 78c8-2 allow
Control Request:
```yaml
id: 78c8-3
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  reason: List resident module files fully.
```
/answer 78c8-3 allow
Control Request:
```yaml
id: 78c8-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".:"
      - backend.rs
      - error.rs
      - mock.rs
      - mod.rs
      - telemetry.rs
      - "./facade:"
      - mod.rs
      - "./index:"
      - mod.rs
      - "./kernel:"
      - cuda.rs
      - mod.rs
      - "./manager:"
      - mod.rs
      - "./memory:"
      - dtype.rs
      - freelist.rs
      - layout.rs
      - manager.rs
      - mod.rs
      - region.rs
      - "./op:"
      - mod.rs
      - "./planner:"
      - mod.rs
      - "./precision:"
      - mod.rs
      - "./processor:"
      - mod.rs
      - "./queue:"
      - mod.rs
      - node.rs
      - queue.rs
      - state.rs
      - task.rs
      - "./subengine:"
      - device.rs
      - mod.rs
```
/answer 78c8-4 allow
Control Request:
```yaml
id: 78c8-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
    ids: []
  reason: Read op stub
```
/answer 78c8-5 allow
Control Request:
```yaml
id: 78c8-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Operation, requires(), Value::Host|Device."
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 78c8-6 allow
Control Request:
```yaml
id: 78c8-7
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/node.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/state.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  reason: Understand B1/B2 types to integrate with.
```
/answer 78c8-7 allow
Control Request:
```yaml
id: 78c8-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Resident engine: embedding/similarity substrate, kept warm."
      - id: backend
        signature: pub mod backend;
      - id: error
        signature: pub mod error;
      - id: telemetry
        signature: pub mod telemetry;
      - id: mock
        signature: "#[cfg(any(test, feature = \"mock\"))]"
      - id: queue
        signature: pub mod queue;
      - id: memory
        signature: pub mod memory;
      - id: op
        signature: pub mod op;
      - id: index
        signature: pub mod index;
      - id: kernel
        signature: pub mod kernel;
      - id: precision
        signature: pub mod precision;
      - id: facade
        signature: pub mod facade;
      - id: processor
        signature: pub mod processor;
      - id: planner
        signature: pub mod planner;
      - id: subengine
        signature: pub mod subengine;
      - id: manager
        signature: pub mod manager;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: ModuleDoc
        signature: //! Error taxonomy.
      - id: MS3n2e|NqxAEG
        signature: use thiserror::Error;
      - id: IsRetryable
        signature: "/// Classifies an error as retryable (transient) vs. fatal, for backoff decisio…"
        docstring: "Classifies an error as retryable (transient) vs. fatal, for backoff decisions."
        children:
        - id: IsRetryable.is_retryable
          signature: fn is_retryable(&self) -> bool;
      - id: IndexError
        signature: "/// Index/scope layer errors. #[derive(Error, Debug)] #[non_exhaustive] pub enu…"
        docstring: Index/scope layer errors.
      - id: impl_IsRetryable_for_IndexError
        signature: "impl IsRetryable for IndexError {"
        children:
        - id: impl_IsRetryable_for_IndexError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: ManagerError
        signature: /// Internal index-manager errors; mapped to a public variant or retried at the…
        docstring: Internal index-manager errors; mapped to a public variant or retried at the API…
      - id: impl_IsRetryable_for_ManagerError
        signature: "impl IsRetryable for ManagerError {"
        children:
        - id: impl_IsRetryable_for_ManagerError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: PrecisionError
        signature: "/// Precision/calibration layer errors. #[derive(Error, Debug)] #[non_exhaustiv…"
        docstring: Precision/calibration layer errors.
      - id: impl_IsRetryable_for_PrecisionError
        signature: "impl IsRetryable for PrecisionError {"
        children:
        - id: impl_IsRetryable_for_PrecisionError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: PlannerError
        signature: "/// Planner/task-expansion layer errors. #[derive(Error, Debug)] #[non_exhausti…"
        docstring: Planner/task-expansion layer errors.
      - id: impl_IsRetryable_for_PlannerError
        signature: "impl IsRetryable for PlannerError {"
        children:
        - id: impl_IsRetryable_for_PlannerError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: CapabilityError
        signature: "/// Capability/profile-matching layer errors. #[derive(Error, Debug)] #[non_exh…"
        docstring: Capability/profile-matching layer errors.
      - id: ConfigError
        signature: "/// Facade/configuration errors raised by the builder and `init()`. #[derive(Er…"
        docstring: Facade/configuration errors raised by the builder and `init()`.
      - id: impl_IsRetryable_for_CapabilityError
        signature: "impl IsRetryable for CapabilityError {"
        children:
        - id: impl_IsRetryable_for_CapabilityError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: impl_IsRetryable_for_ConfigError
        signature: "impl IsRetryable for ConfigError {"
        children:
        - id: impl_IsRetryable_for_ConfigError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: ResidentError
        signature: "/// Top-level resident engine error, wrapping every layer error via `#[from]`. …"
        docstring: "Top-level resident engine error, wrapping every layer error via `#[from]`."
      - id: impl_IsRetryable_for_ResidentError
        signature: "impl IsRetryable for ResidentError {"
        children:
        - id: impl_IsRetryable_for_ResidentError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Region/Layout/DType and MemoryManager trait.
      - id: dtype
        signature: pub mod dtype;
      - id: freelist
        signature: pub mod freelist;
      - id: layout
        signature: pub mod layout;
      - id: manager
        signature: pub mod manager;
      - id: region
        signature: pub mod region;
      - id: hGYNaC|1FHY01
        signature: "pub use dtype::{DType, UnsupportedDType};"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
      nodes:
      - id: ModuleDoc
        signature: //! Pointer-free memory handle; resolution to a device address is backend-inter…
      - id: ArenaId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      - id: Region
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub struct Region {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Queue & task primitives.
      - id: node
        signature: mod node;
      - id: queue
        signature: mod queue;
      - id: state
        signature: mod state;
      - id: task
        signature: pub mod task;
      - id: wmzZM3|rZ2gWB
        signature: "pub use node::{Iter, Node};"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/node.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Append-only chain node. Public API hides the sentinel used by [`super::Queu…"
      - id: 0rN2mY|ncgPy1
        signature: use std::sync::Arc;
      - id: Content
        signature: "enum Content<P, D, O, E> {"
      - id: Node
        signature: "pub struct Node<P, D, O, E> {"
      - id: impl_Node_P_D_O_E_
        signature: "impl<P, D, O, E> Node<P, D, O, E> {"
        children:
        - id: impl_Node_P_D_O_E_.sentinel
          signature: "pub(crate) fn sentinel() -> Arc<Self> {"
        - id: impl_Node_P_D_O_E_.new
          signature: "pub fn new(params: P, state: Arc<TaskState<O, E>>, data: D) -> Arc<Self> {"
        - id: impl_Node_P_D_O_E_.params
          signature: /// `None` for the sentinel; otherwise the immutable filter metadata. pub fn pa…
          docstring: '`None` for the sentinel; otherwise the immutable filter metadata.'
        - id: impl_Node_P_D_O_E_.task_state
          signature: "pub fn task_state(&self) -> Option<&Arc<TaskState<O, E>>> {"
        - id: impl_Node_P_D_O_E_.claim
          signature: "/// Attempts `QUEUED -> TAKEN`; on success takes the payload. Never blocks, ///…"
          docstring: "Attempts `QUEUED -> TAKEN`; on success takes the payload. Never blocks,"
        - id: impl_Node_P_D_O_E_.peek_next
          signature: "pub(crate) fn peek_next(&self) -> Option<Arc<Node<P, D, O, E>>> {"
        - id: impl_Node_P_D_O_E_.try_link_next
          signature: /// CAS-appends `new` as this node's successor. `Ok` on success; `Err` /// retu…
          docstring: CAS-appends `new` as this node's successor. `Ok` on success; `Err`
      - id: impl_Drop_for_Node_P_D_O_E_
        signature: "impl<P, D, O, E> Drop for Node<P, D, O, E> {"
        children:
        - id: impl_Drop_for_Node_P_D_O_E_.drop
          signature: "fn drop(&mut self) {"
      - id: Iter
        signature: /// Traverses live nodes (sentinel excluded) in age order. Holds one `Arc<Node>…
        docstring: Traverses live nodes (sentinel excluded) in age order. Holds one `Arc<Node>`
      - id: impl_Iter_P_D_O_E_
        signature: "impl<P, D, O, E> Iter<P, D, O, E> {"
        children:
        - id: impl_Iter_P_D_O_E_.from_sentinel
          signature: "pub(crate) fn from_sentinel(sentinel: Arc<Node<P, D, O, E>>) -> Self {"
      - id: impl_Iterator_for_Iter_P_D_O_E_
        signature: "impl<P, D, O, E> Iterator for Iter<P, D, O, E> {"
        children:
        - id: impl_Iterator_for_Iter_P_D_O_E_.jAhS8I|ey0usa
          signature: "type Item = Arc<Node<P, D, O, E>>;"
        - id: impl_Iterator_for_Iter_P_D_O_E_.next
          signature: "fn next(&mut self) -> Option<Self::Item> {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
      nodes:
      - id: ModuleDoc
        signature: //! Task-queue instantiation notes for C2/B3 (package plan B1 §5).
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/state.rs
      nodes:
      - id: ModuleDoc
        signature: //! Claim state machine and result cell shared by every queue node.
      - id: JhEESM|gG3YBi
        signature: use std::sync::atomic::Ordering;
      - id: DjgTyS|P8ZoIE
        signature: "const QUEUED: u8 = 0;"
      - id: ClaimState
        signature: "/// Snapshot of a [`TaskState`]'s atomic state. #[derive(Debug, Clone, Copy, Pa…"
        docstring: "Snapshot of a [`TaskState`]'s atomic state."
      - id: impl_From_u8__for_ClaimState
        signature: "impl From<u8> for ClaimState {"
        children:
        - id: impl_From_u8__for_ClaimState.from
          signature: "fn from(value: u8) -> Self {"
      - id: TaskState
        signature: /// Per-task claim/result/wakeup cell. Every transition is a single /// `compar…
        docstring: Per-task claim/result/wakeup cell. Every transition is a single
      - id: impl_Default_for_TaskState_O_E_
        signature: "impl<O, E> Default for TaskState<O, E> {"
        children:
        - id: impl_Default_for_TaskState_O_E_.default
          signature: "fn default() -> Self {"
      - id: impl_TaskState_O_E_
        signature: "impl<O, E> TaskState<O, E> {"
        children:
        - id: impl_TaskState_O_E_.new
          signature: "pub fn new() -> Self {"
        - id: impl_TaskState_O_E_.claim_state
          signature: "pub fn claim_state(&self) -> ClaimState {"
        - id: impl_TaskState_O_E_.claim
          signature: /// `QUEUED -> TAKEN`. The only transition where cancellation matters. pub(crat…
          docstring: '`QUEUED -> TAKEN`. The only transition where cancellation matters.'
        - id: impl_TaskState_O_E_.complete
          signature: /// Sets the result and transitions `TAKEN -> DONE`. Must be called at /// most…
          docstring: Sets the result and transitions `TAKEN -> DONE`. Must be called at
        - id: impl_TaskState_O_E_.cancel
          signature: /// `QUEUED -> CANCELLED`. No-op if already `TAKEN`/`DONE`/`CANCELLED`. pub fn …
          docstring: '`QUEUED -> CANCELLED`. No-op if already `TAKEN`/`DONE`/`CANCELLED`.'
        - id: impl_TaskState_O_E_.result
          signature: "pub fn result(&self) -> Option<&Result<O, E>> {"
        - id: impl_TaskState_O_E_.wait
          signature: /// Waits for `Done`/`Cancelled`. Registers interest before checking state /// …
          docstring: Waits for `Done`/`Cancelled`. Registers interest before checking state
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.claim_once_wins
          signature: "#[test] fn claim_once_wins() {"
        - id: tests.cancel_before_claim_blocks_claim
          signature: "#[test] fn cancel_before_claim_blocks_claim() {"
        - id: tests.cancel_after_claim_is_noop
          signature: "#[test] fn cancel_after_claim_is_noop() {"
        - id: tests.complete_sets_result_before_done_is_visible
          signature: "#[test] fn complete_sets_result_before_done_is_visible() {"
        - id: tests.wait_sees_completion_set_before_await
          signature: "#[tokio::test] async fn wait_sees_completion_set_before_await() {"
        - id: tests.wait_wakes_on_later_completion
          signature: "#[tokio::test] async fn wait_wakes_on_later_completion() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Shared subengine lifecycle & pull model, tokenizer/normalizer (CPU)."
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: CpuBackend
        signature: "/// CPU backend; default without `cuda`, and the fallback target if CUDA init f…"
        docstring: "CPU backend; default without `cuda`, and the fallback target if CUDA init fails."
      - id: impl_ComputeBackend_for_CpuBackend
        signature: "impl ComputeBackend for CpuBackend {"
        children:
        - id: impl_ComputeBackend_for_CpuBackend.run_batch
          signature: "fn run_batch(&self, input: &[f32]) -> Vec<f32> {"
      - id: device
        signature: "#[cfg(feature = \"cuda\")]"
      - id: p03Xlc|0j1nS6
        signature: "#[cfg(feature = \"cuda\")]"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Device/Lane, execution context, loaded encoder, GPU memory arenas."
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: CudaBackend
        signature: /// Real CUDA backend; the single Candle/CUDA adapter point for the subengine (…
        docstring: Real CUDA backend; the single Candle/CUDA adapter point for the subengine (per
      - id: impl_CudaBackend
        signature: "impl CudaBackend {"
        children:
        - id: impl_CudaBackend.try_new
          signature: "pub fn try_new() -> Result<Self, crate::resident::error::IndexError> {"
      - id: impl_ComputeBackend_for_CudaBackend
        signature: "impl ComputeBackend for CudaBackend {"
        children:
        - id: impl_ComputeBackend_for_CudaBackend.run_batch
          signature: "fn run_batch(&self, _input: &[f32]) -> Vec<f32> {"
```
/answer 78c8-8 allow
Control Request:
```yaml
id: 78c8-9
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
    ids:
    - ModuleDoc
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
    ids:
    - ArenaId
    - Region
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - PlannerError
    - CapabilityError
    - ConfigError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    ids:
    - ModuleDoc
  reason: "Read Region, task notes, error variants, precision stub for integration."
```
/answer 78c8-9 allow
ReasoningToken: 61
Control Request:
```yaml
id: 78c8-a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/region.rs
      nodes:
      - id: ArenaId
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub struct ArenaId(pub u32);
      - id: Region
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub struct Region {
              pub arena: ArenaId,
              pub offset: u64,
              pub len: u64,
              pub gen: u32,
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: PlannerError
        type: ItemEnum
        code: |
          /// Planner/task-expansion layer errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum PlannerError {
              #[error("invalid planner config: {reason}")]
              InvalidConfig { reason: String },
              #[error("missing capability: {capability}")]
              MissingCapability { capability: String },
              #[error("unknown model: {model}")]
              UnknownModel { model: String },
          }
      - id: CapabilityError
        type: ItemEnum
        code: |
          /// Capability/profile-matching layer errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum CapabilityError {
              #[error("missing capability: {capability}")]
              Missing { capability: String },
              #[error("unknown capability: {capability}")]
              Unknown { capability: String },
          }
      - id: ConfigError
        type: ItemEnum
        code: |
          /// Facade/configuration errors raised by the builder and `init()`.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum ConfigError {
              #[error("missing required field: {field}")]
              MissingField { field: String },
              #[error("unknown model: {model}")]
              UnknownModel { model: String },
              #[error("invalid value for '{field}': {reason}")]
              InvalidValue { field: String, reason: String },
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! CapabilitySet/Profile.
```
/answer 78c8-a allow
Control Request:
```yaml
id: 78c8-b
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: CapabilitySet|DeviceKind|DeviceId|ModelId|IndexId|HostData|PinCount
    include:
    - '*.rs'
    limit: 20
  reason: Check whether supporting types already exist.
```
/answer 78c8-b allow
Control Request:
```yaml
id: 78c8-c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/precision/mod.rs
        matches:
        - lineno: 1
          match: //! CapabilitySet/Profile.
      - path: resident/memory/mod.rs
        matches:
        - lineno: 12
          match: "pub use manager::{compute_dtype, BookkeepingArena, DeviceKind, MemoryManager};"
      - path: resident/memory/manager.rs
        matches:
        - lineno: 12
          match: "pub enum DeviceKind {"
        - lineno: 17
          match: "pub fn compute_dtype(dev: DeviceKind) -> DType {"
        - lineno: 19
          match: "DeviceKind::Gpu => DType::F16,"
        - lineno: 20
          match: "DeviceKind::Cpu => DType::F32,"
        - lineno: 25
          match: "fn alloc(&self, dev: DeviceKind, bytes: u64, align: u64) -> Result<Region, IndexError>;"
        - lineno: 83
          match: "fn alloc(&self, _dev: DeviceKind, bytes: u64, align: u64) -> Result<Region, IndexError> {"
        - lineno: 118
          match: "assert_eq!(compute_dtype(DeviceKind::Gpu), DType::F16);"
        - lineno: 119
          match: "assert_eq!(compute_dtype(DeviceKind::Cpu), DType::F32);"
        - lineno: 125
          match: "let r1 = a.alloc(DeviceKind::Cpu, 16, 8).unwrap();"
        - lineno: 127
          match: "let r2 = a.alloc(DeviceKind::Cpu, 16, 8).unwrap();"
        - lineno: 138
          match: "let _r = a.alloc(DeviceKind::Gpu, 24, 1).unwrap();"
        - lineno: 140
          match: "a.alloc(DeviceKind::Gpu, 16, 1),"
```
/answer 78c8-c allow
ReasoningToken: 239
Control Request:
```yaml
id: 78c8-d
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^(tracing|thiserror|tokio|std)|^\\[|^mock|^cuda"
    include:
    - Cargo.toml
    exclude:
    - .xyrag/**
    limit: 40
  reason: Check available dependencies (tracing) and features.
```
/answer 78c8-d allow
Control Request:
```yaml
id: 78c8-e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: .xyrag/layers/trigram/mirror/Cargo.toml
        matches:
        - lineno: 16
          match: tokio version 1 features rt multi thread macros sync time
        - lineno: 17
          match: tokio util 0 7
      - path: Cargo.toml
        matches:
        - lineno: 1
          match: "[package]"
        - lineno: 7
          match: "[lib]"
        - lineno: 11
          match: "[[bin]]"
        - lineno: 15
          match: "[dependencies]"
        - lineno: 16
          match: "tokio = { version = \"1\", features = [\"rt-multi-thread\", \"macros\", \"sync\", \"time\"] }"
        - lineno: 17
          match: tokio-util = "0.7"
        - lineno: 32
          match: thiserror = "2.0.20"
        - lineno: 33
          match: tracing = "0.1.44"
        - lineno: 34
          match: "tracing-subscriber = { version = \"0.3.23\", features = [\"env-filter\"] }"
        - lineno: 35
          match: tracing-appender = "0.2.3"
        - lineno: 48
          match: "cudarc = { version = \"=0.19.9\", optional = true }"
        - lineno: 50
          match: "[features]"
        - lineno: 53
          match: "cuda = [\"candle\", \"dep:candle-transformers\", \"dep:cudarc\", \"candle-core/cuda\", \"candle-transformers/cuda\"]"
        - lineno: 54
          match: "mock = []"
        - lineno: 57
          match: "[dev-dependencies]"
        - lineno: 60
          match: mockall = "0.15.0"
        - lineno: 64
          match: "tokio = { version = \"1\", features = [\"test-util\"] }"
        - lineno: 66
          match: "[[bench]]"
```
/answer 78c8-e allow
ReasoningToken: 474
Control Request:
```yaml
id: 78c8-f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/manager.rs
  name: DeviceKind
  reason: Check DeviceKind derives.
```
/answer 78c8-f allow
Control Request:
```yaml
id: 78c8-10
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/manager.rs
      nodes:
      - id: DeviceKind
        code: |
          #[non_exhaustive]
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub enum DeviceKind {
              Cpu,
              Gpu,
          }
```
/answer 78c8-10 allow
Control Request:
```yaml
id: 78c8-11
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    id: ModuleDoc
    position: after
    source: |2

      use std::collections::BTreeSet;

      /// Named capabilities; an empty set is satisfied by any provider.
      #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
      pub struct CapabilitySet(BTreeSet<String>);

      impl CapabilitySet {
          pub fn new() -> Self {
              Self::default()
          }

          pub fn with(mut self, cap: impl Into<String>) -> Self {
              self.0.insert(cap.into());
              self
          }

          pub fn contains(&self, cap: &str) -> bool {
              self.0.contains(cap)
          }

          /// `self` (required) is a subset of `available`.
          pub fn satisfies(&self, available: &CapabilitySet) -> bool {
              self.0.is_subset(&available.0)
          }
      }
  reason: Minimal CapabilitySet with satisfies() needed by Requirements.
```
/answer 78c8-11 allow
ReasoningToken: 827
Control Request:
```yaml
id: 78c8-12
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
    source: |
      //! Resource requirements of an operation.

      use crate::resident::memory::{DeviceKind, Region};
      use crate::resident::precision::CapabilitySet;

      #[derive(Clone, Debug, PartialEq, Eq, Hash)]
      pub struct ModelId(pub String);

      #[derive(Clone, Debug, PartialEq, Eq, Hash)]
      pub struct IndexId(pub String);

      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub struct DeviceId(pub u32);

      #[non_exhaustive]
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub enum ResourceKind {
          Tokenizer,
          Normalizer,
          Encoder,
          Scorer,
          Index,
          CrossEncoder,
      }

      #[non_exhaustive]
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub enum EngineSelector {
          Cpu,
          Gpu,
          Either,
      }

      impl EngineSelector {
          pub fn accepts(self, dev: DeviceKind) -> bool {
              matches!(
                  (self, dev),
                  (EngineSelector::Either, _)
                      | (EngineSelector::Cpu, DeviceKind::Cpu)
                      | (EngineSelector::Gpu, DeviceKind::Gpu)
              )
          }
      }

      /// Placement hint; inactive in the scheduler for now.
      #[non_exhaustive]
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub enum Affinity {
          Handle(Region),
          Device(DeviceId),
      }

      #[non_exhaustive]
      #[derive(Clone, Debug, PartialEq, Eq, Hash)]
      pub struct Requirements {
          pub resource: ResourceKind,
          pub model: Option<ModelId>,
          pub index: Option<IndexId>,
          pub engine: EngineSelector,
          pub caps: CapabilitySet,
          pub affinity: Option<Affinity>,
      }

      impl Requirements {
          pub fn new(resource: ResourceKind, engine: EngineSelector) -> Self {
              Self {
                  resource,
                  model: None,
                  index: None,
                  engine,
                  caps: CapabilitySet::new(),
                  affinity: None,
              }
          }

          pub fn with_model(mut self, model: ModelId) -> Self {
              self.model = Some(model);
              self
          }

          pub fn with_index(mut self, index: IndexId) -> Self {
              self.index = Some(index);
              self
          }

          pub fn with_caps(mut self, caps: CapabilitySet) -> Self {
              self.caps = caps;
              self
          }

          pub fn with_affinity(mut self, affinity: Affinity) -> Self {
              self.affinity = Some(affinity);
              self
          }

          /// Affinity is deliberately ignored.
          pub fn satisfied_by(&self, dev: DeviceKind, available: &CapabilitySet) -> bool {
              self.engine.accepts(dev) && self.caps.satisfies(available)
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          #[test]
          fn engine_and_caps_matching() {
              let req = Requirements::new(ResourceKind::Encoder, EngineSelector::Gpu)
                  .with_caps(CapabilitySet::new().with("f16"));
              let have = CapabilitySet::new().with("f16").with("graph");
              assert!(req.satisfied_by(DeviceKind::Gpu, &have));
              assert!(!req.satisfied_by(DeviceKind::Cpu, &have));
              assert!(!req.satisfied_by(DeviceKind::Gpu, &CapabilitySet::new()));
          }

          #[test]
          fn either_accepts_both_and_empty_caps_pass() {
              let req = Requirements::new(ResourceKind::Tokenizer, EngineSelector::Either);
              let none = CapabilitySet::new();
              assert!(req.satisfied_by(DeviceKind::Cpu, &none));
              assert!(req.satisfied_by(DeviceKind::Gpu, &none));
          }

          #[test]
          fn affinity_does_not_affect_matching() {
              let req = Requirements::new(ResourceKind::Scorer, EngineSelector::Either)
                  .with_affinity(Affinity::Device(DeviceId(3)));
              assert!(req.satisfied_by(DeviceKind::Cpu, &CapabilitySet::new()));
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
    source: |
      //! RAII lease on a device region; pinned regions are not evicted.

      use std::sync::atomic::{AtomicUsize, Ordering};
      use std::sync::Arc;

      use tokio::sync::mpsc::UnboundedSender;

      use crate::resident::memory::Region;

      #[derive(Debug, Default)]
      pub struct PinCount(AtomicUsize);

      impl PinCount {
          pub fn new() -> Arc<Self> {
              Arc::new(Self::default())
          }

          pub fn get(&self) -> usize {
              self.0.load(Ordering::Acquire)
          }

          pub fn is_unpinned(&self) -> bool {
              self.get() == 0
          }
      }

      #[derive(Debug)]
      pub struct Lease {
          region: Region,
          pin: Arc<PinCount>,
          release: Option<UnboundedSender<Region>>,
      }

      pub type DeviceHandle = Lease;

      impl Lease {
          /// Increments the pin count; `release` receives the region on drop.
          pub fn new(region: Region, pin: Arc<PinCount>, release: Option<UnboundedSender<Region>>) -> Self {
              pin.0.fetch_add(1, Ordering::AcqRel);
              Self { region, pin, release }
          }

          pub fn region(&self) -> Region {
              self.region
          }
      }

      impl Drop for Lease {
          fn drop(&mut self) {
              self.pin.0.fetch_sub(1, Ordering::AcqRel);
              if let Some(tx) = &self.release {
                  let _ = tx.send(self.region);
              }
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use crate::resident::memory::region::ArenaId;

          fn region() -> Region {
              Region { arena: ArenaId(0), offset: 0, len: 16, gen: 1 }
          }

          #[test]
          fn drop_decrements_pin() {
              let pin = PinCount::new();
              let a = Lease::new(region(), pin.clone(), None);
              let b = Lease::new(region(), pin.clone(), None);
              assert_eq!(pin.get(), 2);
              drop(a);
              assert!(!pin.is_unpinned());
              drop(b);
              assert!(pin.is_unpinned());
          }

          #[test]
          fn panic_does_not_leak_pin() {
              let pin = PinCount::new();
              let p = pin.clone();
              let r = std::panic::catch_unwind(move || {
                  let _l = Lease::new(region(), p, None);
                  panic!("boom");
              });
              assert!(r.is_err());
              assert!(pin.is_unpinned());
          }

          #[test]
          fn drop_sends_region_without_blocking() {
              let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
              let lease = Lease::new(region(), PinCount::new(), Some(tx));
              drop(lease);
              assert_eq!(rx.try_recv().unwrap(), region());
          }
      }
  reason: Create requirements and lease modules for B3.
```
/answer 78c8-12 allow
Control Request:
```yaml
id: 78c8-13
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
    overwrite: true
    source: |
      //! Operation, requires(), Value::Host|Device.

      mod lease;
      mod requirements;

      use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
      use std::sync::{Arc, OnceLock};

      pub use lease::{DeviceHandle, Lease, PinCount};
      pub use requirements::{
          Affinity, DeviceId, EngineSelector, IndexId, ModelId, Requirements, ResourceKind,
      };

      use crate::resident::error::ResidentError;

      #[non_exhaustive]
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub enum OpKind {
          Tokenize,
          Embed,
          Pool,
          L2Norm,
          Similarity,
          TopK,
          WriteIndex,
          Infer,
          /// Explicit device-to-host transfer.
          IntoHost,
      }

      /// Immutable parameters; usable as cache/graph key.
      #[derive(Clone, Debug, PartialEq, Eq, Hash)]
      pub struct OpParams {
          pub requires: Requirements,
          pub top_k: Option<u32>,
          pub dim: Option<u32>,
      }

      impl OpParams {
          pub fn new(requires: Requirements) -> Self {
              Self { requires, top_k: None, dim: None }
          }
      }

      pub trait Op: Send + Sync {
          fn kind(&self) -> OpKind;
          fn params(&self) -> &OpParams;
          fn requires(&self) -> Requirements {
              self.params().requires.clone()
          }
      }

      /// Plain-data operation description.
      #[derive(Clone, Debug, PartialEq, Eq, Hash)]
      pub struct OpDesc {
          kind: OpKind,
          params: OpParams,
      }

      impl OpDesc {
          pub fn new(kind: OpKind, params: OpParams) -> Self {
              Self { kind, params }
          }
      }

      impl Op for OpDesc {
          fn kind(&self) -> OpKind {
              self.kind
          }

          fn params(&self) -> &OpParams {
              &self.params
          }
      }

      #[non_exhaustive]
      #[derive(Clone, Debug, PartialEq)]
      pub enum HostData {
          Text(Vec<String>),
          Tokens(Vec<Vec<u32>>),
          F32(Vec<f32>),
          Hits(Vec<(u64, f32)>),
      }

      /// Op input/output; never carries a tensor or raw pointer.
      #[derive(Debug)]
      pub enum Value {
          Host(HostData),
          Device(DeviceHandle),
      }

      /// Per-task shared state: open-op counter, first error, cancellation.
      #[derive(Debug)]
      pub struct TaskCtx {
          id: u64,
          priority: u8,
          open: AtomicUsize,
          cancelled: AtomicBool,
          error: OnceLock<ResidentError>,
      }

      impl TaskCtx {
          pub fn new(id: u64, priority: u8, ops: usize) -> Arc<Self> {
              Arc::new(Self {
                  id,
                  priority,
                  open: AtomicUsize::new(ops),
                  cancelled: AtomicBool::new(false),
                  error: OnceLock::new(),
              })
          }

          pub fn id(&self) -> u64 {
              self.id
          }

          pub fn priority(&self) -> u8 {
              self.priority
          }

          pub fn open_ops(&self) -> usize {
              self.open.load(Ordering::Acquire)
          }

          pub fn is_cancelled(&self) -> bool {
              self.cancelled.load(Ordering::Acquire)
          }

          pub fn error(&self) -> Option<&ResidentError> {
              self.error.get()
          }

          /// Returns `true` when this call closed the last open op of a non-failed task.
          pub fn finish_op(&self) -> bool {
              let prev = self.open.fetch_sub(1, Ordering::AcqRel);
              debug_assert!(prev > 0, "finish_op without open op");
              prev == 1 && !self.is_cancelled()
          }

          /// Cancels the task; only the first error is kept. Returns `true` if this call was first.
          pub fn fail(&self, err: ResidentError) -> bool {
              let first = self.error.set(err).is_ok();
              self.cancelled.store(true, Ordering::Release);
              if first {
                  tracing::debug!(task = self.id, "task cancelled by first error");
              }
              first
          }
      }

      pub struct OperationNode {
          pub op: Arc<dyn Op>,
          pub input: Value,
          pub output: OnceLock<Value>,
          /// `true`: result returns to host; `false`: device handle stays resident.
          pub materialize: bool,
          pub task: Arc<TaskCtx>,
      }

      impl OperationNode {
          pub fn new(op: Arc<dyn Op>, input: Value, materialize: bool, task: Arc<TaskCtx>) -> Self {
              tracing::debug!(
                  task = task.id(),
                  kind = ?op.kind(),
                  resource = ?op.params().requires.resource,
                  materialize,
                  "op created"
              );
              Self { op, input, output: OnceLock::new(), materialize, task }
          }

          /// Sets the output once; a second call returns the rejected value.
          pub fn set_output(&self, value: Value) -> Result<(), Value> {
              self.output.set(value)
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use crate::resident::error::PlannerError;
          use crate::resident::memory::region::ArenaId;
          use crate::resident::memory::Region;

          fn desc(kind: OpKind) -> Arc<dyn Op> {
              let req = Requirements::new(ResourceKind::Encoder, EngineSelector::Either);
              Arc::new(OpDesc::new(kind, OpParams::new(req)))
          }

          fn err() -> ResidentError {
              PlannerError::InvalidConfig { reason: "x".into() }.into()
          }

          #[test]
          fn params_clone_hash_eq() {
              use std::collections::HashSet;
              let p = OpParams::new(Requirements::new(ResourceKind::Scorer, EngineSelector::Gpu));
              let mut set = HashSet::new();
              set.insert(p.clone());
              assert!(set.contains(&p));
              let mut q = p.clone();
              q.top_k = Some(5);
              assert_ne!(p, q);
          }

          #[test]
          fn requires_comes_from_params() {
              let op = desc(OpKind::Embed);
              assert_eq!(op.requires(), op.params().requires);
          }

          #[test]
          fn task_done_only_at_zero() {
              let ctx = TaskCtx::new(1, 0, 3);
              assert!(!ctx.finish_op());
              assert!(!ctx.finish_op());
              assert!(ctx.finish_op());
              assert_eq!(ctx.open_ops(), 0);
          }

          #[test]
          fn first_error_cancels_and_is_kept() {
              let ctx = TaskCtx::new(1, 0, 2);
              assert!(ctx.fail(err()));
              assert!(!ctx.fail(err()));
              assert!(ctx.is_cancelled());
              assert!(ctx.error().is_some());
              ctx.finish_op();
              assert!(!ctx.finish_op());
          }

          #[test]
          fn output_set_once_and_device_value_holds_lease() {
              let ctx = TaskCtx::new(1, 0, 1);
              let node = OperationNode::new(desc(OpKind::Tokenize), Value::Host(HostData::Text(vec![])), true, ctx);
              assert!(node.set_output(Value::Host(HostData::F32(vec![1.0]))).is_ok());
              assert!(node.set_output(Value::Host(HostData::F32(vec![]))).is_err());

              let pin = PinCount::new();
              let region = Region { arena: ArenaId(0), offset: 0, len: 8, gen: 1 };
              let v = Value::Device(Lease::new(region, pin.clone(), None));
              assert_eq!(pin.get(), 1);
              drop(v);
              assert!(pin.is_unpinned());
          }

          #[test]
          fn affinity_is_non_exhaustive() {
              let a = Affinity::Device(DeviceId(0));
              #[allow(unreachable_patterns)]
              let _ = match a {
                  Affinity::Handle(_) | Affinity::Device(_) => (),
                  _ => (),
              };
          }
      }
  reason: "Implement Op, OpKind, OpParams, Value, TaskCtx, OperationNode."
```
/answer 78c8-13 allow
ReasoningToken: 110
Control Request:
```yaml
id: 78c8-14
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
  reason: Syntax check.
```
/answer 78c8-14 allow
Control Request:
```yaml
id: 78c8-15
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "from.*PlannerError|ResidentError \\{|#\\[from\\]"
    include:
    - error.rs
  reason: Verify PlannerError converts into ResidentError via From.
```
/answer 78c8-15 allow
Control Request:
```yaml
id: 78c8-16
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: error.rs
        matches:
        - lineno: 3
          match: "//! One enum per layer; upper layers wrap lower ones via `#[from]`/`#[source]`. All variants"
        - lineno: 112
          match: "/// Top-level resident engine error, wrapping every layer error via `#[from]`."
        - lineno: 115
          match: "pub enum ResidentError {"
        - lineno: 117
          match: "Index(#[from] IndexError),"
        - lineno: 119
          match: "Precision(#[from] PrecisionError),"
        - lineno: 121
          match: "Planner(#[from] PlannerError),"
        - lineno: 123
          match: "Config(#[from] ConfigError),"
        - lineno: 125
          match: "Capability(#[from] CapabilityError),"
        - lineno: 127
          match: "impl IsRetryable for ResidentError {"
```
/answer 78c8-16 allow
Result Stats: id=eb74b449-aa62-41c4-8a82-0783f7fcf37d, total: 47117, in: 18, out: 11643, reason: 0, read: 167286, write: 35456, start: 1792311791742, end: 1792311796481, time: 4682, total: 62261, io: 0, control: 57, sent: 0, recv: 123
B3 ist umgesetzt, aber ich habe es nicht kompiliert und die Tests nicht ausgeführt. Ich habe nur die Syntax der vier Dateien geprüft. `cargo build` (mit und ohne `cuda`) und `cargo test` stehen noch aus.

Neue Dateien in `rag/src/resident/op/`:
- **`mod.rs`:** `Op`-Trait, `OpKind`, `OpParams`, `OpDesc`, `HostData`, `Value`, `TaskCtx` und `OperationNode`. `TaskCtx` hält den Zähler offener Ops und merkt sich nur den ersten Fehler. Dieser Fehler cancelt den Task.
- **`requirements.rs`:** `Requirements`, `ResourceKind`, `EngineSelector`, `Affinity`, `ModelId`, `IndexId` und `DeviceId`. `Affinity` ist vorhanden, wird aber beim Matching ignoriert.
- **`lease.rs`:** `Lease` mit `PinCount`, `DeviceHandle` als Alias. `Drop` dekrementiert den Pin und sendet die `Region` über einen unbounded Channel, ohne zu blockieren.

In `precision/mod.rs` habe ich ein minimales `CapabilitySet` mit `satisfies` ergänzt. Es ist ein Platzhalter, bis B6 kommt.

Die Unit-Tests stehen jeweils im Modul. Sie decken Matching, `Hash`/`Eq` der Params, Lease-RAII (auch bei Panic), den Op-Zähler und den ersten Fehler ab.

Zwei Annahmen, die ich nicht geprüft habe: `ResidentError` ist `Send + Sync`, und `Region.gen` kompiliert in der verwendeten Edition (der bestehende Code nutzt es bereits).
