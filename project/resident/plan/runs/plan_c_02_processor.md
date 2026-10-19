Setze C2 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben

SystemInit:  id=0c4e46b9-8322-4727-9459-cd1119c8b4d7, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst den vorhandenen Code im Modul `resident` an.
Control Request:
```yaml
id: 4eac-ab
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
  reason: "Bestehende Stubs von Processor, Queue, Planner und Lease prüfen, bevor C2 implementiert wird."
```
/answer 4eac-ab allow
ReasoningToken: 84
Control Request:
```yaml
id: 4eac-ac
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Orchestration: task queue + op queues."
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Stateless task expansion.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
      nodes:
      - id: ModuleDoc
        signature: //! RAII lease on a device region; pinned regions are not evicted.
      - id: 07O91Q|1abKmK
        signature: "use std::sync::atomic::{AtomicUsize, Ordering};"
      - id: PinCount
        signature: "#[derive(Debug, Default)]"
      - id: impl_PinCount
        signature: "impl PinCount {"
        children:
        - id: impl_PinCount.new
          signature: "pub fn new() -> Arc<Self> {"
        - id: impl_PinCount.get
          signature: "pub fn get(&self) -> usize {"
        - id: impl_PinCount.is_unpinned
          signature: "pub fn is_unpinned(&self) -> bool {"
      - id: Lease
        signature: "#[derive(Debug)] pub struct Lease {"
      - id: DeviceHandle
        signature: pub type DeviceHandle = Lease;
      - id: impl_Lease
        signature: "impl Lease {"
        children:
        - id: impl_Lease.new
          signature: /// Increments the pin count; `release` receives the region on drop. pub fn new…
          docstring: Increments the pin count; `release` receives the region on drop.
        - id: impl_Lease.region
          signature: "pub fn region(&self) -> Region {"
      - id: impl_Drop_for_Lease
        signature: "impl Drop for Lease {"
        children:
        - id: impl_Drop_for_Lease.drop
          signature: "fn drop(&mut self) {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.v1Pf6f|oszynZ
          signature: use super::*;
        - id: tests.region
          signature: "fn region() -> Region {"
        - id: tests.drop_decrements_pin
          signature: "#[test] fn drop_decrements_pin() {"
        - id: tests.panic_does_not_leak_pin
          signature: "#[test] fn panic_does_not_leak_pin() {"
        - id: tests.drop_sends_region_without_blocking
          signature: "#[test] fn drop_sends_region_without_blocking() {"
```
/answer 4eac-ac allow
Control Request:
```yaml
id: 4eac-ad
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^\\s*(pub )?(mod|use) |^\\[features\\]|^cuda|candle|cudarc|tokio|thiserror|tracing|libc"
    include:
    - Cargo.toml
    - mod.rs
    - lib.rs
    exclude:
    - .bin/**
    - target/**
    limit: 50
  reason: "Modulstruktur und Abhängigkeiten (Cargo.toml, mod-Deklarationen) ermitteln."
```
/answer 4eac-ad allow
Control Request:
```yaml
id: 4eac-ae
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-kernels/src/lib.rs
        matches:
        - lineno: 1
          match: "mod ptx {"
        - lineno: 82
          match: pub mod ffi;
      - path: third_party/candle-kernels/Cargo.toml
        matches:
        - lineno: 14
          match: name = "candle-kernels"
        - lineno: 31
          match: repository = "https://github.com/huggingface/candle"
        - lineno: 34
          match: name = "candle_kernels"
      - path: .xyrag/layers/trigram/mirror/src/lib.rs
        matches:
        - lineno: 6
          match: pub mod core
        - lineno: 7
          match: pub mod layers
      - path: .xyrag/layers/trigram/mirror/src/core/mod.rs
        matches:
        - lineno: 3
          match: pub mod engine
        - lineno: 4
          match: pub mod executor
        - lineno: 5
          match: pub mod layer
        - lineno: 6
          match: pub mod persistence
        - lineno: 7
          match: pub mod query
        - lineno: 8
          match: pub mod registry
        - lineno: 9
          match: pub mod result
        - lineno: 10
          match: pub mod topology
      - path: .xyrag/layers/trigram/mirror/src/layers/mod.rs
        matches:
        - lineno: 2
          match: pub mod dir cache
        - lineno: 3
          match: pub mod glob layer
        - lineno: 4
          match: pub mod grep layer
        - lineno: 5
          match: pub mod merge layer
        - lineno: 6
          match: pub mod trigram
      - path: .xyrag/layers/trigram/mirror/src/layers/trigram/mod.rs
        matches:
        - lineno: 21
          match: pub mod index
        - lineno: 22
          match: pub mod layer
        - lineno: 23
          match: pub mod normalize
        - lineno: 24
          match: pub mod signature
        - lineno: 25
          match: pub mod trie
        - lineno: 26
          match: pub mod vocab
        - lineno: 27
          match: pub use layer trigram layer
      - path: .xyrag/layers/trigram/mirror/Cargo.toml
        matches:
        - lineno: 16
          match: tokio version 1 features rt multi thread macros sync time
        - lineno: 17
          match: tokio util 0 7
      - path: src/lib.rs
        matches:
        - lineno: 5
          match: pub mod core;
        - lineno: 6
          match: pub mod layers;
        - lineno: 7
          match: pub mod resident;
      - path: src/resident/mod.rs
        matches:
        - lineno: 2
          match: pub mod backend;
        - lineno: 3
          match: pub mod error;
        - lineno: 4
          match: pub mod failpoint;
        - lineno: 5
          match: pub mod telemetry;
        - lineno: 7
          match: pub mod mock;
        - lineno: 8
          match: pub mod queue;
        - lineno: 9
          match: pub mod memory;
        - lineno: 10
          match: pub mod op;
        - lineno: 11
          match: pub mod index;
        - lineno: 12
          match: pub mod kernel;
        - lineno: 13
          match: pub mod precision;
        - lineno: 14
          match: pub mod facade;
        - lineno: 15
          match: pub mod processor;
        - lineno: 16
          match: pub mod planner;
        - lineno: 17
          match: pub mod subengine;
        - lineno: 18
          match: pub mod manager;
      - path: src/resident/precision/mod.rs
        matches:
        - lineno: 3
          match: mod axis;
        - lineno: 4
          match: mod capability;
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4eac-ae allow
Control Request:
```yaml
id: 4eac-af
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: .
    include:
    - mod.rs
    - error.rs
    - telemetry.rs
    - failpoint.rs
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^(\\[|[a-z_-]+ *=)"
    include:
    - Cargo.toml
    exclude:
    - third_party/**
    - .bin/**
    - .xyrag/**
    limit: 50
  reason: "Inhalt der Modul-Wurzel, Fehler-/Telemetrie-Module und Cargo.toml lesen."
```
/answer 4eac-af allow
ReasoningToken: 42
Control Request:
```yaml
id: 4eac-b0
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: mod.rs
        matches:
        - lineno: 1
          match: "//! Resident engine: embedding/similarity substrate, kept warm."
        - lineno: 2
          match: pub mod backend;
        - lineno: 3
          match: pub mod error;
        - lineno: 4
          match: pub mod failpoint;
        - lineno: 5
          match: pub mod telemetry;
        - lineno: 6
          match: "#[cfg(any(test, feature = \"mock\"))]"
        - lineno: 7
          match: pub mod mock;
        - lineno: 8
          match: pub mod queue;
        - lineno: 9
          match: pub mod memory;
        - lineno: 10
          match: pub mod op;
        - lineno: 11
          match: pub mod index;
        - lineno: 12
          match: pub mod kernel;
        - lineno: 13
          match: pub mod precision;
        - lineno: 14
          match: pub mod facade;
        - lineno: 15
          match: pub mod processor;
        - lineno: 16
          match: pub mod planner;
        - lineno: 17
          match: pub mod subengine;
        - lineno: 18
          match: pub mod manager;
      - path: processor/mod.rs
        matches:
        - lineno: 1
          match: "//! Orchestration: task queue + op queues."
      - path: precision/mod.rs
        matches:
        - lineno: 1
          match: //! CapabilitySet/Profile.
        - lineno: 3
          match: mod axis;
        - lineno: 4
          match: mod capability;
        - lineno: 5
          match: pub mod known;
        - lineno: 6
          match: mod pareto;
        - lineno: 7
          match: mod profile;
        - lineno: 9
          match: "pub use axis::{AxisId, ValueId};"
        - lineno: 10
          match: pub use capability::CapabilitySet;
        - lineno: 11
          match: "pub use pareto::{pareto, positions};"
        - lineno: 12
          match: "pub use profile::{Profile, ProfileConfig};"
      - path: facade/mod.rs
        matches:
        - lineno: 1
          match: //! Public resident facade API.
        - lineno: 3
          match: mod blocking;
        - lineno: 4
          match: mod error;
        - lineno: 5
          match: mod handle;
        - lineno: 6
          match: mod params;
        - lineno: 7
          match: mod resident;
        - lineno: 8
          match: mod task;
        - lineno: 10
          match: pub use blocking::ResidentBlocking;
        - lineno: 11
          match: pub use error::TaskError;
        - lineno: 12
          match: pub use handle::TaskHandle;
        - lineno: 13
          match: "pub use params::{"
        - lineno: 14
          match: "EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, TextRole,"
        - lineno: 15
          match: "VectorInput,"
        - lineno: 16
          match: "};"
        - lineno: 17
          match: "pub use resident::{"
        - lineno: 18
          match: "IndexInfo, Residency, Resident, ResidentBuilder, ResidentConfig, WorkerConfig,"
        - lineno: 19
          match: "};"
        - lineno: 20
          match: "pub use task::{Payload, TaskKind, TaskMeta, TaskQueue};"
      - path: facade/error.rs
        matches:
        - lineno: 1
          match: use std::sync::Arc;
        - lineno: 3
          match: use thiserror::Error;
        - lineno: 5
          match: use crate::resident::error::ResidentError;
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-kernels/Cargo.toml
        matches:
        - lineno: 12
          match: "[package]"
        - lineno: 13
          match: edition = "2021"
        - lineno: 14
          match: name = "candle-kernels"
        - lineno: 15
          match: version = "0.11.0"
        - lineno: 16
          match: build = "build.rs"
        - lineno: 17
          match: autolib = false
        - lineno: 18
          match: autobins = false
        - lineno: 19
          match: autoexamples = false
        - lineno: 20
          match: autotests = false
        - lineno: 21
          match: autobenches = false
        - lineno: 22
          match: description = "CUDA kernels for Candle"
        - lineno: 23
          match: readme = "README.md"
        - lineno: 24
          match: "keywords = ["
        - lineno: 29
          match: "categories = [\"science\"]"
        - lineno: 30
          match: license = "MIT OR Apache-2.0"
        - lineno: 31
          match: repository = "https://github.com/huggingface/candle"
        - lineno: 33
          match: "[lib]"
        - lineno: 34
          match: name = "candle_kernels"
        - lineno: 35
          match: path = "src/lib.rs"
        - lineno: 37
          match: "[dependencies]"
        - lineno: 39
          match: "[build-dependencies.cudaforge]"
        - lineno: 40
          match: version = "0.1.2"
      - path: Cargo.toml
        matches:
        - lineno: 1
          match: "[package]"
        - lineno: 2
          match: name = "xy-ai-rag"
        - lineno: 3
          match: version = "0.1.0"
        - lineno: 4
          match: edition = "2021"
        - lineno: 5
          match: description = "xy.ai.rag - Layered Anytime Retrieval Engine"
        - lineno: 7
          match: "[lib]"
        - lineno: 8
          match: name = "xy_ai_rag"
        - lineno: 9
          match: path = "src/lib.rs"
        - lineno: 11
          match: "[[bin]]"
        - lineno: 12
          match: name = "xyrag"
        - lineno: 13
          match: path = "src/bin/xyrag.rs"
        - lineno: 15
          match: "[dependencies]"
        - lineno: 16
          match: "tokio = { version = \"1\", features = [\"rt-multi-thread\", \"macros\", \"sync\", \"time\"] }"
        - lineno: 17
          match: tokio-util = "0.7"
        - lineno: 18
          match: async-trait = "0.1"
        - lineno: 19
          match: futures = "0.3"
        - lineno: 20
          match: "serde = { version = \"1\", features = [\"derive\"] }"
        - lineno: 21
          match: serde_json = "1"
        - lineno: 22
          match: serde_yaml = "0.9"
        - lineno: 23
          match: "rusqlite = { version = \"0.31\", features = [\"bundled\"] }"
        - lineno: 24
          match: "clap = { version = \"4\", features = [\"derive\"] }"
        - lineno: 25
          match: anyhow = "1"
        - lineno: 26
          match: rayon = "1"
        - lineno: 27
          match: "rkyv = { version = \"0.7\", features = [\"validation\"] }"
        - lineno: 28
          match: glob = "0.3"
        - lineno: 29
          match: lru = "0.12"
        - lineno: 30
          match: regex = "1"
        - lineno: 31
          match: unicode-normalization = "0.1"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4eac-b0 allow
Control Request:
```yaml
id: 4eac-b1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/queue.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/node.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/state.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
  reason: "Queue-, Node-, State-, Task- und Op-Verträge (B1/B3) für die Verdrahtung kennen."
```
/answer 4eac-b1 allow
Control Request:
```yaml
id: 4eac-b2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/queue.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Generic lock-free queue: sentinel head, CAS append, lazy logical unlink."
      - id: 0rN2mY|q6aI7k
        signature: use std::sync::Arc;
      - id: Queue
        signature: /// Generic over `P` (filter metadata) and `D` (payload); `O`/`E` are the /// r…
        docstring: Generic over `P` (filter metadata) and `D` (payload); `O`/`E` are the
      - id: impl_Queue_P_D_O_E_
        signature: "impl<P, D, O, E> Queue<P, D, O, E> {"
        children:
        - id: impl_Queue_P_D_O_E_.new
          signature: /// `relays` are notified (append-relay) on every push; one per subengine. pub …
          docstring: '`relays` are notified (append-relay) on every push; one per subengine.'
        - id: impl_Queue_P_D_O_E_.push
          signature: "pub fn push(     &self,     params: P,     state: Arc<TaskState<O, E>>,     dat…"
        - id: impl_Queue_P_D_O_E_.append
          signature: "fn append(&self, new: Arc<Node<P, D, O, E>>) {"
        - id: impl_Queue_P_D_O_E_.tail_hint
          signature: "fn tail_hint(&self) -> Arc<Node<P, D, O, E>> {"
        - id: impl_Queue_P_D_O_E_.iter
          signature: "/// Non-blocking traversal, age order, sentinel excluded. pub fn iter(&self) ->…"
          docstring: "Non-blocking traversal, age order, sentinel excluded."
        - id: impl_Queue_P_D_O_E_.iter_live
          signature: "/// Like [`Self::iter`], skipping logically unlinked nodes (`Done`/`Cancelled`)…"
          docstring: "Like [`Self::iter`], skipping logically unlinked nodes (`Done`/`Cancelled`)."
        - id: impl_Queue_P_D_O_E_.claim
          signature: "pub fn claim(&self, node: &Node<P, D, O, E>) -> Option<D> {"
      - id: tests
        signature: "#[cfg(all(test, not(loom)))] mod tests {"
        children:
        - id: tests.GEQxfX|CN1g8X
          signature: use super::*;
        - id: tests.task_state
          signature: "fn task_state() -> Arc<TaskState<u32, ()>> {"
        - id: tests.push_claim_takes_payload_once
          signature: "#[test] fn push_claim_takes_payload_once() {"
        - id: tests.iter_yields_age_order_and_skips_sentinel
          signature: "#[test] fn iter_yields_age_order_and_skips_sentinel() {"
        - id: tests.push_notifies_all_relays
          signature: "#[test] fn push_notifies_all_relays() {"
        - id: tests.drop_long_chain_does_not_overflow_stack
          signature: "#[test] fn drop_long_chain_does_not_overflow_stack() {"
        - id: tests.unlinked_nodes_are_skipped_by_live_iteration
          signature: "#[test] fn unlinked_nodes_are_skipped_by_live_iteration() {"
        - id: tests.stale_tail_hint_keeps_order
          signature: "#[test] fn stale_tail_hint_keeps_order() {"
        - id: tests.concurrent_pushes_are_all_linked
          signature: "#[test] fn concurrent_pushes_are_all_linked() {"
        - id: tests.uc75fA|oCZQjC
          signature: "proptest::proptest! {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/node.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Append-only chain node. Public API hides the sentinel used by [`super::Queu…"
      - id: RVGwig|nFnuFl
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
        - id: impl_Node_P_D_O_E_.is_live
          signature: /// `false` once the task is `Done`/`Cancelled`; such nodes stay in the chain /…
          docstring: '`false` once the task is `Done`/`Cancelled`; such nodes stay in the chain'
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
        signature: "#[cfg(all(test, not(loom)))] mod tests {"
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
      - id: loom_tests
        signature: "#[cfg(loom)] mod loom_tests {"
        children:
        - id: loom_tests.ekaEGn|wKY6lI
          signature: use super::*;
        - id: loom_tests.state
          signature: "fn state() -> Arc<TaskState<u32, ()>> {"
        - id: loom_tests.concurrent_claims_have_one_winner
          signature: "#[test] fn concurrent_claims_have_one_winner() {"
        - id: loom_tests.claim_and_cancel_are_exclusive
          signature: "#[test] fn claim_and_cancel_are_exclusive() {"
        - id: loom_tests.done_implies_result_visible
          signature: "#[test] fn done_implies_result_visible() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
      nodes:
      - id: ModuleDoc
        signature: //! Task-queue instantiation notes.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Operation, requires(), Value::Host|Device."
      - id: lease
        signature: mod lease;
      - id: requirements
        signature: mod requirements;
      - id: o3owZE|ImBb5C
        signature: "use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};"
      - id: OpKind
        signature: "#[non_exhaustive] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub enum O…"
      - id: OpParams
        signature: "/// Immutable parameters; usable as cache/graph key. #[derive(Clone, Debug, Par…"
        docstring: Immutable parameters; usable as cache/graph key.
      - id: impl_OpParams
        signature: "impl OpParams {"
        children:
        - id: impl_OpParams.new
          signature: "pub fn new(requires: Requirements) -> Self {"
      - id: Op
        signature: "pub trait Op: Send + Sync {"
        children:
        - id: Op.kind
          signature: fn kind(&self) -> OpKind;
        - id: Op.params
          signature: fn params(&self) -> &OpParams;
        - id: Op.requires
          signature: "fn requires(&self) -> Requirements {"
      - id: OpDesc
        signature: "/// Plain-data operation description. #[derive(Clone, Debug, PartialEq, Eq, Has…"
        docstring: Plain-data operation description.
      - id: impl_OpDesc
        signature: "impl OpDesc {"
        children:
        - id: impl_OpDesc.new
          signature: "pub fn new(kind: OpKind, params: OpParams) -> Self {"
      - id: impl_Op_for_OpDesc
        signature: "impl Op for OpDesc {"
        children:
        - id: impl_Op_for_OpDesc.kind
          signature: "fn kind(&self) -> OpKind {"
        - id: impl_Op_for_OpDesc.params
          signature: "fn params(&self) -> &OpParams {"
      - id: HostData
        signature: "#[non_exhaustive] #[derive(Clone, Debug, PartialEq)] pub enum HostData {"
      - id: Value
        signature: "/// Op input/output; never carries a tensor or raw pointer. #[derive(Debug)] pu…"
        docstring: Op input/output; never carries a tensor or raw pointer.
      - id: TaskCtx
        signature: "/// Per-task shared state: open-op counter, first error, cancellation. #[derive…"
        docstring: "Per-task shared state: open-op counter, first error, cancellation."
      - id: impl_TaskCtx
        signature: "impl TaskCtx {"
        children:
        - id: impl_TaskCtx.new
          signature: "pub fn new(id: u64, priority: u8, ops: usize) -> Arc<Self> {"
        - id: impl_TaskCtx.id
          signature: "pub fn id(&self) -> u64 {"
        - id: impl_TaskCtx.priority
          signature: "pub fn priority(&self) -> u8 {"
        - id: impl_TaskCtx.open_ops
          signature: "pub fn open_ops(&self) -> usize {"
        - id: impl_TaskCtx.is_cancelled
          signature: "pub fn is_cancelled(&self) -> bool {"
        - id: impl_TaskCtx.error
          signature: "pub fn error(&self) -> Option<&ResidentError> {"
        - id: impl_TaskCtx.finish_op
          signature: /// Returns `true` when this call closed the last open op of a non-failed task.…
          docstring: Returns `true` when this call closed the last open op of a non-failed task.
        - id: impl_TaskCtx.fail
          signature: /// Cancels the task; only the first error is kept. Returns `true` if this call…
          docstring: Cancels the task; only the first error is kept. Returns `true` if this call was…
      - id: OperationNode
        signature: "pub struct OperationNode {"
      - id: impl_OperationNode
        signature: "impl OperationNode {"
        children:
        - id: impl_OperationNode.new
          signature: "pub fn new(     op: Arc<dyn Op>,     input: Value,     materialize: bool,     t…"
        - id: impl_OperationNode.set_output
          signature: /// Sets the output once; a second call returns the rejected value. pub fn set_…
          docstring: Sets the output once; a second call returns the rejected value.
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.7YB2Yw|bqr4NJ
          signature: use super::*;
        - id: tests.desc
          signature: "fn desc(kind: OpKind) -> Arc<dyn Op> {"
        - id: tests.err
          signature: "fn err() -> ResidentError {"
        - id: tests.params_clone_hash_eq
          signature: "#[test] fn params_clone_hash_eq() {"
        - id: tests.requires_comes_from_params
          signature: "#[test] fn requires_comes_from_params() {"
        - id: tests.task_done_only_at_zero
          signature: "#[test] fn task_done_only_at_zero() {"
        - id: tests.first_error_cancels_and_is_kept
          signature: "#[test] fn first_error_cancels_and_is_kept() {"
        - id: tests.output_set_once_and_device_value_holds_lease
          signature: "#[test] fn output_set_once_and_device_value_holds_lease() {"
        - id: tests.affinity_is_non_exhaustive
          signature: "#[test] fn affinity_is_non_exhaustive() {"
```
/answer 4eac-b2 allow
Control Request:
```yaml
id: 4eac-b3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
    ids:
    - OpKind
    - OpParams
    - Op
    - OpDesc
    - HostData
    - Value
    - TaskCtx
    - impl_TaskCtx
    - OperationNode
    - impl_OperationNode
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/queue.rs
    ids:
    - Queue
    - impl_Queue_P_D_O_E_
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
    ids:
    - ModuleDoc
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
    ids:
    - PinCount
    - Lease
    - impl_Lease
  reason: "Vollständige Definitionen von Op, TaskCtx, OperationNode, Queue und Lease lesen, um sie im Processor korrekt zu verdrahten."
```
/answer 4eac-b3 allow
ReasoningToken: 72
Control Request:
```yaml
id: 4eac-b4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
      nodes:
      - id: OpKind
        type: ItemEnum
        code: |
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
      - id: OpParams
        type: ItemStruct
        code: |
          /// Immutable parameters; usable as cache/graph key.
          #[derive(Clone, Debug, PartialEq, Eq, Hash)]
          pub struct OpParams {
              pub requires: Requirements,
              pub top_k: Option<u32>,
              pub dim: Option<u32>,
          }
      - id: Op
        type: ItemTrait
        signature: "pub trait Op: Send + Sync {"
        children:
        - id: Op.kind
          type: TraitItemFn
          code: |2
                fn kind(&self) -> OpKind;
        - id: Op.params
          type: TraitItemFn
          code: |2
                fn params(&self) -> &OpParams;
        - id: Op.requires
          type: TraitItemFn
          code: |2
                fn requires(&self) -> Requirements {
                    self.params().requires.clone()
                }
      - id: OpDesc
        type: ItemStruct
        code: |
          /// Plain-data operation description.
          #[derive(Clone, Debug, PartialEq, Eq, Hash)]
          pub struct OpDesc {
              kind: OpKind,
              params: OpParams,
          }
      - id: HostData
        type: ItemEnum
        code: |
          #[non_exhaustive]
          #[derive(Clone, Debug, PartialEq)]
          pub enum HostData {
              Text(Vec<String>),
              Tokens(Vec<Vec<u32>>),
              F32(Vec<f32>),
              Hits(Vec<(u64, f32)>),
          }
      - id: Value
        type: ItemEnum
        code: |
          /// Op input/output; never carries a tensor or raw pointer.
          #[derive(Debug)]
          pub enum Value {
              Host(HostData),
              Device(DeviceHandle),
          }
      - id: TaskCtx
        type: ItemStruct
        code: |
          /// Per-task shared state: open-op counter, first error, cancellation.
          #[derive(Debug)]
          pub struct TaskCtx {
              id: u64,
              priority: u8,
              open: AtomicUsize,
              cancelled: AtomicBool,
              error: OnceLock<ResidentError>,
          }
      - id: impl_TaskCtx
        type: ItemImpl
        signature: "impl TaskCtx {"
        children:
        - id: impl_TaskCtx.new
          type: ImplItemFn
          code: |2
                pub fn new(id: u64, priority: u8, ops: usize) -> Arc<Self> {
                    Arc::new(Self {
                        id,
                        priority,
                        open: AtomicUsize::new(ops),
                        cancelled: AtomicBool::new(false),
                        error: OnceLock::new(),
                    })
                }
        - id: impl_TaskCtx.id
          type: ImplItemFn
          code: |2
                pub fn id(&self) -> u64 {
                    self.id
                }
        - id: impl_TaskCtx.priority
          type: ImplItemFn
          code: |2
                pub fn priority(&self) -> u8 {
                    self.priority
                }
        - id: impl_TaskCtx.open_ops
          type: ImplItemFn
          code: |2
                pub fn open_ops(&self) -> usize {
                    self.open.load(Ordering::Acquire)
                }
        - id: impl_TaskCtx.is_cancelled
          type: ImplItemFn
          code: |2
                pub fn is_cancelled(&self) -> bool {
                    self.cancelled.load(Ordering::Acquire)
                }
        - id: impl_TaskCtx.error
          type: ImplItemFn
          code: |2
                pub fn error(&self) -> Option<&ResidentError> {
                    self.error.get()
                }
        - id: impl_TaskCtx.finish_op
          type: ImplItemFn
          code: |2
                /// Returns `true` when this call closed the last open op of a non-failed task.
                pub fn finish_op(&self) -> bool {
                    let prev = self.open.fetch_sub(1, Ordering::AcqRel);
                    debug_assert!(prev > 0, "finish_op without open op");
                    prev == 1 && !self.is_cancelled()
                }
        - id: impl_TaskCtx.fail
          type: ImplItemFn
          code: |2
                /// Cancels the task; only the first error is kept. Returns `true` if this call was first.
                pub fn fail(&self, err: ResidentError) -> bool {
                    let first = self.error.set(err).is_ok();
                    self.cancelled.store(true, Ordering::Release);
                    if first {
                        tracing::debug!(task = self.id, "task cancelled by first error");
                    }
                    first
                }
      - id: OperationNode
        type: ItemStruct
        code: |
          pub struct OperationNode {
              pub op: Arc<dyn Op>,
              pub input: Value,
              pub output: OnceLock<Value>,
              /// `true`: result returns to host; `false`: device handle stays resident.
              pub materialize: bool,
              pub task: Arc<TaskCtx>,
          }
      - id: impl_OperationNode
        type: ItemImpl
        signature: "impl OperationNode {"
        children:
        - id: impl_OperationNode.new
          type: ImplItemFn
          code: |2
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
        - id: impl_OperationNode.set_output
          type: ImplItemFn
          code: |2
                /// Sets the output once; a second call returns the rejected value.
                pub fn set_output(&self, value: Value) -> Result<(), Value> {
                    self.output.set(value)
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/queue.rs
      nodes:
      - id: Queue
        type: ItemStruct
        code: |
          /// Generic over `P` (filter metadata) and `D` (payload); `O`/`E` are the
          /// result/error types carried by each node's [`TaskState`]. Reused as-is for
          /// op-queues, whose completion is tracked externally (task-op counter) rather
          /// than through `TaskState::result` (see `task` submodule).
          pub struct Queue<P, D, O, E> {
              head: Arc<Node<P, D, O, E>>,
              tail: ArcSwapOption<Node<P, D, O, E>>,
              relays: Vec<Arc<Notify>>,
          }
      - id: impl_Queue_P_D_O_E_
        type: ItemImpl
        signature: "impl<P, D, O, E> Queue<P, D, O, E> {"
        children:
        - id: impl_Queue_P_D_O_E_.new
          type: ImplItemFn
          code: |2
                /// `relays` are notified (append-relay) on every push; one per subengine.
                pub fn new(relays: Vec<Arc<Notify>>) -> Self {
                    Self {
                        head: Node::sentinel(),
                        tail: ArcSwapOption::from(None),
                        relays,
                    }
                }
        - id: impl_Queue_P_D_O_E_.push
          type: ImplItemFn
          code: |2
                pub fn push(
                    &self,
                    params: P,
                    state: Arc<TaskState<O, E>>,
                    data: D,
                ) -> Arc<Node<P, D, O, E>> {
                    let node = Node::new(params, state, data);
                    self.append(node.clone());
                    for relay in &self.relays {
                        relay.notify_one();
                    }
                    tracing::debug!("push");
                    node
                }
        - id: impl_Queue_P_D_O_E_.append
          type: ImplItemFn
          code: |2
                fn append(&self, new: Arc<Node<P, D, O, E>>) {
                    let mut cur = self.tail_hint();
                    loop {
                        if let Some(next) = cur.peek_next() {
                            cur = next;
                            continue;
                        }
                        match cur.try_link_next(new.clone()) {
                            Ok(()) => {
                                if !failpoint::hit("queue.skip_tail_store", self as *const Self as u64) {
                                    self.tail.store(Some(new));
                                }
                                return;
                            }
                            Err(existing) => cur = existing,
                        }
                    }
                }
        - id: impl_Queue_P_D_O_E_.tail_hint
          type: ImplItemFn
          code: |2
                fn tail_hint(&self) -> Arc<Node<P, D, O, E>> {
                    (*self.tail.load()).clone().unwrap_or_else(|| self.head.clone())
                }
        - id: impl_Queue_P_D_O_E_.iter
          type: ImplItemFn
          code: |2
                /// Non-blocking traversal, age order, sentinel excluded.
                pub fn iter(&self) -> Iter<P, D, O, E> {
                    Iter::from_sentinel(self.head.clone())
                }
        - id: impl_Queue_P_D_O_E_.iter_live
          type: ImplItemFn
          code: |2
                /// Like [`Self::iter`], skipping logically unlinked nodes (`Done`/`Cancelled`).
                pub fn iter_live(&self) -> impl Iterator<Item = Arc<Node<P, D, O, E>>> {
                    self.iter().filter(|n| n.is_live())
                }
        - id: impl_Queue_P_D_O_E_.claim
          type: ImplItemFn
          code: |2
                pub fn claim(&self, node: &Node<P, D, O, E>) -> Option<D> {
                    node.claim()
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Task-queue instantiation notes.
          //!
          //! The task queue is `Queue<P, D, O, E>` where `P`/`D` are the task's
          //! params/payload types and `O`/`E` are its output/error types, all supplied
          //! by the caller (concrete types land in later packages). `TaskState<O, E>`
          //! carries exactly one `Result<O, E>`, set once by the claim winner.
          //!
          //! Op-queues (CPU/GPU/shared) reuse the same `Queue`/`Node` with their own
          //! `OpParams`/value payload; completion is tracked via the task's op counter
          //! (processor/op layer), not through `TaskState::result`. The shared op-queue is created
          //! empty and carries no scheduling logic yet; a subengine checks its own
          //! queue before falling back to shared.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
      nodes:
      - id: PinCount
        type: ItemStruct
        code: |
          #[derive(Debug, Default)]
          pub struct PinCount(AtomicUsize);
      - id: Lease
        type: ItemStruct
        code: |
          #[derive(Debug)]
          pub struct Lease {
              region: Region,
              pin: Arc<PinCount>,
              release: Option<UnboundedSender<Region>>,
          }
      - id: impl_Lease
        type: ItemImpl
        signature: "impl Lease {"
        children:
        - id: impl_Lease.new
          type: ImplItemFn
          code: |2
                /// Increments the pin count; `release` receives the region on drop.
                pub fn new(region: Region, pin: Arc<PinCount>, release: Option<UnboundedSender<Region>>) -> Self {
                    pin.0.fetch_add(1, Ordering::AcqRel);
                    Self { region, pin, release }
                }
        - id: impl_Lease.region
          type: ImplItemFn
          code: |2
                pub fn region(&self) -> Region {
                    self.region
                }
```
/answer 4eac-b4 allow
Control Request:
```yaml
id: 4eac-b5
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: WorkerConfig|pub trait |pub enum ResidentError|ResourceKind|LaneId|Starved
    include:
    - '*.rs'
    limit: 50
  reason: "Vorhandene WorkerConfig, Traits, Fehlerenum und eventuell bereits definierte ResourceKind/LaneId finden."
```
/answer 4eac-b5 allow
Control Request:
```yaml
id: 4eac-b6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: facade/mod.rs
        matches:
        - lineno: 18
          match: "IndexInfo, Residency, Resident, ResidentBuilder, ResidentConfig, WorkerConfig,"
      - path: facade/resident.rs
        matches:
        - lineno: 20
          match: "pub struct WorkerConfig {"
        - lineno: 25
          match: "impl WorkerConfig {"
        - lineno: 31
          match: "impl Default for WorkerConfig {"
        - lineno: 41
          match: "workers: WorkerConfig,"
        - lineno: 50
          match: "pub fn workers(&self) -> WorkerConfig {"
        - lineno: 62
          match: "workers: WorkerConfig,"
        - lineno: 76
          match: "pub fn worker_hint(mut self, workers: WorkerConfig) -> Self {"
      - path: index/mod.rs
        matches:
        - lineno: 158
          match: "pub trait Index {"
      - path: kernel/mod.rs
        matches:
        - lineno: 75
          match: "pub trait Kernel: Send + Sync {"
      - path: error.rs
        matches:
        - lineno: 8
          match: "pub trait IsRetryable {"
        - lineno: 115
          match: "pub enum ResidentError {"
      - path: backend.rs
        matches:
        - lineno: 11
          match: "pub trait ComputeBackend: Send + Sync {"
      - path: memory/manager.rs
        matches:
        - lineno: 25
          match: "pub trait MemoryManager {"
      - path: mock.rs
        matches:
        - lineno: 10
          match: "pub trait Clock: Send + Sync {"
        - lineno: 32
          match: "pub trait Measure {"
      - path: op/mod.rs
        matches:
        - lineno: 11
          match: "Affinity, DeviceId, EngineSelector, IndexId, ModelId, Requirements, ResourceKind,"
        - lineno: 45
          match: "pub trait Op: Send + Sync {"
        - lineno: 186
          match: "let req = Requirements::new(ResourceKind::Encoder, EngineSelector::Either);"
        - lineno: 197
          match: "let p = OpParams::new(Requirements::new(ResourceKind::Scorer, EngineSelector::Gpu));"
      - path: op/requirements.rs
        matches:
        - lineno: 12
          match: "pub enum ResourceKind {"
        - lineno: 45
          match: "pub resource: ResourceKind,"
        - lineno: 53
          match: "pub fn new(resource: ResourceKind, engine: EngineSelector) -> Self {"
        - lineno: 89
          match: "let req = Requirements::new(ResourceKind::Encoder, EngineSelector::Gpu)"
        - lineno: 98
          match: "let req = Requirements::new(ResourceKind::Tokenizer, EngineSelector::Either);"
        - lineno: 105
          match: "let req = Requirements::new(ResourceKind::Scorer, EngineSelector::Either)"
```
/answer 4eac-b6 allow
Control Request:
```yaml
id: 4eac-b7
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
    ids:
    - ResourceKind
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
    ids:
    - WorkerConfig
    - impl_WorkerConfig
    - impl_Default_for_WorkerConfig
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - ResidentError
  reason: "ResourceKind-Varianten, bestehende WorkerConfig (Facade) und ResidentError für Processor-Fehlerpfade lesen."
```
/answer 4eac-b7 allow
Control Request:
```yaml
id: 4eac-b8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
      nodes:
      - id: ResourceKind
        type: ItemEnum
        code: |
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
      nodes:
      - id: WorkerConfig
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          #[non_exhaustive]
          pub struct WorkerConfig {
              pub cpu: usize,
              pub gpu: usize,
          }
      - id: impl_WorkerConfig
        type: ItemImpl
        signature: "impl WorkerConfig {"
        children:
        - id: impl_WorkerConfig.new
          type: ImplItemFn
          code: |2
                pub fn new(cpu: usize, gpu: usize) -> Self {
                    Self { cpu, gpu }
                }
      - id: impl_Default_for_WorkerConfig
        type: ItemImpl
        signature: "impl Default for WorkerConfig {"
        children:
        - id: impl_Default_for_WorkerConfig.default
          type: ImplItemFn
          code: |2
                fn default() -> Self {
                    Self::new(1, 0)
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: ResidentError
        type: ItemEnum
        code: |
          /// Top-level resident engine error, wrapping every layer error via `#[from]`.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum ResidentError {
              #[error(transparent)]
              Index(#[from] IndexError),
              #[error(transparent)]
              Precision(#[from] PrecisionError),
              #[error(transparent)]
              Planner(#[from] PlannerError),
              #[error(transparent)]
              Config(#[from] ConfigError),
              #[error(transparent)]
              Capability(#[from] CapabilityError),
          }
```
/answer 4eac-b8 allow
Control Request:
```yaml
id: 4eac-b9
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^(arc-swap|thiserror|tracing|libc|candle|cudarc|loom|proptest|\\[features\\]|cuda|mock|\\[dev-dep|\\[target|\\[lints)"
    include:
    - Cargo.toml
    exclude:
    - third_party/**
    - .bin/**
    - .xyrag/**
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: \b(Requirements|EngineSelector|Affinity)\b.*(struct|enum)|pub (fn|struct|enum)
    include:
    - requirements.rs
    - mock.rs
    limit: 40
  reason: Features/Dependencies in Cargo.toml und Requirements-/Mock-API prüfen.
```
/answer 4eac-b9 allow
Control Request:
```yaml
id: 4eac-ba
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: .xyrag/layers/trigram/mirror/Cargo.toml
        matches:
        - lineno: 34
          match: proptest 1
      - path: Cargo.toml
        matches:
        - lineno: 32
          match: thiserror = "2.0.20"
        - lineno: 33
          match: tracing = "0.1.44"
        - lineno: 34
          match: "tracing-subscriber = { version = \"0.3.23\", features = [\"env-filter\"] }"
        - lineno: 35
          match: tracing-appender = "0.2.3"
        - lineno: 39
          match: arc-swap = "1.9.2"
        - lineno: 47
          match: "candle-core = { version = \"0.11.0\", optional = true, default-features = false }"
        - lineno: 48
          match: "candle-transformers = { version = \"0.11.0\", optional = true, default-features = false }"
        - lineno: 49
          match: "cudarc = { version = \"=0.19.9\", optional = true }"
        - lineno: 52
          match: "[features]"
        - lineno: 54
          match: "candle = [\"dep:candle-core\"]"
        - lineno: 55
          match: "cuda = [\"candle\", \"dep:candle-transformers\", \"dep:cudarc\", \"candle-core/cuda\", \"candle-transformers/cuda\"]"
        - lineno: 56
          match: "mock = [\"dep:rand\"]"
        - lineno: 59
          match: "[dev-dependencies]"
        - lineno: 60
          match: proptest = "1"
        - lineno: 62
          match: mockall = "0.15.0"
        - lineno: 63
          match: loom = "0.7"
        - lineno: 73
          match: "candle-kernels = { path = \"third_party/candle-kernels\" }"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: kernel/mock.rs
        matches:
        - lineno: 13
          match: "pub enum Call {"
        - lineno: 39
          match: "pub struct MockKernel {"
        - lineno: 48
          match: "pub fn scoring() -> Self {"
        - lineno: 52
          match: "pub fn embedding(capacity: u64) -> Self {"
        - lineno: 67
          match: "pub fn fail_after(mut self, n: u64) -> Self {"
        - lineno: 73
          match: "pub fn grant(&self, bytes: u64) {"
        - lineno: 77
          match: "pub fn calls(&self) -> Vec<Call> {"
      - path: mock.rs
        matches:
        - lineno: 14
          match: "pub struct FakeClock {"
        - lineno: 24
          match: pub struct MockBackend;
        - lineno: 36
          match: "pub enum NoiseModel {"
        - lineno: 43
          match: "pub struct FakeMeasure {"
        - lineno: 51
          match: "pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {"
      - path: op/requirements.rs
        matches:
        - lineno: 5
          match: pub struct ModelId(pub String);
        - lineno: 7
          match: pub struct IndexId(pub String);
        - lineno: 9
          match: pub struct DeviceId(pub u32);
        - lineno: 12
          match: "pub enum ResourceKind {"
        - lineno: 22
          match: "pub enum EngineSelector {"
        - lineno: 28
          match: "pub fn accepts(self, dev: DeviceKind) -> bool {"
        - lineno: 38
          match: "pub enum Affinity {"
        - lineno: 44
          match: "pub struct Requirements {"
        - lineno: 53
          match: "pub fn new(resource: ResourceKind, engine: EngineSelector) -> Self {"
        - lineno: 63
          match: "pub fn with_model(mut self, model: ModelId) -> Self {"
        - lineno: 67
          match: "pub fn with_index(mut self, index: IndexId) -> Self {"
        - lineno: 71
          match: "pub fn with_caps(mut self, caps: CapabilitySet) -> Self {"
        - lineno: 75
          match: "pub fn with_affinity(mut self, affinity: Affinity) -> Self {"
        - lineno: 80
          match: "pub fn satisfied_by(&self, dev: DeviceKind, available: &CapabilitySet) -> bool {"
```
/answer 4eac-ba allow
Control Request:
```yaml
id: 4eac-bb
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/backend.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/telemetry.rs
  reason: "Fehler-Enums, Subengine-/Device-Verträge, Backend und Telemetrie als Eingänge für den Processor kennen."
```
/answer 4eac-bb allow
ReasoningToken: 280
Control Request:
```yaml
id: 4eac-bc
result:
  structuredContent:
    results:
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
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.assert_send_sync_static
          signature: "fn assert_send_sync_static<T: Send + Sync + 'static>() {"
        - id: tests.all_errors_are_send_sync_static
          signature: "#[test] fn all_errors_are_send_sync_static() {"
        - id: tests.layer_errors_convert_into_resident_error
          signature: "#[test] fn layer_errors_convert_into_resident_error() {"
        - id: tests.transparent_wrapper_keeps_message
          signature: "#[test] fn transparent_wrapper_keeps_message() {"
        - id: tests.retryability_classification
          signature: "#[test] fn retryability_classification() {"
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
        signature: mod device;
      - id: l9ZlBG|zqWesy
        signature: pub use device::*;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Device/Lane, execution context, loaded encoder, GPU memory arenas."
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: CudaBackend
        signature: /// Real CUDA backend; the single Candle/CUDA adapter point for the subengine. …
        docstring: Real CUDA backend; the single Candle/CUDA adapter point for the subengine. Cons…
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Stateless task expansion.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! StreamedIndexManager, ScopeResolver, update queue, residency."
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/backend.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Compute backend abstraction and the single `#[cfg(feature = \"cuda\")]` decis…"
      - id: fgO4rd|wZT8Ld
        signature: use crate::resident::subengine::CpuBackend;
      - id: ComputeBackend
        signature: /// Backend abstraction the orchestration layer runs against. pub trait Compute…
        docstring: Backend abstraction the orchestration layer runs against.
        children:
        - id: ComputeBackend.run_batch
          signature: "fn run_batch(&self, input: &[f32]) -> Vec<f32>;"
      - id: create_backend
        signature: "/// Constructs the active backend: CPU without `cuda`; with `cuda`, attempts th…"
        docstring: "Constructs the active backend: CPU without `cuda`; with `cuda`, attempts the CU…"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/telemetry.rs
      nodes:
      - id: ModuleDoc
        signature: //! Tracing conventions and init.
      - id: k3B3JK|9oFaU5
        signature: use tracing::Span;
      - id: 6qwbAj|88i4m1
        signature: /// Stable span/event field names (verbatim everywhere so filters and JSON pipe…
      - id: init_tracing
        signature: "/// Initializes the global subscriber from `RUST_LOG`, defaulting to `info`. //…"
        docstring: "Initializes the global subscriber from `RUST_LOG`, defaulting to `info`."
      - id: init_tracing_non_blocking
        signature: "/// Builds a non-blocking, optionally JSON-formatted subscriber from `RUST_LOG`…"
        docstring: "Builds a non-blocking, optionally JSON-formatted subscriber from `RUST_LOG` (de…"
      - id: task_span
        signature: "/// Creates the task span: one per task, carrying [`FIELD_TASK_ID`]. Parent of …"
        docstring: "Creates the task span: one per task, carrying [`FIELD_TASK_ID`]. Parent of reso…"
      - id: resource_span
        signature: "/// Creates the resource span: one per leased resource (device/index), nested u…"
        docstring: "Creates the resource span: one per leased resource (device/index), nested under…"
      - id: manager_cycle_span
        signature: "/// Creates the manager-cycle span: one per communication slot / scan cycle, ca…"
        docstring: "Creates the manager-cycle span: one per communication slot / scan cycle, carryi…"
      - id: for_blocking
        signature: "/// Clones `span` for use inside a `spawn_blocking`/plain-thread closure, per t…"
        docstring: "Clones `span` for use inside a `spawn_blocking`/plain-thread closure, per the p…"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.Clqc8p|8mJott
          signature: use super::*;
        - id: tests.Capture
          signature: "/// Minimal subscriber capturing span field names/values for assertions, since …"
          docstring: "Minimal subscriber capturing span field names/values for assertions, since we o…"
        - id: tests.FieldDump
          signature: "struct FieldDump<'a>(&'a mut Vec<(String, String)>);"
        - id: tests.impl_Visit_for_FieldDump____
          signature: "impl Visit for FieldDump<'_> {"
          children:
          - id: tests.impl_Visit_for_FieldDump____.record_debug
            signature: "fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {"
        - id: tests.impl_Subscriber_for_Capture
          signature: "impl Subscriber for Capture {"
          children:
          - id: tests.impl_Subscriber_for_Capture.enabled
            signature: "fn enabled(&self, _: &Metadata<'_>) -> bool {"
          - id: tests.impl_Subscriber_for_Capture.register_callsite
            signature: "fn register_callsite(&self, _: &'static Metadata<'static>) -> Interest {"
          - id: tests.impl_Subscriber_for_Capture.new_span
            signature: "fn new_span(&self, attrs: &Attributes<'_>) -> Id {"
          - id: tests.impl_Subscriber_for_Capture.record
            signature: "fn record(&self, _: &Id, _: &tracing::span::Record<'_>) {"
          - id: tests.impl_Subscriber_for_Capture.record_follows_from
            signature: "fn record_follows_from(&self, _: &Id, _: &Id) {"
          - id: tests.impl_Subscriber_for_Capture.event
            signature: "fn event(&self, _: &tracing::Event<'_>) {"
          - id: tests.impl_Subscriber_for_Capture.enter
            signature: "fn enter(&self, _: &Id) {"
          - id: tests.impl_Subscriber_for_Capture.exit
            signature: "fn exit(&self, _: &Id) {"
        - id: tests.task_span_carries_stable_task_id_field
          signature: "#[test] fn task_span_carries_stable_task_id_field() {"
        - id: tests.for_blocking_preserves_span_identity_fields
          signature: "#[test] fn for_blocking_preserves_span_identity_fields() {"
        - id: tests.stable_field_names_match_convention
          signature: "#[test] fn stable_field_names_match_convention() {"
```
/answer 4eac-bc allow
Control Request:
```yaml
id: 4eac-bd
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
    ids:
    - EngineSelector
    - Requirements
  reason: EngineSelector/DeviceKind-Semantik für die Op-Queue-Zuordnung lesen.
```
/answer 4eac-bd allow
Control Request:
```yaml
id: 4eac-be
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
      nodes:
      - id: EngineSelector
        type: ItemEnum
        code: |
          #[non_exhaustive]
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub enum EngineSelector {
              Cpu,
              Gpu,
              Either,
          }
      - id: Requirements
        type: ItemStruct
        code: |
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
```
/answer 4eac-be allow
Control Request:
```yaml
id: 4eac-bf
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - IndexError
    - ManagerError
    - PlannerError
    - impl_IsRetryable_for_ResidentError
    - impl_IsRetryable_for_PlannerError
    - tests.layer_errors_convert_into_resident_error
    - tests.retryability_classification
    - tests.all_errors_are_send_sync_static
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/telemetry.rs
    ids:
    - task_span
    - resource_span
    - for_blocking
  reason: "Fehlervarianten und Telemetrie-Helfer lesen, um OrchestrationError sauber in ResidentError einzuhängen."
```
/answer 4eac-bf allow
ReasoningToken: 96
Control Request:
```yaml
id: 4eac-c0
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: IndexError
        type: ItemEnum
        code: |
          /// Index/scope layer errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum IndexError {
              #[error("access method '{method}' is not supported for this index")]
              UnsupportedScope { method: String },
              #[error("index for scope '{scope}' is not available")]
              IndexUnavailable { scope: String },
              #[error("arena exhausted: requested {requested} bytes, available {available} bytes")]
              ArenaExhausted { requested: usize, available: usize },
              #[error("transfer failed: {reason}")]
              TransferFailed { reason: String },
              #[error("kernel failed: {reason}")]
              KernelFailed { reason: String },
              #[error("unknown kernel batch {batch}")]
              UnknownBatch { batch: u64 },
          }
      - id: ManagerError
        type: ItemEnum
        code: |
          /// Internal index-manager errors; mapped to a public variant or retried at the API boundary,
          /// never exposed directly.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub(crate) enum ManagerError {
              #[error("stale region in arena {arena} (gen {gen})")]
              StaleRegion { arena: u32, gen: u32 },
          }
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
      - id: impl_IsRetryable_for_ResidentError
        type: ItemImpl
        signature: "impl IsRetryable for ResidentError {"
        children:
        - id: impl_IsRetryable_for_ResidentError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    match self {
                        ResidentError::Index(e) => e.is_retryable(),
                        ResidentError::Precision(e) => e.is_retryable(),
                        ResidentError::Planner(e) => e.is_retryable(),
                        ResidentError::Config(e) => e.is_retryable(),
                        ResidentError::Capability(e) => e.is_retryable(),
                    }
                }
      - id: impl_IsRetryable_for_PlannerError
        type: ItemImpl
        signature: "impl IsRetryable for PlannerError {"
        children:
        - id: impl_IsRetryable_for_PlannerError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    false
                }
      - id: tests.layer_errors_convert_into_resident_error
        type: ItemFn
        code: |2
              #[test]
              fn layer_errors_convert_into_resident_error() {
                  let e: ResidentError = IndexError::UnknownBatch { batch: 3 }.into();
                  assert!(matches!(e, ResidentError::Index(IndexError::UnknownBatch { batch: 3 })));
                  let e: ResidentError = ConfigError::MissingField { field: "model".into() }.into();
                  assert!(matches!(e, ResidentError::Config(_)));
                  let e: ResidentError = PrecisionError::NoActiveProfile.into();
                  assert!(matches!(e, ResidentError::Precision(_)));
                  let e: ResidentError = PlannerError::UnknownModel { model: "m".into() }.into();
                  assert!(matches!(e, ResidentError::Planner(_)));
                  let e: ResidentError = CapabilityError::Missing { capability: "c".into() }.into();
                  assert!(matches!(e, ResidentError::Capability(_)));
              }
      - id: tests.retryability_classification
        type: ItemFn
        code: |2
              #[test]
              fn retryability_classification() {
                  assert!(IndexError::ArenaExhausted { requested: 1, available: 0 }.is_retryable());
                  assert!(IndexError::IndexUnavailable { scope: "s".into() }.is_retryable());
                  assert!(!IndexError::KernelFailed { reason: "r".into() }.is_retryable());
                  assert!(ManagerError::StaleRegion { arena: 1, gen: 2 }.is_retryable());
                  assert!(PrecisionError::CalibrationUnstable { reason: "r".into() }.is_retryable());
                  assert!(!PrecisionError::NoActiveProfile.is_retryable());
                  assert!(!ConfigError::InvalidValue { field: "f".into(), reason: "r".into() }.is_retryable());
                  assert!(!CapabilityError::Unknown { capability: "c".into() }.is_retryable());
                  let wrapped: ResidentError = IndexError::ArenaExhausted { requested: 1, available: 0 }.into();
                  assert!(wrapped.is_retryable());
                  let fatal: ResidentError = PlannerError::InvalidConfig { reason: "r".into() }.into();
                  assert!(!fatal.is_retryable());
              }
      - id: tests.all_errors_are_send_sync_static
        type: ItemFn
        code: |2
              #[test]
              fn all_errors_are_send_sync_static() {
                  assert_send_sync_static::<ResidentError>();
                  assert_send_sync_static::<IndexError>();
                  assert_send_sync_static::<ManagerError>();
                  assert_send_sync_static::<PrecisionError>();
                  assert_send_sync_static::<PlannerError>();
                  assert_send_sync_static::<CapabilityError>();
                  assert_send_sync_static::<ConfigError>();
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/telemetry.rs
      nodes:
      - id: task_span
        type: ItemFn
        code: |
          /// Creates the task span: one per task, carrying [`FIELD_TASK_ID`]. Parent of resource and
          /// op spans for that task.
          pub fn task_span(task_id: &str) -> Span {
              tracing::info_span!("task", task_id = % task_id)
          }
      - id: resource_span
        type: ItemFn
        code: |
          /// Creates the resource span: one per leased resource (device/index), nested under the
          /// current task span. Carries [`FIELD_TASK_ID`] and [`FIELD_RESOURCE_KIND`].
          pub fn resource_span(task_id: &str, resource_kind: &str) -> Span {
              tracing::info_span!("resource", task_id = % task_id, resource_kind = % resource_kind)
          }
      - id: for_blocking
        type: ItemFn
        code: |
          /// Clones `span` for use inside a `spawn_blocking`/plain-thread closure, per the propagation
          /// rule above.
          pub fn for_blocking(span: &Span) -> Span {
              span.clone()
          }
```
/answer 4eac-c0 allow
Control Request:
```yaml
id: 4eac-c1
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "libc|parking_lot|crossbeam|^\\[(dev-)?dependencies\\]|^rand|^\\[lints"
    include:
    - Cargo.toml
    exclude:
    - third_party/**
    - .bin/**
    - .xyrag/**
    limit: 20
  reason: "Prüfen, ob libc als Abhängigkeit für die Nice-Herabstufung vorhanden ist."
```
/answer 4eac-c1 allow
Control Request:
```yaml
id: 4eac-c2
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-kernels/Cargo.toml
        matches:
        - lineno: 37
          match: "[dependencies]"
      - path: Cargo.toml
        matches:
        - lineno: 15
          match: "[dependencies]"
        - lineno: 36
          match: crossbeam = "0.8.5"
        - lineno: 37
          match: crossbeam-queue = "0.3.14"
        - lineno: 38
          match: parking_lot = "0.12.5"
        - lineno: 50
          match: "rand = { version = \"0.8\", optional = true }"
        - lineno: 59
          match: "[dev-dependencies]"
        - lineno: 65
          match: rand = "0.8"
```
/answer 4eac-c2 allow
Text:
Die Verträge aus B1/B3 sowie das Fehlerenum sind klar, Planner und Processor sind noch leere Stubs. Ich prüfe jetzt noch die Facade-Task-Typen und die Cargo.toml, bevor ich das Modul schreibe.
ReasoningToken: 1607
Control Request:
```yaml
id: 4eac-c3
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    pattern: TaskQueue|pub struct|pub enum|pub type|pub fn
    include:
    - task.rs
    - handle.rs
    - error.rs
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: .
    include:
    - Cargo.toml
    exclude:
    - third_party/**
    - .bin/**
    - .xyrag/**
    limit: 50
  reason: "Facade-Task-Typen (TaskQueue, Payload, TaskMeta) und vollständige Cargo.toml für die Edit-Position lesen."
```
/answer 4eac-c3 allow
Control Request:
```yaml
id: 4eac-c4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      matches:
      - path: error.rs
        matches:
        - lineno: 10
          match: "pub enum TaskError {"
      - path: task.rs
        matches:
        - lineno: 9
          match: "pub enum TaskKind {"
        - lineno: 16
          match: "pub struct TaskMeta {"
        - lineno: 23
          match: "pub enum Payload {"
        - lineno: 38
          match: "pub type TaskQueue = Queue<TaskMeta, Payload, Output, TaskError>;"
      - path: handle.rs
        matches:
        - lineno: 13
          match: "pub struct TaskHandle {"
        - lineno: 23
          match: "pub fn id(&self) -> u64 {"
        - lineno: 27
          match: "pub fn try_get(&self) -> Option<Result<Output, TaskError>> {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-kernels/Cargo.toml
        matches:
        - lineno: 1
          match: "# THIS FILE IS AUTOMATICALLY GENERATED BY CARGO"
        - lineno: 2
          match: "#"
        - lineno: 3
          match: "# When uploading crates to the registry Cargo will automatically"
        - lineno: 4
          match: "# \"normalize\" Cargo.toml files for maximal compatibility"
        - lineno: 5
          match: "# with all versions of Cargo and also rewrite `path` dependencies"
        - lineno: 6
          match: "# to registry (e.g., crates.io) dependencies."
        - lineno: 7
          match: "#"
        - lineno: 8
          match: "# If you are reading this file be aware that the original Cargo.toml"
        - lineno: 9
          match: "# will likely look very different (and much more reasonable)."
        - lineno: 10
          match: "# See Cargo.toml.orig for the original contents."
        - lineno: 12
          match: "[package]"
        - lineno: 13
          match: edition = "2021"
        - lineno: 14
          match: name = "candle-kernels"
        - lineno: 15
          match: version = "0.11.0"
        - lineno: 16
          match: build = "build.rs"
        - lineno: 17
          match: autolib = false
        - lineno: 18
          match: autobins = false
        - lineno: 19
          match: autoexamples = false
        - lineno: 20
          match: autotests = false
        - lineno: 21
          match: autobenches = false
        - lineno: 22
          match: description = "CUDA kernels for Candle"
        - lineno: 23
          match: readme = "README.md"
        - lineno: 24
          match: "keywords = ["
        - lineno: 25
          match: "\"blas\","
        - lineno: 26
          match: "\"tensor\","
        - lineno: 27
          match: "\"machine-learning\","
        - lineno: 28
          match: "]"
        - lineno: 29
          match: "categories = [\"science\"]"
        - lineno: 30
          match: license = "MIT OR Apache-2.0"
        - lineno: 31
          match: repository = "https://github.com/huggingface/candle"
        - lineno: 33
          match: "[lib]"
        - lineno: 34
          match: name = "candle_kernels"
        - lineno: 35
          match: path = "src/lib.rs"
        - lineno: 37
          match: "[dependencies]"
        - lineno: 39
          match: "[build-dependencies.cudaforge]"
        - lineno: 40
          match: version = "0.1.2"
      - path: .xyrag/layers/trigram/mirror/Cargo.toml
        matches:
        - lineno: 1
          match: package
        - lineno: 2
          match: name xy ai rag
        - lineno: 3
          match: version 0 1 0
        - lineno: 4
          match: edition 2021
        - lineno: 5
          match: description xy ai rag layered anytime retrieval engine
        - lineno: 7
          match: lib
        - lineno: 8
          match: name xy ai rag
        - lineno: 9
          match: path src lib rs
        - lineno: 11
          match: bin
        - lineno: 12
          match: name xyrag
        - lineno: 13
          match: path src bin xyrag rs
        - lineno: 15
          match: dependencies
        - lineno: 16
          match: tokio version 1 features rt multi thread macros sync time
        - lineno: 17
          match: tokio util 0 7
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4eac-c4 allow
Control Request:
```yaml
id: 4eac-c5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
    ids:
    - TaskKind
    - TaskMeta
    - Payload
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/error.rs
    ids:
    - TaskError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/state.rs
    ids:
    - TaskState
    - impl_TaskState_O_E_
  reason: TaskMeta/Payload/TaskError und TaskState-Complete-Semantik für Ergebniszustellung lesen.
```
/answer 4eac-c5 allow
ReasoningToken: 248
Control Request:
```yaml
id: 4eac-c6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
      nodes:
      - id: TaskKind
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          #[non_exhaustive]
          pub enum TaskKind {
              Embedding,
              Similarity,
              Inference,
          }
      - id: TaskMeta
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub struct TaskMeta {
              pub id: u64,
              pub kind: TaskKind,
          }
      - id: Payload
        type: ItemEnum
        code: |
          #[derive(Debug)]
          #[non_exhaustive]
          pub enum Payload {
              Embedding {
                  input: TextInput,
                  params: EmbeddingParams,
              },
              Similarity {
                  input: VectorInput,
                  params: SimilarityParams,
              },
              Inference {
                  input: PairInput,
                  params: InferenceParams,
              },
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/error.rs
      nodes:
      - id: TaskError
        type: ItemEnum
        code: |
          /// Cloneable so one outcome can be delivered to several waiters.
          #[derive(Error, Debug, Clone)]
          #[non_exhaustive]
          pub enum TaskError {
              #[error("task cancelled")]
              Cancelled,
              #[error("timed out waiting for task result")]
              Timeout,
              #[error("precision must not be NaN")]
              InvalidPrecision,
              #[error(transparent)]
              Engine(Arc<ResidentError>),
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/state.rs
      nodes:
      - id: TaskState
        type: ItemStruct
        code: |
          /// Per-task claim/result/wakeup cell. Every transition is a single
          /// `compare_exchange`; completion and cancellation notify all waiters.
          pub struct TaskState<O, E> {
              state: AtomicU8,
              result: OnceLock<Result<O, E>>,
              ready: Notify,
          }
      - id: impl_TaskState_O_E_
        type: ItemImpl
        signature: "impl<O, E> TaskState<O, E> {"
        children:
        - id: impl_TaskState_O_E_.new
          type: ImplItemFn
          code: |2
                pub fn new() -> Self {
                    Self {
                        state: AtomicU8::new(QUEUED),
                        result: OnceLock::new(),
                        ready: Notify::new(),
                    }
                }
        - id: impl_TaskState_O_E_.claim_state
          type: ImplItemFn
          code: |2
                pub fn claim_state(&self) -> ClaimState {
                    ClaimState::from(self.state.load(Ordering::Acquire))
                }
        - id: impl_TaskState_O_E_.claim
          type: ImplItemFn
          code: |2
                /// `QUEUED -> TAKEN`. The only transition where cancellation matters.
                pub(crate) fn claim(&self) -> bool {
                    let ok = self
                        .state
                        .compare_exchange(QUEUED, TAKEN, Ordering::AcqRel, Ordering::Acquire)
                        .is_ok();
                    tracing::debug!(ok, "claim");
                    ok
                }
        - id: impl_TaskState_O_E_.complete
          type: ImplItemFn
          code: |2
                /// Sets the result and transitions `TAKEN -> DONE`. Must be called at
                /// most once, by the claim winner only.
                pub fn complete(&self, out: Result<O, E>) {
                    if self.result.set(out).is_err() {
                        debug_assert!(false, "TaskState::result set more than once");
                        tracing::warn!("task result already set; ignoring duplicate completion");
                        return;
                    }
                    self.state.store(DONE, Ordering::Release);
                    self.ready.notify_waiters();
                    tracing::debug!("complete");
                }
        - id: impl_TaskState_O_E_.cancel
          type: ImplItemFn
          code: |2
                /// `QUEUED -> CANCELLED`. No-op if already `TAKEN`/`DONE`/`CANCELLED`.
                pub fn cancel(&self) -> bool {
                    let ok = self
                        .state
                        .compare_exchange(QUEUED, CANCELLED, Ordering::AcqRel, Ordering::Acquire)
                        .is_ok();
                    if ok {
                        self.ready.notify_waiters();
                    }
                    tracing::debug!(ok, "cancel");
                    ok
                }
        - id: impl_TaskState_O_E_.result
          type: ImplItemFn
          code: |2
                pub fn result(&self) -> Option<&Result<O, E>> {
                    self.result.get()
                }
        - id: impl_TaskState_O_E_.wait
          type: ImplItemFn
          code: |2
                /// Waits for `Done`/`Cancelled`. Registers interest before checking state
                /// so a completion landing between check and await is never missed.
                pub async fn wait(&self) {
                    loop {
                        let notified = self.ready.notified();
                        tokio::pin!(notified);
                        notified.as_mut().enable();
                        match self.claim_state() {
                            ClaimState::Done | ClaimState::Cancelled => return,
                            _ => notified.await,
                        }
                    }
                }
```
/answer 4eac-c6 allow
Control Request:
```yaml
id: 4eac-c7
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    pattern: "enum Output|Output \\{|^pub enum Output"
    include:
    - params.rs
    limit: 5
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^\\[(features|dev-dependencies|dependencies)\\]|^loom|^mockall|^candle-kernels|cfg\\(loom\\)|unexpected_cfgs"
    include:
    - Cargo.toml
    exclude:
    - third_party/**
    - .bin/**
    - .xyrag/**
    limit: 10
  reason: Output-Typ und Cargo-Abschnittsgrenzen für gezielte Edits finden.
```
/answer 4eac-c7 allow
Control Request:
```yaml
id: 4eac-c8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      matches:
      - path: params.rs
        matches:
        - lineno: 219
          match: "pub enum Output {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-kernels/Cargo.toml
        matches:
        - lineno: 37
          match: "[dependencies]"
      - path: Cargo.toml
        matches:
        - lineno: 15
          match: "[dependencies]"
        - lineno: 52
          match: "[features]"
        - lineno: 59
          match: "[dev-dependencies]"
        - lineno: 62
          match: mockall = "0.15.0"
        - lineno: 63
          match: loom = "0.7"
        - lineno: 73
          match: "candle-kernels = { path = \"third_party/candle-kernels\" }"
```
/answer 4eac-c8 allow
Control Request:
```yaml
id: 4eac-c9
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/lib.rs
    ids:
    - ModuleDoc
  reason: "Prüfen, ob lib.rs unsafe_code verbietet (relevant für setpriority-Aufruf)."
```
/answer 4eac-c9 allow
ReasoningToken: 6418
Control Request:
```yaml
id: 4eac-ca
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/lib.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! xy.ai.rag - Layered Anytime Retrieval Engine.
          //!
          //! Used both as a library (`xy_ai_rag::core::...`) and as an on-demand CLI
          //! utility (`xyrag`, see `src/bin/xyrag.rs`) - no daemon or server process.
```
/answer 4eac-ca allow
Control Request:
```yaml
id: 4eac-cb
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: "unsafe_code|forbid\\(|deny\\(|#!\\["
    include:
    - '*.rs'
    limit: 10
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: name = "libc"
    include:
    - Cargo.lock
    limit: 3
  reason: Crate-weite Lint-Attribute und Verfügbarkeit von libc im Lockfile klären.
```
/answer 4eac-cb allow
Control Request:
```yaml
id: 4eac-cc
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/mock.rs
        matches:
        - lineno: 8
          match: "#![cfg(any(test, feature = \"mock\"))]"
      - path: layers/trigram/trie.rs
        matches:
        - lineno: 11
          match: "#![forbid(unsafe_code)]"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: Cargo.lock
        matches:
        - lineno: 1895
          match: name = "libc"
```
/answer 4eac-cc allow
ReasoningToken: 103
Control Request:
```yaml
id: 4eac-cd
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
  reason: Node-IDs der Cargo.toml für das Hinzufügen der libc-Abhängigkeit ermitteln.
```
/answer 4eac-cd allow
Control Request:
```yaml
id: 4eac-ce
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
      nodes:
      - id: h8JWUc|XOXuKk
        signature: "[package]"
      - id: mN2I6g|SIE1cj
        signature: "[lib]"
      - id: cW2ehx|QePTHE
        signature: "[[bin]]"
      - id: uUBhmF|EGrpec
        signature: "[dependencies]"
      - id: adh0oF|NNwOkR
        signature: "[features]"
      - id: UeF6gB|FaChiw
        signature: "[dev-dependencies]"
      - id: 9OxNvg|OtfeRA
        signature: "[[bench]]"
      - id: uCHafc|L1C1dJ
        signature: "[patch.crates-io]"
```
/answer 4eac-ce allow
ReasoningToken: 384
Control Request:
```yaml
id: 4eac-cf
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    pattern: "^(pub )?(use|mod)|^    [A-Za-z, ]+,$|^\\};"
    include:
    - mod.rs
    limit: 15
  reason: "Re-Exports in op/mod.rs prüfen (ResourceKind, DeviceKind, Region-Importe)."
```
/answer 4eac-cf allow
Control Request:
```yaml
id: 4eac-d0
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      matches:
      - path: mod.rs
        matches:
        - lineno: 3
          match: mod lease;
        - lineno: 4
          match: mod requirements;
        - lineno: 6
          match: "use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};"
        - lineno: 7
          match: "use std::sync::{Arc, OnceLock};"
        - lineno: 9
          match: "pub use lease::{DeviceHandle, Lease, PinCount};"
        - lineno: 10
          match: "pub use requirements::{"
        - lineno: 11
          match: "Affinity, DeviceId, EngineSelector, IndexId, ModelId, Requirements, ResourceKind,"
        - lineno: 12
          match: "};"
        - lineno: 14
          match: use crate::resident::error::ResidentError;
        - lineno: 19
          match: "Tokenize,"
        - lineno: 20
          match: "Embed,"
        - lineno: 21
          match: "Pool,"
        - lineno: 23
          match: "Similarity,"
        - lineno: 24
          match: "TopK,"
        - lineno: 25
          match: "WriteIndex,"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4eac-d0 allow
Text:
 Jetzt die Änderungen an bestehenden Dateien: Cargo.toml, `TaskCtx::add_ops`, Top-Level-Fehlerenum.
Control Request:
```yaml
id: 4eac-d1
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    id: adh0oF|NNwOkR
    position: before
    source: |
      [target.'cfg(target_os = "linux")'.dependencies]
      libc = "0.2"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
    id: impl_TaskCtx.open_ops
    position: before
    source: |2+
          /// Registers `n` follow-up ops. Must run before they are enqueued and before the
          /// producing op is finished, so the counter never reaches zero early.
          pub fn add_ops(&self, n: usize) {
              self.open.fetch_add(n, Ordering::Relaxed);
          }

  reason: libc als Linux-Abhängigkeit ergänzen und TaskCtx um add_ops für das Folgeop-Zählprotokoll erweitern.
```
/answer 4eac-d1 allow
Control Request:
```yaml
id: 4eac-d2
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: ResidentError
    old_text: |2-
          Capability(#[from] CapabilityError),
      }
    new_text: |2-
          Capability(#[from] CapabilityError),
          #[error(transparent)]
          Orchestration(#[from] OrchestrationError),
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: impl_IsRetryable_for_ResidentError.is_retryable
    old_text: "            ResidentError::Capability(e) => e.is_retryable(),"
    new_text: |2-
                  ResidentError::Capability(e) => e.is_retryable(),
                  ResidentError::Orchestration(e) => e.is_retryable(),
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: MS3n2e|NqxAEG
    old_text: use thiserror::Error;
    new_text: |-
      use thiserror::Error;

      pub use crate::resident::processor::OrchestrationError;
  reason: "OrchestrationError per #[from] in das Top-Level-Fehlerenum aggregieren."
```
/answer 4eac-d2 allow
ReasoningToken: 3525
Control Request:
```yaml
id: 4eac-d3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/node.rs
    ids:
    - Node
    - impl_Node_P_D_O_E_.is_live
    - impl_Node_P_D_O_E_.claim
    - impl_Node_P_D_O_E_.params
    - Iter
    - impl_Iterator_for_Iter_P_D_O_E_.next
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/state.rs
    ids:
    - ClaimState
  reason: "Claim-/is_live-Semantik des Nodes prüfen, um Op-Nodes nach dem Claim korrekt abzuschließen."
```
/answer 4eac-d3 allow
Control Request:
```yaml
id: 4eac-d4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/node.rs
      nodes:
      - id: Node
        type: ItemStruct
        code: |
          pub struct Node<P, D, O, E> {
              content: Content<P, D, O, E>,
              next: ArcSwapOption<Node<P, D, O, E>>,
          }
      - id: impl_Node_P_D_O_E_.is_live
        type: ImplItemFn
        code: |2
              /// `false` once the task is `Done`/`Cancelled`; such nodes stay in the chain
              /// (append-only, reclaimed with the queue) but are skipped by live iteration.
              pub fn is_live(&self) -> bool {
                  match &self.content {
                      Content::Real { state, .. } => {
                          matches!(state.claim_state(), ClaimState::Queued | ClaimState::Taken)
                      }
                      Content::Sentinel => false,
                  }
              }
      - id: impl_Node_P_D_O_E_.claim
        type: ImplItemFn
        code: |2
              /// Attempts `QUEUED -> TAKEN`; on success takes the payload. Never blocks,
              /// never awaits under the payload lock.
              pub fn claim(&self) -> Option<D> {
                  match &self.content {
                      Content::Real { state, payload, .. } => {
                          if state.claim() {
                              payload.lock().take()
                          } else {
                              None
                          }
                      }
                      Content::Sentinel => None,
                  }
              }
      - id: impl_Node_P_D_O_E_.params
        type: ImplItemFn
        code: |2
              /// `None` for the sentinel; otherwise the immutable filter metadata.
              pub fn params(&self) -> Option<&P> {
                  match &self.content {
                      Content::Real { params, .. } => Some(params),
                      Content::Sentinel => None,
                  }
              }
      - id: Iter
        type: ItemStruct
        code: |
          /// Traverses live nodes (sentinel excluded) in age order. Holds one `Arc<Node>`
          /// per step so a concurrently unlinked node stays valid; never blocks.
          pub struct Iter<P, D, O, E> {
              next: Option<Arc<Node<P, D, O, E>>>,
          }
      - id: impl_Iterator_for_Iter_P_D_O_E_.next
        type: ImplItemFn
        code: |2
              fn next(&mut self) -> Option<Self::Item> {
                  loop {
                      let cur = self.next.take()?;
                      self.next = cur.peek_next();
                      if cur.params().is_some() {
                          return Some(cur);
                      }
                      // Only the initial sentinel hits this branch; loop once more.
                  }
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/state.rs
      nodes:
      - id: ClaimState
        type: ItemEnum
        code: |
          /// Snapshot of a [`TaskState`]'s atomic state.
          #[derive(Debug, Clone, Copy, PartialEq, Eq)]
          pub enum ClaimState {
              Queued,
              Taken,
              Done,
              Cancelled,
          }
```
/answer 4eac-d4 allow
ReasoningToken: 2149
Control Request:
```yaml
id: 4eac-d5
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: "TaskQueue|Queue::new|processor::"
    include:
    - '*.rs'
    limit: 20
  reason: "Klären, wer die Task-Queue erzeugt und ob der Processor bereits referenziert wird."
```
/answer 4eac-d5 allow
Control Request:
```yaml
id: 4eac-d6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/facade/mod.rs
        matches:
        - lineno: 20
          match: "pub use task::{Payload, TaskKind, TaskMeta, TaskQueue};"
      - path: resident/facade/resident.rs
        matches:
        - lineno: 11
          match: "use super::task::{Payload, TaskKind, TaskMeta, TaskQueue};"
        - lineno: 104
          match: "queue: Arc::new(Queue::new(Vec::new())),"
        - lineno: 133
          match: "queue: Arc<TaskQueue>,"
        - lineno: 148
          match: "pub fn queue(&self) -> &Arc<TaskQueue> {"
      - path: resident/facade/task.rs
        matches:
        - lineno: 38
          match: "pub type TaskQueue = Queue<TaskMeta, Payload, Output, TaskError>;"
      - path: resident/queue/queue.rs
        matches:
        - lineno: 83
          match: "let q: Queue<&str, u32, u32, ()> = Queue::new(vec![]);"
        - lineno: 90
          match: "let q: Queue<i32, i32, u32, ()> = Queue::new(vec![]);"
        - lineno: 100
          match: "let q: Queue<(), (), u32, ()> = Queue::new(vec![relay.clone()]);"
        - lineno: 108
          match: "let q: Queue<u32, u32, u32, ()> = Queue::new(vec![]);"
        - lineno: 117
          match: "let q: Queue<i32, i32, u32, ()> = Queue::new(vec![]);"
        - lineno: 131
          match: "let q: Queue<i32, i32, u32, ()> = Queue::new(vec![]);"
        - lineno: 143
          match: "let q: Arc<Queue<u32, u32, u32, ()>> = Arc::new(Queue::new(vec![]));"
        - lineno: 170
          match: "let q: Queue<i32, (), u32, ()> = Queue::new(vec![]);"
      - path: resident/error.rs
        matches:
        - lineno: 7
          match: pub use crate::resident::processor::OrchestrationError;
```
/answer 4eac-d6 allow
ReasoningToken: 4619
Control Request:
```yaml
id: 4eac-d7
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
    ids:
    - Output
    - TextInput
    - EmbeddingParams
  reason: Output-/Input-Typen für Processor-Integrationstest und Collect-Signatur prüfen.
```
/answer 4eac-d7 allow
Control Request:
```yaml
id: 4eac-d8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
      nodes:
      - id: Output
        type: ItemEnum
        code: |
          #[derive(Clone, Debug, PartialEq)]
          #[non_exhaustive]
          pub enum Output {
              Embeddings(Vec<Vec<f32>>),
              Hits(Vec<(EntryId, f32)>),
              Scores(Vec<f32>),
          }
      - id: TextInput
        type: ItemStruct
        code: |
          #[derive(Clone, Debug, Default)]
          #[non_exhaustive]
          pub struct TextInput {
              texts: Vec<String>,
          }
      - id: EmbeddingParams
        type: ItemStruct
        code: |
          #[derive(Clone, Debug)]
          #[non_exhaustive]
          pub struct EmbeddingParams {
              role: TextRole,
              write_index: Option<IndexId>,
              precision: f32,
          }
```
/answer 4eac-d8 allow
ReasoningToken: 82
Control Request:
```yaml
id: 4eac-d9
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    pattern: pub fn (new|default|embedding|from)|impl (EmbeddingParams|TextInput)|impl Default for EmbeddingParams|pub (async )?fn (embed|submit)
    include:
    - params.rs
    - resident.rs
    limit: 15
  reason: "Konstruktoren für Payload-Bestandteile finden, um im Test Tasks einzureihen."
```
/answer 4eac-d9 allow
Control Request:
```yaml
id: 4eac-da
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      matches:
      - path: resident.rs
        matches:
        - lineno: 26
          match: "pub fn new(cpu: usize, gpu: usize) -> Self {"
        - lineno: 67
          match: "pub fn new() -> Self {"
        - lineno: 177
          match: "pub fn submit_embedding(&self, input: TextInput, params: EmbeddingParams) -> TaskHandle {"
        - lineno: 186
          match: "pub fn submit_similarity(&self, input: VectorInput, params: SimilarityParams) -> TaskHandle {"
        - lineno: 195
          match: "pub fn submit_inference(&self, input: PairInput, params: InferenceParams) -> TaskHandle {"
      - path: params.rs
        matches:
        - lineno: 21
          match: "impl TextInput {"
        - lineno: 22
          match: "pub fn new(texts: impl IntoIterator<Item = impl Into<String>>) -> Self {"
        - lineno: 46
          match: "pub fn new(vectors: Vec<Vec<f32>>) -> Self {"
        - lineno: 64
          match: "pub fn new(query: impl Into<String>, chunk: impl Into<String>) -> Self {"
        - lineno: 88
          match: "impl EmbeddingParams {"
        - lineno: 89
          match: "pub fn new(role: TextRole) -> Self {"
        - lineno: 120
          match: "impl Default for EmbeddingParams {"
        - lineno: 136
          match: "pub fn new(index: IndexId) -> Self {"
        - lineno: 185
          match: "pub fn new() -> Self {"
```
/answer 4eac-da allow
ReasoningToken: 300
Control Request:
```yaml
id: 4eac-db
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/error.rs
    source: |
      use std::sync::Arc;

      use thiserror::Error;

      use crate::resident::error::{IsRetryable, ResidentError};
      use crate::resident::op::{EngineSelector, ResourceKind};

      #[derive(Error, Debug)]
      #[non_exhaustive]
      pub enum OrchestrationError {
          #[error("task {task} cancelled")]
          Cancelled { task: u64 },
          #[error("resource {resource:?} is backing off")]
          Backoff { resource: ResourceKind },
          #[error("no workers configured for {engine:?} operations")]
          EngineUnavailable { engine: EngineSelector },
          #[error("lane {lane} lost its completion signal")]
          CompletionLost { lane: u32 },
          #[error("operation of task {task} failed")]
          OpFailed {
              task: u64,
              #[source]
              source: Arc<ResidentError>,
          },
          #[error("processor is shutting down")]
          Shutdown,
      }

      impl IsRetryable for OrchestrationError {
          fn is_retryable(&self) -> bool {
              match self {
                  Self::Backoff { .. } => true,
                  Self::OpFailed { source, .. } => source.is_retryable(),
                  _ => false,
              }
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use crate::resident::error::IndexError;

          #[test]
          fn retryability_follows_cause() {
              assert!(OrchestrationError::Backoff { resource: ResourceKind::Encoder }.is_retryable());
              assert!(!OrchestrationError::Cancelled { task: 1 }.is_retryable());
              let transient: ResidentError = IndexError::ArenaExhausted { requested: 1, available: 0 }.into();
              let e = OrchestrationError::OpFailed { task: 1, source: Arc::new(transient) };
              assert!(e.is_retryable());
          }

          #[test]
          fn converts_into_resident_error() {
              let e: ResidentError = OrchestrationError::Shutdown.into();
              assert!(matches!(e, ResidentError::Orchestration(OrchestrationError::Shutdown)));
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/config.rs
    source: |
      use std::time::Duration;

      use crate::resident::facade::WorkerConfig;

      #[derive(Clone, Debug, PartialEq, Eq)]
      #[non_exhaustive]
      pub struct ProcessorConfig {
          pub cpu_workers: usize,
          pub gpu_workers: usize,
          /// Lower waterline of queued operations before the planner is asked for more.
          pub min_op_buffer: usize,
          /// Nice value for CPU workers while GPU workers exist; `0` disables.
          pub cpu_nice: i32,
          pub starved_ttl: Duration,
      }

      impl ProcessorConfig {
          pub fn new(cpu_workers: usize, gpu_workers: usize) -> Self {
              Self {
                  cpu_workers,
                  gpu_workers,
                  min_op_buffer: 2,
                  cpu_nice: 5,
                  starved_ttl: Duration::from_millis(200),
              }
          }

          pub fn with_min_op_buffer(mut self, n: usize) -> Self {
              self.min_op_buffer = n;
              self
          }

          pub fn with_cpu_nice(mut self, nice: i32) -> Self {
              self.cpu_nice = nice;
              self
          }

          pub fn with_starved_ttl(mut self, ttl: Duration) -> Self {
              self.starved_ttl = ttl;
              self
          }
      }

      impl Default for ProcessorConfig {
          fn default() -> Self {
              Self::new(1, 0)
          }
      }

      impl From<WorkerConfig> for ProcessorConfig {
          fn from(w: WorkerConfig) -> Self {
              Self::new(w.cpu, w.gpu)
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/completion.rs
    source: |
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
              self.states.iter().enumerate().find_map(|(i, s)| {
                  s.compare_exchange(IDLE, RESERVED, Ordering::AcqRel, Ordering::Relaxed)
                      .is_ok()
                      .then_some(LaneId(i as u32))
              })
          }

          /// Returns a reserved lane that was never launched.
          pub fn abort(&self, lane: LaneId) {
              let _ = self.states[lane.0 as usize].compare_exchange(
                  RESERVED,
                  IDLE,
                  Ordering::AcqRel,
                  Ordering::Relaxed,
              );
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
          pub fn reap(&self, completion: &dyn Completion, mut on_done: impl FnMut(LaneId, LaneOutcome)) -> usize {
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
                      .compare_exchange(LAUNCHED, COMPLETING, Ordering::AcqRel, Ordering::Acquire)
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
              Self { cells: (0..lanes).map(|_| AtomicU8::new(0)).collect() }
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
      mod tests {
          use std::sync::atomic::AtomicUsize;
          use std::sync::Barrier;

          use super::*;

          fn board(lanes: usize) -> (Arc<LaneBoard>, Arc<ManualCompletion>) {
              (
                  Arc::new(LaneBoard::new(lanes, Arc::new(Notify::new()))),
                  Arc::new(ManualCompletion::new(lanes)),
              )
          }

          #[test]
          fn missed_callback_is_picked_up_by_poll() {
              let (b, c) = board(1);
              let lane = b.reserve().unwrap();
              b.launch(lane, &*c);
              assert_eq!(b.reap(&*c, |_, _| panic!("not finished")), 0);
              c.finish(lane, LaneOutcome::Success);
              let mut seen = vec![];
              assert_eq!(b.reap(&*c, |l, o| seen.push((l, o))), 1);
              assert_eq!(seen, vec![(lane, LaneOutcome::Success)]);
              assert!(b.is_idle(lane));
          }

          #[test]
          fn callback_without_poll_completes() {
              let (b, c) = board(1);
              let lane = b.reserve().unwrap();
              b.launch(lane, &*c);
              b.signal(lane);
              assert_eq!(b.reap(&*c, |_, o| assert_eq!(o, LaneOutcome::Success)), 1);
          }

          #[test]
          fn failure_is_reported_by_poll() {
              let (b, c) = board(1);
              let lane = b.reserve().unwrap();
              b.launch(lane, &*c);
              c.finish(lane, LaneOutcome::Failed);
              assert_eq!(b.reap(&*c, |_, o| assert_eq!(o, LaneOutcome::Failed)), 1);
          }

          #[test]
          fn signal_before_launch_completes_is_not_lost() {
              let (b, c) = board(1);
              let lane = b.reserve().unwrap();
              b.launch(lane, &*c);
              b.signal(lane);
              b.signal(lane);
              assert_eq!(b.reap(&*c, |_, _| {}), 1);
              assert_eq!(b.reap(&*c, |_, _| {}), 0);
          }

          #[test]
          fn reserved_lane_is_not_reaped_and_can_abort() {
              let (b, c) = board(1);
              let lane = b.reserve().unwrap();
              assert!(b.reserve().is_none());
              b.signal(lane);
              assert_eq!(b.reap(&*c, |_, _| {}), 0);
              b.abort(lane);
              assert!(b.is_idle(lane));
          }

          #[test]
          fn concurrent_callback_and_pollers_process_once() {
              for _ in 0..200 {
                  let (b, c) = board(1);
                  let lane = b.reserve().unwrap();
                  b.launch(lane, &*c);
                  c.finish(lane, LaneOutcome::Success);
                  let count = Arc::new(AtomicUsize::new(0));
                  let gate = Arc::new(Barrier::new(4));
                  let mut threads = vec![];
                  for t in 0..4 {
                      let (b, c, count, gate) = (b.clone(), c.clone(), count.clone(), gate.clone());
                      threads.push(std::thread::spawn(move || {
                          gate.wait();
                          if t == 0 {
                              b.signal(lane);
                          }
                          b.reap(&*c, |_, _| {
                              count.fetch_add(1, Ordering::SeqCst);
                          });
                      }));
                  }
                  for t in threads {
                      t.join().unwrap();
                  }
                  b.reap(&*c, |_, _| {
                      count.fetch_add(1, Ordering::SeqCst);
                  });
                  assert_eq!(count.load(Ordering::SeqCst), 1);
              }
          }
      }
  reason: "Neue Processor-Dateien anlegen: Fehlertyp, Konfiguration und die Lane-Completion-Mechanik mit Mock und Tests."
```
/answer 4eac-db allow
Text:
 Zwischenstand: Fehlertyp, Konfiguration und Lane-Completion (mit Mock und Race-Tests) sind angelegt. Als Nächstes kommen Starved-Set, Ressourcen-Zustandsautomat, Queues und der Worker-Kern.
Control Request:
```yaml
id: 4eac-dc
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/starved.rs
    source: |
      use std::sync::atomic::{AtomicU64, Ordering};
      use std::time::{Duration, Instant};

      use crate::resident::op::ResourceKind;

      const KINDS: usize = 6;

      fn slot(kind: ResourceKind) -> usize {
          match kind {
              ResourceKind::Tokenizer => 0,
              ResourceKind::Normalizer => 1,
              ResourceKind::Encoder => 2,
              ResourceKind::Scorer => 3,
              ResourceKind::Index => 4,
              ResourceKind::CrossEncoder => 5,
          }
      }

      #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
      pub struct StarvedSnapshot(u32);

      impl StarvedSnapshot {
          pub fn contains(self, kind: ResourceKind) -> bool {
              self.0 & (1 << slot(kind)) != 0
          }

          pub fn is_empty(self) -> bool {
              self.0 == 0
          }
      }

      /// Decaying per-kind demand hints; a hint is cleared by a pick or by occupancy and
      /// expires after `ttl`.
      pub struct StarvedSet {
          marks: [AtomicU64; KINDS],
          ttl_ms: u64,
          origin: Instant,
      }

      impl StarvedSet {
          pub fn new(ttl: Duration) -> Self {
              Self {
                  marks: std::array::from_fn(|_| AtomicU64::new(0)),
                  ttl_ms: ttl.as_millis() as u64,
                  origin: Instant::now(),
              }
          }

          fn now_ms(&self) -> u64 {
              self.origin.elapsed().as_millis() as u64
          }

          pub fn mark(&self, kind: ResourceKind) {
              self.mark_at(kind, self.now_ms());
          }

          pub fn clear(&self, kind: ResourceKind) {
              self.marks[slot(kind)].store(0, Ordering::Release);
          }

          pub fn clear_all(&self, snapshot: StarvedSnapshot) {
              for (i, mark) in self.marks.iter().enumerate() {
                  if snapshot.0 & (1 << i) != 0 {
                      mark.store(0, Ordering::Release);
                  }
              }
          }

          pub fn snapshot(&self) -> StarvedSnapshot {
              self.snapshot_at(self.now_ms())
          }

          pub(crate) fn mark_at(&self, kind: ResourceKind, now_ms: u64) {
              self.marks[slot(kind)].store(now_ms + 1, Ordering::Release);
          }

          pub(crate) fn snapshot_at(&self, now_ms: u64) -> StarvedSnapshot {
              let mut bits = 0;
              for (i, mark) in self.marks.iter().enumerate() {
                  let m = mark.load(Ordering::Acquire);
                  if m != 0 && now_ms.saturating_sub(m - 1) <= self.ttl_ms {
                      bits |= 1 << i;
                  }
              }
              StarvedSnapshot(bits)
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          fn set() -> StarvedSet {
              StarvedSet::new(Duration::from_millis(100))
          }

          #[test]
          fn mark_is_visible_until_ttl() {
              let s = set();
              s.mark_at(ResourceKind::Encoder, 10);
              assert!(s.snapshot_at(50).contains(ResourceKind::Encoder));
              assert!(!s.snapshot_at(50).contains(ResourceKind::Scorer));
              assert!(s.snapshot_at(111).is_empty());
          }

          #[test]
          fn pick_clears_only_its_kind() {
              let s = set();
              s.mark_at(ResourceKind::Encoder, 0);
              s.mark_at(ResourceKind::Scorer, 0);
              s.clear(ResourceKind::Encoder);
              let snap = s.snapshot_at(1);
              assert!(!snap.contains(ResourceKind::Encoder));
              assert!(snap.contains(ResourceKind::Scorer));
          }

          #[test]
          fn remark_extends_lifetime() {
              let s = set();
              s.mark_at(ResourceKind::Index, 0);
              s.mark_at(ResourceKind::Index, 90);
              assert!(s.snapshot_at(150).contains(ResourceKind::Index));
          }

          #[test]
          fn clear_all_consumes_snapshot() {
              let s = set();
              s.mark_at(ResourceKind::Encoder, 0);
              s.mark_at(ResourceKind::Tokenizer, 0);
              let snap = s.snapshot_at(1);
              s.clear_all(snap);
              assert!(s.snapshot_at(1).is_empty());
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
    source: |
      use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering};
      use std::sync::Arc;

      use crate::resident::op::PinCount;

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum ResourceState {
          Uninit,
          Loading,
          Ready,
          Backoff,
          Evicting,
      }

      impl ResourceState {
          fn code(self) -> u8 {
              self as u8
          }

          fn from_code(v: u8) -> Self {
              match v {
                  0 => Self::Uninit,
                  1 => Self::Loading,
                  2 => Self::Ready,
                  3 => Self::Backoff,
                  _ => Self::Evicting,
              }
          }
      }

      /// Lifecycle cell of one lazily initialised resource. Time is passed in by the caller.
      pub struct ResourceSlot {
          state: AtomicU8,
          in_flight: AtomicUsize,
          pins: Arc<PinCount>,
          last_used_ms: AtomicU64,
          retry_at_ms: AtomicU64,
      }

      impl ResourceSlot {
          pub fn new(pins: Arc<PinCount>) -> Self {
              Self {
                  state: AtomicU8::new(ResourceState::Uninit.code()),
                  in_flight: AtomicUsize::new(0),
                  pins,
                  last_used_ms: AtomicU64::new(0),
                  retry_at_ms: AtomicU64::new(0),
              }
          }

          pub fn state(&self) -> ResourceState {
              ResourceState::from_code(self.state.load(Ordering::Acquire))
          }

          fn transition(&self, from: ResourceState, to: ResourceState) -> bool {
              self.state
                  .compare_exchange(from.code(), to.code(), Ordering::SeqCst, Ordering::SeqCst)
                  .is_ok()
          }

          /// Only the caller that wins this transition performs the load.
          pub fn begin_load(&self, now_ms: u64) -> bool {
              if self.transition(ResourceState::Uninit, ResourceState::Loading) {
                  return true;
              }
              self.retry_at_ms.load(Ordering::Acquire) <= now_ms
                  && self.transition(ResourceState::Backoff, ResourceState::Loading)
          }

          pub fn finish_load(&self, ok: bool, now_ms: u64, backoff_ms: u64) {
              self.last_used_ms.store(now_ms, Ordering::Release);
              if ok {
                  self.state.store(ResourceState::Ready.code(), Ordering::SeqCst);
              } else {
                  self.retry_at_ms.store(now_ms + backoff_ms, Ordering::Release);
                  self.state.store(ResourceState::Backoff.code(), Ordering::SeqCst);
              }
          }

          /// Registers an operation in flight; fails unless the resource is `Ready`.
          pub fn enter(&self) -> bool {
              self.in_flight.fetch_add(1, Ordering::SeqCst);
              if self.state() == ResourceState::Ready {
                  return true;
              }
              self.in_flight.fetch_sub(1, Ordering::SeqCst);
              false
          }

          pub fn leave(&self, now_ms: u64) {
              self.last_used_ms.store(now_ms, Ordering::Release);
              let prev = self.in_flight.fetch_sub(1, Ordering::SeqCst);
              debug_assert!(prev > 0, "leave without enter");
          }

          pub fn in_flight(&self) -> usize {
              self.in_flight.load(Ordering::Acquire)
          }

          /// `Ready -> Evicting` only when idle, unleased and past the TTL.
          pub fn try_evict(&self, now_ms: u64, ttl_ms: u64) -> bool {
              let idle_for = now_ms.saturating_sub(self.last_used_ms.load(Ordering::Acquire));
              if idle_for < ttl_ms || !self.transition(ResourceState::Ready, ResourceState::Evicting) {
                  return false;
              }
              if self.in_flight.load(Ordering::SeqCst) == 0 && self.pins.is_unpinned() {
                  return true;
              }
              self.state.store(ResourceState::Ready.code(), Ordering::SeqCst);
              false
          }

          pub fn finish_evict(&self) {
              debug_assert_eq!(self.state(), ResourceState::Evicting);
              self.state.store(ResourceState::Uninit.code(), Ordering::SeqCst);
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use crate::resident::memory::Region;
          use crate::resident::op::Lease;

          fn slot() -> (ResourceSlot, Arc<PinCount>) {
              let pins = PinCount::new();
              (ResourceSlot::new(pins.clone()), pins)
          }

          fn ready(s: &ResourceSlot) {
              assert!(s.begin_load(0));
              s.finish_load(true, 0, 0);
          }

          #[test]
          fn single_loader_wins() {
              let (s, _) = slot();
              assert!(s.begin_load(0));
              assert!(!s.begin_load(0));
              assert_eq!(s.state(), ResourceState::Loading);
          }

          #[test]
          fn failed_load_backs_off_until_deadline() {
              let (s, _) = slot();
              assert!(s.begin_load(0));
              s.finish_load(false, 10, 100);
              assert_eq!(s.state(), ResourceState::Backoff);
              assert!(!s.enter());
              assert!(!s.begin_load(50));
              assert!(s.begin_load(110));
          }

          #[test]
          fn enter_requires_ready() {
              let (s, _) = slot();
              assert!(!s.enter());
              ready(&s);
              assert!(s.enter());
              assert_eq!(s.in_flight(), 1);
              s.leave(5);
              assert_eq!(s.in_flight(), 0);
          }

          #[test]
          fn evict_waits_for_ttl_and_in_flight() {
              let (s, _) = slot();
              ready(&s);
              assert!(!s.try_evict(50, 100));
              assert!(s.enter());
              assert!(!s.try_evict(500, 100));
              assert_eq!(s.state(), ResourceState::Ready);
              s.leave(500);
              assert!(!s.try_evict(550, 100));
              assert!(s.try_evict(600, 100));
              assert!(!s.enter());
              s.finish_evict();
              assert_eq!(s.state(), ResourceState::Uninit);
          }

          #[test]
          fn open_lease_blocks_eviction() {
              let (s, pins) = slot();
              ready(&s);
              let lease = Lease::new(Region::default(), pins, None);
              assert!(!s.try_evict(1_000, 10));
              drop(lease);
              assert!(s.try_evict(1_000, 10));
          }
      }
  reason: Starved-Set mit Verfall und den Ressourcen-Zustandsautomaten (Lazy-Init/Backoff/Eviction) anlegen.
```
/answer 4eac-dc allow
ReasoningToken: 183
Control Request:
```yaml
id: 4eac-dd
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    pattern: pub struct Region|derive.*\n?|impl Default for Region|pub use
    include:
    - mod.rs
    - region.rs
    limit: 15
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    pattern: Region
    include:
    - lease.rs
    limit: 8
  reason: "Prüfen, ob Region Default implementiert und wie sie im Lease-Test konstruiert wird (resource.rs-Test)."
```
/answer 4eac-dd allow
Control Request:
```yaml
id: 4eac-de
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      matches:
      - path: mod.rs
        matches:
        - lineno: 9
          match: "pub use dtype::{DType, UnsupportedDType};"
        - lineno: 10
          match: pub use freelist::FreeList;
        - lineno: 11
          match: pub use layout::Layout;
        - lineno: 12
          match: "pub use manager::{compute_dtype, BookkeepingArena, DeviceKind, MemoryManager};"
        - lineno: 13
          match: "pub use region::{ArenaId, Region};"
      - path: region.rs
        matches:
        - lineno: 3
          match: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
        - lineno: 6
          match: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
        - lineno: 7
          match: "pub struct Region {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      matches:
      - path: lease.rs
        matches:
        - lineno: 8
          match: use crate::resident::memory::Region;
        - lineno: 29
          match: "region: Region,"
        - lineno: 31
          match: "release: Option<UnboundedSender<Region>>,"
        - lineno: 38
          match: "pub fn new(region: Region, pin: Arc<PinCount>, release: Option<UnboundedSender<Region>>) -> Self {"
        - lineno: 43
          match: "pub fn region(&self) -> Region {"
        - lineno: 62
          match: "fn region() -> Region {"
        - lineno: 63
          match: "Region { arena: ArenaId(0), offset: 0, len: 16, gen: 1 }"
```
/answer 4eac-de allow
Text:
 No `Default` für `Region` — ich passe den Test an die in `lease.rs` verwendete Konstruktion an.
Control Request:
```yaml
id: 4eac-df
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
    id: tests.open_lease_blocks_eviction
    old_text: "Lease::new(Region::default(), pins, None);"
    new_text: "Lease::new(Region { arena: ArenaId(0), offset: 0, len: 16, gen: 1 }, pins, None);"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
    id: tests.7YB2Yw|bqr4NJ
    old_text: use crate::resident::memory::Region;
    new_text: "use crate::resident::memory::{ArenaId, Region};"
  reason: Region-Konstruktion im Lease-Test korrigieren (kein Default vorhanden).
```
/answer 4eac-df allow
Control Request:
```yaml
id: 4eac-e0
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
      result: success
      id: tests.open_lease_blocks_eviction
      new_id: tests.open_lease_blocks_eviction
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
      error: No node matched the selector.
      id: tests.7YB2Yw|bqr4NJ
```
/answer 4eac-e0 allow
Control Request:
```yaml
id: 4eac-e1
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
  text: memory::Region
  reason: Node-ID der use-Zeile im Test-Modul finden.
```
/answer 4eac-e1 allow
Control Request:
```yaml
id: 4eac-e2
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
      nodes:
      - id: tests.mwhpHE|Tt8h2F
        code: |2
              use super::*;
              use crate::resident::memory::Region;
              use crate::resident::op::Lease;
```
/answer 4eac-e2 allow
Control Request:
```yaml
id: 4eac-e3
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
    id: tests.mwhpHE|Tt8h2F
    old_text: use crate::resident::memory::Region;
    new_text: "use crate::resident::memory::{ArenaId, Region};"
  reason: Import von ArenaId im Test-Modul ergänzen.
```
/answer 4eac-e3 allow
ReasoningToken: 305
Control Request:
```yaml
id: 4eac-e4
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: use .*(Queue|TaskState|queue::)|pub use|Output
    include:
    - task.rs
    - mod.rs
    exclude:
    - precision/*
    - kernel/*
    limit: 25
  reason: Re-Exports für Queue/TaskState und Output (facade) bestätigen.
```
/answer 4eac-e4 allow
Control Request:
```yaml
id: 4eac-e5
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: precision/mod.rs
        matches:
        - lineno: 9
          match: "pub use axis::{AxisId, ValueId};"
        - lineno: 10
          match: pub use capability::CapabilitySet;
        - lineno: 11
          match: "pub use pareto::{pareto, positions};"
        - lineno: 12
          match: "pub use profile::{Profile, ProfileConfig};"
      - path: facade/mod.rs
        matches:
        - lineno: 10
          match: pub use blocking::ResidentBlocking;
        - lineno: 11
          match: pub use error::TaskError;
        - lineno: 12
          match: pub use handle::TaskHandle;
        - lineno: 13
          match: "pub use params::{"
        - lineno: 14
          match: "EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, TextRole,"
        - lineno: 17
          match: "pub use resident::{"
        - lineno: 20
          match: "pub use task::{Payload, TaskKind, TaskMeta, TaskQueue};"
      - path: facade/task.rs
        matches:
        - lineno: 3
          match: "EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, VectorInput,"
        - lineno: 5
          match: use crate::resident::queue::Queue;
        - lineno: 38
          match: "pub type TaskQueue = Queue<TaskMeta, Payload, Output, TaskError>;"
      - path: index/mod.rs
        matches:
        - lineno: 17
          match: "pub use stamp::{content_hash, FileStamp, StampCounter, Verdict, RACY_WINDOW_NS};"
      - path: queue/mod.rs
        matches:
        - lineno: 19
          match: "pub use node::{Iter, Node};"
        - lineno: 20
          match: pub use queue::Queue;
        - lineno: 21
          match: "pub use state::{ClaimState, TaskState};"
      - path: queue/task.rs
        matches:
        - lineno: 8
          match: //! Op-queues (CPU/GPU/shared) reuse the same `Queue`/`Node` with their own
      - path: kernel/mod.rs
        matches:
        - lineno: 69
          match: "pub enum KernelOutput {"
        - lineno: 77
          match: "fn run(&self, batch: BatchId, input: &[RegionSpan]) -> Result<KernelOutput, IndexError>;"
        - lineno: 144
          match: "assert!(matches!(&a.0, KernelOutput::Hits(h) if h.len() == 5));"
        - lineno: 153
          match: "assert_eq!(k.run(b, &input).unwrap(), KernelOutput::Partial);"
        - lineno: 157
          match: "assert!(matches!(k.run(b, &rest).unwrap(), KernelOutput::Hits(_)));"
      - path: subengine/mod.rs
        matches:
        - lineno: 12
          match: pub use device::*;
      warning: "Limit of 25 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4eac-e5 allow
ReasoningToken: 72
Control Request:
```yaml
id: 4eac-e6
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/ops.rs
    source: |
      use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
      use std::sync::Arc;

      use parking_lot::Mutex;
      use tokio::sync::Notify;

      use crate::resident::error::ResidentError;
      use crate::resident::facade::{Output, TaskError};
      use crate::resident::op::{EngineSelector, OpParams, OperationNode, TaskCtx};
      use crate::resident::queue::{Queue, TaskState};

      use super::error::OrchestrationError;

      pub type Collect = Box<dyn FnOnce() -> Result<Output, TaskError> + Send>;

      /// Per-task completion bookkeeping shared by all of the task's operations.
      pub struct TaskRun {
          ctx: Arc<TaskCtx>,
          state: Arc<TaskState<Output, TaskError>>,
          collect: Mutex<Option<Collect>>,
          completed: AtomicBool,
      }

      impl TaskRun {
          pub fn new(ctx: Arc<TaskCtx>, state: Arc<TaskState<Output, TaskError>>, collect: Collect) -> Arc<Self> {
              Arc::new(Self { ctx, state, collect: Mutex::new(Some(collect)), completed: AtomicBool::new(false) })
          }

          pub fn ctx(&self) -> &Arc<TaskCtx> {
              &self.ctx
          }

          pub fn is_cancelled(&self) -> bool {
              self.ctx.is_cancelled()
          }

          fn complete_once(&self, result: impl FnOnce() -> Result<Output, TaskError>) {
              if !self.completed.swap(true, Ordering::AcqRel) {
                  self.state.complete(result());
              }
          }

          /// Completes the task with its collected result.
          pub fn finish(&self) {
              let collect = self.collect.lock().take();
              self.complete_once(|| match collect {
                  Some(collect) => collect(),
                  None => Err(TaskError::Cancelled),
              });
          }

          /// Closes one operation; the last one of a healthy task completes it.
          pub fn op_done(&self) {
              if self.ctx.finish_op() {
                  self.finish();
              }
          }

          /// Closes an operation that is dropped unexecuted because the task already failed.
          pub fn skip(&self) {
              let _ = self.ctx.finish_op();
          }

          /// Records the first error and completes the task with it. Does not touch the op counter.
          pub fn fail(&self, err: ResidentError) {
              let err = Arc::new(err);
              let wrapped = OrchestrationError::OpFailed { task: self.ctx.id(), source: err.clone() };
              if self.ctx.fail(wrapped.into()) {
                  self.complete_once(|| Err(TaskError::Engine(err)));
              }
          }
      }

      pub struct OpJob {
          pub node: Arc<OperationNode>,
          pub run: Arc<TaskRun>,
      }

      impl OpJob {
          pub fn new(node: Arc<OperationNode>, run: Arc<TaskRun>) -> Self {
              Self { node, run }
          }

          pub fn params(&self) -> &OpParams {
              self.node.op.params()
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum Side {
          Cpu,
          Gpu,
      }

      pub type OpQueue = Queue<OpParams, OpJob, (), ()>;

      const SHARED_SKIP_LIMIT: usize = 8;

      struct Counted {
          queue: OpQueue,
          queued: AtomicUsize,
      }

      impl Counted {
          fn new(relays: Vec<Arc<Notify>>) -> Self {
              Self { queue: Queue::new(relays), queued: AtomicUsize::new(0) }
          }

          fn push(&self, job: OpJob) {
              let params = job.params().clone();
              self.queued.fetch_add(1, Ordering::AcqRel);
              self.queue.push(params, Arc::new(TaskState::new()), job);
          }

          /// Claims the oldest acceptable job, skipping at most `max_skip` foreign entries.
          fn claim(&self, accepts: &dyn Fn(&OpParams) -> bool, max_skip: usize) -> Option<OpJob> {
              let mut skipped = 0;
              for node in self.queue.iter_live() {
                  let Some(params) = node.params() else { continue };
                  if !accepts(params) {
                      skipped += 1;
                      if skipped >= max_skip {
                          break;
                      }
                      continue;
                  }
                  let Some(job) = self.queue.claim(&node) else { continue };
                  self.queued.fetch_sub(1, Ordering::AcqRel);
                  if let Some(state) = node.task_state() {
                      state.complete(Ok(()));
                  }
                  if job.run.is_cancelled() {
                      job.run.skip();
                      continue;
                  }
                  return Some(job);
              }
              None
          }
      }

      /// CPU, GPU and shared operation queues. Shared is wired in but never filled by routing.
      pub struct OpQueues {
          cpu: Counted,
          gpu: Counted,
          shared: Counted,
          has_gpu: bool,
      }

      impl OpQueues {
          pub fn new(cpu_wake: Arc<Notify>, gpu_wake: Arc<Notify>, has_gpu: bool) -> Self {
              Self {
                  cpu: Counted::new(vec![cpu_wake.clone()]),
                  gpu: Counted::new(vec![gpu_wake.clone()]),
                  shared: Counted::new(vec![cpu_wake, gpu_wake]),
                  has_gpu,
              }
          }

          pub fn route(&self, engine: EngineSelector) -> Result<Side, OrchestrationError> {
              match engine {
                  EngineSelector::Gpu if !self.has_gpu => Err(OrchestrationError::EngineUnavailable { engine }),
                  EngineSelector::Gpu => Ok(Side::Gpu),
                  _ => Ok(Side::Cpu),
              }
          }

          pub fn push(&self, job: OpJob) -> Result<(), OrchestrationError> {
              let side = self.route(job.params().requires.engine)?;
              self.side(side).push(job);
              Ok(())
          }

          pub fn push_shared(&self, job: OpJob) {
              self.shared.push(job);
          }

          fn side(&self, side: Side) -> &Counted {
              match side {
                  Side::Cpu => &self.cpu,
                  Side::Gpu => &self.gpu,
              }
          }

          /// Own queue first, then shared.
          pub fn claim(&self, side: Side, accepts: &dyn Fn(&OpParams) -> bool) -> Option<OpJob> {
              self.side(side)
                  .claim(accepts, usize::MAX)
                  .or_else(|| self.shared.claim(accepts, SHARED_SKIP_LIMIT))
          }

          pub fn queued(&self, side: Side) -> usize {
              self.side(side).queued.load(Ordering::Acquire)
          }

          pub fn total_queued(&self) -> usize {
              self.cpu.queued.load(Ordering::Acquire)
                  + self.gpu.queued.load(Ordering::Acquire)
                  + self.shared.queued.load(Ordering::Acquire)
          }
      }

      #[cfg(test)]
      pub(crate) mod testutil {
          use super::*;
          use crate::resident::op::{Op, OpDesc, OpKind, Requirements, ResourceKind, Value, HostData};

          pub fn node(task: &Arc<TaskCtx>, resource: ResourceKind, engine: EngineSelector) -> Arc<OperationNode> {
              let params = OpParams::new(Requirements::new(resource, engine));
              let op: Arc<dyn Op> = Arc::new(OpDesc::new(OpKind::Embed, params));
              Arc::new(OperationNode::new(op, Value::Host(HostData::F32(vec![])), true, task.clone()))
          }

          pub fn run(id: u64) -> (Arc<TaskRun>, Arc<TaskState<Output, TaskError>>) {
              let state = Arc::new(TaskState::new());
              let ctx = TaskCtx::new(id, 0, 0);
              let run = TaskRun::new(ctx, state.clone(), Box::new(|| Ok(Output::Scores(vec![1.0]))));
              (run, state)
          }
      }

      #[cfg(test)]
      mod tests {
          use super::testutil::*;
          use super::*;
          use crate::resident::op::ResourceKind;

          fn queues(has_gpu: bool) -> OpQueues {
              OpQueues::new(Arc::new(Notify::new()), Arc::new(Notify::new()), has_gpu)
          }

          fn job(run: &Arc<TaskRun>, resource: ResourceKind, engine: EngineSelector) -> OpJob {
              OpJob::new(node(run.ctx(), resource, engine), run.clone())
          }

          fn any(_: &OpParams) -> bool {
              true
          }

          #[test]
          fn routes_by_engine_and_rejects_missing_gpu() {
              let q = queues(false);
              let (run, _) = run(1);
              run.ctx().add_ops(2);
              q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Either)).unwrap();
              let err = q.push(job(&run, ResourceKind::Encoder, EngineSelector::Gpu)).unwrap_err();
              assert!(matches!(err, OrchestrationError::EngineUnavailable { .. }));
              assert_eq!(q.queued(Side::Cpu), 1);
              assert_eq!(q.queued(Side::Gpu), 0);
          }

          #[test]
          fn own_queue_precedes_shared_and_fifo_holds() {
              let q = queues(true);
              let (run, _) = run(1);
              run.ctx().add_ops(3);
              q.push_shared(job(&run, ResourceKind::Scorer, EngineSelector::Either));
              q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu)).unwrap();
              q.push(job(&run, ResourceKind::Normalizer, EngineSelector::Cpu)).unwrap();
              let kinds: Vec<_> = (0..3)
                  .map(|_| q.claim(Side::Cpu, &any).unwrap().params().requires.resource)
                  .collect();
              assert_eq!(kinds, [ResourceKind::Tokenizer, ResourceKind::Normalizer, ResourceKind::Scorer]);
              assert_eq!(q.total_queued(), 0);
          }

          #[test]
          fn foreign_head_does_not_block_shared_queue() {
              let q = queues(true);
              let (run, _) = run(1);
              run.ctx().add_ops(3);
              q.push_shared(job(&run, ResourceKind::Encoder, EngineSelector::Gpu));
              q.push_shared(job(&run, ResourceKind::Encoder, EngineSelector::Gpu));
              q.push_shared(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu));
              let cpu_only = |p: &OpParams| p.requires.engine == EngineSelector::Cpu;
              let got = q.claim(Side::Cpu, &cpu_only).unwrap();
              assert_eq!(got.params().requires.resource, ResourceKind::Tokenizer);
              assert!(q.claim(Side::Cpu, &cpu_only).is_none());
              assert_eq!(q.total_queued(), 2);
          }

          #[test]
          fn cancelled_task_ops_are_dropped_at_claim() {
              let q = queues(false);
              let (run, _) = run(1);
              run.ctx().add_ops(2);
              q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu)).unwrap();
              q.push(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu)).unwrap();
              run.fail(crate::resident::error::PlannerError::UnknownModel { model: "m".into() }.into());
              assert!(q.claim(Side::Cpu, &any).is_none());
              assert_eq!(run.ctx().open_ops(), 0);
          }

          #[test]
          fn last_op_completes_task_once() {
              let (run, state) = run(7);
              run.ctx().add_ops(2);
              run.op_done();
              assert!(state.result().is_none());
              run.op_done();
              assert_eq!(state.result().unwrap().as_ref().unwrap(), &Output::Scores(vec![1.0]));
          }

          #[test]
          fn followups_counted_before_predecessor_closes() {
              let (run, state) = run(7);
              run.ctx().add_ops(1);
              run.ctx().add_ops(2);
              run.op_done();
              assert!(state.result().is_none());
              run.op_done();
              assert!(state.result().is_none());
              run.op_done();
              assert!(state.result().is_some());
          }

          #[test]
          fn first_error_wins_and_completes_exactly_once() {
              let (run, state) = run(7);
              run.ctx().add_ops(3);
              run.fail(crate::resident::error::PlannerError::UnknownModel { model: "a".into() }.into());
              run.fail(crate::resident::error::PlannerError::UnknownModel { model: "b".into() }.into());
              run.skip();
              run.skip();
              run.skip();
              let msg = format!("{:?}", state.result().unwrap());
              assert!(msg.contains("Engine") && msg.contains("\"a\""), "{msg}");
              assert!(run.ctx().is_cancelled());
          }
      }
  reason: "TaskRun (Abschluss-/Fehlerprotokoll), OpJob und die drei Op-Queues mit Shared-Überspringen anlegen."
```
/answer 4eac-e6 allow
ReasoningToken: 985
Control Request:
```yaml
id: 4eac-e7
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/hub.rs
    source: |
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
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/worker.rs
    source: |
      use std::panic::{catch_unwind, AssertUnwindSafe};
      use std::sync::Arc;
      use std::thread::JoinHandle;

      use crate::resident::facade::TaskQueue;

      use super::config::ProcessorConfig;
      use super::hub::{Engine, Expander, Hub, Step, Wakeups};
      use super::ops::Side;

      pub struct Processor {
          hub: Arc<Hub>,
          workers: Vec<JoinHandle<()>>,
      }

      impl Processor {
          /// `wakeups.relays()` must be the relays of `tasks` so submissions wake the workers.
          /// `gpu` is required when `cfg.gpu_workers > 0`.
          pub fn start(
              cfg: ProcessorConfig,
              tasks: Arc<TaskQueue>,
              wakeups: Wakeups,
              expander: Arc<dyn Expander>,
              cpu: Arc<dyn Engine>,
              gpu: Option<Arc<dyn Engine>>,
          ) -> std::io::Result<Self> {
              let gpu_workers = if gpu.is_some() { cfg.gpu_workers } else { 0 };
              let cfg = ProcessorConfig { gpu_workers, ..cfg };
              let lower_cpu = (gpu_workers > 0 && cfg.cpu_nice != 0).then_some(cfg.cpu_nice);
              let hub = Arc::new(Hub::new(cfg.clone(), tasks, wakeups, expander));
              let mut workers = Vec::with_capacity(cfg.cpu_workers + gpu_workers);
              for index in 0..cfg.cpu_workers {
                  workers.push(spawn(&hub, Side::Cpu, index, cpu.clone(), lower_cpu)?);
              }
              if let Some(gpu) = gpu {
                  for index in 0..gpu_workers {
                      workers.push(spawn(&hub, Side::Gpu, index, gpu.clone(), None)?);
                  }
              }
              Ok(Self { hub, workers })
          }

          pub fn hub(&self) -> &Arc<Hub> {
              &self.hub
          }

          pub fn shutdown(&mut self) {
              self.hub.begin_shutdown();
              for _ in 0..self.workers.len().max(1) {
                  self.hub.wakeups().cpu.notify_one();
                  self.hub.wakeups().gpu.notify_one();
              }
              for worker in self.workers.drain(..) {
                  let _ = worker.join();
              }
          }
      }

      impl Drop for Processor {
          fn drop(&mut self) {
              self.shutdown();
          }
      }

      fn spawn(
          hub: &Arc<Hub>,
          side: Side,
          index: usize,
          engine: Arc<dyn Engine>,
          nice: Option<i32>,
      ) -> std::io::Result<JoinHandle<()>> {
          let hub = hub.clone();
          let name = format!("resident-{}-{index}", if side == Side::Gpu { "gpu" } else { "cpu" });
          std::thread::Builder::new().name(name).spawn(move || {
              let span = tracing::info_span!("worker", ?side, index);
              let _guard = span.enter();
              if let Some(nice) = nice {
                  lower_priority(nice);
              }
              run(&hub, side, &*engine);
          })
      }

      fn run(hub: &Hub, side: Side, engine: &dyn Engine) {
          let wake = hub.wakeups().side(side).clone();
          loop {
              // Registering before the state check closes the lost-wakeup window.
              let notified = wake.notified();
              tokio::pin!(notified);
              notified.as_mut().enable();
              if hub.is_shutdown() {
                  break;
              }
              match catch_unwind(AssertUnwindSafe(|| pass(hub, side, engine))) {
                  Ok(true) => continue,
                  Ok(false) => {}
                  Err(_) => tracing::error!("worker pass panicked"),
              }
              futures::executor::block_on(notified);
          }
      }

      /// One prioritised pass: device results, device feeding, host work, then expansion.
      fn pass(hub: &Hub, side: Side, engine: &dyn Engine) -> bool {
          if engine.harvest(hub) || engine.feed(hub) {
              return true;
          }
          match engine.step(hub) {
              Step::Worked => return true,
              Step::Idle(Some(kind)) => hub.starved(side).mark(kind),
              Step::Idle(None) => {}
          }
          hub.expand_if_needed(side)
      }

      #[cfg(target_os = "linux")]
      fn lower_priority(nice: i32) {
          // SAFETY: `gettid` takes no arguments and `setpriority` only reads scalar arguments.
          let rc = unsafe {
              let tid = libc::syscall(libc::SYS_gettid) as libc::id_t;
              libc::setpriority(libc::PRIO_PROCESS, tid, nice)
          };
          if rc != 0 {
              tracing::warn!(nice, error = %std::io::Error::last_os_error(), "could not lower worker priority");
          }
      }

      #[cfg(not(target_os = "linux"))]
      fn lower_priority(_nice: i32) {}
  reason: "Hub (Expansion, Claim, Abschluss-/Fehlerpfad, Starved-Trigger), Engine-/Expander-Verträge sowie Worker-Loop und Processor-Lebenszyklus anlegen."
```
/answer 4eac-e7 allow
ReasoningToken: 2915
Control Request:
```yaml
id: 4eac-e8
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
    id: ModuleDoc
    position: after
    source: |2

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

      pub use completion::{Completion, LaneBoard, LaneId, LaneOutcome, ManualCompletion, PollState};
      pub use config::ProcessorConfig;
      pub use error::OrchestrationError;
      pub use hub::{Engine, Expander, Expansion, Hub, Step, Wakeups};
      pub use ops::{Collect, OpJob, OpQueue, OpQueues, Side, TaskRun};
      pub use resource::{ResourceSlot, ResourceState};
      pub use starved::{StarvedSet, StarvedSnapshot};
      pub use worker::Processor;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/worker.rs
    id: pass
    position: before
    source: ""
  reason: Modul-Deklarationen und Re-Exports im Processor-mod.rs ergänzen.
```
/answer 4eac-e8 allow
Control Request:
```yaml
id: 4eac-e9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
      result: success
      inserted: 17
      id: ModuleDoc
      ids:
      - completion
      - config
      - error
      - hub
      - ops
      - resource
      - starved
      - worker
      - tests
      - flBf5A|Is9ANd
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/worker.rs
      error: "no items found:"
      id: pass
```
/answer 4eac-e9 allow
Text:
 Jetzt `pass` für die Tests sichtbar machen und die Integrationstests schreiben.
Control Request:
```yaml
id: 4eac-ea
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/worker.rs
    id: pass
    old_text: "fn pass(hub: &Hub, side: Side, engine: &dyn Engine) -> bool {"
    new_text: "pub(super) fn pass(hub: &Hub, side: Side, engine: &dyn Engine) -> bool {"
  reason: pass für die Prioritäts-Tests im Geschwistermodul sichtbar machen.
```
/answer 4eac-ea allow
Text:
 Der Aufruf wurde wegen eines Formatfehlers abgelehnt. Ich sende ihn erneut.
ReasoningToken: 56
Control Request:
```yaml
id: 4eac-eb
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/tests.rs
    source: |
      use std::sync::atomic::{AtomicUsize, Ordering};
      use std::sync::Arc;
      use std::time::{Duration, Instant};

      use parking_lot::Mutex;

      use crate::resident::error::{PlannerError, ResidentError};
      use crate::resident::facade::{
          EmbeddingParams, Output, Payload, SimilarityParams, TaskError, TaskKind, TaskMeta, TaskQueue, TextInput,
          VectorInput,
      };
      use crate::resident::op::{EngineSelector, IndexId, OperationNode, ResourceKind, TaskCtx};
      use crate::resident::queue::{Queue, TaskState};

      use super::ops::testutil::node;
      use super::worker::pass;
      use super::*;

      type TaskStateRef = Arc<TaskState<Output, TaskError>>;

      struct TestExpander {
          ops_per_task: usize,
          fail_expand: bool,
          expanded: Mutex<Vec<u64>>,
      }

      impl TestExpander {
          fn new(ops_per_task: usize) -> Arc<Self> {
              Arc::new(Self { ops_per_task, fail_expand: false, expanded: Mutex::new(vec![]) })
          }
      }

      impl Expander for TestExpander {
          fn first_resource(&self, meta: &TaskMeta) -> Option<ResourceKind> {
              match meta.kind {
                  TaskKind::Similarity => Some(ResourceKind::Scorer),
                  _ => None,
              }
          }

          fn expand(&self, meta: &TaskMeta, _payload: Payload, ctx: &Arc<TaskCtx>) -> Result<Expansion, ResidentError> {
              self.expanded.lock().push(meta.id);
              if self.fail_expand {
                  return Err(PlannerError::UnknownModel { model: "m".into() }.into());
              }
              let n = self.ops_per_task;
              let ops: Vec<Arc<OperationNode>> =
                  (0..n).map(|_| node(ctx, ResourceKind::Tokenizer, EngineSelector::Cpu)).collect();
              Ok(Expansion { ops, collect: Box::new(move || Ok(Output::Scores(vec![n as f32]))) })
          }
      }

      #[derive(Clone, Copy)]
      enum Mode {
          Run,
          FailAll,
          FollowUp,
          Idle(Option<ResourceKind>),
      }

      struct TestEngine {
          side: Side,
          mode: Mode,
          executed: AtomicUsize,
          harvest: AtomicUsize,
          feed: AtomicUsize,
          log: Mutex<Vec<&'static str>>,
      }

      impl TestEngine {
          fn new(side: Side, mode: Mode) -> Arc<Self> {
              Arc::new(Self {
                  side,
                  mode,
                  executed: AtomicUsize::new(0),
                  harvest: AtomicUsize::new(0),
                  feed: AtomicUsize::new(0),
                  log: Mutex::new(vec![]),
              })
          }

          fn take(counter: &AtomicUsize) -> bool {
              counter
                  .fetch_update(Ordering::AcqRel, Ordering::Acquire, |v| v.checked_sub(1))
                  .is_ok()
          }
      }

      impl Engine for TestEngine {
          fn harvest(&self, _hub: &Hub) -> bool {
              let hit = Self::take(&self.harvest);
              if hit {
                  self.log.lock().push("harvest");
              }
              hit
          }

          fn feed(&self, _hub: &Hub) -> bool {
              let hit = Self::take(&self.feed);
              if hit {
                  self.log.lock().push("feed");
              }
              hit
          }

          fn step(&self, hub: &Hub) -> Step {
              if let Mode::Idle(kind) = self.mode {
                  return Step::Idle(kind);
              }
              let Some(job) = hub.claim(self.side, &|_| true) else {
                  return Step::Idle(None);
              };
              self.log.lock().push("step");
              self.executed.fetch_add(1, Ordering::AcqRel);
              match self.mode {
                  Mode::FailAll => {
                      hub.fail_op(job, PlannerError::UnknownModel { model: "x".into() }.into());
                  }
                  Mode::FollowUp if job.params().requires.resource == ResourceKind::Tokenizer => {
                      let next = node(job.run.ctx(), ResourceKind::Normalizer, EngineSelector::Cpu);
                      hub.complete_op(job, vec![next]);
                  }
                  _ => hub.complete_op(job, vec![]),
              }
              Step::Worked
          }
      }

      fn embedding(id: u64) -> (TaskMeta, Payload) {
          (
              TaskMeta { id, kind: TaskKind::Embedding },
              Payload::Embedding { input: TextInput::new(["a"]), params: EmbeddingParams::default() },
          )
      }

      fn similarity(id: u64) -> (TaskMeta, Payload) {
          (
              TaskMeta { id, kind: TaskKind::Similarity },
              Payload::Similarity {
                  input: VectorInput::new(vec![vec![0.0]]),
                  params: SimilarityParams::new(IndexId("i".into())),
              },
          )
      }

      fn submit(tasks: &TaskQueue, (meta, payload): (TaskMeta, Payload)) -> TaskStateRef {
          let state = Arc::new(TaskState::new());
          tasks.push(meta, state.clone(), payload);
          state
      }

      fn hub(cfg: ProcessorConfig, expander: Arc<dyn Expander>) -> (Hub, Arc<TaskQueue>) {
          let wakeups = Wakeups::new();
          let tasks = Arc::new(Queue::new(wakeups.relays()));
          (Hub::new(cfg, tasks.clone(), wakeups, expander), tasks)
      }

      fn await_result(state: &TaskStateRef) -> Result<Output, TaskError> {
          let deadline = Instant::now() + Duration::from_secs(5);
          while state.result().is_none() {
              assert!(Instant::now() < deadline, "task did not complete");
              std::thread::sleep(Duration::from_millis(1));
          }
          state.result().unwrap().clone()
      }

      fn start(
          cfg: ProcessorConfig,
          expander: Arc<dyn Expander>,
          cpu: Arc<TestEngine>,
      ) -> (Processor, Arc<TaskQueue>) {
          let wakeups = Wakeups::new();
          let tasks = Arc::new(Queue::new(wakeups.relays()));
          let p = Processor::start(cfg, tasks.clone(), wakeups, expander, cpu, None).unwrap();
          (p, tasks)
      }

      #[test]
      fn worker_threads_run_tasks_to_completion() {
          let cpu = TestEngine::new(Side::Cpu, Mode::Run);
          let (_p, tasks) = start(ProcessorConfig::new(2, 0).with_cpu_nice(0), TestExpander::new(3), cpu.clone());
          let states: Vec<_> = (0..20).map(|i| submit(&tasks, embedding(i))).collect();
          for s in &states {
              assert_eq!(await_result(s).unwrap(), Output::Scores(vec![3.0]));
          }
          assert_eq!(cpu.executed.load(Ordering::Acquire), 60);
      }

      #[test]
      fn first_failing_op_fails_task_and_remaining_ops_are_dropped() {
          let cpu = TestEngine::new(Side::Cpu, Mode::FailAll);
          let (_p, tasks) = start(ProcessorConfig::new(1, 0).with_cpu_nice(0), TestExpander::new(4), cpu.clone());
          let s = submit(&tasks, embedding(1));
          assert!(matches!(await_result(&s), Err(TaskError::Engine(_))));
          std::thread::sleep(Duration::from_millis(50));
          assert_eq!(cpu.executed.load(Ordering::Acquire), 1);
      }

      #[test]
      fn followup_ops_keep_task_open_until_they_finish() {
          let cpu = TestEngine::new(Side::Cpu, Mode::FollowUp);
          let (_p, tasks) = start(ProcessorConfig::new(1, 0).with_cpu_nice(0), TestExpander::new(1), cpu.clone());
          let s = submit(&tasks, embedding(1));
          assert!(await_result(&s).is_ok());
          assert_eq!(cpu.executed.load(Ordering::Acquire), 2);
      }

      #[test]
      fn expansion_failure_completes_task_with_error() {
          let expander = Arc::new(TestExpander { ops_per_task: 1, fail_expand: true, expanded: Mutex::new(vec![]) });
          let cpu = TestEngine::new(Side::Cpu, Mode::Run);
          let (_p, tasks) = start(ProcessorConfig::new(1, 0).with_cpu_nice(0), expander, cpu);
          let s = submit(&tasks, embedding(1));
          assert!(matches!(await_result(&s), Err(TaskError::Engine(_))));
      }

      #[test]
      fn gpu_ops_without_gpu_workers_fail_the_task() {
          struct GpuExpander;
          impl Expander for GpuExpander {
              fn expand(&self, _m: &TaskMeta, _p: Payload, ctx: &Arc<TaskCtx>) -> Result<Expansion, ResidentError> {
                  Ok(Expansion {
                      ops: vec![node(ctx, ResourceKind::Encoder, EngineSelector::Gpu)],
                      collect: Box::new(|| Ok(Output::Scores(vec![]))),
                  })
              }
          }
          let cpu = TestEngine::new(Side::Cpu, Mode::Run);
          let (_p, tasks) = start(ProcessorConfig::new(1, 0).with_cpu_nice(0), Arc::new(GpuExpander), cpu);
          let s = submit(&tasks, embedding(1));
          assert!(matches!(await_result(&s), Err(TaskError::Engine(_))));
      }

      #[test]
      fn shutdown_joins_idle_workers() {
          let cpu = TestEngine::new(Side::Cpu, Mode::Run);
          let (mut p, _tasks) = start(ProcessorConfig::new(3, 0).with_cpu_nice(0), TestExpander::new(1), cpu);
          std::thread::sleep(Duration::from_millis(20));
          p.shutdown();
      }

      #[test]
      fn pass_prefers_harvest_then_feed_then_step() {
          let cfg = ProcessorConfig::new(1, 1).with_min_op_buffer(1);
          let (hub, tasks) = hub(cfg, TestExpander::new(1));
          let engine = TestEngine::new(Side::Cpu, Mode::Run);
          engine.harvest.store(1, Ordering::Release);
          engine.feed.store(1, Ordering::Release);
          submit(&tasks, embedding(1));
          let mut passes = 0;
          while pass(&hub, Side::Cpu, &*engine) {
              passes += 1;
              assert!(passes < 20);
          }
          assert_eq!(*engine.log.lock(), vec!["harvest", "feed", "step"]);
      }

      #[test]
      fn expansion_only_on_hint_or_low_buffer() {
          let cfg = ProcessorConfig::new(1, 0).with_min_op_buffer(0);
          let expander = TestExpander::new(1);
          let (hub, tasks) = hub(cfg, expander.clone());
          let quiet = TestEngine::new(Side::Cpu, Mode::Idle(None));
          submit(&tasks, embedding(1));
          assert!(!pass(&hub, Side::Cpu, &*quiet));
          assert!(expander.expanded.lock().is_empty());

          let hungry = TestEngine::new(Side::Cpu, Mode::Idle(Some(ResourceKind::Encoder)));
          assert!(pass(&hub, Side::Cpu, &*hungry));
          assert_eq!(*expander.expanded.lock(), vec![1]);
          assert!(hub.starved(Side::Cpu).snapshot().is_empty());
      }

      #[test]
      fn low_buffer_triggers_expansion_without_hint() {
          let cfg = ProcessorConfig::new(1, 0).with_min_op_buffer(2);
          let expander = TestExpander::new(1);
          let (hub, tasks) = hub(cfg, expander.clone());
          let quiet = TestEngine::new(Side::Cpu, Mode::Idle(None));
          submit(&tasks, embedding(1));
          submit(&tasks, embedding(2));
          assert!(pass(&hub, Side::Cpu, &*quiet));
          assert!(pass(&hub, Side::Cpu, &*quiet));
          assert!(!pass(&hub, Side::Cpu, &*quiet));
          assert_eq!(*expander.expanded.lock(), vec![1, 2]);
      }

      #[test]
      fn hint_selects_task_needing_the_starved_resource() {
          let cfg = ProcessorConfig::new(1, 0).with_min_op_buffer(0);
          let expander = TestExpander::new(1);
          let (hub, tasks) = hub(cfg, expander.clone());
          submit(&tasks, embedding(1));
          submit(&tasks, similarity(2));
          let scorer_hungry = TestEngine::new(Side::Cpu, Mode::Idle(Some(ResourceKind::Scorer)));
          assert!(pass(&hub, Side::Cpu, &*scorer_hungry));
          assert_eq!(*expander.expanded.lock(), vec![2]);
      }

      #[test]
      fn pick_of_a_kind_clears_its_hint() {
          let cfg = ProcessorConfig::new(1, 0).with_min_op_buffer(0);
          let (hub, tasks) = hub(cfg, TestExpander::new(1));
          hub.starved(Side::Cpu).mark(ResourceKind::Tokenizer);
          hub.starved(Side::Cpu).mark(ResourceKind::Encoder);
          let state = submit(&tasks, embedding(1));
          let engine = TestEngine::new(Side::Cpu, Mode::Run);
          while pass(&hub, Side::Cpu, &*engine) {}
          let snap = hub.starved(Side::Cpu).snapshot();
          assert!(!snap.contains(ResourceKind::Tokenizer));
          assert!(await_result(&state).is_ok());
      }
  reason: "Integrations-Tests des Processors anlegen (Worker, Fehlerpfad, Folgeops, Prioritäten, Starved-Trigger, Shutdown)."
```
/answer 4eac-eb allow
Control Request:
```yaml
id: 4eac-ec
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: ResidentError::(Index|Capability)\(|match .*err
    include:
    - '*.rs'
    exclude:
    - error.rs
    limit: 15
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    pattern: impl SimilarityParams|impl VectorInput|impl Default for SimilarityParams
    include:
    - params.rs
    limit: 5
  reason: Weitere exhaustive Matches auf ResidentError und SimilarityParams-Konstruktor prüfen.
```
/answer 4eac-ec allow
ReasoningToken: 4130
Control Request:
```yaml
id: 4eac-ed
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches: []
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      matches:
      - path: params.rs
        matches:
        - lineno: 45
          match: "impl VectorInput {"
        - lineno: 135
          match: "impl SimilarityParams {"
```
/answer 4eac-ed allow
Control Request:
```yaml
id: 4eac-ee
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/tests.rs
    id: pick_of_a_kind_clears_its_hint
    start_marker: "fn pick_of_a_kind_clears_its_hint() {"
    end_marker: |-
      assert!(await_result(&state).is_ok());
      }
    content: |-
      fn pick_of_a_kind_clears_its_hint() {
          let cfg = ProcessorConfig::new(1, 0).with_min_op_buffer(1);
          let (hub, tasks) = hub(cfg, TestExpander::new(1));
          submit(&tasks, embedding(1));
          let quiet = TestEngine::new(Side::Cpu, Mode::Idle(None));
          assert!(pass(&hub, Side::Cpu, &*quiet));
          hub.starved(Side::Cpu).mark(ResourceKind::Tokenizer);
          hub.starved(Side::Cpu).mark(ResourceKind::Encoder);
          let job = hub.claim(Side::Cpu, &|_| true).unwrap();
          let snap = hub.starved(Side::Cpu).snapshot();
          assert!(!snap.contains(ResourceKind::Tokenizer));
          assert!(snap.contains(ResourceKind::Encoder));
          hub.complete_op(job, vec![]);
      }
  reason: "Test so umschreiben, dass er das Löschen des Hints beim Claim wirklich prüft."
```
/answer 4eac-ee allow
ReasoningToken: 62
Control Request:
```yaml
id: 4eac-ef
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/config.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/completion.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/starved.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/ops.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/hub.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/worker.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/tests.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
  reason: Syntaktische Prüfung aller neuen und geänderten Dateien; ein cargo-Lauf steht mir hier nicht zur Verfügung.
```
/answer 4eac-ef allow
ReasoningToken: 198
Control Request:
```yaml
id: 4eac-f0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/completion.rs
    id: tests.signal_before_launch_completes_is_not_lost
    old_text: "fn signal_before_launch_completes_is_not_lost() {"
    new_text: "fn repeated_signals_complete_lane_once() {"
  reason: Irreführenden Testnamen korrigieren.
```
/answer 4eac-f0 allow
Result Stats: id=0c4e46b9-8322-4727-9459-cd1119c8b4d7, total: 206517, in: 62, out: 74230, reason: 0, read: 2457233, write: 132225, start: 1792320080880, end: 1792320089169, time: 8245, total: 482755, io: 0, control: 44, sent: 0, recv: 207
C2 ist im Code umgesetzt, aber ungeprüft: Ich hatte hier nur die Syntaxprüfung, kein `cargo build` und keinen Testlauf. Bitte lass `cargo test` und `cargo check --features cuda` laufen.

Die CUDA-Implementierungen `HostFnCompletion` und `PolledCompletion` fehlen. Plan-Schritt 1 (Signatur von `launch_host_function` und Stream-Erzeugung in der gepinnten cudarc-Version prüfen) ist noch offen, und ich wollte keinen Code gegen eine ungeprüfte API schreiben. Das `Completion`-Trait, das Lane-Protokoll und `ManualCompletion` sind fertig.

Der Code liegt unter `rag/src/resident/processor/`. Außerhalb davon habe ich drei Dinge geändert:
- **`Cargo.toml`:** `libc` als Linux-Abhängigkeit, für die Nice-Herabstufung der CPU-Worker.
- **`TaskCtx::add_ops`:** neu in `op/mod.rs`, für das Zählprotokoll der Folgeoperationen.
- **`ResidentError`:** neue Variante `Orchestration(#[from] OrchestrationError)`.

Abweichungen und offene Punkte:
- **Planner:** Er ist noch ein Stub. Der Processor arbeitet deshalb gegen ein neues Trait `Expander`, das der Planner später implementiert.
- **Facade:** Sie legt die Task-Queue derzeit ohne Relays an (`Queue::new(Vec::new())`). Damit Einreihen die Worker aufweckt, muss sie `Wakeups::relays()` übergeben und den Processor mit derselben `Wakeups` starten.
- **Namen:** Die Konfiguration heißt `ProcessorConfig`, weil `WorkerConfig` in der Facade schon existiert. Es gibt eine `From`-Konvertierung.
- **Routing:** `Either`-Operationen landen in der CPU-Queue. Die Shared-Queue ist vorhanden und wird bei Claims abgefragt, aber vom Routing nicht befüllt.
- **Task-Timeout:** Er ist nicht umgesetzt.
- **Lease-`gen`-Prüfung:** Die `gen`-Debug-Assertion beim Claim habe ich nicht ergänzt. Der RAII-Lease aus B3 wird unverändert genutzt.

Getestet sind die CAS-Races zwischen Callback und Poller, Starved-Verfall, Shared-Queue ohne Head-of-Line-Blocking, `pending`-Zähler, Fehlerpfad, Worker-Priorität und Shutdown. Die `proptest-state-machine`-Tests aus dem Plan fehlen.
