Setze D1 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben

SystemInit:  id=85283fc8-f4ab-45d8-9965-40450393f998, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 4eac-193
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  - path: /home/user/xyan/xy.ai.workbench/rag
    recursive: false
  reason: Inspect existing resident module layout before adding the subengine lifecycle code
```
/answer 4eac-193 allow
ReasoningToken: 39
Control Request:
```yaml
id: 4eac-194
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
/answer 4eac-194 allow
Control Request:
```yaml
id: 4eac-195
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: mod |pub use
    include:
    - lib.rs
    - mod.rs
    limit: 50
  reason: Find module declarations to see how resident is wired
```
/answer 4eac-195 allow
Control Request:
```yaml
id: 4eac-196
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: lib.rs
        matches:
        - lineno: 5
          match: pub mod core;
        - lineno: 6
          match: pub mod layers;
        - lineno: 7
          match: pub mod resident;
      - path: resident/mod.rs
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
      - path: resident/processor/mod.rs
        matches:
        - lineno: 2
          match: mod completion;
        - lineno: 4
          match: mod completion_cuda;
        - lineno: 5
          match: mod config;
        - lineno: 6
          match: mod error;
        - lineno: 7
          match: mod hub;
        - lineno: 8
          match: mod ops;
        - lineno: 9
          match: mod resource;
        - lineno: 10
          match: mod starved;
        - lineno: 11
          match: mod worker;
        - lineno: 13
          match: mod tests;
        - lineno: 14
          match: "pub use completion::{"
        - lineno: 18
          match: "pub use completion_cuda::{HostFnCompletion, PolledCompletion};"
        - lineno: 19
          match: pub use config::WorkerConfig;
        - lineno: 20
          match: pub use error::OrchestrationError;
        - lineno: 21
          match: "pub use hub::{Engine, Expander, Expansion, Hub, RegionCheck, Step, Wakeups};"
        - lineno: 22
          match: "pub use ops::{Collect, OpJob, OpQueue, OpQueues, Side, TaskRun};"
        - lineno: 23
          match: "pub use resource::{ResourceSlot, ResourceState};"
        - lineno: 24
          match: "pub use starved::{StarvedSet, StarvedSnapshot};"
        - lineno: 25
          match: pub use worker::Processor;
      - path: resident/precision/mod.rs
        matches:
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
      - path: resident/facade/mod.rs
        matches:
        - lineno: 3
          match: mod blocking;
        - lineno: 4
          match: mod error;
        - lineno: 5
          match: mod handle;
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 4eac-196 allow
Control Request:
```yaml
id: 4eac-197
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    recursive: false
  reason: See existing subengine files and neighbours
```
/answer 4eac-197 allow
Control Request:
```yaml
id: 4eac-198
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".:"
      - backend.rs
      - error.rs
      - failpoint.rs
      - mock.rs
      - mod.rs
      - telemetry.rs
```
/answer 4eac-198 allow
Control Request:
```yaml
id: 4eac-199
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^\\[|^(tokio|parking_lot|thiserror|tracing|loom|arc-swap|candle|cudarc)|^cuda|^default"
    include:
    - Cargo.toml
    limit: 50
  reason: Check dependencies and features available
```
/answer 4eac-199 allow
Control Request:
```yaml
id: 4eac-19a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-kernels/Cargo.toml
        matches:
        - lineno: 12
          match: "[package]"
        - lineno: 33
          match: "[lib]"
        - lineno: 37
          match: "[dependencies]"
        - lineno: 39
          match: "[build-dependencies.cudaforge]"
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
        - lineno: 38
          match: parking_lot = "0.12.5"
        - lineno: 39
          match: arc-swap = "1.9.2"
        - lineno: 48
          match: "candle-core = { version = \"0.11.0\", optional = true, default-features = false }"
        - lineno: 49
          match: "candle-transformers = { version = \"0.11.0\", optional = true, default-features = false }"
        - lineno: 50
          match: "cudarc = { version = \"=0.19.9\", optional = true }"
        - lineno: 53
          match: "[target.'cfg(target_os = \"linux\")'.dependencies]"
        - lineno: 56
          match: "[features]"
        - lineno: 57
          match: "default = []"
        - lineno: 58
          match: "candle = [\"dep:candle-core\"]"
        - lineno: 59
          match: "cuda = [\"candle\", \"dep:candle-transformers\", \"dep:cudarc\", \"candle-core/cuda\", \"candle-transformers/cuda\"]"
        - lineno: 63
          match: "[dev-dependencies]"
        - lineno: 67
          match: loom = "0.7"
        - lineno: 70
          match: "tokio = { version = \"1\", features = [\"test-util\"] }"
        - lineno: 72
          match: "[[bench]]"
        - lineno: 76
          match: "[patch.crates-io]"
        - lineno: 77
          match: "candle-kernels = { path = \"third_party/candle-kernels\" }"
```
/answer 4eac-19a allow
ReasoningToken: 65
Control Request:
```yaml
id: 4eac-19b
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: subengine|manager|processor/resource|error|mod\.rs$
  reason: Locate subengine and manager files
```
/answer 4eac-19b allow
Control Request:
```yaml
id: 4eac-19c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".:"
      - error.rs
      - mod.rs
      - "./facade:"
      - error.rs
      - mod.rs
      - "./index:"
      - mod.rs
      - "./kernel:"
      - mod.rs
      - "./manager:"
      - mod.rs
      - "./memory:"
      - manager.rs
      - mod.rs
      - "./op:"
      - mod.rs
      - "./planner:"
      - mod.rs
      - "./precision:"
      - mod.rs
      - "./processor:"
      - error.rs
      - mod.rs
      - resource.rs
      - "./queue:"
      - mod.rs
      - "./subengine:"
      - device.rs
      - mod.rs
```
/answer 4eac-19c allow
Control Request:
```yaml
id: 4eac-19d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
    ids: []
  reason: Read existing files (empty ids to get whole content)
```
/answer 4eac-19d allow
Control Request:
```yaml
id: 4eac-19e
result:
  structuredContent:
    results:
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
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
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
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
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
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
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
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 4eac-19e allow
Control Request:
```yaml
id: 4eac-19f
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: struct PinCount|Clock|struct .*Lease|trait Subengine|Ewma|EWMA|struct Capacity|ResourceKey
    include:
    - '*.rs'
    limit: 40
  reason: Find existing lease/pin/clock/capacity types to reuse and avoid duplication
```
/answer 4eac-19f allow
ReasoningToken: 160
Control Request:
```yaml
id: 4eac-1a0
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: mock.rs
        matches:
        - lineno: 10
          match: "pub trait Clock: Send + Sync {"
        - lineno: 14
          match: "pub struct FakeClock {"
        - lineno: 17
          match: "impl Clock for FakeClock {"
      - path: op/lease.rs
        matches:
        - lineno: 11
          match: pub struct PinCount(AtomicUsize);
        - lineno: 28
          match: "pub struct Lease {"
```
/answer 4eac-1a0 allow
Control Request:
```yaml
id: 4eac-1a1
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
    ids:
    - ResourceState
    - impl_ResourceState
    - ResourceSlot
    - impl_ResourceSlot
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
    ids:
    - CpuBackend
    - impl_ComputeBackend_for_CpuBackend
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - IndexError
    - ManagerError
    - ResidentError
  reason: "Read the existing lifecycle slot, CPU backend and error enums to integrate with them"
```
/answer 4eac-1a1 allow
Control Request:
```yaml
id: 4eac-1a2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
      nodes:
      - id: ResourceState
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub enum ResourceState {
              Uninit,
              Loading,
              Ready,
              Backoff,
              Evicting,
          }
      - id: impl_ResourceState
        type: ItemImpl
        signature: "impl ResourceState {"
        children:
        - id: impl_ResourceState.code
          type: ImplItemFn
          code: |2
                fn code(self) -> u8 {
                    self as u8
                }
        - id: impl_ResourceState.from_code
          type: ImplItemFn
          code: |2
                fn from_code(v: u8) -> Self {
                    match v {
                        0 => Self::Uninit,
                        1 => Self::Loading,
                        2 => Self::Ready,
                        3 => Self::Backoff,
                        _ => Self::Evicting,
                    }
                }
      - id: ResourceSlot
        type: ItemStruct
        code: |
          /// Lifecycle cell of one lazily initialised resource. Time is passed in by the caller.
          pub struct ResourceSlot {
              state: AtomicU8,
              in_flight: AtomicUsize,
              pins: Arc<PinCount>,
              last_used_ms: AtomicU64,
              retry_at_ms: AtomicU64,
          }
      - id: impl_ResourceSlot
        type: ItemImpl
        signature: "impl ResourceSlot {"
        children:
        - id: impl_ResourceSlot.new
          type: ImplItemFn
          code: |2
                pub fn new(pins: Arc<PinCount>) -> Self {
                    Self {
                        state: AtomicU8::new(ResourceState::Uninit.code()),
                        in_flight: AtomicUsize::new(0),
                        pins,
                        last_used_ms: AtomicU64::new(0),
                        retry_at_ms: AtomicU64::new(0),
                    }
                }
        - id: impl_ResourceSlot.state
          type: ImplItemFn
          code: |2
                pub fn state(&self) -> ResourceState {
                    ResourceState::from_code(self.state.load(Ordering::Acquire))
                }
        - id: impl_ResourceSlot.transition
          type: ImplItemFn
          code: |2
                fn transition(&self, from: ResourceState, to: ResourceState) -> bool {
                    self.state
                        .compare_exchange(from.code(), to.code(), Ordering::SeqCst, Ordering::SeqCst)
                        .is_ok()
                }
        - id: impl_ResourceSlot.begin_load
          type: ImplItemFn
          code: |2
                /// Only the caller that wins this transition performs the load.
                pub fn begin_load(&self, now_ms: u64) -> bool {
                    if self.transition(ResourceState::Uninit, ResourceState::Loading) {
                        return true;
                    }
                    self.retry_at_ms.load(Ordering::Acquire) <= now_ms
                        && self.transition(ResourceState::Backoff, ResourceState::Loading)
                }
        - id: impl_ResourceSlot.finish_load
          type: ImplItemFn
          code: |2
                pub fn finish_load(&self, ok: bool, now_ms: u64, backoff_ms: u64) {
                    self.last_used_ms.store(now_ms, Ordering::Release);
                    if ok {
                        self.state.store(ResourceState::Ready.code(), Ordering::SeqCst);
                    } else {
                        self.retry_at_ms.store(now_ms + backoff_ms, Ordering::Release);
                        self.state.store(ResourceState::Backoff.code(), Ordering::SeqCst);
                    }
                }
        - id: impl_ResourceSlot.enter
          type: ImplItemFn
          code: |2
                /// Registers an operation in flight; fails unless the resource is `Ready`.
                pub fn enter(&self) -> bool {
                    self.in_flight.fetch_add(1, Ordering::SeqCst);
                    if self.state() == ResourceState::Ready {
                        return true;
                    }
                    self.in_flight.fetch_sub(1, Ordering::SeqCst);
                    false
                }
        - id: impl_ResourceSlot.leave
          type: ImplItemFn
          code: |2
                pub fn leave(&self, now_ms: u64) {
                    self.last_used_ms.store(now_ms, Ordering::Release);
                    let prev = self.in_flight.fetch_sub(1, Ordering::SeqCst);
                    debug_assert!(prev > 0, "leave without enter");
                }
        - id: impl_ResourceSlot.in_flight
          type: ImplItemFn
          code: |2
                pub fn in_flight(&self) -> usize {
                    self.in_flight.load(Ordering::Acquire)
                }
        - id: impl_ResourceSlot.try_evict
          type: ImplItemFn
          code: |2
                /// `Ready -> Evicting` only when idle, unleased and past the TTL.
                pub fn try_evict(&self, now_ms: u64, ttl_ms: u64) -> bool {
                    let idle_for = now_ms.saturating_sub(self.last_used_ms.load(Ordering::Acquire));
                    if idle_for < ttl_ms
                        || !self.transition(ResourceState::Ready, ResourceState::Evicting)
                    {
                        return false;
                    }
                    if self.in_flight.load(Ordering::SeqCst) == 0 && self.pins.is_unpinned() {
                        return true;
                    }
                    self.state.store(ResourceState::Ready.code(), Ordering::SeqCst);
                    false
                }
        - id: impl_ResourceSlot.finish_evict
          type: ImplItemFn
          code: |2
                pub fn finish_evict(&self) {
                    debug_assert_eq!(self.state(), ResourceState::Evicting);
                    self.state.store(ResourceState::Uninit.code(), Ordering::SeqCst);
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: CpuBackend
        type: ItemStruct
        code: |
          /// CPU backend; default without `cuda`, and the fallback target if CUDA init fails.
          /// Placeholder until the real embedding path is wired in.
          pub struct CpuBackend;
      - id: impl_ComputeBackend_for_CpuBackend
        type: ItemImpl
        signature: "impl ComputeBackend for CpuBackend {"
        children:
        - id: impl_ComputeBackend_for_CpuBackend.run_batch
          type: ImplItemFn
          code: |2
                fn run_batch(&self, input: &[f32]) -> Vec<f32> {
                    input.to_vec()
                }
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
              #[error(transparent)]
              Orchestration(#[from] OrchestrationError),
          }
```
/answer 4eac-1a2 allow
ReasoningToken: 148
Control Request:
```yaml
id: 4eac-1a3
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/backend.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/error.rs
  reason: "Understand Clock, PinCount/Lease, backend trait, module gating and orchestration error before designing"
```
/answer 4eac-1a3 allow
Control Request:
```yaml
id: 4eac-1a4
result:
  structuredContent:
    results:
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
      nodes:
      - id: ModuleDoc
        signature: //! Mock/test-double conventions.
      - id: Clock
        signature: /// Example of a narrow trait a hot-path component depends on generically. pub …
        docstring: Example of a narrow trait a hot-path component depends on generically.
        children:
        - id: Clock.now_millis
          signature: fn now_millis(&self) -> u64;
      - id: FakeClock
        signature: "/// Hand-written deterministic fake, per the convention above. pub struct FakeC…"
        docstring: "Hand-written deterministic fake, per the convention above."
      - id: impl_Clock_for_FakeClock
        signature: "impl Clock for FakeClock {"
        children:
        - id: impl_Clock_for_FakeClock.now_millis
          signature: "fn now_millis(&self) -> u64 {"
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: MockBackend
        signature: "/// Identity backend: sufficient for orchestration tests that do not care about…"
        docstring: "Identity backend: sufficient for orchestration tests that do not care about num…"
      - id: impl_ComputeBackend_for_MockBackend
        signature: "impl ComputeBackend for MockBackend {"
        children:
        - id: impl_ComputeBackend_for_MockBackend.run_batch
          signature: "fn run_batch(&self, input: &[f32]) -> Vec<f32> {"
      - id: Measure
        signature: /// Measurement trait calibration logic depends on; `FakeMeasure` stands in for…
        docstring: Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
        children:
        - id: Measure.sample
          signature: fn sample(&mut self) -> f64;
      - id: NoiseModel
        signature: "/// Noise models for calibration-stability tests. pub enum NoiseModel {"
        docstring: Noise models for calibration-stability tests.
      - id: FakeMeasure
        signature: "/// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation…"
        docstring: "Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so"
      - id: impl_FakeMeasure
        signature: "impl FakeMeasure {"
        children:
        - id: impl_FakeMeasure.new
          signature: "pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {"
      - id: impl_Measure_for_FakeMeasure
        signature: "impl Measure for FakeMeasure {"
        children:
        - id: impl_Measure_for_FakeMeasure.sample
          signature: "fn sample(&mut self) -> f64 {"
      - id: sync
        signature: "/// `std::sync` vs. `loom::sync` shim for CAS-protocol modules: swap the /// im…"
        docstring: "`std::sync` vs. `loom::sync` shim for CAS-protocol modules: swap the"
      - id: sync_2
        signature: "#[cfg(not(loom))] pub mod sync {"
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/error.rs
      nodes:
      - id: eRggY0|54nLj5
        signature: use std::sync::Arc;
      - id: OrchestrationError
        signature: "#[derive(Error, Debug)] #[non_exhaustive] pub enum OrchestrationError {"
      - id: impl_IsRetryable_for_OrchestrationError
        signature: "impl IsRetryable for OrchestrationError {"
        children:
        - id: impl_IsRetryable_for_OrchestrationError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: tests
        signature: "#[cfg(test)] mod tests {"
        children:
        - id: tests.JGlh3Y|LPaZQx
          signature: use super::*;
        - id: tests.retryability_follows_cause
          signature: "#[test] fn retryability_follows_cause() {"
        - id: tests.converts_into_resident_error
          signature: "#[test] fn converts_into_resident_error() {"
```
/answer 4eac-1a4 allow
ReasoningToken: 232
Control Request:
```yaml
id: 4eac-1a5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
    ids:
    - Clock
    - FakeClock
    - impl_Clock_for_FakeClock
    - sync
    - sync_2
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/error.rs
    ids:
    - OrchestrationError
    - impl_IsRetryable_for_OrchestrationError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - PlannerError
    - impl_IsRetryable_for_IndexError
    - tests.layer_errors_convert_into_resident_error
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
    ids:
    - ModuleDoc
    - device
    - l9ZlBG|zqWesy
  reason: "Read Clock/FakeClock, loom sync shim and error style to match conventions"
```
/answer 4eac-1a5 allow
Control Request:
```yaml
id: 4eac-1a6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
      nodes:
      - id: Clock
        type: ItemTrait
        signature: /// Example of a narrow trait a hot-path component depends on generically. pub …
        docstring: Example of a narrow trait a hot-path component depends on generically.
        children:
        - id: Clock.now_millis
          type: TraitItemFn
          code: |2
                fn now_millis(&self) -> u64;
      - id: FakeClock
        type: ItemStruct
        code: |
          /// Hand-written deterministic fake, per the convention above.
          pub struct FakeClock {
              pub fixed_millis: u64,
          }
      - id: impl_Clock_for_FakeClock
        type: ItemImpl
        signature: "impl Clock for FakeClock {"
        children:
        - id: impl_Clock_for_FakeClock.now_millis
          type: ImplItemFn
          code: |2
                fn now_millis(&self) -> u64 {
                    self.fixed_millis
                }
      - id: sync
        type: ItemMod
        code: |
          /// `std::sync` vs. `loom::sync` shim for CAS-protocol modules: swap the
          /// import under `cfg(loom)` so the same production code runs exhaustively under loom without
          /// a second implementation. Run with `RUSTFLAGS="--cfg loom" cargo test --test <loom_test>`.
          #[cfg(loom)]
          pub mod sync {
              pub use loom::sync::{atomic, Arc};
          }
      - id: sync_2
        type: ItemMod
        code: |
          #[cfg(not(loom))]
          pub mod sync {
              pub use std::sync::{atomic, Arc};
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/error.rs
      nodes:
      - id: OrchestrationError
        type: ItemEnum
        code: |
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
              #[error("task {task} timed out")]
              Timeout { task: u64 },
              #[error("processor is shutting down")]
              Shutdown,
          }
      - id: impl_IsRetryable_for_OrchestrationError
        type: ItemImpl
        signature: "impl IsRetryable for OrchestrationError {"
        children:
        - id: impl_IsRetryable_for_OrchestrationError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    match self {
                        Self::Backoff { .. } => true,
                        Self::OpFailed { source, .. } => source.is_retryable(),
                        _ => false,
                    }
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
      - id: impl_IsRetryable_for_IndexError
        type: ItemImpl
        signature: "impl IsRetryable for IndexError {"
        children:
        - id: impl_IsRetryable_for_IndexError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    matches!(
                        self, IndexError::IndexUnavailable { .. } | IndexError::ArenaExhausted { .. }
                    )
                }
      - id: tests.layer_errors_convert_into_resident_error
        type: ItemFn
        code: |2
              #[test]
              fn layer_errors_convert_into_resident_error() {
                  let e: ResidentError = IndexError::UnknownBatch {
                      batch: 3,
                  }
                      .into();
                  assert!(
                      matches!(e, ResidentError::Index(IndexError::UnknownBatch { batch : 3 }))
                  );
                  let e: ResidentError = ConfigError::MissingField {
                      field: "model".into(),
                  }
                      .into();
                  assert!(matches!(e, ResidentError::Config(_)));
                  let e: ResidentError = PrecisionError::NoActiveProfile.into();
                  assert!(matches!(e, ResidentError::Precision(_)));
                  let e: ResidentError = PlannerError::UnknownModel {
                      model: "m".into(),
                  }
                      .into();
                  assert!(matches!(e, ResidentError::Planner(_)));
                  let e: ResidentError = CapabilityError::Missing {
                      capability: "c".into(),
                  }
                      .into();
                  assert!(matches!(e, ResidentError::Capability(_)));
              }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Shared subengine lifecycle & pull model, tokenizer/normalizer (CPU).
      - id: device
        type: ItemMod
        code: |
          mod device;
      - id: l9ZlBG|zqWesy
        type: ImportGroup
        code: |
          pub use device::*;
```
/answer 4eac-1a6 allow
Control Request:
```yaml
id: 4eac-1a7
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - DmI3w7|wIeZIJ
    - impl_IsRetryable_for_ResidentError
    - tests.all_errors_are_send_sync_static
  reason: See ResidentError retry impl and imports to add the Subengine variant
```
/answer 4eac-1a7 allow
ReasoningToken: 7808
Control Request:
```yaml
id: 4eac-1a8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: DmI3w7|wIeZIJ
        type: ImportGroup
        code: |
          use thiserror::Error;
          pub use crate::resident::processor::OrchestrationError;
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
                        ResidentError::Orchestration(e) => e.is_retryable(),
                    }
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
```
/answer 4eac-1a8 allow
Control Request:
```yaml
id: 4eac-1a9
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: ResidentError::|^use .*(OrchestrationError|ResidentError)
    include:
    - '*.rs'
    limit: 40
  reason: Check for exhaustive matches on ResidentError that a new variant could break
```
/answer 4eac-1a9 allow
Control Request:
```yaml
id: 4eac-1aa
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/processor/tests.rs
        matches:
        - lineno: 5
          match: "use crate::resident::error::{PlannerError, ResidentError};"
      - path: resident/processor/hub.rs
        matches:
        - lineno: 7
          match: use crate::resident::error::ResidentError;
        - lineno: 15
          match: use super::error::OrchestrationError;
      - path: resident/processor/error.rs
        matches:
        - lineno: 5
          match: "use crate::resident::error::{IsRetryable, ResidentError};"
        - lineno: 58
          match: "assert!(matches!(e, ResidentError::Orchestration(OrchestrationError::Shutdown)));"
      - path: resident/processor/ops.rs
        matches:
        - lineno: 8
          match: use crate::resident::error::ResidentError;
        - lineno: 13
          match: use super::error::OrchestrationError;
      - path: resident/facade/resident.rs
        matches:
        - lineno: 11
          match: "use crate::resident::error::{ConfigError, PlannerError, ResidentError};"
      - path: resident/facade/error.rs
        matches:
        - lineno: 5
          match: use crate::resident::error::ResidentError;
      - path: resident/error.rs
        matches:
        - lineno: 133
          match: "ResidentError::Index(e) => e.is_retryable(),"
        - lineno: 134
          match: "ResidentError::Precision(e) => e.is_retryable(),"
        - lineno: 135
          match: "ResidentError::Planner(e) => e.is_retryable(),"
        - lineno: 136
          match: "ResidentError::Config(e) => e.is_retryable(),"
        - lineno: 137
          match: "ResidentError::Capability(e) => e.is_retryable(),"
        - lineno: 138
          match: "ResidentError::Orchestration(e) => e.is_retryable(),"
        - lineno: 163
          match: "matches!(e, ResidentError::Index(IndexError::UnknownBatch { batch : 3 }))"
        - lineno: 169
          match: "assert!(matches!(e, ResidentError::Config(_)));"
        - lineno: 171
          match: "assert!(matches!(e, ResidentError::Precision(_)));"
        - lineno: 176
          match: "assert!(matches!(e, ResidentError::Planner(_)));"
        - lineno: 181
          match: "assert!(matches!(e, ResidentError::Capability(_)));"
        - lineno: 190
          match: "assert_eq!(ResidentError::from(inner).to_string(), msg);"
      - path: resident/planner/adapter.rs
        matches:
        - lineno: 7
          match: "use crate::resident::error::{PlannerError, ResidentError};"
        - lineno: 65
          match: "ResidentError::from(PlannerError::InvalidConfig { reason: reason.into() }).into()"
      - path: resident/op/mod.rs
        matches:
        - lineno: 10
          match: use crate::resident::error::ResidentError;
```
/answer 4eac-1aa allow
Text:
 Only one exhaustive match exists. I'll write the new files and wire the error variant in.
Control Request:
```yaml
id: 4eac-1ab
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/error.rs
    source: |
      use thiserror::Error;

      use crate::resident::error::IsRetryable;

      /// Subengine resource lifecycle errors.
      #[derive(Error, Debug)]
      #[non_exhaustive]
      pub enum SubengineError {
          #[error("resource {resource} failed to initialise: {reason}")]
          ResourceInit { resource: String, reason: String },
          #[error("resource {resource} is being evicted")]
          ResourceEvicting { resource: String },
          #[error("resource {resource} is backing off until {until_ms} ms")]
          Backoff { resource: String, until_ms: u64 },
          #[error("subengine capacity exhausted")]
          CapacityExhausted,
          #[error("incompatible resource: {reason}")]
          Incompatible { reason: String },
      }

      impl IsRetryable for SubengineError {
          fn is_retryable(&self) -> bool {
              matches!(
                  self,
                  Self::ResourceEvicting { .. } | Self::Backoff { .. } | Self::CapacityExhausted
              )
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/resource.rs
    source: |
      use std::collections::HashMap;
      use std::fmt;
      use std::hash::Hash;

      use parking_lot::Mutex;

      #[derive(Clone, Debug, PartialEq, Eq, Hash)]
      pub struct ResourceKey(String);

      impl ResourceKey {
          pub fn new(key: impl Into<String>) -> Self {
              Self(key.into())
          }
      }

      impl fmt::Display for ResourceKey {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              f.write_str(&self.0)
          }
      }

      pub trait Resource: Send + Sync + 'static {
          fn key(&self) -> ResourceKey;

          /// Relative cost of re-initialising after eviction; lets callers prefer cheap victims.
          fn init_cost_hint(&self) -> u64 {
              0
          }

          /// Veto for resource-specific reasons beyond lease and TTL checks.
          fn evictable(&self, _now_ms: u64) -> bool {
              true
          }
      }

      /// Exponentially weighted moving average; the first sample primes the value.
      #[derive(Clone, Copy, Debug)]
      pub struct Ewma {
          value: f32,
          alpha: f32,
          primed: bool,
      }

      impl Ewma {
          pub fn new(alpha: f32) -> Self {
              Self { value: 0.0, alpha: alpha.clamp(f32::EPSILON, 1.0), primed: false }
          }

          pub fn observe(&mut self, sample: f32) -> f32 {
              if self.primed {
                  self.value += self.alpha * (sample - self.value);
              } else {
                  self.value = sample;
                  self.primed = true;
              }
              self.value
          }

          pub fn get(&self) -> Option<f32> {
              self.primed.then_some(self.value)
          }
      }

      #[derive(Clone, Copy, Debug)]
      pub struct PullConfig {
          pub alpha: f32,
          /// Wall time one batch should take at the observed rate.
          pub target_batch_ms: f32,
          pub min_batch: usize,
          pub max_batch: usize,
      }

      impl Default for PullConfig {
          fn default() -> Self {
              Self { alpha: 0.2, target_batch_ms: 20.0, min_batch: 1, max_batch: 256 }
          }
      }

      /// Derives the batch size to pull from observed throughput; the clamp bounds load oscillation.
      pub struct PullController {
          rate_per_ms: Mutex<Ewma>,
          cfg: PullConfig,
      }

      impl PullController {
          pub fn new(cfg: PullConfig) -> Self {
              Self { rate_per_ms: Mutex::new(Ewma::new(cfg.alpha)), cfg }
          }

          pub fn observe(&self, items: usize, elapsed_ms: f32) {
              if items > 0 && elapsed_ms > 0.0 {
                  self.rate_per_ms.lock().observe(items as f32 / elapsed_ms);
              }
          }

          /// Items per second; zero until the first observation.
          pub fn throughput(&self) -> f32 {
              self.rate_per_ms.lock().get().map_or(0.0, |r| r * 1000.0)
          }

          pub fn batch_size(&self) -> usize {
              let min = self.cfg.min_batch.max(1);
              let max = self.cfg.max_batch.max(min);
              match self.rate_per_ms.lock().get() {
                  Some(rate) => ((rate * self.cfg.target_batch_ms).round() as usize).clamp(min, max),
                  None => min,
              }
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub struct Capacity {
          pub in_flight: usize,
          pub max_in_flight: usize,
      }

      impl Capacity {
          pub fn free(&self) -> usize {
              self.max_in_flight.saturating_sub(self.in_flight)
          }
      }

      /// Restricts which pending work a subengine is willing to take.
      #[derive(Clone, Debug, Default, PartialEq, Eq)]
      pub struct ClaimFilter {
          pub model: Option<ResourceKey>,
          pub index: Option<ResourceKey>,
      }

      /// Implemented by the processor: hands out at most `budget` jobs matching `filter`.
      pub trait Claim<J> {
          fn claim(&self, filter: &ClaimFilter, budget: usize) -> Vec<J>;
      }

      /// Groups items by key, keeping first-seen group order and item order within each group.
      pub fn coalesce<T, K: Eq + Hash + Clone>(
          items: impl IntoIterator<Item = T>,
          key: impl Fn(&T) -> K,
      ) -> Vec<(K, Vec<T>)> {
          let mut slots: HashMap<K, usize> = HashMap::new();
          let mut groups: Vec<(K, Vec<T>)> = Vec::new();
          for item in items {
              let k = key(&item);
              match slots.get(&k) {
                  Some(&i) => groups[i].1.push(item),
                  None => {
                      slots.insert(k.clone(), groups.len());
                      groups.push((k, vec![item]));
                  }
              }
          }
          groups
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          #[test]
          fn ewma_primes_then_smooths() {
              let mut e = Ewma::new(0.5);
              assert_eq!(e.get(), None);
              assert_eq!(e.observe(10.0), 10.0);
              assert_eq!(e.observe(20.0), 15.0);
          }

          #[test]
          fn batch_size_is_clamped_and_starts_at_min() {
              let c = PullController::new(PullConfig { min_batch: 2, max_batch: 8, ..Default::default() });
              assert_eq!(c.batch_size(), 2);
              c.observe(1000, 1.0);
              assert_eq!(c.batch_size(), 8);
              let slow = PullController::new(PullConfig { min_batch: 2, max_batch: 8, ..Default::default() });
              slow.observe(1, 1000.0);
              assert_eq!(slow.batch_size(), 2);
          }

          #[test]
          fn noisy_samples_do_not_oscillate_batch_size() {
              let c = PullController::new(PullConfig { target_batch_ms: 100.0, min_batch: 1, max_batch: 64, ..Default::default() });
              let mut state = 0x2545_F491_4F6C_DD1Du64;
              let mut sizes = Vec::new();
              for _ in 0..200 {
                  state ^= state << 13;
                  state ^= state >> 7;
                  state ^= state << 17;
                  let jitter = 0.8 + (state % 400) as f32 / 1000.0;
                  c.observe(10, 10.0 * jitter);
                  sizes.push(c.batch_size() as i64);
              }
              let tail = &sizes[100..];
              let spread = tail.iter().max().unwrap() - tail.iter().min().unwrap();
              assert!(spread <= 30, "spread {spread}");
              assert!(c.throughput() > 0.0);
          }

          #[test]
          fn capacity_free_saturates() {
              assert_eq!(Capacity { in_flight: 5, max_in_flight: 3 }.free(), 0);
              assert_eq!(Capacity { in_flight: 1, max_in_flight: 3 }.free(), 2);
          }

          #[test]
          fn coalesce_groups_in_first_seen_order() {
              let g = coalesce([("a", 1), ("b", 2), ("a", 3)], |x| x.0);
              assert_eq!(g, vec![("a", vec![("a", 1), ("a", 3)]), ("b", vec![("b", 2)])]);
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/lifecycle.rs
    source: |
      use std::future::Future;
      use std::ops::Deref;
      use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering};
      use std::sync::Arc;

      use parking_lot::RwLock;
      use tokio::sync::{OnceCell, Semaphore};

      use super::error::SubengineError;
      use super::resource::{Resource, ResourceKey};

      const UNLOADED: u8 = 0;
      const LOADING: u8 = 1;
      const RESIDENT: u8 = 2;
      const EVICTING: u8 = 3;
      const FAILED: u8 = 4;

      /// Marks "no idle period measured yet"; set whenever the last lease is released.
      const IDLE_UNSET: u64 = u64::MAX;

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum ResourceState {
          Unloaded,
          Loading,
          Resident,
          Leased(usize),
          Idle,
          Evicting,
          Failed,
      }

      #[derive(Clone, Copy, Debug)]
      pub struct CellConfig {
          pub ttl_ms: u64,
          pub backoff_ms: u64,
      }

      /// Bounds the number of resources initialising concurrently across all cells.
      #[derive(Clone, Debug)]
      pub struct InitLimiter(Arc<Semaphore>);

      impl InitLimiter {
          pub fn new(max_parallel: usize) -> Self {
              Self(Arc::new(Semaphore::new(max_parallel.max(1))))
          }

          async fn acquire(&self) -> Result<tokio::sync::SemaphorePermit<'_>, SubengineError> {
              self.0.acquire().await.map_err(|_| SubengineError::CapacityExhausted)
          }
      }

      struct Shared {
          phase: AtomicU8,
          leases: AtomicUsize,
          in_flight: AtomicUsize,
          idle_since_ms: AtomicU64,
          backoff_until_ms: AtomicU64,
      }

      impl Shared {
          fn release_lease(&self) {
              if self.leases.fetch_sub(1, Ordering::SeqCst) == 1 {
                  self.idle_since_ms.store(IDLE_UNSET, Ordering::SeqCst);
              }
          }
      }

      /// A lazily initialised, evictable resource.
      ///
      /// Acquiring a lease takes the slot's read lock while bumping the lease count, and eviction
      /// re-checks the count under the write lock, so a lease can never be handed out for a resource
      /// that is being torn down.
      pub struct ResourceCell<R> {
          key: ResourceKey,
          cfg: CellConfig,
          shared: Arc<Shared>,
          slot: RwLock<Arc<OnceCell<Arc<R>>>>,
      }

      impl<R: Resource> ResourceCell<R> {
          pub fn new(key: ResourceKey, cfg: CellConfig) -> Self {
              Self {
                  key,
                  cfg,
                  shared: Arc::new(Shared {
                      phase: AtomicU8::new(UNLOADED),
                      leases: AtomicUsize::new(0),
                      in_flight: AtomicUsize::new(0),
                      idle_since_ms: AtomicU64::new(IDLE_UNSET),
                      backoff_until_ms: AtomicU64::new(0),
                  }),
                  slot: RwLock::new(Arc::new(OnceCell::new())),
              }
          }

          pub fn key(&self) -> &ResourceKey {
              &self.key
          }

          pub fn state(&self) -> ResourceState {
              match self.shared.phase.load(Ordering::Acquire) {
                  UNLOADED => ResourceState::Unloaded,
                  LOADING => ResourceState::Loading,
                  RESIDENT => match self.shared.leases.load(Ordering::Acquire) {
                      0 => ResourceState::Idle,
                      n => ResourceState::Leased(n),
                  },
                  EVICTING => ResourceState::Evicting,
                  _ => ResourceState::Failed,
              }
          }

          /// Leases the resource, running `init` if it is not resident. Concurrent callers share one
          /// initialisation; a failure leaves the cell empty and rejects callers until the backoff ends.
          pub async fn get_or_init<F, Fut>(
              &self,
              limiter: &InitLimiter,
              now_ms: u64,
              init: F,
          ) -> Result<ResourceLease<R>, SubengineError>
          where
              F: FnOnce() -> Fut,
              Fut: Future<Output = Result<R, SubengineError>>,
          {
              let shared = &*self.shared;
              let cell = {
                  let slot = self.slot.read();
                  match shared.phase.load(Ordering::SeqCst) {
                      EVICTING => {
                          return Err(SubengineError::ResourceEvicting { resource: self.key.to_string() });
                      }
                      FAILED => {
                          let until_ms = shared.backoff_until_ms.load(Ordering::Acquire);
                          if now_ms < until_ms {
                              return Err(SubengineError::Backoff {
                                  resource: self.key.to_string(),
                                  until_ms,
                              });
                          }
                      }
                      _ => {}
                  }
                  shared.leases.fetch_add(1, Ordering::SeqCst);
                  Arc::clone(&slot)
              };

              let backoff_ms = self.cfg.backoff_ms;
              let key = &self.key;
              let loaded = cell
                  .get_or_try_init(|| async move {
                      let _permit = limiter.acquire().await?;
                      shared.phase.store(LOADING, Ordering::SeqCst);
                      tracing::debug!(resource = %key, "loading");
                      match init().await {
                          Ok(r) => {
                              shared.phase.store(RESIDENT, Ordering::SeqCst);
                              tracing::debug!(resource = %key, "resident");
                              Ok(Arc::new(r))
                          }
                          Err(e) => {
                              shared
                                  .backoff_until_ms
                                  .store(now_ms.saturating_add(backoff_ms), Ordering::Release);
                              shared.phase.store(FAILED, Ordering::SeqCst);
                              tracing::warn!(resource = %key, error = %e, "failed");
                              Err(e)
                          }
                      }
                  })
                  .await;

              match loaded {
                  Ok(value) => {
                      tracing::trace!(resource = %self.key, "lease_acquired");
                      Ok(ResourceLease { value: Arc::clone(value), shared: Arc::clone(&self.shared) })
                  }
                  Err(e) => {
                      shared.release_lease();
                      Err(e)
                  }
              }
          }

          /// Unloads the resource when unleased, idle past the TTL and not vetoed by the resource.
          /// The first call after the last lease ends only starts the idle clock.
          pub fn evict_idle(&self, now_ms: u64) -> bool {
              let shared = &*self.shared;
              if shared.leases.load(Ordering::Acquire) != 0
                  || shared.phase.load(Ordering::Acquire) != RESIDENT
              {
                  return false;
              }

              let mut slot = self.slot.write();
              if shared.leases.load(Ordering::SeqCst) != 0
                  || shared.in_flight.load(Ordering::SeqCst) != 0
                  || shared.phase.load(Ordering::SeqCst) != RESIDENT
              {
                  return false;
              }
              let since = shared.idle_since_ms.load(Ordering::SeqCst);
              if since == IDLE_UNSET {
                  shared.idle_since_ms.store(now_ms, Ordering::SeqCst);
                  return false;
              }
              if now_ms.saturating_sub(since) < self.cfg.ttl_ms {
                  return false;
              }
              match slot.get() {
                  Some(value) if value.evictable(now_ms) => {}
                  _ => return false,
              }

              shared.phase.store(EVICTING, Ordering::SeqCst);
              let old = std::mem::replace(&mut *slot, Arc::new(OnceCell::new()));
              shared.idle_since_ms.store(IDLE_UNSET, Ordering::SeqCst);
              shared.phase.store(UNLOADED, Ordering::SeqCst);
              drop(slot);
              drop(old);
              tracing::debug!(resource = %self.key, "evicted");
              true
          }
      }

      pub struct ResourceLease<R> {
          value: Arc<R>,
          shared: Arc<Shared>,
      }

      impl<R> ResourceLease<R> {
          /// Marks an operation as running on the resource; eviction waits for the guard to drop.
          pub fn begin_op(&self) -> OpGuard {
              self.shared.in_flight.fetch_add(1, Ordering::SeqCst);
              OpGuard { shared: Arc::clone(&self.shared) }
          }
      }

      impl<R> Deref for ResourceLease<R> {
          type Target = R;

          fn deref(&self) -> &R {
              &self.value
          }
      }

      impl<R> Drop for ResourceLease<R> {
          fn drop(&mut self) {
              self.shared.release_lease();
          }
      }

      pub struct OpGuard {
          shared: Arc<Shared>,
      }

      impl Drop for OpGuard {
          fn drop(&mut self) {
              self.shared.in_flight.fetch_sub(1, Ordering::SeqCst);
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use std::sync::atomic::AtomicUsize;

          struct Dummy(u32);

          impl Resource for Dummy {
              fn key(&self) -> ResourceKey {
                  ResourceKey::new(format!("dummy-{}", self.0))
              }
          }

          fn cell(ttl_ms: u64, backoff_ms: u64) -> ResourceCell<Dummy> {
              ResourceCell::new(ResourceKey::new("dummy"), CellConfig { ttl_ms, backoff_ms })
          }

          fn init_err() -> SubengineError {
              SubengineError::ResourceInit { resource: "dummy".into(), reason: "boom".into() }
          }

          #[tokio::test]
          async fn parallel_callers_share_one_initialiser() {
              let c = cell(10, 10);
              let limiter = InitLimiter::new(4);
              let inits = AtomicUsize::new(0);
              let call = || {
                  c.get_or_init(&limiter, 0, || async {
                      inits.fetch_add(1, Ordering::SeqCst);
                      tokio::task::yield_now().await;
                      Ok(Dummy(1))
                  })
              };
              let leases = futures_join(vec![call(), call(), call(), call()]).await;
              assert_eq!(inits.load(Ordering::SeqCst), 1);
              assert!(leases.iter().all(|l| l.as_ref().is_ok_and(|l| l.0 == 1)));
              assert_eq!(c.state(), ResourceState::Leased(4));
              drop(leases);
              assert_eq!(c.state(), ResourceState::Idle);
          }

          async fn futures_join<F: Future>(fs: Vec<F>) -> Vec<F::Output> {
              let mut out = Vec::new();
              let mut set = Vec::new();
              for f in fs {
                  set.push(Box::pin(f));
              }
              // Polling sequentially would serialise the callers; interleave them via select-free manual join.
              let mut pending: Vec<Option<std::pin::Pin<Box<F>>>> = set.into_iter().map(Some).collect();
              let mut results: Vec<Option<F::Output>> = (0..pending.len()).map(|_| None).collect();
              std::future::poll_fn(|cx| {
                  let mut done = true;
                  for (i, slot) in pending.iter_mut().enumerate() {
                      if let Some(f) = slot {
                          match f.as_mut().poll(cx) {
                              std::task::Poll::Ready(v) => {
                                  results[i] = Some(v);
                                  *slot = None;
                              }
                              std::task::Poll::Pending => done = false,
                          }
                      }
                  }
                  if done { std::task::Poll::Ready(()) } else { std::task::Poll::Pending }
              })
              .await;
              out.extend(results.into_iter().map(|r| r.unwrap()));
              out
          }

          #[tokio::test]
          async fn failed_init_leaves_cell_empty_and_backs_off() {
              let c = cell(10, 100);
              let limiter = InitLimiter::new(1);
              let err = c.get_or_init(&limiter, 0, || async { Err(init_err()) }).await.err().unwrap();
              assert!(matches!(err, SubengineError::ResourceInit { .. }));
              assert_eq!(c.state(), ResourceState::Failed);

              let err = c.get_or_init(&limiter, 50, || async { Ok(Dummy(1)) }).await.err().unwrap();
              assert!(matches!(err, SubengineError::Backoff { until_ms: 100, .. }));

              let lease = c.get_or_init(&limiter, 100, || async { Ok(Dummy(2)) }).await.unwrap();
              assert_eq!(lease.0, 2);
              assert_eq!(c.state(), ResourceState::Leased(1));
          }

          #[tokio::test]
          async fn limiter_bounds_parallel_inits_across_cells() {
              let limiter = InitLimiter::new(1);
              let running = AtomicUsize::new(0);
              let peak = AtomicUsize::new(0);
              let cells: Vec<_> = (0..3).map(|_| cell(10, 10)).collect();
              let calls = cells.iter().map(|c| {
                  c.get_or_init(&limiter, 0, || async {
                      let now = running.fetch_add(1, Ordering::SeqCst) + 1;
                      peak.fetch_max(now, Ordering::SeqCst);
                      tokio::task::yield_now().await;
                      running.fetch_sub(1, Ordering::SeqCst);
                      Ok(Dummy(0))
                  })
              });
              let leases = futures_join(calls.collect()).await;
              assert!(leases.iter().all(Result::is_ok));
              assert_eq!(peak.load(Ordering::SeqCst), 1);
          }

          #[tokio::test]
          async fn eviction_waits_for_lease_op_and_ttl() {
              let c = cell(100, 10);
              let limiter = InitLimiter::new(1);
              let lease = c.get_or_init(&limiter, 0, || async { Ok(Dummy(1)) }).await.unwrap();
              assert!(!c.evict_idle(1_000));

              let op = lease.begin_op();
              drop(lease);
              assert!(!c.evict_idle(1_000));
              drop(op);

              assert!(!c.evict_idle(1_000), "first call only starts the idle clock");
              assert!(!c.evict_idle(1_050));
              assert!(c.evict_idle(1_100));
              assert_eq!(c.state(), ResourceState::Unloaded);

              let lease = c.get_or_init(&limiter, 1_200, || async { Ok(Dummy(2)) }).await.unwrap();
              assert_eq!(lease.0, 2);
          }

          #[tokio::test]
          async fn lease_taken_after_idle_clock_blocks_eviction() {
              let c = cell(10, 10);
              let limiter = InitLimiter::new(1);
              drop(c.get_or_init(&limiter, 0, || async { Ok(Dummy(1)) }).await.unwrap());
              assert!(!c.evict_idle(0));
              let lease = c.get_or_init(&limiter, 5, || async { Ok(Dummy(9)) }).await.unwrap();
              assert_eq!(lease.0, 1);
              assert!(!c.evict_idle(1_000));
              drop(lease);
              assert!(!c.evict_idle(1_000));
              assert!(c.evict_idle(1_010));
          }

          #[test]
          fn concurrent_leases_and_eviction_never_lose_a_resident_lease() {
              let c = Arc::new(cell(0, 1));
              let limiter = InitLimiter::new(1);
              let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(2).build().unwrap();
              let evictor = {
                  let c = Arc::clone(&c);
                  std::thread::spawn(move || {
                      for t in 0..5_000 {
                          c.evict_idle(t);
                      }
                  })
              };
              rt.block_on(async {
                  for _ in 0..2_000 {
                      let lease = c.get_or_init(&limiter, 0, || async { Ok(Dummy(7)) }).await.unwrap();
                      assert_eq!(lease.0, 7);
                      tokio::task::yield_now().await;
                      assert_eq!(lease.0, 7);
                  }
              });
              evictor.join().unwrap();
          }
      }
  reason: "Create the D1 subengine lifecycle modules: error, resource helpers and the lease-based lifecycle cell"
```
/answer 4eac-1ab allow
ReasoningToken: 133
Control Request:
```yaml
id: 4eac-1ac
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/lifecycle.rs
    start_marker: "async fn futures_join<F: Future>"
    end_marker: |-
      Ok(Dummy(0))
                  })
              });
    content: |-
      async fn futures_join<F: Future>(fs: Vec<F>) -> Vec<F::Output> {
              let mut pending: Vec<_> = fs.into_iter().map(|f| Some(Box::pin(f))).collect();
              let mut results: Vec<Option<F::Output>> = (0..pending.len()).map(|_| None).collect();
              std::future::poll_fn(|cx| {
                  let mut done = true;
                  for (slot, result) in pending.iter_mut().zip(results.iter_mut()) {
                      if let Some(f) = slot {
                          match f.as_mut().poll(cx) {
                              std::task::Poll::Ready(v) => {
                                  *result = Some(v);
                                  *slot = None;
                              }
                              std::task::Poll::Pending => done = false,
                          }
                      }
                  }
                  if done { std::task::Poll::Ready(()) } else { std::task::Poll::Pending }
              })
              .await;
              results.into_iter().flatten().collect()
          }

          #[tokio::test]
          async fn failed_init_leaves_cell_empty_and_backs_off() {
              let c = cell(10, 100);
              let limiter = InitLimiter::new(1);
              let err = c.get_or_init(&limiter, 0, || async { Err(init_err()) }).await.err().unwrap();
              assert!(matches!(err, SubengineError::ResourceInit { .. }));
              assert_eq!(c.state(), ResourceState::Failed);

              let err = c.get_or_init(&limiter, 50, || async { Ok(Dummy(1)) }).await.err().unwrap();
              assert!(matches!(err, SubengineError::Backoff { until_ms: 100, .. }));

              let lease = c.get_or_init(&limiter, 100, || async { Ok(Dummy(2)) }).await.unwrap();
              assert_eq!(lease.0, 2);
              assert_eq!(c.state(), ResourceState::Leased(1));
          }

          #[tokio::test]
          async fn limiter_bounds_parallel_inits_across_cells() {
              let limiter = InitLimiter::new(1);
              let running = AtomicUsize::new(0);
              let peak = AtomicUsize::new(0);
              let cells: Vec<_> = (0..3).map(|_| cell(10, 10)).collect();
              let calls = cells.iter().map(|c| {
                  c.get_or_init(&limiter, 0, || async {
                      let now = running.fetch_add(1, Ordering::SeqCst) + 1;
                      peak.fetch_max(now, Ordering::SeqCst);
                      tokio::task::yield_now().await;
                      running.fetch_sub(1, Ordering::SeqCst);
                      Ok(Dummy(0))
                  })
              });
  reason: "Simplify the test join helper, deduplicating the test region it spanned"
```
/answer 4eac-1ac allow
Control Request:
```yaml
id: 4eac-1ad
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
    id: device
    source: |
      mod device;
      mod error;
      mod lifecycle;
      mod resource;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
    id: l9ZlBG|zqWesy
    source: |
      pub use device::*;
      pub use error::SubengineError;
      pub use lifecycle::{
          CellConfig, InitLimiter, OpGuard, ResourceCell, ResourceLease, ResourceState,
      };
      pub use resource::{
          coalesce, Capacity, Claim, ClaimFilter, Ewma, PullConfig, PullController, Resource,
          ResourceKey,
      };

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum SubengineKind {
          Cpu,
          Cuda,
      }

      /// Resources and execution capacity of one compute substrate. Work arrives through the
      /// processor's op queues; the subengine only reports how much it can take.
      pub trait Subengine: Send + Sync {
          fn kind(&self) -> SubengineKind;
          fn capacity(&self) -> Capacity;
          /// Smoothed items per second.
          fn throughput(&self) -> f32;
          /// Throughput-derived batch size, independent of current load.
          fn batch_size(&self) -> usize;

          /// Upper bound on the jobs to hand this subengine in one `Claim::claim` call.
          fn pull_budget(&self) -> usize {
              self.capacity().free().min(self.batch_size())
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: ResidentError
    source: |
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
          #[error(transparent)]
          Orchestration(#[from] OrchestrationError),
          #[error(transparent)]
          Subengine(#[from] SubengineError),
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: DmI3w7|wIeZIJ
    source: |
      use thiserror::Error;
      pub use crate::resident::processor::OrchestrationError;
      pub use crate::resident::subengine::SubengineError;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: impl_IsRetryable_for_ResidentError.is_retryable
    source: |2
          fn is_retryable(&self) -> bool {
              match self {
                  ResidentError::Index(e) => e.is_retryable(),
                  ResidentError::Precision(e) => e.is_retryable(),
                  ResidentError::Planner(e) => e.is_retryable(),
                  ResidentError::Config(e) => e.is_retryable(),
                  ResidentError::Capability(e) => e.is_retryable(),
                  ResidentError::Orchestration(e) => e.is_retryable(),
                  ResidentError::Subengine(e) => e.is_retryable(),
              }
          }
  reason: "Export the new modules, add Subengine trait/kind, and wire SubengineError into ResidentError"
```
/answer 4eac-1ad allow
ReasoningToken: 102
Control Request:
```yaml
id: 4eac-1ae
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/lifecycle.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/resource.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  reason: Syntax-check edited files
```
/answer 4eac-1ae allow
ReasoningToken: 1679
Control Request:
```yaml
id: 4eac-1af
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/resource.rs
    id: tests.noisy_samples_do_not_oscillate_batch_size
    old_text: "min_batch: 1, max_batch: 64, ..Default::default()"
    new_text: "min_batch: 1, max_batch: 256, ..Default::default()"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/resource.rs
    id: tests.noisy_samples_do_not_oscillate_batch_size
    old_text: "assert!(spread <= 30, \"spread {spread}\");"
    new_text: "assert!(spread <= 40, \"spread {spread}\");"
  reason: Make the oscillation test non-trivial by keeping batch size below the clamp
```
/answer 4eac-1af allow
Result Stats: id=85283fc8-f4ab-45d8-9965-40450393f998, total: 90042, in: 26, out: 27680, reason: 0, read: 415217, write: 62336, start: 1792324632037, end: 1792324639978, time: 7899, total: 177190, io: 0, control: 42, sent: 0, recv: 213
I implemented D1 in `rag/src/resident/subengine/`. I couldn't run `cargo` here. The files pass a syntax check, but nothing has been compiled or tested yet. Please run `cargo test` (without `cuda`) before relying on it.

- **Errors:** `SubengineError` is new and has the five variants from the plan, with a retryable classification. I added it to `ResidentError` as `Subengine(#[from])`, including the match in `is_retryable`.
- **Lifecycle:**
  - `ResourceCell<R>` loads lazily, and parallel callers share one initialiser.
  - `InitLimiter` limits parallel inits across cells, and each init holds its permit inside the init closure.
  - A failed init leaves the cell empty and rejects callers until the backoff ends.
  - `ResourceLease<R>` is released on drop. `begin_op()` returns an `OpGuard` that counts in-flight work.
  - `evict_idle` re-checks leases, in-flight work, idle time and a per-resource veto under the write lock.
  - `ResourceState` reports Unloaded, Loading, Resident, Leased(n), Idle, Evicting and Failed.
- **Pull model:**
  - `Ewma` smooths the throughput samples.
  - `PullController` turns the smoothed throughput into a clamped batch size.
  - `Capacity` reports free slots.
  - The `Subengine` trait (with `SubengineKind`) exposes `pull_budget()`.
  - `ClaimFilter` and the `Claim<J>` trait define what the processor serves.
  - `coalesce` groups waiting work by key.
- **Tests:** they cover single-initialiser sharing, the init limit, failed-init backoff, eviction rules, the EWMA and batch clamp, `coalesce`, and a threaded lease/evict stress test.

I deviated from the plan in three places:
- **Time:** it is passed in as `now_ms: u64`, as in the existing `processor::ResourceSlot`. `Instant` and the `Clock` trait don't fit, because `Clock` only exists under `cfg(test)` or the `mock` feature. The eviction idle clock starts on the first `evict_idle` call after the last lease ends, so dropping a lease needs no clock.
- **`OnceCell`:** eviction replaces the whole `Arc<OnceCell>` inside an `RwLock`, because a `OnceCell` can't be reset in place. Taking a lease and evicting are therefore mutually exclusive, which also means the `Evicting` state is only briefly visible.
- **`Subengine` trait:** it has no `device()`, `resource()` or `run_ops()` yet, since D2 and the op types aren't wired in. There is also no `Subengine` implementation yet, only the trait.

The concurrency test uses plain threads rather than `loom`, because the cell relies on `parking_lot::RwLock`, which `loom` doesn't model.
