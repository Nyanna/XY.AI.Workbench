SystemInit:  id=02c72708-f03d-4efe-ba73-55c395a6f29c, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst den vorhandenen Code an.
Control Request:
```yaml
id: 4eac-124
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  - path: /home/user/xyan/xy.ai.workbench/rag
    recursive: false
  reason: Bestehende Struktur des resident-Moduls und Cargo-Setup erfassen
```
/answer 4eac-124 allow
ReasoningToken: 37
Control Request:
```yaml
id: 4eac-125
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag
      entries:
      - ".:"
      - .gitignore
      - .project
      - Cargo.lock
      - Cargo.toml
      - xyrag.sh
```
/answer 4eac-125 allow
Control Request:
```yaml
id: 4eac-126
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
  reason: Aktuellen Stand von Planner und Processor lesen
```
/answer 4eac-126 allow
Control Request:
```yaml
id: 4eac-127
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Stateless task expansion.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Orchestration: task queue + op queues."
      - id: completion
        signature: mod completion;
      - id: completion_cuda
        signature: "#[cfg(feature = \"cuda\")]"
      - id: config
        signature: mod config;
      - id: error
        signature: mod error;
      - id: hub
        signature: mod hub;
      - id: ops
        signature: mod ops;
      - id: resource
        signature: mod resource;
      - id: starved
        signature: mod starved;
      - id: worker
        signature: mod worker;
      - id: tests
        signature: "#[cfg(test)]"
      - id: flBf5A|OTJLmF
        signature: "pub use completion::{"
```
/answer 4eac-127 allow
Control Request:
```yaml
id: 4eac-128
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  reason: Dateiliste des resident-Moduls
```
/answer 4eac-128 allow
ReasoningToken: 27
Control Request:
```yaml
id: 4eac-129
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".: (6 files)"
      - backend.rs
      - "./facade: (7 files)"
      - blocking.rs
      - "./index: (3 files)"
      - memory.rs
      - "./kernel: (3 files)"
      - cuda.rs
      - "./manager: (1 file)"
      - mod.rs
      - "./memory: (6 files)"
      - dtype.rs
      - "./op: (3 files)"
      - lease.rs
      - "./planner: (1 file)"
      - mod.rs
      - "./precision: (6 files)"
      - axis.rs
      - "./processor: (11 files)"
      - completion.rs
      - "./queue: (5 files)"
      - mod.rs
      - "./subengine: (2 files)"
      - device.rs
      warning: Result reduced because the total number of entries (54) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 4eac-129 allow
Control Request:
```yaml
id: 4eac-12a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    ids: []
  reason: "Cargo-Abhängigkeiten prüfen (serde, toml, arc-swap)"
```
/answer 4eac-12a allow
Control Request:
```yaml
id: 4eac-12b
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
      - id: anLBDO|YAkuwn
        signature: "[target.'cfg(target_os = \"linux\")'.dependencies]"
      - id: adh0oF|NNwOkR
        signature: "[features]"
      - id: UeF6gB|FaChiw
        signature: "[dev-dependencies]"
      - id: 9OxNvg|OtfeRA
        signature: "[[bench]]"
      - id: uCHafc|L1C1dJ
        signature: "[patch.crates-io]"
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 4eac-12b allow
Control Request:
```yaml
id: 4eac-12c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    ids:
    - uUBhmF|EGrpec
    - adh0oF|NNwOkR
    - UeF6gB|FaChiw
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
    ids:
    - ModuleDoc
  reason: Dependencies/Features und Planner-Stub lesen
```
/answer 4eac-12c allow
Control Request:
```yaml
id: 4eac-12d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
      nodes:
      - id: uUBhmF|EGrpec
        type: table
        code: |+
          [dependencies]
          tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync", "time"] }
          tokio-util = "0.7"
          async-trait = "0.1"
          futures = "0.3"
          serde = { version = "1", features = ["derive"] }
          serde_json = "1"
          serde_yaml = "0.9"
          rusqlite = { version = "0.31", features = ["bundled"] }
          clap = { version = "4", features = ["derive"] }
          anyhow = "1"
          rayon = "1"
          rkyv = { version = "0.7", features = ["validation"] }
          glob = "0.3"
          lru = "0.12"
          regex = "1"
          unicode-normalization = "0.1"
          thiserror = "2.0.20"
          tracing = "0.1.44"
          tracing-subscriber = { version = "0.3.23", features = ["env-filter"] }
          tracing-appender = "0.2.3"
          crossbeam = "0.8.5"
          crossbeam-queue = "0.3.14"
          parking_lot = "0.12.5"
          arc-swap = "1.9.2"
          half = "2.7.1"
          memmap2 = "0.9.3"
          xxhash-rust = { version = "0.8", features = ["xxh3"] }
          parquet = "60.0.0"
          arrow = "60.0.0"
          tokenizers = { version = "0.23.2", default-features = false, features = ["fancy-regex"] }
          safetensors = "0.8.0"
          candle-core = { version = "0.11.0", optional = true, default-features = false }
          candle-transformers = { version = "0.11.0", optional = true, default-features = false }
          cudarc = { version = "=0.19.9", optional = true }
          rand = { version = "0.8", optional = true }

      - id: adh0oF|NNwOkR
        type: table
        code: |+
          [features]
          default = []
          candle = ["dep:candle-core"]
          cuda = ["candle", "dep:candle-transformers", "dep:cudarc", "candle-core/cuda", "candle-transformers/cuda"]
          mock = ["dep:rand"]
          json = ["tracing-subscriber/json"]

      - id: UeF6gB|FaChiw
        type: table
        code: |+
          [dev-dependencies]
          proptest = "1"
          criterion = "0.5"
          mockall = "0.15.0"
          loom = "0.7"
          shuttle = "0.7"
          rand = "0.8"
          tokio = { version = "1", features = ["test-util"] }

    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Stateless task expansion.
```
/answer 4eac-12d allow
Control Request:
```yaml
id: 4eac-12e
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: (op/|queue/|processor/|precision/|facade/|subengine/|error|mod\.rs)
  reason: "Alle Dateien der Verträge (op, queue, processor, precision, facade) finden"
```
/answer 4eac-12e allow
Control Request:
```yaml
id: 4eac-12f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".:"
      - error.rs
      - mod.rs
      - "./facade:"
      - blocking.rs
      - error.rs
      - handle.rs
      - mod.rs
      - params.rs
      - resident.rs
      - task.rs
      - "./index:"
      - mod.rs
      - "./kernel:"
      - mod.rs
      - "./manager:"
      - mod.rs
      - "./memory:"
      - mod.rs
      - "./op:"
      - lease.rs
      - mod.rs
      - requirements.rs
      - "./planner:"
      - mod.rs
      - "./precision:"
      - axis.rs
      - capability.rs
      - known.rs
      - mod.rs
      - pareto.rs
      - profile.rs
      - "./processor:"
      - completion.rs
      - completion_cuda.rs
      - config.rs
      - error.rs
      - hub.rs
      - mod.rs
      - ops.rs
      - resource.rs
      - starved.rs
      - tests.rs
      - worker.rs
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
/answer 4eac-12f allow
ReasoningToken: 57
Control Request:
```yaml
id: 4eac-130
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/capability.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/starved.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
  reason: "Verträge (Operation, Requirement, Params, CapabilitySet, Starved) erfassen"
```
/answer 4eac-130 allow
Control Request:
```yaml
id: 4eac-131
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
      - id: failpoint
        signature: pub mod failpoint;
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
      - id: DmI3w7|wIeZIJ
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
        - id: impl_TaskCtx.add_ops
          signature: /// Registers `n` follow-up ops. Must run before they are enqueued and before t…
          docstring: Registers `n` follow-up ops. Must run before they are enqueued and before the
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
      nodes:
      - id: ModuleDoc
        signature: //! Resource requirements of an operation.
      - id: 2iXFMc|Cm8Kf6
        signature: "use crate::resident::memory::{DeviceKind, Region};"
      - id: ModelId
        signature: "#[derive(Clone, Debug, PartialEq, Eq, Hash)]"
      - id: IndexId
        signature: "#[derive(Clone, Debug, PartialEq, Eq, Hash)]"
      - id: DeviceId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      - id: ResourceKind
        signature: "#[non_exhaustive] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub enum R…"
      - id: EngineSelector
        signature: "#[non_exhaustive] #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub enum E…"
      - id: impl_EngineSelector
        signature: "impl EngineSelector {"
        children:
        - id: impl_EngineSelector.accepts
          signature: "pub fn accepts(self, dev: DeviceKind) -> bool {"
      - id: Affinity
        signature: "/// Placement hint; inactive in the scheduler for now. #[non_exhaustive] #[deri…"
        docstring: Placement hint; inactive in the scheduler for now.
      - id: Requirements
        signature: "#[non_exhaustive] #[derive(Clone, Debug, PartialEq, Eq, Hash)] pub struct Requi…"
      - id: impl_Requirements
        signature: "impl Requirements {"
        children:
        - id: impl_Requirements.new
          signature: "pub fn new(resource: ResourceKind, engine: EngineSelector) -> Self {"
        - id: impl_Requirements.with_model
          signature: "pub fn with_model(mut self, model: ModelId) -> Self {"
        - id: impl_Requirements.with_index
          signature: "pub fn with_index(mut self, index: IndexId) -> Self {"
        - id: impl_Requirements.with_caps
          signature: "pub fn with_caps(mut self, caps: CapabilitySet) -> Self {"
        - id: impl_Requirements.with_affinity
          signature: "pub fn with_affinity(mut self, affinity: Affinity) -> Self {"
        - id: impl_Requirements.satisfied_by
          signature: "/// Affinity is deliberately ignored. pub fn satisfied_by(&self, dev: DeviceKin…"
          docstring: Affinity is deliberately ignored.
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.engine_and_caps_matching
          signature: "#[test] fn engine_and_caps_matching() {"
        - id: tests.either_accepts_both_and_empty_caps_pass
          signature: "#[test] fn either_accepts_both_and_empty_caps_pass() {"
        - id: tests.affinity_does_not_affect_matching
          signature: "#[test] fn affinity_does_not_affect_matching() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
      nodes:
      - id: lIKA3E|PBeih5
        signature: "use crate::resident::index::{EntryId, Scope};"
      - id: jTwlTq|iUpnXF
        signature: "const DEFAULT_PRECISION: f32 = 1.0;"
      - id: TextRole
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)] #[non_exhaustive] p…"
      - id: TextInput
        signature: "#[derive(Clone, Debug, Default)] #[non_exhaustive] pub struct TextInput {"
      - id: impl_TextInput
        signature: "impl TextInput {"
        children:
        - id: impl_TextInput.new
          signature: "pub fn new(texts: impl IntoIterator<Item = impl Into<String>>) -> Self {"
        - id: impl_TextInput.single
          signature: "pub fn single(text: impl Into<String>) -> Self {"
        - id: impl_TextInput.texts
          signature: "pub fn texts(&self) -> &[String] {"
      - id: VectorInput
        signature: "#[derive(Clone, Debug, Default)] #[non_exhaustive] pub struct VectorInput {"
      - id: impl_VectorInput
        signature: "impl VectorInput {"
        children:
        - id: impl_VectorInput.new
          signature: "pub fn new(vectors: Vec<Vec<f32>>) -> Self {"
        - id: impl_VectorInput.vectors
          signature: "pub fn vectors(&self) -> &[Vec<f32>] {"
      - id: PairInput
        signature: "/// Cross-encoder input; kept apart from [`TextRole`] so role and pair scoring …"
        docstring: "Cross-encoder input; kept apart from [`TextRole`] so role and pair scoring stay…"
      - id: impl_PairInput
        signature: "impl PairInput {"
        children:
        - id: impl_PairInput.new
          signature: "pub fn new(query: impl Into<String>, chunk: impl Into<String>) -> Self {"
        - id: impl_PairInput.query
          signature: "pub fn query(&self) -> &str {"
        - id: impl_PairInput.chunk
          signature: "pub fn chunk(&self) -> &str {"
      - id: EmbeddingParams
        signature: "#[derive(Clone, Debug)] #[non_exhaustive] pub struct EmbeddingParams {"
      - id: impl_EmbeddingParams
        signature: "impl EmbeddingParams {"
        children:
        - id: impl_EmbeddingParams.new
          signature: "pub fn new(role: TextRole) -> Self {"
        - id: impl_EmbeddingParams.write_index
          signature: "pub fn write_index(mut self, index: IndexId) -> Self {"
        - id: impl_EmbeddingParams.precision
          signature: "pub fn precision(mut self, precision: f32) -> Self {"
        - id: impl_EmbeddingParams.role
          signature: "pub fn role(&self) -> TextRole {"
        - id: impl_EmbeddingParams.target_index
          signature: "pub fn target_index(&self) -> Option<&IndexId> {"
        - id: impl_EmbeddingParams.get_precision
          signature: "pub fn get_precision(&self) -> f32 {"
      - id: impl_Default_for_EmbeddingParams
        signature: "impl Default for EmbeddingParams {"
        children:
        - id: impl_Default_for_EmbeddingParams.default
          signature: "fn default() -> Self {"
      - id: SimilarityParams
        signature: "#[derive(Clone, Debug)] #[non_exhaustive] pub struct SimilarityParams {"
      - id: impl_SimilarityParams
        signature: "impl SimilarityParams {"
        children:
        - id: impl_SimilarityParams.new
          signature: "pub fn new(index: IndexId) -> Self {"
        - id: impl_SimilarityParams.top_k
          signature: "pub fn top_k(mut self, top_k: usize) -> Self {"
        - id: impl_SimilarityParams.scope
          signature: "pub fn scope(mut self, scope: Scope) -> Self {"
        - id: impl_SimilarityParams.precision
          signature: "pub fn precision(mut self, precision: f32) -> Self {"
        - id: impl_SimilarityParams.index
          signature: "pub fn index(&self) -> &IndexId {"
        - id: impl_SimilarityParams.get_top_k
          signature: "pub fn get_top_k(&self) -> usize {"
        - id: impl_SimilarityParams.get_scope
          signature: "pub fn get_scope(&self) -> &Scope {"
        - id: impl_SimilarityParams.get_precision
          signature: "pub fn get_precision(&self) -> f32 {"
      - id: InferenceParams
        signature: "#[derive(Clone, Debug)] #[non_exhaustive] pub struct InferenceParams {"
      - id: impl_InferenceParams
        signature: "impl InferenceParams {"
        children:
        - id: impl_InferenceParams.new
          signature: "pub fn new() -> Self {"
        - id: impl_InferenceParams.model
          signature: "pub fn model(mut self, model: ModelId) -> Self {"
        - id: impl_InferenceParams.precision
          signature: "pub fn precision(mut self, precision: f32) -> Self {"
        - id: impl_InferenceParams.get_model
          signature: "pub fn get_model(&self) -> Option<&ModelId> {"
        - id: impl_InferenceParams.get_precision
          signature: "pub fn get_precision(&self) -> f32 {"
      - id: impl_Default_for_InferenceParams
        signature: "impl Default for InferenceParams {"
        children:
        - id: impl_Default_for_InferenceParams.default
          signature: "fn default() -> Self {"
      - id: Output
        signature: "#[derive(Clone, Debug, PartialEq)] #[non_exhaustive] pub enum Output {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
      nodes:
      - id: jAY2Rz|M8Kowd
        signature: use super::error::TaskError;
      - id: TaskKind
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] #[non_exhaustive] pub enum T…"
      - id: TaskMeta
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub struct TaskMeta {"
      - id: Payload
        signature: "#[derive(Debug)] #[non_exhaustive] pub enum Payload {"
      - id: TaskQueue
        signature: "pub type TaskQueue = Queue<TaskMeta, Payload, Output, TaskError>;"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/capability.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Capability sets: axes mapped to the values a provider offers or a task requ…"
      - id: mKJOyz|BKgeCZ
        signature: "use std::collections::{BTreeMap, BTreeSet};"
      - id: CapabilitySet
        signature: "/// Ordered containers keep iteration, hashing and serialization reproducible."
        docstring: "Ordered containers keep iteration, hashing and serialization reproducible."
      - id: impl_CapabilitySet
        signature: "impl CapabilitySet {"
        children:
        - id: impl_CapabilitySet.new
          signature: "pub fn new() -> Self {"
        - id: impl_CapabilitySet.with
          signature: "pub fn with(mut self, axis: impl Into<AxisId>, value: impl Into<ValueId>) -> Se…"
        - id: impl_CapabilitySet.insert
          signature: "pub fn insert(     &mut self,     axis: impl Into<AxisId>,     value: impl Into…"
        - id: impl_CapabilitySet.contains
          signature: "pub fn contains(&self, axis: &AxisId, value: &ValueId) -> bool {"
        - id: impl_CapabilitySet.values
          signature: "pub fn values(&self, axis: &AxisId) -> Option<&BTreeSet<ValueId>> {"
        - id: impl_CapabilitySet.axes
          signature: "pub fn axes(&self) -> impl Iterator<Item = &AxisId> {"
        - id: impl_CapabilitySet.iter
          signature: "pub fn iter(&self) -> impl Iterator<Item = (&AxisId, &BTreeSet<ValueId>)> {"
        - id: impl_CapabilitySet.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_CapabilitySet.satisfies
          signature: /// Every required value on every required axis is present in `self`. pub fn sa…
          docstring: Every required value on every required axis is present in `self`.
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.B4FIpz|8lZGDC
          signature: use super::*;
        - id: tests.hash_of
          signature: "fn hash_of(c: &CapabilitySet) -> u64 {"
        - id: tests.satisfies_is_axiswise_subset
          signature: "#[test] fn satisfies_is_axiswise_subset() {"
        - id: tests.missing_axis_fails_and_empty_requirement_passes
          signature: "#[test] fn missing_axis_fails_and_empty_requirement_passes() {"
        - id: tests.insertion_order_does_not_affect_identity
          signature: "#[test] fn insertion_order_does_not_affect_identity() {"
        - id: tests.unknown_axes_and_values_are_accepted
          signature: "#[test] fn unknown_axes_and_values_are_accepted() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/starved.rs
      nodes:
      - id: PaFWtS|puswEo
        signature: "use std::sync::atomic::{AtomicU64, Ordering};"
      - id: XOiMUZ|1rkWwz
        signature: "const KINDS: usize = 6;"
      - id: slot
        signature: "fn slot(kind: ResourceKind) -> usize {"
      - id: StarvedSnapshot
        signature: "#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]"
      - id: impl_StarvedSnapshot
        signature: "impl StarvedSnapshot {"
        children:
        - id: impl_StarvedSnapshot.contains
          signature: "pub fn contains(self, kind: ResourceKind) -> bool {"
        - id: impl_StarvedSnapshot.is_empty
          signature: "pub fn is_empty(self) -> bool {"
      - id: StarvedSet
        signature: /// Decaying per-kind demand hints; a hint is cleared by a pick or by occupancy…
        docstring: Decaying per-kind demand hints; a hint is cleared by a pick or by occupancy and
      - id: impl_StarvedSet
        signature: "impl StarvedSet {"
        children:
        - id: impl_StarvedSet.new
          signature: "pub fn new(ttl: Duration) -> Self {"
        - id: impl_StarvedSet.now_ms
          signature: "fn now_ms(&self) -> u64 {"
        - id: impl_StarvedSet.mark
          signature: "pub fn mark(&self, kind: ResourceKind) {"
        - id: impl_StarvedSet.clear
          signature: "pub fn clear(&self, kind: ResourceKind) {"
        - id: impl_StarvedSet.clear_all
          signature: "pub fn clear_all(&self, snapshot: StarvedSnapshot) {"
        - id: impl_StarvedSet.snapshot
          signature: "pub fn snapshot(&self) -> StarvedSnapshot {"
        - id: impl_StarvedSet.mark_at
          signature: "pub(crate) fn mark_at(&self, kind: ResourceKind, now_ms: u64) {"
        - id: impl_StarvedSet.snapshot_at
          signature: "pub(crate) fn snapshot_at(&self, now_ms: u64) -> StarvedSnapshot {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.set
          signature: "fn set() -> StarvedSet {"
        - id: tests.mark_is_visible_until_ttl
          signature: "#[test] fn mark_is_visible_until_ttl() {"
        - id: tests.pick_clears_only_its_kind
          signature: "#[test] fn pick_clears_only_its_kind() {"
        - id: tests.remark_extends_lifetime
          signature: "#[test] fn remark_extends_lifetime() {"
        - id: tests.clear_all_consumes_snapshot
          signature: "#[test] fn clear_all_consumes_snapshot() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
      nodes:
      - id: ModuleDoc
        signature: //! Task-queue instantiation notes.
```
/answer 4eac-131 allow
ReasoningToken: 135
Control Request:
```yaml
id: 4eac-132
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
    ids:
    - OpKind
    - OpParams
    - impl_OpParams
    - Op
    - OpDesc
    - impl_OpDesc
    - HostData
    - Value
    - OperationNode
    - impl_OperationNode
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
    ids:
    - 2iXFMc|Cm8Kf6
    - ModelId
    - IndexId
    - ResourceKind
    - EngineSelector
    - Affinity
    - Requirements
    - impl_Requirements
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
    ids:
    - jAY2Rz|M8Kowd
    - TaskKind
    - TaskMeta
    - Payload
    - TaskQueue
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - PlannerError
    - ConfigError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
    ids:
    - TextRole
    - Output
    - EmbeddingParams
    - SimilarityParams
    - InferenceParams
  reason: "Exakte Typdefinitionen lesen, auf denen der Planner aufbaut"
```
/answer 4eac-132 allow
Control Request:
```yaml
id: 4eac-133
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
      - id: impl_OpParams
        type: ItemImpl
        signature: "impl OpParams {"
        children:
        - id: impl_OpParams.new
          type: ImplItemFn
          code: |2
                pub fn new(requires: Requirements) -> Self {
                    Self {
                        requires,
                        top_k: None,
                        dim: None,
                    }
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
      - id: impl_OpDesc
        type: ItemImpl
        signature: "impl OpDesc {"
        children:
        - id: impl_OpDesc.new
          type: ImplItemFn
          code: |2
                pub fn new(kind: OpKind, params: OpParams) -> Self {
                    Self { kind, params }
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
                pub fn new(
                    op: Arc<dyn Op>,
                    input: Value,
                    materialize: bool,
                    task: Arc<TaskCtx>,
                ) -> Self {
                    tracing::debug!(
                        task = task.id(), kind = ? op.kind(), resource = ? op.params().requires
                        .resource, materialize, "op created"
                    );
                    Self {
                        op,
                        input,
                        output: OnceLock::new(),
                        materialize,
                        task,
                    }
                }
        - id: impl_OperationNode.set_output
          type: ImplItemFn
          code: |2
                /// Sets the output once; a second call returns the rejected value.
                pub fn set_output(&self, value: Value) -> Result<(), Value> {
                    self.output.set(value)
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
      nodes:
      - id: 2iXFMc|Cm8Kf6
        type: ImportGroup
        code: |
          use crate::resident::memory::{DeviceKind, Region};
          use crate::resident::precision::CapabilitySet;
      - id: ModelId
        type: ItemStruct
        code: |
          #[derive(Clone, Debug, PartialEq, Eq, Hash)]
          pub struct ModelId(pub String);
      - id: IndexId
        type: ItemStruct
        code: |
          #[derive(Clone, Debug, PartialEq, Eq, Hash)]
          pub struct IndexId(pub String);
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
      - id: Affinity
        type: ItemEnum
        code: |
          /// Placement hint; inactive in the scheduler for now.
          #[non_exhaustive]
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub enum Affinity {
              Handle(Region),
              Device(DeviceId),
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
      - id: impl_Requirements
        type: ItemImpl
        signature: "impl Requirements {"
        children:
        - id: impl_Requirements.new
          type: ImplItemFn
          code: |2
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
        - id: impl_Requirements.with_model
          type: ImplItemFn
          code: |2
                pub fn with_model(mut self, model: ModelId) -> Self {
                    self.model = Some(model);
                    self
                }
        - id: impl_Requirements.with_index
          type: ImplItemFn
          code: |2
                pub fn with_index(mut self, index: IndexId) -> Self {
                    self.index = Some(index);
                    self
                }
        - id: impl_Requirements.with_caps
          type: ImplItemFn
          code: |2
                pub fn with_caps(mut self, caps: CapabilitySet) -> Self {
                    self.caps = caps;
                    self
                }
        - id: impl_Requirements.with_affinity
          type: ImplItemFn
          code: |2
                pub fn with_affinity(mut self, affinity: Affinity) -> Self {
                    self.affinity = Some(affinity);
                    self
                }
        - id: impl_Requirements.satisfied_by
          type: ImplItemFn
          code: |2
                /// Affinity is deliberately ignored.
                pub fn satisfied_by(&self, dev: DeviceKind, available: &CapabilitySet) -> bool {
                    self.engine.accepts(dev) && available.satisfies(&self.caps)
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
      nodes:
      - id: jAY2Rz|M8Kowd
        type: ImportGroup
        code: |
          use super::error::TaskError;
          use super::params::{
              EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, VectorInput,
          };
          use crate::resident::queue::Queue;
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
      - id: TaskQueue
        type: ItemType
        code: |
          pub type TaskQueue = Queue<TaskMeta, Payload, Output, TaskError>;
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
      nodes:
      - id: TextRole
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
          #[non_exhaustive]
          pub enum TextRole {
              Query,
              #[default]
              Passage,
          }
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
      - id: SimilarityParams
        type: ItemStruct
        code: |
          #[derive(Clone, Debug)]
          #[non_exhaustive]
          pub struct SimilarityParams {
              index: IndexId,
              top_k: usize,
              scope: Scope,
              precision: f32,
          }
      - id: InferenceParams
        type: ItemStruct
        code: |
          #[derive(Clone, Debug)]
          #[non_exhaustive]
          pub struct InferenceParams {
              model: Option<ModelId>,
              precision: f32,
          }
```
/answer 4eac-133 allow
Text:
Die Verträge (OpDesc, Requirements, Payload, StarvedSnapshot, PlannerError) sind klar, der Planner aber noch ein leerer Stub. Als Nächstes prüfe ich, wie der Processor Ops konsumiert und welche Capability-Achsen existieren.


Control Request:
```yaml
id: 4eac-134
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/ops.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/axis.rs
  reason: Konsum der Ops im Processor und bekannte Capability-Achsen verstehen
```
/answer 4eac-134 allow
ReasoningToken: 420
Control Request:
```yaml
id: 4eac-135
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/ops.rs
      nodes:
      - id: fb1s3C|M4aGfc
        signature: "use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};"
      - id: Collect
        signature: "pub type Collect = Box<dyn FnOnce() -> Result<Output, TaskError> + Send>;"
      - id: TaskRun
        signature: /// Per-task completion bookkeeping shared by all of the task's operations. pub…
        docstring: Per-task completion bookkeeping shared by all of the task's operations.
      - id: impl_TaskRun
        signature: "impl TaskRun {"
        children:
        - id: impl_TaskRun.new
          signature: "pub fn new(     ctx: Arc<TaskCtx>,     state: Arc<TaskState<Output, TaskError>>…"
        - id: impl_TaskRun.ctx
          signature: "pub fn ctx(&self) -> &Arc<TaskCtx> {"
        - id: impl_TaskRun.is_cancelled
          signature: "pub fn is_cancelled(&self) -> bool {"
        - id: impl_TaskRun.expire_if_due
          signature: /// Fails the task with a timeout once its deadline has passed; returns whether…
          docstring: Fails the task with a timeout once its deadline has passed; returns whether it …
        - id: impl_TaskRun.complete_once
          signature: "fn complete_once(&self, result: impl FnOnce() -> Result<Output, TaskError>) {"
        - id: impl_TaskRun.finish
          signature: "/// Completes the task with its collected result. pub fn finish(&self) {"
          docstring: Completes the task with its collected result.
        - id: impl_TaskRun.op_done
          signature: /// Closes one operation; the last one of a healthy task completes it. pub fn o…
          docstring: Closes one operation; the last one of a healthy task completes it.
        - id: impl_TaskRun.skip
          signature: /// Closes an operation that is dropped unexecuted because the task already fai…
          docstring: Closes an operation that is dropped unexecuted because the task already failed.
        - id: impl_TaskRun.fail
          signature: /// Records the first error and completes the task with it. Does not touch the …
          docstring: Records the first error and completes the task with it. Does not touch the op c…
      - id: OpJob
        signature: "pub struct OpJob {"
      - id: impl_OpJob
        signature: "impl OpJob {"
        children:
        - id: impl_OpJob.new
          signature: "pub fn new(node: Arc<OperationNode>, run: Arc<TaskRun>) -> Self {"
        - id: impl_OpJob.params
          signature: "pub fn params(&self) -> &OpParams {"
      - id: Side
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum Side {"
      - id: OpQueue
        signature: "pub type OpQueue = Queue<OpParams, OpJob, (), ()>;"
      - id: W2aLBb|xTscqP
        signature: "const SHARED_SKIP_LIMIT: usize = 8;"
      - id: Counted
        signature: "struct Counted {"
      - id: impl_Counted
        signature: "impl Counted {"
        children:
        - id: impl_Counted.new
          signature: "fn new(relays: Vec<Arc<Notify>>) -> Self {"
        - id: impl_Counted.push
          signature: "fn push(&self, job: OpJob) {"
        - id: impl_Counted.claim
          signature: "/// Claims the oldest acceptable job, skipping at most `max_skip` foreign entri…"
          docstring: "Claims the oldest acceptable job, skipping at most `max_skip` foreign entries."
      - id: OpQueues
        signature: "/// CPU, GPU and shared operation queues. Shared is wired in but never filled b…"
        docstring: "CPU, GPU and shared operation queues. Shared is wired in but never filled by ro…"
      - id: impl_OpQueues
        signature: "impl OpQueues {"
        children:
        - id: impl_OpQueues.new
          signature: "pub fn new(cpu_wake: Arc<Notify>, gpu_wake: Arc<Notify>, has_gpu: bool) -> Self…"
        - id: impl_OpQueues.route
          signature: "pub fn route(&self, engine: EngineSelector) -> Result<Side, OrchestrationError>…"
        - id: impl_OpQueues.push
          signature: "pub fn push(&self, job: OpJob) -> Result<(), OrchestrationError> {"
        - id: impl_OpQueues.push_shared
          signature: "pub fn push_shared(&self, job: OpJob) {"
        - id: impl_OpQueues.side
          signature: "fn side(&self, side: Side) -> &Counted {"
        - id: impl_OpQueues.claim
          signature: "/// Own queue first, then shared. pub fn claim(     &self,     side: Side,     …"
          docstring: "Own queue first, then shared."
        - id: impl_OpQueues.queued
          signature: "pub fn queued(&self, side: Side) -> usize {"
        - id: impl_OpQueues.total_queued
          signature: "pub fn total_queued(&self) -> usize {"
      - id: testutil
        signature: "#[cfg(test)] pub(crate) mod testutil {"
        children:
        - id: testutil.dCtZYA|LDZ8RQ
          signature: use super::*;
        - id: testutil.node
          signature: "pub fn node(     task: &Arc<TaskCtx>,     resource: ResourceKind,     engine: E…"
        - id: testutil.run
          signature: "pub fn run(id: u64) -> (Arc<TaskRun>, Arc<TaskState<Output, TaskError>>) {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.w0kGdx|63BXKW
          signature: use super::testutil::*;
        - id: tests.queues
          signature: "fn queues(has_gpu: bool) -> OpQueues {"
        - id: tests.job
          signature: "fn job(run: &Arc<TaskRun>, resource: ResourceKind, engine: EngineSelector) -> O…"
        - id: tests.any
          signature: "fn any(_: &OpParams) -> bool {"
        - id: tests.routes_by_engine_and_rejects_missing_gpu
          signature: "#[test] fn routes_by_engine_and_rejects_missing_gpu() {"
        - id: tests.own_queue_precedes_shared_and_fifo_holds
          signature: "#[test] fn own_queue_precedes_shared_and_fifo_holds() {"
        - id: tests.foreign_head_does_not_block_shared_queue
          signature: "#[test] fn foreign_head_does_not_block_shared_queue() {"
        - id: tests.cancelled_task_ops_are_dropped_at_claim
          signature: "#[test] fn cancelled_task_ops_are_dropped_at_claim() {"
        - id: tests.last_op_completes_task_once
          signature: "#[test] fn last_op_completes_task_once() {"
        - id: tests.followups_counted_before_predecessor_closes
          signature: "#[test] fn followups_counted_before_predecessor_closes() {"
        - id: tests.first_error_wins_and_completes_exactly_once
          signature: "#[test] fn first_error_wins_and_completes_exactly_once() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
      nodes:
      - id: I6MAMl|9uLxLO
        signature: "use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering};"
      - id: ResourceState
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum ResourceState {"
      - id: impl_ResourceState
        signature: "impl ResourceState {"
        children:
        - id: impl_ResourceState.code
          signature: "fn code(self) -> u8 {"
        - id: impl_ResourceState.from_code
          signature: "fn from_code(v: u8) -> Self {"
      - id: ResourceSlot
        signature: /// Lifecycle cell of one lazily initialised resource. Time is passed in by the…
        docstring: Lifecycle cell of one lazily initialised resource. Time is passed in by the cal…
      - id: impl_ResourceSlot
        signature: "impl ResourceSlot {"
        children:
        - id: impl_ResourceSlot.new
          signature: "pub fn new(pins: Arc<PinCount>) -> Self {"
        - id: impl_ResourceSlot.state
          signature: "pub fn state(&self) -> ResourceState {"
        - id: impl_ResourceSlot.transition
          signature: "fn transition(&self, from: ResourceState, to: ResourceState) -> bool {"
        - id: impl_ResourceSlot.begin_load
          signature: /// Only the caller that wins this transition performs the load. pub fn begin_l…
          docstring: Only the caller that wins this transition performs the load.
        - id: impl_ResourceSlot.finish_load
          signature: "pub fn finish_load(&self, ok: bool, now_ms: u64, backoff_ms: u64) {"
        - id: impl_ResourceSlot.enter
          signature: /// Registers an operation in flight; fails unless the resource is `Ready`. pub…
          docstring: Registers an operation in flight; fails unless the resource is `Ready`.
        - id: impl_ResourceSlot.leave
          signature: "pub fn leave(&self, now_ms: u64) {"
        - id: impl_ResourceSlot.in_flight
          signature: "pub fn in_flight(&self) -> usize {"
        - id: impl_ResourceSlot.try_evict
          signature: "/// `Ready -> Evicting` only when idle, unleased and past the TTL. pub fn try_e…"
          docstring: "`Ready -> Evicting` only when idle, unleased and past the TTL."
        - id: impl_ResourceSlot.finish_evict
          signature: "pub fn finish_evict(&self) {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.mwhpHE|Z4s05W
          signature: use super::*;
        - id: tests.slot
          signature: "fn slot() -> (ResourceSlot, Arc<PinCount>) {"
        - id: tests.ready
          signature: "fn ready(s: &ResourceSlot) {"
        - id: tests.single_loader_wins
          signature: "#[test] fn single_loader_wins() {"
        - id: tests.failed_load_backs_off_until_deadline
          signature: "#[test] fn failed_load_backs_off_until_deadline() {"
        - id: tests.enter_requires_ready
          signature: "#[test] fn enter_requires_ready() {"
        - id: tests.evict_waits_for_ttl_and_in_flight
          signature: "#[test] fn evict_waits_for_ttl_and_in_flight() {"
        - id: tests.open_lease_blocks_eviction
          signature: "#[test] fn open_lease_blocks_eviction() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Well-known axes and values for the target hardware (GTX 1660, sm_75)."
      - id: D6Tc1t|3ttI7x
        signature: use std::sync::LazyLock;
      - id: cX8Pw2|HaFKmz
        signature: "macro_rules! known_ids {"
      - id: gpu_sm75
        signature: /// Upper bound of what the GPU can offer; the real set is measured at runtime.…
        docstring: Upper bound of what the GPU can offer; the real set is measured at runtime.
      - id: cpu
        signature: "/// Numeric axes (`threads`, `tile`) are not listed. pub fn cpu() -> Capability…"
        docstring: "Numeric axes (`threads`, `tile`) are not listed."
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.gpu_offers_no_bf16
          signature: "#[test] fn gpu_offers_no_bf16() {"
        - id: tests.gpu_satisfies_cpu_independent_requirements
          signature: "#[test] fn gpu_satisfies_cpu_independent_requirements() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/axis.rs
      nodes:
      - id: ModuleDoc
        signature: //! Extensible axis and value identifiers.
      - id: v3RfJl|Xp4U57
        signature: use std::fmt;
      - id: vAh2Ds|1Xi56W
        signature: "macro_rules! string_id {"
      - id: dC7yQX|7HVxmG
        signature: string_id!(
      - id: impl_ValueId
        signature: "impl ValueId {"
        children:
        - id: impl_ValueId.numeric
          signature: "pub fn numeric(n: u32) -> Self {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.LwlKpn|JFVpeI
          signature: use super::*;
        - id: tests.ids_compare_by_content
          signature: "#[test] fn ids_compare_by_content() {"
        - id: tests.display_is_plain
          signature: "#[test] fn display_is_plain() {"
```
/answer 4eac-135 allow
Control Request:
```yaml
id: 4eac-136
toolName: grep
arguments:
  items:
  - pattern: planner|Planner
    directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    - /home/user/xyan/xy.ai.workbench/rag/tests
    include:
    - '*.rs'
    limit: 30
  - pattern: toml
    directory:
    - /home/user/xyan/xy.ai.workbench/rag
    include:
    - Cargo.lock
    - Cargo.toml
    limit: 10
  reason: Bestehende Planner-Referenzen und ob eine toml-Crate vorhanden ist
```
/answer 4eac-136 allow
Control Request:
```yaml
id: 4eac-137
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      - /home/user/xyan/xy.ai.workbench/rag/tests
      matches:
      - path: resident/mod.rs
        matches:
        - lineno: 16
          match: pub mod planner;
      - path: resident/processor/tests.rs
        matches:
        - lineno: 5
          match: "use crate::resident::error::{PlannerError, ResidentError};"
        - lineno: 48
          match: "PlannerError::UnknownModel {"
        - lineno: 124
          match: "PlannerError::UnknownModel {"
      - path: resident/processor/config.rs
        matches:
        - lineno: 8
          match: /// Lower waterline of queued operations before the planner is asked for more.
      - path: resident/processor/hub.rs
        matches:
        - lineno: 24
          match: /// Task-to-operation expansion; implemented by the planner.
      - path: resident/processor/ops.rs
        matches:
        - lineno: 311
          match: "run.fail(crate::resident::error::PlannerError::UnknownModel { model: \"m\".into() }.into());"
        - lineno: 343
          match: "run.fail(crate::resident::error::PlannerError::UnknownModel { model: \"a\".into() }.into());"
        - lineno: 344
          match: "run.fail(crate::resident::error::PlannerError::UnknownModel { model: \"b\".into() }.into());"
      - path: resident/facade/resident.rs
        matches:
        - lineno: 11
          match: "use crate::resident::error::{ConfigError, PlannerError, ResidentError};"
        - lineno: 41
          match: "/// Stands in until a planner is supplied: tasks fail instead of waiting forever."
        - lineno: 51
          match: "Err(PlannerError::MissingCapability {"
        - lineno: 52
          match: "capability: \"planner\".into(),"
      - path: resident/error.rs
        matches:
        - lineno: 67
          match: /// Planner/task-expansion layer errors.
        - lineno: 70
          match: "pub enum PlannerError {"
        - lineno: 71
          match: "#[error(\"invalid planner config: {reason}\")]"
        - lineno: 78
          match: "impl IsRetryable for PlannerError {"
        - lineno: 122
          match: "Planner(#[from] PlannerError),"
        - lineno: 135
          match: "ResidentError::Planner(e) => e.is_retryable(),"
        - lineno: 152
          match: assert_send_sync_static::<PlannerError>();
        - lineno: 172
          match: "let e: ResidentError = PlannerError::UnknownModel {"
        - lineno: 176
          match: "assert!(matches!(e, ResidentError::Planner(_)));"
        - lineno: 216
          match: "let fatal: ResidentError = PlannerError::InvalidConfig {"
      - path: resident/op/mod.rs
        matches:
        - lineno: 171
          match: use crate::resident::error::PlannerError;
        - lineno: 179
          match: "PlannerError::InvalidConfig {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-kernels/Cargo.toml
        matches:
        - lineno: 4
          match: "# \"normalize\" Cargo.toml files for maximal compatibility"
        - lineno: 8
          match: "# If you are reading this file be aware that the original Cargo.toml"
        - lineno: 10
          match: "# See Cargo.toml.orig for the original contents."
```
/answer 4eac-137 allow
Text:
 The processor already defines a hub-side expansion trait. I'll read it, along with the facade stub that stands in for the planner.
Control Request:
```yaml
id: 4eac-138
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/hub.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
    ids: []
  reason: Expander-Schnittstelle des Hubs und Planner-Platzhalter in der Fassade ansehen
```
/answer 4eac-138 allow
Control Request:
```yaml
id: 4eac-139
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/hub.rs
      nodes:
      - id: mo8wep|Hrrsil
        signature: "use std::sync::atomic::{AtomicBool, Ordering};"
      - id: Expansion
        signature: "pub struct Expansion {"
      - id: Expander
        signature: /// Task-to-operation expansion; implemented by the planner. pub trait Expander…
        docstring: Task-to-operation expansion; implemented by the planner.
        children:
        - id: Expander.first_resource
          signature: "/// Resource the task's first operation needs, used to serve starved hints firs…"
          docstring: "Resource the task's first operation needs, used to serve starved hints first."
        - id: Expander.expand
          signature: fn expand(
      - id: Step
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] pub enum Step {"
      - id: Engine
        signature: /// Subengine side of the pull model. Each method does at most one unit of work…
        docstring: Subengine side of the pull model. Each method does at most one unit of work.
        children:
        - id: Engine.harvest
          signature: /// Takes over finished device work and returns results to their tasks. fn harv…
          docstring: Takes over finished device work and returns results to their tasks.
        - id: Engine.feed
          signature: "/// Synchronises indexes and launches work onto a free lane. fn feed(&self, _hu…"
          docstring: Synchronises indexes and launches work onto a free lane.
        - id: Engine.step
          signature: /// Claims and runs operations.
          docstring: Claims and runs operations.
      - id: Wakeups
        signature: "pub struct Wakeups {"
      - id: impl_Wakeups
        signature: "impl Wakeups {"
        children:
        - id: impl_Wakeups.new
          signature: "pub fn new() -> Self {"
        - id: impl_Wakeups.relays
          signature: /// Relays for the task queue so every submit wakes both worker groups. pub fn …
          docstring: Relays for the task queue so every submit wakes both worker groups.
        - id: impl_Wakeups.side
          signature: "pub fn side(&self, side: Side) -> &Arc<Notify> {"
      - id: impl_Default_for_Wakeups
        signature: "impl Default for Wakeups {"
        children:
        - id: impl_Default_for_Wakeups.default
          signature: "fn default() -> Self {"
      - id: RegionCheck
        signature: /// Reports whether a region handle still refers to the current generation of i…
        docstring: Reports whether a region handle still refers to the current generation of its a…
      - id: Hub
        signature: "pub struct Hub {"
      - id: impl_Hub
        signature: "impl Hub {"
        children:
        - id: impl_Hub.new
          signature: "pub(super) fn new(     cfg: WorkerConfig,     tasks: Arc<TaskQueue>,     wakeup…"
        - id: impl_Hub.config
          signature: "pub fn config(&self) -> &WorkerConfig {"
        - id: impl_Hub.queues
          signature: "pub fn queues(&self) -> &OpQueues {"
        - id: impl_Hub.wakeups
          signature: "pub fn wakeups(&self) -> &Wakeups {"
        - id: impl_Hub.starved
          signature: "pub fn starved(&self, side: Side) -> &StarvedSet {"
        - id: impl_Hub.set_region_check
          signature: /// Enables the debug-build staleness assertion for device handles at claim tim…
          docstring: Enables the debug-build staleness assertion for device handles at claim time.
        - id: impl_Hub.is_shutdown
          signature: "pub(super) fn is_shutdown(&self) -> bool {"
        - id: impl_Hub.begin_shutdown
          signature: "pub(super) fn begin_shutdown(&self) {"
        - id: impl_Hub.claim
          signature: /// A pick of a resource kind satisfies that kind's starved hint. pub fn claim(…
          docstring: A pick of a resource kind satisfies that kind's starved hint.
        - id: impl_Hub.complete_op
          signature: /// Closes `job` successfully after registering and enqueueing its follow-up op…
          docstring: Closes `job` successfully after registering and enqueueing its follow-up operat…
        - id: impl_Hub.fail_op
          signature: "pub fn fail_op(&self, job: OpJob, err: ResidentError) {"
        - id: impl_Hub.enqueue
          signature: "/// Counts first, then enqueues, so the task's open-op counter never dips to ze…"
          docstring: "Counts first, then enqueues, so the task's open-op counter never dips to zero e…"
        - id: impl_Hub.expand_if_needed
          signature: /// Expands one queued task when hinted or when the op buffer ran below its wat…
          docstring: Expands one queued task when hinted or when the op buffer ran below its waterli…
        - id: impl_Hub.expand_one
          signature: "fn expand_one(&self, hint: StarvedSnapshot) -> bool {"
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
      nodes:
      - id: RY1QMi|Bs72Bv
        signature: use std::collections::HashMap;
      - id: ResidentConfig
        signature: "#[derive(Clone, Debug)] #[non_exhaustive] pub struct ResidentConfig {"
      - id: impl_ResidentConfig
        signature: "impl ResidentConfig {"
        children:
        - id: impl_ResidentConfig.model
          signature: "pub fn model(&self) -> &ModelId {"
        - id: impl_ResidentConfig.workers
          signature: "pub fn workers(&self) -> &WorkerConfig {"
        - id: impl_ResidentConfig.min_op_buffer
          signature: "pub fn min_op_buffer(&self) -> usize {"
      - id: Unplanned
        signature: "/// Stands in until a planner is supplied: tasks fail instead of waiting foreve…"
        docstring: "Stands in until a planner is supplied: tasks fail instead of waiting forever."
      - id: impl_Expander_for_Unplanned
        signature: "impl Expander for Unplanned {"
        children:
        - id: impl_Expander_for_Unplanned.expand
          signature: "fn expand(     &self,     _meta: &TaskMeta,     _payload: Payload,     _ctx: &A…"
      - id: NoEngine
        signature: struct NoEngine;
      - id: impl_Engine_for_NoEngine
        signature: "impl Engine for NoEngine {"
        children:
        - id: impl_Engine_for_NoEngine.step
          signature: "fn step(&self, _hub: &Hub) -> Step {"
      - id: ResidentBuilder
        signature: "#[derive(Default)] pub struct ResidentBuilder {"
      - id: impl_ResidentBuilder
        signature: "impl ResidentBuilder {"
        children:
        - id: impl_ResidentBuilder.new
          signature: "pub fn new() -> Self {"
        - id: impl_ResidentBuilder.model
          signature: "pub fn model(mut self, model: ModelId) -> Self {"
        - id: impl_ResidentBuilder.worker_hint
          signature: "pub fn worker_hint(mut self, workers: WorkerConfig) -> Self {"
        - id: impl_ResidentBuilder.min_op_buffer
          signature: "pub fn min_op_buffer(mut self, n: usize) -> Self {"
        - id: impl_ResidentBuilder.expander
          signature: "pub fn expander(mut self, expander: Arc<dyn Expander>) -> Self {"
        - id: impl_ResidentBuilder.engines
          signature: /// GPU workers are only started when a GPU engine is supplied. pub fn engines(…
          docstring: GPU workers are only started when a GPU engine is supplied.
        - id: impl_ResidentBuilder.init
          signature: /// Starts the processor and its worker threads; model and device resources are…
          docstring: Starts the processor and its worker threads; model and device resources are
      - id: Residency
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[non_exhaustive] pub enum Residen…"
      - id: IndexInfo
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[non_exhaustive] pub struct Index…"
      - id: IndexEntry
        signature: "struct IndexEntry {"
      - id: Resident
        signature: "pub struct Resident {"
      - id: impl_Resident
        signature: "impl Resident {"
        children:
        - id: impl_Resident.builder
          signature: "pub fn builder() -> ResidentBuilder {"
        - id: impl_Resident.config
          signature: "pub fn config(&self) -> &ResidentConfig {"
        - id: impl_Resident.queue
          signature: "pub fn queue(&self) -> &Arc<TaskQueue> {"
        - id: impl_Resident.processor
          signature: "pub fn processor(&self) -> &Processor {"
        - id: impl_Resident.add_index
          signature: /// Registers the index without loading it; loading is triggered by a task. pub…
          docstring: Registers the index without loading it; loading is triggered by a task.
        - id: impl_Resident.index_info
          signature: "pub fn index_info(&self, id: &IndexId) -> Option<IndexInfo> {"
        - id: impl_Resident.submit_embedding
          signature: "pub fn submit_embedding(     &self,     input: TextInput,     params: Embedding…"
        - id: impl_Resident.submit_similarity
          signature: "pub fn submit_similarity(     &self,     input: VectorInput,     params: Simila…"
        - id: impl_Resident.submit_inference
          signature: "pub fn submit_inference(     &self,     input: PairInput,     params: Inference…"
        - id: impl_Resident.shutdown
          signature: "/// Joins the workers, then cancels every task that no worker has claimed. pub …"
          docstring: "Joins the workers, then cancels every task that no worker has claimed."
        - id: impl_Resident.submit
          signature: "fn submit(&self, kind: TaskKind, precision: f32, payload: Payload) -> TaskHandl…"
        - id: impl_Resident.lock_indexes
          signature: "fn lock_indexes(&self) -> std::sync::MutexGuard<'_, HashMap<IndexId, IndexEntry…"
      - id: impl_Drop_for_Resident
        signature: "impl Drop for Resident {"
        children:
        - id: impl_Drop_for_Resident.drop
          signature: "fn drop(&mut self) {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.MMewlK|l8O41s
          signature: use std::time::Duration;
        - id: tests.resident
          signature: "fn resident() -> Resident {"
        - id: tests.serve_one
          signature: "fn serve_one(r: &Resident, out: Output) -> bool {"
        - id: tests.builder_requires_model
          signature: "#[test] fn builder_requires_model() {"
        - id: tests.builder_rejects_zero_op_buffer
          signature: "#[test] fn builder_rejects_zero_op_buffer() {"
        - id: tests.submitted_task_resolves_with_output
          signature: "#[tokio::test] async fn submitted_task_resolves_with_output() {"
        - id: tests.nan_precision_fails_without_enqueue
          signature: "#[tokio::test] async fn nan_precision_fails_without_enqueue() {"
        - id: tests.dropping_handle_cancels_queued_task
          signature: "#[test] fn dropping_handle_cancels_queued_task() {"
        - id: tests.dropping_handle_after_claim_keeps_result
          signature: "#[test] fn dropping_handle_after_claim_keeps_result() {"
        - id: tests.wait_timeout_reports_timeout_and_cancels
          signature: "#[tokio::test(start_paused = true)] async fn wait_timeout_reports_timeout_and_c…"
        - id: tests.shutdown_cancels_pending_tasks
          signature: "#[test] fn shutdown_cancels_pending_tasks() {"
        - id: tests.submit_wakes_workers_and_unplanned_task_fails
          signature: "#[tokio::test] async fn submit_wakes_workers_and_unplanned_task_fails() {"
        - id: tests.blocking_wrapper_returns_task_result
          signature: "#[test] fn blocking_wrapper_returns_task_result() {"
        - id: tests.add_index_registers_without_loading
          signature: "#[test] fn add_index_registers_without_loading() {"
        - id: tests.builder_min_op_buffer_overrides_worker_hint
          signature: "#[test] fn builder_min_op_buffer_overrides_worker_hint() {"
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 4eac-139 allow
Control Request:
```yaml
id: 4eac-13a
toolName: grep
arguments:
  items:
  - pattern: Expand|expand
    directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    include:
    - '*.rs'
    limit: 40
  reason: Alle Verwendungsstellen der Expansions-Schnittstelle finden
```
/answer 4eac-13a allow
Control Request:
```yaml
id: 4eac-13b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: processor/mod.rs
        matches:
        - lineno: 21
          match: "pub use hub::{Engine, Expander, Expansion, Hub, RegionCheck, Step, Wakeups};"
      - path: processor/worker.rs
        matches:
        - lineno: 6
          match: "use super::hub::{Engine, Expander, Hub, Step, Wakeups};"
        - lineno: 19
          match: "expander: Arc<dyn Expander>,"
        - lineno: 29
          match: "let hub = Arc::new(Hub::new(cfg.clone(), tasks, wakeups, expander));"
        - lineno: 109
          match: hub.expand_if_needed(side)
      - path: processor/tests.rs
        matches:
        - lineno: 18
          match: "struct TestExpander {"
        - lineno: 20
          match: "fail_expand: bool,"
        - lineno: 21
          match: "expanded: Mutex<Vec<u64>>,"
        - lineno: 23
          match: "impl TestExpander {"
        - lineno: 27
          match: "fail_expand: false,"
        - lineno: 28
          match: "expanded: Mutex::new(vec![]),"
        - lineno: 32
          match: "impl Expander for TestExpander {"
        - lineno: 39
          match: fn expand(
        - lineno: 45
          match: self.expanded.lock().push(meta.id);
        - lineno: 46
          match: "if self.fail_expand {"
        - lineno: 173
          match: "fn hub(cfg: WorkerConfig, expander: Arc<dyn Expander>) -> (Hub, Arc<TaskQueue>) {"
        - lineno: 176
          match: "(Hub::new(cfg, tasks.clone(), wakeups, expander), tasks)"
        - lineno: 188
          match: "expander: Arc<dyn Expander>,"
        - lineno: 193
          match: "let p = Processor::start(cfg, tasks.clone(), wakeups, expander, cpu, None).unwrap();"
        - lineno: 201
          match: "TestExpander::new(3),"
        - lineno: 215
          match: "TestExpander::new(4),"
        - lineno: 228
          match: "TestExpander::new(1),"
        - lineno: 237
          match: "let expander = Arc::new(TestExpander {"
        - lineno: 239
          match: "fail_expand: true,"
        - lineno: 240
          match: "expanded: Mutex::new(vec![]),"
        - lineno: 243
          match: "let (_p, tasks) = start(WorkerConfig::new(1, 0).with_cpu_nice(0), expander, cpu);"
        - lineno: 249
          match: struct GpuExpander;
        - lineno: 250
          match: "impl Expander for GpuExpander {"
        - lineno: 251
          match: fn expand(
        - lineno: 266
          match: "Arc::new(GpuExpander),"
        - lineno: 277
          match: "TestExpander::new(1),"
        - lineno: 286
          match: "let (hub, tasks) = hub(cfg, TestExpander::new(1));"
        - lineno: 301
          match: let expander = TestExpander::new(1);
        - lineno: 302
          match: "let (hub, tasks) = hub(cfg, expander.clone());"
        - lineno: 306
          match: assert!(expander.expanded.lock().is_empty());
        - lineno: 309
          match: "assert_eq!(* expander.expanded.lock(), vec![1]);"
        - lineno: 315
          match: let expander = TestExpander::new(1);
        - lineno: 316
          match: "let (hub, tasks) = hub(cfg, expander.clone());"
        - lineno: 323
          match: "assert_eq!(* expander.expanded.lock(), vec![1, 2]);"
        - lineno: 328
          match: let expander = TestExpander::new(1);
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4eac-13b allow
ReasoningToken: 73
Control Request:
```yaml
id: 4eac-13c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/hub.rs
    ids:
    - Expansion
    - Expander
    - Expander.first_resource
    - Expander.expand
    - impl_Hub.complete_op
    - impl_Hub.enqueue
    - impl_Hub.expand_one
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
    ids:
    - Unplanned
    - impl_Expander_for_Unplanned
  reason: "Exakte Signatur des Hub-Expander und Expansion-Datentyp lesen, um den Planner anzubinden"
```
/answer 4eac-13c allow
Control Request:
```yaml
id: 4eac-13d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/hub.rs
      nodes:
      - id: Expansion
        type: ItemStruct
        code: |
          pub struct Expansion {
              pub ops: Vec<Arc<OperationNode>>,
              pub collect: Collect,
          }
      - id: Expander
        type: ItemTrait
        signature: /// Task-to-operation expansion; implemented by the planner. pub trait Expander…
        docstring: Task-to-operation expansion; implemented by the planner.
        children:
        - id: Expander.first_resource
          type: TraitItemFn
          code: |2
                /// Resource the task's first operation needs, used to serve starved hints first.
                fn first_resource(&self, _meta: &TaskMeta) -> Option<ResourceKind> {
                    None
                }
        - id: Expander.expand
          type: TraitItemFn
          code: |2
                fn expand(&self, meta: &TaskMeta, payload: Payload, ctx: &Arc<TaskCtx>) -> Result<Expansion, ResidentError>;
      - id: Expander.first_resource
        type: TraitItemFn
        code: |2
              /// Resource the task's first operation needs, used to serve starved hints first.
              fn first_resource(&self, _meta: &TaskMeta) -> Option<ResourceKind> {
                  None
              }
      - id: Expander.expand
        type: TraitItemFn
        code: |2
              fn expand(&self, meta: &TaskMeta, payload: Payload, ctx: &Arc<TaskCtx>) -> Result<Expansion, ResidentError>;
      - id: impl_Hub.complete_op
        type: ImplItemFn
        code: |2
              /// Closes `job` successfully after registering and enqueueing its follow-up operations.
              pub fn complete_op(&self, job: OpJob, followups: Vec<Arc<OperationNode>>) {
                  let OpJob { run, .. } = job;
                  if !followups.is_empty() {
                      if run.expire_if_due() {
                          run.skip();
                          return;
                      }
                      if let Err(err) = self.enqueue(&run, followups) {
                          run.fail(err.into());
                          run.skip();
                          return;
                      }
                  }
                  run.op_done();
              }
      - id: impl_Hub.enqueue
        type: ImplItemFn
        code: |2
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
      - id: impl_Hub.expand_one
        type: ImplItemFn
        code: |2
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
                              let deadline = self.cfg.task_timeout.map(|t| Instant::now() + t);
                                  let run = TaskRun::new(ctx, state, collect, deadline);
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
      nodes:
      - id: Unplanned
        type: ItemStruct
        code: |
          /// Stands in until a planner is supplied: tasks fail instead of waiting forever.
          struct Unplanned;
      - id: impl_Expander_for_Unplanned
        type: ItemImpl
        signature: "impl Expander for Unplanned {"
        children:
        - id: impl_Expander_for_Unplanned.expand
          type: ImplItemFn
          code: |2
                fn expand(
                    &self,
                    _meta: &TaskMeta,
                    _payload: Payload,
                    _ctx: &Arc<TaskCtx>,
                ) -> Result<Expansion, ResidentError> {
                    Err(PlannerError::MissingCapability {
                        capability: "planner".into(),
                    }
                    .into())
                }
```
/answer 4eac-13d allow
ReasoningToken: 667
Control Request:
```yaml
id: 4eac-13e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/tests.rs
    ids:
    - TestExpander
    - impl_Expander_for_TestExpander
    - GpuExpander
    - impl_Expander_for_GpuExpander
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
    ids:
    - cX8Pw2|HaFKmz
    - gpu_sm75
    - cpu
  reason: "Wie bestehende Expander Ops/Collect bauen, und welche Capability-Achsen existieren"
```
/answer 4eac-13e allow
Control Request:
```yaml
id: 4eac-13f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/tests.rs
      nodes:
      - id: TestExpander
        type: ItemStruct
        code: |
          struct TestExpander {
              ops_per_task: usize,
              fail_expand: bool,
              expanded: Mutex<Vec<u64>>,
          }
      - id: impl_Expander_for_TestExpander
        type: ItemImpl
        signature: "impl Expander for TestExpander {"
        children:
        - id: impl_Expander_for_TestExpander.first_resource
          type: ImplItemFn
          code: |2
                fn first_resource(&self, meta: &TaskMeta) -> Option<ResourceKind> {
                    match meta.kind {
                        TaskKind::Similarity => Some(ResourceKind::Scorer),
                        _ => None,
                    }
                }
        - id: impl_Expander_for_TestExpander.expand
          type: ImplItemFn
          code: |2
                fn expand(
                    &self,
                    meta: &TaskMeta,
                    _payload: Payload,
                    ctx: &Arc<TaskCtx>,
                ) -> Result<Expansion, ResidentError> {
                    self.expanded.lock().push(meta.id);
                    if self.fail_expand {
                        return Err(
                            PlannerError::UnknownModel {
                                model: "m".into(),
                            }
                                .into(),
                        );
                    }
                    let n = self.ops_per_task;
                    let ops: Vec<Arc<OperationNode>> = (0..n)
                        .map(|_| node(ctx, ResourceKind::Tokenizer, EngineSelector::Cpu))
                        .collect();
                    Ok(Expansion {
                        ops,
                        collect: Box::new(move || Ok(Output::Scores(vec![n as f32]))),
                    })
                }
      - id: TestExpander
        type: ItemStruct
        code: |
          struct TestExpander {
              ops_per_task: usize,
              fail_expand: bool,
              expanded: Mutex<Vec<u64>>,
          }
      - id: impl_Expander_for_TestExpander
        type: ItemImpl
        signature: "impl Expander for TestExpander {"
        children:
        - id: impl_Expander_for_TestExpander.first_resource
          type: ImplItemFn
          code: |2
                fn first_resource(&self, meta: &TaskMeta) -> Option<ResourceKind> {
                    match meta.kind {
                        TaskKind::Similarity => Some(ResourceKind::Scorer),
                        _ => None,
                    }
                }
        - id: impl_Expander_for_TestExpander.expand
          type: ImplItemFn
          code: |2
                fn expand(
                    &self,
                    meta: &TaskMeta,
                    _payload: Payload,
                    ctx: &Arc<TaskCtx>,
                ) -> Result<Expansion, ResidentError> {
                    self.expanded.lock().push(meta.id);
                    if self.fail_expand {
                        return Err(
                            PlannerError::UnknownModel {
                                model: "m".into(),
                            }
                                .into(),
                        );
                    }
                    let n = self.ops_per_task;
                    let ops: Vec<Arc<OperationNode>> = (0..n)
                        .map(|_| node(ctx, ResourceKind::Tokenizer, EngineSelector::Cpu))
                        .collect();
                    Ok(Expansion {
                        ops,
                        collect: Box::new(move || Ok(Output::Scores(vec![n as f32]))),
                    })
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
      nodes:
      - id: cX8Pw2|HaFKmz
        type: StatementGroup
        code: |
          macro_rules! known_ids {
              ($ty:ident: $($name:ident = $s:literal),+ $(,)?) => {
                  $(pub static $name: LazyLock<$ty> = LazyLock::new(|| $ty::new($s));)+
              };
          }

          known_ids!(AxisId:
              OPERANDS = "operands",
              ACCUMULATOR = "accumulator",
              SCORING = "scoring",
              FUSION = "fusion",
              BATCH = "batch",
              TILE = "tile",
              THREADS = "threads",
          );

          known_ids!(ValueId:
              FP32 = "fp32",
              FP16 = "fp16",
              FP16_TO_FP32 = "fp16->fp32",
              INT8 = "int8",
              // TODO: verify dp4a usage and bf16 behaviour on sm_75 hardware.
              INT8_DP4A = "int8-dp4a",
              ON = "on",
              OFF = "off",
          );
      - id: gpu_sm75
        type: ItemFn
        code: |
          /// Upper bound of what the GPU can offer; the real set is measured at runtime.
          /// Numeric axes (`batch`, `tile`) carry measured values and are not listed.
          pub fn gpu_sm75() -> CapabilitySet {
              let mut c = CapabilitySet::new();
              for v in [&FP32, &FP16] {
                  c.insert(&*OPERANDS, &**v);
                  c.insert(&*ACCUMULATOR, &**v);
              }
              for v in [&FP32, &FP16, &INT8_DP4A] {
                  c.insert(&*SCORING, &**v);
              }
              for v in [&ON, &OFF] {
                  c.insert(&*FUSION, &**v);
              }
              c
          }
      - id: cpu
        type: ItemFn
        code: |
          /// Numeric axes (`threads`, `tile`) are not listed.
          pub fn cpu() -> CapabilitySet {
              let mut c = CapabilitySet::new();
              for v in [&FP32, &FP16_TO_FP32] {
                  c.insert(&*OPERANDS, &**v);
              }
              for v in [&FP32, &INT8] {
                  c.insert(&*SCORING, &**v);
              }
              c
          }
```
/answer 4eac-13f allow
ReasoningToken: 2270
Control Request:
```yaml
id: 4eac-140
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/ops.rs
    ids:
    - impl_OpQueues.route
    - impl_OpQueues.push
    - OpQueues
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/starved.rs
    ids:
    - StarvedSnapshot
    - slot
  reason: "Routing von EngineSelector::Either und Aufbau des Starved-Snapshots prüfen, um Hint-Semantik festzulegen"
```
/answer 4eac-140 allow
Control Request:
```yaml
id: 4eac-141
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/ops.rs
      nodes:
      - id: impl_OpQueues.route
        type: ImplItemFn
        code: |2
              pub fn route(&self, engine: EngineSelector) -> Result<Side, OrchestrationError> {
                  match engine {
                      EngineSelector::Gpu if !self.has_gpu => Err(OrchestrationError::EngineUnavailable { engine }),
                      EngineSelector::Gpu => Ok(Side::Gpu),
                      _ => Ok(Side::Cpu),
                  }
              }
      - id: impl_OpQueues.push
        type: ImplItemFn
        code: |2
              pub fn push(&self, job: OpJob) -> Result<(), OrchestrationError> {
                  let side = self.route(job.params().requires.engine)?;
                  self.side(side).push(job);
                  Ok(())
              }
      - id: OpQueues
        type: ItemStruct
        code: |
          /// CPU, GPU and shared operation queues. Shared is wired in but never filled by routing.
          pub struct OpQueues {
              cpu: Counted,
              gpu: Counted,
              shared: Counted,
              has_gpu: bool,
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/starved.rs
      nodes:
      - id: StarvedSnapshot
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
          pub struct StarvedSnapshot(u32);
      - id: slot
        type: ItemFn
        code: |
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
```
/answer 4eac-141 allow
ReasoningToken: 4935
Control Request:
```yaml
id: 4eac-142
toolName: grep
arguments:
  items:
  - pattern: pub (enum|struct) \w+
    directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    include:
    - dtype.rs
    - mod.rs
    limit: 20
  - pattern: pub use|pub mod
    directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    include:
    - mod.rs
    limit: 40
  reason: Vorhandenen dtype-Typ und Re-Exports (Pfade für Imports) bestätigen
```
/answer 4eac-142 allow
Control Request:
```yaml
id: 4eac-143
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      matches:
      - path: dtype.rs
        matches:
        - lineno: 7
          match: pub struct UnsupportedDType(pub &'static str);
        - lineno: 11
          match: "pub enum DType {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      matches:
      - path: mod.rs
        matches:
        - lineno: 10
          match: pub use blocking::ResidentBlocking;
        - lineno: 11
          match: pub use error::TaskError;
        - lineno: 12
          match: pub use handle::TaskHandle;
        - lineno: 13
          match: "pub use params::{"
        - lineno: 17
          match: "pub use resident::{"
        - lineno: 20
          match: pub use crate::resident::processor::WorkerConfig;
        - lineno: 21
          match: "pub use task::{Payload, TaskKind, TaskMeta, TaskQueue};"
        - lineno: 5
          match: pub mod known;
        - lineno: 9
          match: "pub use axis::{AxisId, ValueId};"
        - lineno: 10
          match: pub use capability::CapabilitySet;
        - lineno: 11
          match: "pub use pareto::{pareto, positions};"
        - lineno: 12
          match: "pub use profile::{Profile, ProfileConfig};"
        - lineno: 6
          match: "pub use lease::{DeviceHandle, Lease, PinCount};"
        - lineno: 7
          match: "pub use requirements::{"
```
/answer 4eac-143 allow
Control Request:
```yaml
id: 4eac-144
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
    ids:
    - DType
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
    ids:
    - flBf5A|OTJLmF
  reason: DType-Varianten (Serde-tauglich?) und Re-Exports von facade/processor prüfen
```
/answer 4eac-144 allow
Control Request:
```yaml
id: 4eac-145
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/dtype.rs
      nodes:
      - id: DType
        type: ItemEnum
        code: |
          #[non_exhaustive]
          #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
          pub enum DType {
              F32,
              F16,
              Bf16,
              U8,
              U32,
              I64,
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Public resident facade API.
      - id: blocking
        signature: mod blocking;
      - id: error
        signature: mod error;
      - id: handle
        signature: mod handle;
      - id: params
        signature: mod params;
      - id: resident
        signature: mod resident;
      - id: task
        signature: mod task;
      - id: h0TGLs|2czqAS
        signature: pub use blocking::ResidentBlocking;
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
      nodes:
      - id: flBf5A|OTJLmF
        type: ImportGroup
        code: |
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
```
/answer 4eac-145 allow
Text:
Die Verträge sind geklärt: der Hub nutzt einen `Expander`-Trait, der fertige `OperationNode`s mit Eingabewerten erwartet. Deshalb baue ich den Planner als reine Funktion, die `Vec<PlannedOp>` liefert, prüfe noch die Re-Exports und schreibe dann den Code.


ReasoningToken: 360
Control Request:
```yaml
id: 4eac-146
toolName: grep
arguments:
  items:
  - pattern: ^\s+(EmbeddingParams|OpDesc|OpKind|ModelId|EngineSelector|Requirements|ResourceKind|PairInput|StarvedSnapshot)
    directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    include:
    - mod.rs
    limit: 20
  reason: "Sicherstellen, dass die benötigten Typen über facade/op re-exportiert werden"
```
/answer 4eac-146 allow
Control Request:
```yaml
id: 4eac-147
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      matches:
      - path: mod.rs
        matches:
        - lineno: 14
          match: "EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, TextRole,"
        - lineno: 188
          match: "Requirements::new(ResourceKind::Scorer, EngineSelector::Gpu),"
```
/answer 4eac-147 allow
Text:
 Ich lege jetzt Dependency und Module an.
Control Request:
```yaml
id: 4eac-148
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    id: uUBhmF|EGrpec
    old_text: arc-swap = "1.9.2"
    new_text: |-
      arc-swap = "1.9.2"
      toml = "0.9"
  reason: TOML-Parser für die Planner-Regelkonfiguration ergänzen
```
/answer 4eac-148 allow
ReasoningToken: 3051
Control Request:
```yaml
id: 4eac-149
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/config.rs
    source: |
      //! Per-model pipeline data; logic stays in the expanders.

      use serde::Deserialize;

      use crate::resident::error::PlannerError;
      use crate::resident::facade::TextRole;

      #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
      #[serde(rename_all = "lowercase")]
      pub enum Pooling {
          #[default]
          Mean,
          Cls,
          /// Token vectors are kept as they are.
          None,
      }

      #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
      #[serde(rename_all = "lowercase")]
      pub enum PipelineDtype {
          F16,
          #[default]
          F32,
      }

      #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
      #[serde(rename_all = "lowercase")]
      pub enum ScorerKind {
          #[default]
          Cosine,
          MaxSim,
      }

      #[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
      #[serde(default, deny_unknown_fields)]
      pub struct Prefixes {
          pub query: String,
          pub passage: String,
      }

      impl Prefixes {
          pub fn for_role(&self, role: TextRole) -> &str {
              match role {
                  TextRole::Query => &self.query,
                  TextRole::Passage => &self.passage,
              }
          }
      }

      #[non_exhaustive]
      #[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
      #[serde(default, deny_unknown_fields)]
      pub struct PipelineConfig {
          pub prefixes: Prefixes,
          pub pooling: Pooling,
          pub normalize: bool,
          pub max_seq_len: u32,
          pub dim: Option<u32>,
          pub dtype: PipelineDtype,
          pub scorer: ScorerKind,
      }

      impl Default for PipelineConfig {
          fn default() -> Self {
              Self {
                  prefixes: Prefixes::default(),
                  pooling: Pooling::default(),
                  normalize: true,
                  max_seq_len: 512,
                  dim: None,
                  dtype: PipelineDtype::default(),
                  scorer: ScorerKind::default(),
              }
          }
      }

      impl PipelineConfig {
          pub(super) fn validate(&self, model: &str) -> Result<(), PlannerError> {
              let invalid = |reason: &str| PlannerError::InvalidConfig {
                  reason: format!("model '{model}': {reason}"),
              };
              if self.max_seq_len == 0 {
                  return Err(invalid("max_seq_len must be positive"));
              }
              if self.dim == Some(0) {
                  return Err(invalid("dim must be positive"));
              }
              Ok(())
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/offers.rs
    source: |
      //! Which resource kinds each engine side can serve.

      use std::collections::HashSet;

      use crate::resident::error::PlannerError;
      use crate::resident::op::{EngineSelector, ResourceKind};
      use crate::resident::processor::StarvedSnapshot;

      #[derive(Clone, Debug, Default, PartialEq, Eq)]
      pub struct EngineOffers {
          cpu: HashSet<ResourceKind>,
          gpu: HashSet<ResourceKind>,
      }

      impl EngineOffers {
          pub fn new() -> Self {
              Self::default()
          }

          pub fn with_cpu(mut self, kinds: impl IntoIterator<Item = ResourceKind>) -> Self {
              self.cpu.extend(kinds);
              self
          }

          pub fn with_gpu(mut self, kinds: impl IntoIterator<Item = ResourceKind>) -> Self {
              self.gpu.extend(kinds);
              self
          }

          /// A starved kind is offered to either side; otherwise the GPU is preferred.
          /// The hint only picks between equivalent variants and never changes the op set.
          pub fn choose(
              &self,
              kind: ResourceKind,
              starved: StarvedSnapshot,
          ) -> Result<EngineSelector, PlannerError> {
              match (self.cpu.contains(&kind), self.gpu.contains(&kind)) {
                  (true, true) if starved.contains(kind) => Ok(EngineSelector::Either),
                  (true, true) => Ok(EngineSelector::Gpu),
                  (true, false) => Ok(EngineSelector::Cpu),
                  (false, true) => Ok(EngineSelector::Gpu),
                  (false, false) => Err(PlannerError::MissingCapability {
                      capability: format!("{kind:?}"),
                  }),
              }
          }
      }
  reason: Konfigurationstypen (TOML-Daten) und Engine-Angebote für die Device-Wahl anlegen
```
/answer 4eac-149 allow
ReasoningToken: 44
Control Request:
```yaml
id: 4eac-14a
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/expand.rs
    source: |
      //! Expansion rules: one task becomes an ordered operation chain.

      use crate::resident::error::PlannerError;
      use crate::resident::facade::{
          EmbeddingParams, InferenceParams, Payload, SimilarityParams, TaskKind,
      };
      use crate::resident::op::{
          EngineSelector, ModelId, OpDesc, OpKind, OpParams, Requirements, ResourceKind,
      };
      use crate::resident::processor::StarvedSnapshot;

      use super::config::{PipelineConfig, Pooling};
      use super::offers::EngineOffers;

      /// Borrowed view of a task's parameters; the data itself never influences the plan.
      #[derive(Clone, Copy, Debug)]
      #[non_exhaustive]
      pub enum TaskSpec<'a> {
          Embedding(&'a EmbeddingParams),
          Similarity(&'a SimilarityParams),
          Inference(&'a InferenceParams),
      }

      impl<'a> TaskSpec<'a> {
          pub fn from_payload(payload: &'a Payload) -> Self {
              match payload {
                  Payload::Embedding { params, .. } => Self::Embedding(params),
                  Payload::Similarity { params, .. } => Self::Similarity(params),
                  Payload::Inference { params, .. } => Self::Inference(params),
              }
          }

          pub fn kind(&self) -> TaskKind {
              match self {
                  Self::Embedding(_) => TaskKind::Embedding,
                  Self::Similarity(_) => TaskKind::Similarity,
                  Self::Inference(_) => TaskKind::Inference,
              }
          }
      }

      /// Resource the first operation of a task of this kind needs.
      pub fn first_resource(kind: TaskKind) -> ResourceKind {
          match kind {
              TaskKind::Similarity => ResourceKind::Scorer,
              TaskKind::Embedding | TaskKind::Inference => ResourceKind::Tokenizer,
          }
      }

      #[derive(Clone, Debug, PartialEq, Eq)]
      pub struct PlannedOp {
          pub desc: OpDesc,
          /// `false` keeps the result on the device for a GPU successor.
          pub materialize: bool,
      }

      pub struct ExpandContext<'a> {
          pub starved: StarvedSnapshot,
          pub offers: &'a EngineOffers,
          pub model: &'a ModelId,
          pub pipeline: &'a PipelineConfig,
      }

      impl ExpandContext<'_> {
          fn engine(&self, kind: ResourceKind) -> Result<EngineSelector, PlannerError> {
              self.offers.choose(kind, self.starved)
          }

          fn requires(&self, kind: ResourceKind, engine: EngineSelector) -> Requirements {
              Requirements::new(kind, engine).with_model(self.model.clone())
          }
      }

      /// Pure: no state, same inputs give the same operations.
      pub trait PlanExpander: Send + Sync {
          fn expand(
              &self,
              task: &TaskSpec<'_>,
              ctx: &ExpandContext<'_>,
          ) -> Result<Vec<PlannedOp>, PlannerError>;
      }

      #[derive(Default)]
      struct Chain(Vec<(OpKind, OpParams)>);

      impl Chain {
          fn push(&mut self, kind: OpKind, params: OpParams) {
              self.0.push((kind, params));
          }

          /// A result stays on the device only when a GPU operation consumes it next.
          fn finish(self) -> Vec<PlannedOp> {
              let engines: Vec<EngineSelector> =
                  self.0.iter().map(|(_, p)| p.requires.engine).collect();
              self.0
                  .into_iter()
                  .enumerate()
                  .map(|(i, (kind, params))| {
                      let gpu_here = engines[i] == EngineSelector::Gpu;
                      let gpu_next = engines.get(i + 1) == Some(&EngineSelector::Gpu);
                      PlannedOp {
                          desc: OpDesc::new(kind, params),
                          materialize: !(gpu_here && gpu_next),
                      }
                  })
                  .collect()
          }
      }

      fn mismatch(expected: TaskKind, got: &TaskSpec<'_>) -> PlannerError {
          PlannerError::InvalidConfig {
              reason: format!("expander for {expected:?} received {:?} task", got.kind()),
          }
      }

      pub(super) struct EmbeddingExpander;

      impl PlanExpander for EmbeddingExpander {
          fn expand(
              &self,
              task: &TaskSpec<'_>,
              ctx: &ExpandContext<'_>,
          ) -> Result<Vec<PlannedOp>, PlannerError> {
              let TaskSpec::Embedding(params) = task else {
                  return Err(mismatch(TaskKind::Embedding, task));
              };
              let encoder = ctx.engine(ResourceKind::Encoder)?;
              let mut chain = Chain::default();

              let tokenizer = ctx.engine(ResourceKind::Tokenizer)?;
              chain.push(
                  OpKind::Tokenize,
                  OpParams::new(ctx.requires(ResourceKind::Tokenizer, tokenizer)),
              );
              let mut embed = OpParams::new(ctx.requires(ResourceKind::Encoder, encoder));
              embed.dim = ctx.pipeline.dim;
              chain.push(OpKind::Embed, embed);
              if ctx.pipeline.pooling != Pooling::None {
                  chain.push(
                      OpKind::Pool,
                      OpParams::new(ctx.requires(ResourceKind::Encoder, encoder)),
                  );
              }
              if ctx.pipeline.normalize {
                  chain.push(
                      OpKind::L2Norm,
                      OpParams::new(ctx.requires(ResourceKind::Encoder, encoder)),
                  );
              }
              if let Some(index) = params.target_index() {
                  let engine = ctx.engine(ResourceKind::Index)?;
                  chain.push(
                      OpKind::WriteIndex,
                      OpParams::new(
                          ctx.requires(ResourceKind::Index, engine)
                              .with_index(index.clone()),
                      ),
                  );
              }
              Ok(chain.finish())
          }
      }

      pub(super) struct SimilarityExpander;

      impl PlanExpander for SimilarityExpander {
          fn expand(
              &self,
              task: &TaskSpec<'_>,
              ctx: &ExpandContext<'_>,
          ) -> Result<Vec<PlannedOp>, PlannerError> {
              let TaskSpec::Similarity(params) = task else {
                  return Err(mismatch(TaskKind::Similarity, task));
              };
              if params.get_top_k() == 0 {
                  return Err(PlannerError::InvalidConfig {
                      reason: "top_k must be positive".into(),
                  });
              }
              let scorer = ctx.engine(ResourceKind::Scorer)?;
              let requires = ctx
                  .requires(ResourceKind::Scorer, scorer)
                  .with_index(params.index().clone());

              let mut score = OpParams::new(requires.clone());
              score.dim = ctx.pipeline.dim;
              let mut top = OpParams::new(requires);
              top.top_k = Some(u32::try_from(params.get_top_k()).unwrap_or(u32::MAX));

              let mut chain = Chain::default();
              chain.push(OpKind::Similarity, score);
              chain.push(OpKind::TopK, top);
              Ok(chain.finish())
          }
      }

      pub(super) struct InferenceExpander;

      impl PlanExpander for InferenceExpander {
          fn expand(
              &self,
              task: &TaskSpec<'_>,
              ctx: &ExpandContext<'_>,
          ) -> Result<Vec<PlannedOp>, PlannerError> {
              if !matches!(task, TaskSpec::Inference(_)) {
                  return Err(mismatch(TaskKind::Inference, task));
              }
              let tokenizer = ctx.engine(ResourceKind::Tokenizer)?;
              let cross = ctx.engine(ResourceKind::CrossEncoder)?;
              let mut chain = Chain::default();
              chain.push(
                  OpKind::Tokenize,
                  OpParams::new(ctx.requires(ResourceKind::Tokenizer, tokenizer)),
              );
              chain.push(
                  OpKind::Infer,
                  OpParams::new(ctx.requires(ResourceKind::CrossEncoder, cross)),
              );
              Ok(chain.finish())
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
    source: |
      //! Rule set: pipeline data plus the explicit expander registry.

      use std::collections::{BTreeMap, HashMap};

      use serde::Deserialize;

      use crate::resident::error::PlannerError;
      use crate::resident::facade::TaskKind;
      use crate::resident::op::ModelId;
      use crate::resident::processor::StarvedSnapshot;

      use super::config::PipelineConfig;
      use super::expand::{
          EmbeddingExpander, ExpandContext, InferenceExpander, PlanExpander, PlannedOp,
          SimilarityExpander, TaskSpec,
      };
      use super::offers::EngineOffers;

      const ALL_KINDS: [TaskKind; 3] = [TaskKind::Embedding, TaskKind::Similarity, TaskKind::Inference];

      #[derive(Deserialize)]
      #[serde(deny_unknown_fields)]
      struct RawRules {
          default_model: String,
          models: BTreeMap<String, PipelineConfig>,
      }

      pub struct RuleSet {
          default_model: ModelId,
          pipelines: HashMap<ModelId, PipelineConfig>,
          expanders: HashMap<(ModelId, TaskKind), Box<dyn PlanExpander>>,
      }

      impl RuleSet {
          /// Registers the standard expanders for every model and task kind.
          pub fn new(
              default_model: ModelId,
              pipelines: HashMap<ModelId, PipelineConfig>,
          ) -> Result<Self, PlannerError> {
              if !pipelines.contains_key(&default_model) {
                  return Err(PlannerError::UnknownModel {
                      model: default_model.0,
                  });
              }
              for (model, pipeline) in &pipelines {
                  pipeline.validate(&model.0)?;
              }
              let mut expanders: HashMap<(ModelId, TaskKind), Box<dyn PlanExpander>> = HashMap::new();
              for model in pipelines.keys() {
                  expanders.insert((model.clone(), TaskKind::Embedding), Box::new(EmbeddingExpander));
                  expanders.insert((model.clone(), TaskKind::Similarity), Box::new(SimilarityExpander));
                  expanders.insert((model.clone(), TaskKind::Inference), Box::new(InferenceExpander));
              }
              Ok(Self {
                  default_model,
                  pipelines,
                  expanders,
              })
          }

          pub fn from_toml(src: &str) -> Result<Self, PlannerError> {
              let raw: RawRules = toml::from_str(src).map_err(|e| PlannerError::InvalidConfig {
                  reason: e.to_string(),
              })?;
              let pipelines = raw
                  .models
                  .into_iter()
                  .map(|(name, cfg)| (ModelId(name), cfg))
                  .collect();
              Self::new(ModelId(raw.default_model), pipelines)
          }

          /// Replaces the expander for a configured model.
          pub fn register(
              &mut self,
              model: ModelId,
              kind: TaskKind,
              expander: Box<dyn PlanExpander>,
          ) -> Result<(), PlannerError> {
              if !self.pipelines.contains_key(&model) {
                  return Err(PlannerError::UnknownModel { model: model.0 });
              }
              self.expanders.insert((model, kind), expander);
              Ok(())
          }

          pub fn default_model(&self) -> &ModelId {
              &self.default_model
          }

          pub fn pipeline(&self, model: &ModelId) -> Option<&PipelineConfig> {
              self.pipelines.get(model)
          }

          pub fn kinds() -> [TaskKind; 3] {
              ALL_KINDS
          }

          pub(super) fn expand(
              &self,
              task: &TaskSpec<'_>,
              starved: StarvedSnapshot,
              offers: &EngineOffers,
          ) -> Result<Vec<PlannedOp>, PlannerError> {
              let model = match task {
                  TaskSpec::Inference(p) => p.get_model().unwrap_or(&self.default_model),
                  _ => &self.default_model,
              };
              let unknown = || PlannerError::UnknownModel {
                  model: model.0.clone(),
              };
              let pipeline = self.pipelines.get(model).ok_or_else(unknown)?;
              let expander = self
                  .expanders
                  .get(&(model.clone(), task.kind()))
                  .ok_or_else(unknown)?;
              expander.expand(
                  task,
                  &ExpandContext {
                      starved,
                      offers,
                      model,
                      pipeline,
                  },
              )
          }
      }
  reason: "Expansionsregeln (Embedding/Similarity/Inferenz, materialize, Hint) und RuleSet mit Registry anlegen"
```
/answer 4eac-14a allow
ReasoningToken: 317
Control Request:
```yaml
id: 4eac-14b
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
    overwrite: true
    source: |
      //! Stateless task expansion.

      mod config;
      mod expand;
      mod offers;
      mod rules;

      use std::sync::Arc;

      use arc_swap::ArcSwap;

      use crate::resident::error::PlannerError;
      use crate::resident::processor::StarvedSnapshot;

      pub use config::{PipelineConfig, PipelineDtype, Pooling, Prefixes, ScorerKind};
      pub use expand::{first_resource, ExpandContext, PlanExpander, PlannedOp, TaskSpec};
      pub use offers::EngineOffers;
      pub use rules::RuleSet;

      /// Each call works on one rule snapshot, so a reload never affects a running expansion.
      pub struct Planner {
          rules: ArcSwap<RuleSet>,
      }

      impl Planner {
          pub fn new(rules: RuleSet) -> Self {
              Self {
                  rules: ArcSwap::from_pointee(rules),
              }
          }

          pub fn reload(&self, rules: RuleSet) {
              self.rules.store(Arc::new(rules));
          }

          pub fn snapshot(&self) -> Arc<RuleSet> {
              self.rules.load_full()
          }

          pub fn expand(
              &self,
              task: &TaskSpec<'_>,
              starved: StarvedSnapshot,
              offers: &EngineOffers,
          ) -> Result<Vec<PlannedOp>, PlannerError> {
              let ops = self.rules.load().expand(task, starved, offers)?;
              tracing::debug!(kind = ?task.kind(), ops = ops.len(), "task expanded");
              Ok(ops)
          }
      }

      #[cfg(test)]
      mod tests {
          use std::time::Duration;

          use proptest::prelude::*;

          use super::*;
          use crate::resident::facade::{
              EmbeddingParams, InferenceParams, SimilarityParams, TaskKind, TextRole,
          };
          use crate::resident::op::{EngineSelector, IndexId, ModelId, Op, OpKind, ResourceKind};
          use crate::resident::processor::StarvedSet;

          const TOML: &str = r#"
              default_model = "e5-small"

              [models.e5-small]
              pooling = "mean"
              normalize = true
              max_seq_len = 512
              dtype = "f16"

              [models.e5-small.prefixes]
              query = "query: "
              passage = "passage: "
          "#;

          const KINDS: [ResourceKind; 6] = [
              ResourceKind::Tokenizer,
              ResourceKind::Normalizer,
              ResourceKind::Encoder,
              ResourceKind::Scorer,
              ResourceKind::Index,
              ResourceKind::CrossEncoder,
          ];

          fn planner() -> Planner {
              Planner::new(RuleSet::from_toml(TOML).unwrap())
          }

          fn hybrid() -> EngineOffers {
              EngineOffers::new()
                  .with_cpu(KINDS)
                  .with_gpu([
                      ResourceKind::Encoder,
                      ResourceKind::Scorer,
                      ResourceKind::CrossEncoder,
                  ])
          }

          fn gpu_only_models() -> EngineOffers {
              EngineOffers::new()
                  .with_cpu([ResourceKind::Tokenizer, ResourceKind::Index])
                  .with_gpu([
                      ResourceKind::Encoder,
                      ResourceKind::Scorer,
                      ResourceKind::CrossEncoder,
                  ])
          }

          fn kinds(ops: &[PlannedOp]) -> Vec<OpKind> {
              ops.iter().map(|o| o.desc.kind()).collect()
          }

          fn flags(ops: &[PlannedOp]) -> Vec<bool> {
              ops.iter().map(|o| o.materialize).collect()
          }

          fn none() -> StarvedSnapshot {
              StarvedSnapshot::default()
          }

          #[test]
          fn embedding_chain_keeps_gpu_results_on_device() {
              let params = EmbeddingParams::new(TextRole::Query).write_index(IndexId("idx".into()));
              let ops = planner()
                  .expand(&TaskSpec::Embedding(&params), none(), &gpu_only_models())
                  .unwrap();
              assert_eq!(
                  kinds(&ops),
                  [OpKind::Tokenize, OpKind::Embed, OpKind::Pool, OpKind::L2Norm, OpKind::WriteIndex]
              );
              assert_eq!(flags(&ops), [true, false, false, true, true]);
          }

          #[test]
          fn embedding_without_index_ends_on_host() {
              let params = EmbeddingParams::new(TextRole::Passage);
              let ops = planner()
                  .expand(&TaskSpec::Embedding(&params), none(), &gpu_only_models())
                  .unwrap();
              assert_eq!(
                  kinds(&ops),
                  [OpKind::Tokenize, OpKind::Embed, OpKind::Pool, OpKind::L2Norm]
              );
              assert_eq!(flags(&ops), [true, false, false, true]);
          }

          #[test]
          fn cpu_only_offers_select_cpu() {
              let offers = EngineOffers::new().with_cpu(KINDS);
              let params = EmbeddingParams::default();
              let ops = planner()
                  .expand(&TaskSpec::Embedding(&params), none(), &offers)
                  .unwrap();
              assert!(ops
                  .iter()
                  .all(|o| o.desc.params().requires.engine == EngineSelector::Cpu));
              assert!(ops.iter().all(|o| o.materialize));
          }

          #[test]
          fn missing_capability_is_reported() {
              let offers = EngineOffers::new().with_cpu([ResourceKind::Tokenizer]);
              let params = EmbeddingParams::default();
              let err = planner()
                  .expand(&TaskSpec::Embedding(&params), none(), &offers)
                  .unwrap_err();
              assert!(matches!(err, PlannerError::MissingCapability { .. }));
          }

          #[test]
          fn similarity_sets_top_k_and_index() {
              let params = SimilarityParams::new(IndexId("idx".into())).top_k(7);
              let ops = planner()
                  .expand(&TaskSpec::Similarity(&params), none(), &gpu_only_models())
                  .unwrap();
              assert_eq!(kinds(&ops), [OpKind::Similarity, OpKind::TopK]);
              assert_eq!(ops[1].desc.params().top_k, Some(7));
              assert_eq!(
                  ops[0].desc.params().requires.index,
                  Some(IndexId("idx".into()))
              );
              assert_eq!(flags(&ops), [false, true]);
          }

          #[test]
          fn similarity_rejects_zero_top_k() {
              let params = SimilarityParams::new(IndexId("idx".into())).top_k(0);
              let err = planner()
                  .expand(&TaskSpec::Similarity(&params), none(), &hybrid())
                  .unwrap_err();
              assert!(matches!(err, PlannerError::InvalidConfig { .. }));
          }

          #[test]
          fn inference_uses_requested_model() {
              let params = InferenceParams::new().model(ModelId("e5-small".into()));
              let ops = planner()
                  .expand(&TaskSpec::Inference(&params), none(), &hybrid())
                  .unwrap();
              assert_eq!(kinds(&ops), [OpKind::Tokenize, OpKind::Infer]);
              assert_eq!(
                  ops[1].desc.params().requires.model,
                  Some(ModelId("e5-small".into()))
              );
          }

          #[test]
          fn unknown_model_is_rejected() {
              let params = InferenceParams::new().model(ModelId("nope".into()));
              let err = planner()
                  .expand(&TaskSpec::Inference(&params), none(), &hybrid())
                  .unwrap_err();
              assert!(matches!(err, PlannerError::UnknownModel { .. }));
          }

          #[test]
          fn pipeline_flags_shape_the_chain() {
              let src = TOML.replace("pooling = \"mean\"", "pooling = \"none\"")
                  .replace("normalize = true", "normalize = false");
              let planner = Planner::new(RuleSet::from_toml(&src).unwrap());
              let params = EmbeddingParams::default();
              let ops = planner
                  .expand(&TaskSpec::Embedding(&params), none(), &hybrid())
                  .unwrap();
              assert_eq!(kinds(&ops), [OpKind::Tokenize, OpKind::Embed]);
          }

          #[test]
          fn config_validation() {
              assert!(RuleSet::from_toml(TOML).is_ok());
              let unknown_field = TOML.replace("dtype", "dtpye");
              assert!(matches!(
                  RuleSet::from_toml(&unknown_field),
                  Err(PlannerError::InvalidConfig { .. })
              ));
              let zero_len = TOML.replace("512", "0");
              assert!(matches!(
                  RuleSet::from_toml(&zero_len),
                  Err(PlannerError::InvalidConfig { .. })
              ));
              let bad_default = TOML.replace("default_model = \"e5-small\"", "default_model = \"x\"");
              assert!(matches!(
                  RuleSet::from_toml(&bad_default),
                  Err(PlannerError::UnknownModel { .. })
              ));
          }

          #[test]
          fn register_requires_configured_model() {
              let mut rules = RuleSet::from_toml(TOML).unwrap();
              let err = rules
                  .register(ModelId("nope".into()), TaskKind::Embedding, Box::new(Fixed))
                  .unwrap_err();
              assert!(matches!(err, PlannerError::UnknownModel { .. }));
              rules
                  .register(ModelId("e5-small".into()), TaskKind::Embedding, Box::new(Fixed))
                  .unwrap();
          }

          struct Fixed;

          impl PlanExpander for Fixed {
              fn expand(
                  &self,
                  _task: &TaskSpec<'_>,
                  _ctx: &ExpandContext<'_>,
              ) -> Result<Vec<PlannedOp>, PlannerError> {
                  Ok(Vec::new())
              }
          }

          #[test]
          fn reload_swaps_snapshot_atomically() {
              let planner = planner();
              let before = planner.snapshot();
              let src = TOML.replace("e5-small", "other");
              planner.reload(RuleSet::from_toml(&src).unwrap());
              assert_eq!(before.default_model(), &ModelId("e5-small".into()));
              assert_eq!(planner.snapshot().default_model(), &ModelId("other".into()));
          }

          proptest! {
              #[test]
              fn starved_hint_changes_engines_not_ops(mask in prop::array::uniform6(any::<bool>())) {
                  let set = StarvedSet::new(Duration::from_secs(60));
                  for (kind, on) in KINDS.iter().zip(mask) {
                      if on {
                          set.mark(*kind);
                      }
                  }
                  let params = EmbeddingParams::new(TextRole::Query)
                      .write_index(IndexId("idx".into()));
                  let planner = planner();
                  let offers = hybrid();
                  let plain = planner.expand(&TaskSpec::Embedding(&params), none(), &offers).unwrap();
                  let hinted = planner
                      .expand(&TaskSpec::Embedding(&params), set.snapshot(), &offers)
                      .unwrap();
                  prop_assert_eq!(kinds(&plain), kinds(&hinted));
                  let again = planner
                      .expand(&TaskSpec::Embedding(&params), set.snapshot(), &offers)
                      .unwrap();
                  prop_assert_eq!(hinted, again);
              }
          }

          #[test]
          fn first_resource_matches_expansion() {
              assert_eq!(first_resource(TaskKind::Similarity), ResourceKind::Scorer);
              assert_eq!(first_resource(TaskKind::Embedding), ResourceKind::Tokenizer);
              assert_eq!(first_resource(TaskKind::Inference), ResourceKind::Tokenizer);
          }
      }
  reason: "Planner-Fassade (ArcSwap-Snapshot, reine expand) und Tests inkl. Property-Test für den Starved-Hint"
```
/answer 4eac-14b allow
Control Request:
```yaml
id: 4eac-14c
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
    id: kinds
    start_marker: "    pub fn kinds() -> [TaskKind; 3] {"
    end_marker: |2+
              ALL_KINDS
          }

    content: ""
  reason: Ungenutzte kinds()-Hilfsfunktion entfernen
```
/answer 4eac-14c allow
ReasoningToken: 343
Control Request:
```yaml
id: 4eac-14d
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
      error: No node matched the selector.
      id: kinds
```
/answer 4eac-14d allow
Control Request:
```yaml
id: 4eac-14e
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
    id: ALL_KINDS
    start_marker: "const ALL_KINDS: [TaskKind; 3]"
    end_marker: |+
      TaskKind::Inference];

    content: ""
  reason: Ungenutzte Konstante entfernen
```
/answer 4eac-14e allow
Control Request:
```yaml
id: 4eac-14f
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
      error: No node matched the selector.
      id: ALL_KINDS
```
/answer 4eac-14f allow
Text:
 Die Edits brauchen echte Node-IDs. Ich hole sie über die Outline.
Control Request:
```yaml
id: 4eac-150
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
  reason: Node-IDs für die Entfernung der ungenutzten Elemente ermitteln
```
/answer 4eac-150 allow
Control Request:
```yaml
id: 4eac-151
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Rule set: pipeline data plus the explicit expander registry."
      - id: nHvg68|3bDP83
        signature: "use std::collections::{BTreeMap, HashMap};"
      - id: jjhVGr|ng3Ene
        signature: "const ALL_KINDS: [TaskKind; 3] = [TaskKind::Embedding, TaskKind::Similarity, Ta…"
      - id: RawRules
        signature: "#[derive(Deserialize)] #[serde(deny_unknown_fields)] struct RawRules {"
      - id: RuleSet
        signature: "pub struct RuleSet {"
      - id: impl_RuleSet
        signature: "impl RuleSet {"
        children:
        - id: impl_RuleSet.new
          signature: /// Registers the standard expanders for every model and task kind. pub fn new(…
          docstring: Registers the standard expanders for every model and task kind.
        - id: impl_RuleSet.from_toml
          signature: "pub fn from_toml(src: &str) -> Result<Self, PlannerError> {"
        - id: impl_RuleSet.register
          signature: /// Replaces the expander for a configured model. pub fn register(     &mut sel…
          docstring: Replaces the expander for a configured model.
        - id: impl_RuleSet.default_model
          signature: "pub fn default_model(&self) -> &ModelId {"
        - id: impl_RuleSet.pipeline
          signature: "pub fn pipeline(&self, model: &ModelId) -> Option<&PipelineConfig> {"
        - id: impl_RuleSet.kinds
          signature: "pub fn kinds() -> [TaskKind; 3] {"
        - id: impl_RuleSet.expand
          signature: "pub(super) fn expand(     &self,     task: &TaskSpec<'_>,     starved: StarvedS…"
```
/answer 4eac-151 allow
Control Request:
```yaml
id: 4eac-152
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
    id: impl_RuleSet.kinds
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
    id: jjhVGr|ng3Ene
  reason: Ungenutzte Konstante ALL_KINDS und kinds() entfernen
```
/answer 4eac-152 allow
Control Request:
```yaml
id: 4eac-153
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
      id: impl_RuleSet.kinds
      result: success
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
      id: jjhVGr|ng3Ene
      error: No node matched the selector.
```
/answer 4eac-153 allow
Text:
 IDs ändern sich nach dem ersten Löschen. Ich versuche es erneut.
Control Request:
```yaml
id: 4eac-154
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
  text: ALL_KINDS
  reason: Aktuelle ID der Konstante nach dem ersten Löschen finden
```
/answer 4eac-154 allow
Control Request:
```yaml
id: 4eac-155
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
      nodes:
      - id: IC8ctQ|4uJVIa
        code: |
          const ALL_KINDS: [TaskKind; 3] = [
              TaskKind::Embedding,
              TaskKind::Similarity,
              TaskKind::Inference,
          ];
```
/answer 4eac-155 allow
Control Request:
```yaml
id: 4eac-156
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
    id: IC8ctQ|4uJVIa
  reason: Ungenutzte Konstante entfernen
```
/answer 4eac-156 allow
Control Request:
```yaml
id: 4eac-157
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/config.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/offers.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/expand.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
  reason: Syntax der neuen Planner-Dateien prüfen
```
/answer 4eac-157 allow
Text:
 Die Syntax ist in Ordnung. Zuletzt prüfe ich, ob die Import-Pfade (`op::…`, `facade::TextRole`) tatsächlich re-exportiert werden, weil ich nicht kompilieren kann.
ReasoningToken: 54
Control Request:
```yaml
id: 4eac-158
toolName: grep
arguments:
  items:
  - pattern: "ModelId|IndexId|EngineSelector|ResourceKind|Requirements,"
    directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    include:
    - mod.rs
    limit: 8
  - pattern: TextRole|SimilarityParams
    directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    include:
    - mod.rs
    limit: 4
  reason: Re-Export-Pfade der verwendeten Typen bestätigen
```
/answer 4eac-158 allow
Control Request:
```yaml
id: 4eac-159
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      matches:
      - path: mod.rs
        matches:
        - lineno: 8
          match: "Affinity, DeviceId, EngineSelector, IndexId, ModelId, Requirements, ResourceKind,"
        - lineno: 28
          match: "pub requires: Requirements,"
        - lineno: 175
          match: "let req = Requirements::new(ResourceKind::Encoder, EngineSelector::Either);"
        - lineno: 188
          match: "Requirements::new(ResourceKind::Scorer, EngineSelector::Gpu),"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      matches:
      - path: mod.rs
        matches:
        - lineno: 14
          match: "EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, TextRole,"
```
/answer 4eac-159 allow
Result Stats: id=02c72708-f03d-4efe-ba73-55c395a6f29c, total: 124300, in: 48, out: 33360, reason: 0, read: 1343742, write: 90892, start: 1792321972387, end: 1792321982950, time: 10514, total: 230845, io: 0, control: 49, sent: 0, recv: 285
C3 ist umgesetzt, aber ich konnte nichts kompilieren oder testen. Mir stand nur die AST-Werkzeugschiene zur Verfügung, ohne Shell. Die fünf neuen Dateien bestehen die Syntaxprüfung, und die Import-Pfade habe ich gegen die Re-Exports geprüft. Bitte lauf `cargo test -p` auf dem Crate, bevor du dich darauf verlässt.

**Was jetzt in `rag/src/resident/planner/` liegt**
- **`mod.rs`:** Der `Planner` hält die Regeln als `ArcSwap<RuleSet>`. `expand` ist eine reine Funktion über einen Snapshot, `reload` tauscht ihn atomar aus. Die Tests stehen ebenfalls hier, darunter ein Property-Test zum Starved-Hint.
- **`config.rs`:** `PipelineConfig` mit Präfixen, Pooling, Normalisierung, `max_seq_len`, `dim`, dtype und Scorer. Das Deserialisieren lehnt unbekannte Felder ab, `validate` prüft die Werte.
- **`rules.rs`:** Das `RuleSet` lädt per `from_toml` und hält eine explizite Registry `(ModelId, TaskKind) → Box<dyn PlanExpander>`. `register` lehnt unbekannte Modelle ab.
- **`expand.rs`:** Hier stehen die drei Expander und die `materialize`-Regel. Ein Ergebnis bleibt nur dann im VRAM, wenn die nächste Op auch auf der GPU läuft. Das letzte Glied, ein CPU-Nachfolger und `WriteIndex` führen zur Host-Kopie.
- **`offers.rs`:** `EngineOffers` beschreibt, welche Ressourcenart CPU und GPU bedienen. Wenn beide eine Art anbieten, wird die GPU bevorzugt. Ist die Art gerade als starved markiert, wird `Either` gesetzt. Der Hint ändert also nur die Engine, nie die Op-Menge.

**Die Ketten**
- **Embedding:** `Tokenize → Embed → [Pool] → [L2Norm] → [WriteIndex]`. `Pool` und `L2Norm` entfallen laut Pipeline-Config, `WriteIndex` entfällt ohne Zielindex.
- **Similarity:** `Similarity → TopK`. Die Query-Vektoren kommen bereits fertig im Task an, deshalb enthält die Kette kein `Tokenize` und kein `Embed`.
- **Inference:** `Tokenize → Infer(CrossEncoder)`. Das Modell kommt aus den Params, sonst aus `default_model`.

**Was von deinem Plan abweicht**
- **Hub-Anbindung fehlt noch.** Der Hub-`Expander` verlangt fertige `OperationNode`s mit festem Eingabewert. Folge-Ops sollen aber erst beim Abschluss entstehen, und diese Engine-Seite gibt es noch nicht (nur `NoEngine`). Der Planner liefert deshalb `Vec<PlannedOp>` (`OpDesc` plus `materialize`). Ob der Adapter alle Ops auf einmal einreiht oder nur die erste und den Rest über die Engine, musst du entscheiden.
- **`EngineOffers` statt `CapabilitySet`.** Das vorhandene `CapabilitySet` bildet Präzisionsachsen ab, nicht "Engine X bietet Ressource Y". Für die Device-Wahl habe ich deshalb einen eigenen Typ angelegt.
- **Kein `notify`-Watcher.** Hot-Reload läuft nur über `Planner::reload`, den Datei-Watcher habe ich weggelassen.
- **Neue Abhängigkeit `toml = "0.9"`** in `Cargo.toml`. `Cargo.lock` muss einmal aktualisiert werden.
