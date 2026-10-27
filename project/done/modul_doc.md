Ich brauche in `/home/user/xyan/xy.ai.workbench/rag/src/resident` eine englischsprachige kompakte Dokumentation (englisch) der Module (README.md pro Ebene), der die Module, seinen Kontext, seine Konzeption und der Funktion nach und den Komponenten fassbar macht. Mir fällt die Zuordnung und Einordnung schwer und ich möchte nicht jede Source einzeln dafür öffnen müssen. Die Orientierung soll für Menschen und Agenten vereinfacht werden, ohne Codekommentare zu duplizieren und ohne in jedem Modul die ganze Projektkonzeption zu wiederholen. Der Informationsumfang soll ausreichend aber nicht redundant oder zu übergreifend sein.
Die jede Umbenennung einer function soll eine Änderung benötigen. Dateinamen sind jedoch final.

`/home/user/xyan/xy.ai.workbench/project/resident/resident_first.md`
`/home/user/xyan/xy.ai.workbench/project/resident/resident_index.md`
`/home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md`
`/home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md`
`/home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md`
`/home/user/xyan/xy.ai.workbench/project/resident/resident_subengine.md`
`/home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md`
SystemInit:  id=3da6f405-9126-4575-ae25-c66a27d28d06, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich verschaffe mir zuerst einen Überblick über die Quellen und die Projektdokumente.
Control Request:
```yaml
id: cc1b-1
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  reason: Struktur des resident-Quellverzeichnisses und der Projektdokumente erfassen
```
/answer cc1b-1 allow
ReasoningToken: 16
Control Request:
```yaml
id: cc1b-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".: (6 files)"
      - backend.rs
      - "./facade: (9 files)"
      - blocking.rs
      - "./facade/test: (4 files)"
      - addon.rs
      - "./index: (6 files)"
      - memory.rs
      - "./index/test: (3 files)"
      - memory.rs
      - "./kernel: (14 files)"
      - catalog.rs
      - "./kernel/test: (10 files)"
      - cosine.rs
      - "./manager: (19 files)"
      - advice.rs
      - "./manager/test: (16 files)"
      - advice.rs
      - "./memory: (13 files)"
      - arena.rs
      - "./memory/test: (9 files)"
      - arena.rs
      - "./model: (8 files)"
      - batch.rs
      - "./model/test: (6 files)"
      - batch.rs
      - "./op: (3 files)"
      - lease.rs
      - "./op/test: (3 files)"
      - lease.rs
      - "./planner: (6 files)"
      - adapter.rs
      - "./planner/test: (2 files)"
      - adapter.rs
      - "./precision: (20 files)"
      - axis.rs
      - "./precision/test: (17 files)"
      - axis.rs
      - "./processor: (11 files)"
      - completion.rs
      - "./processor/test: (6 files)"
      - completion.rs
      - "./queue: (5 files)"
      - mod.rs
      - "./segment: (5 files)"
      - error.rs
      - "./segment/test: (4 files)"
      - mapping.rs
      - "./subengine: (6 files)"
      - device.rs
      - "./subengine/test: (4 files)"
      - device.rs
      - "./test: (2 files)"
      - error.rs
      - "./tokenize: (4 files)"
      - error.rs
      - "./tokenize/test: (1 file)"
      - mod.rs
      warning: Result reduced because the total number of entries (222) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer cc1b-2 allow
Control Request:
```yaml
id: cc1b-3
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_first.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_index.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_subengine.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  reason: Die sieben genannten Projektkonzeptdokumente lesen
```
/answer cc1b-3 allow
Text:
 Jetzt die tatsächliche Quellstruktur (ohne Tests) genauer ansehen.
ReasoningToken: 33
Control Request:
```yaml
id: cc1b-4
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: ^(?!.*/test/).*$
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: README
  reason: "Alle Nicht-Test-Dateien auflisten und prüfen, ob bereits READMEs existieren"
```
/answer cc1b-4 allow
Control Request:
```yaml
id: cc1b-5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".: (6 files)"
      - backend.rs
      - "./facade: (9 files)"
      - blocking.rs
      - "./index: (6 files)"
      - memory.rs
      - "./kernel: (14 files)"
      - catalog.rs
      - "./manager: (19 files)"
      - advice.rs
      - "./memory: (13 files)"
      - arena.rs
      - "./model: (8 files)"
      - batch.rs
      - "./op: (3 files)"
      - lease.rs
      - "./planner: (6 files)"
      - adapter.rs
      - "./precision: (20 files)"
      - axis.rs
      - "./processor: (11 files)"
      - completion.rs
      - "./queue: (5 files)"
      - mod.rs
      - "./segment: (5 files)"
      - error.rs
      - "./subengine: (6 files)"
      - device.rs
      - "./tokenize: (4 files)"
      - error.rs
      warning: Result reduced because the total number of entries (135) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer cc1b-5 allow
ReasoningToken: 31
Control Request:
```yaml
id: cc1b-6
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize
    recursive: false
  reason: "Dateien jeder Modulebene einzeln auflisten, da die rekursive Liste gekürzt wurde"
```
/answer cc1b-6 allow
Control Request:
```yaml
id: cc1b-7
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      entries:
      - ".:"
      - blocking.rs
      - error.rs
      - extensions.rs
      - handle.rs
      - mod.rs
      - params.rs
      - report.rs
      - resident.rs
      - task.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index
      entries:
      - ".:"
      - memory.rs
      - mod.rs
      - parquet_io.rs
      - payload.rs
      - schema.rs
      - stamp.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
      entries:
      - ".:"
      - catalog.rs
      - cosine.rs
      - cpu_simd.rs
      - cuda.rs
      - error.rs
      - golden.rs
      - launcher.rs
      - maxsim.rs
      - mock.rs
      - mod.rs
      - registry.rs
      - slice.rs
      - slots.rs
      - topk.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      entries:
      - ".:"
      - advice.rs
      - arena.rs
      - blocking.rs
      - build.rs
      - doorbell.rs
      - evict.rs
      - flight.rs
      - mod.rs
      - residency.rs
      - runtime.rs
      - scope.rs
      - seqlock.rs
      - slot.rs
      - streamed.rs
      - swap.rs
      - sync.rs
      - tracker.rs
      - transfer.rs
      - upload.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      entries:
      - ".:"
      - arena.rs
      - budget.rs
      - cpu.rs
      - cuda.rs
      - dtype.rs
      - error.rs
      - freelist.rs
      - layout.rs
      - manager.rs
      - mod.rs
      - region.rs
      - staging.rs
      - wrap_gate.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model
      entries:
      - ".:"
      - batch.rs
      - candle_adapter.rs
      - encoder.rs
      - error.rs
      - head.rs
      - mod.rs
      - pooling.rs
      - slot.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      entries:
      - ".:"
      - lease.rs
      - mod.rs
      - requirements.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
      entries:
      - ".:"
      - adapter.rs
      - config.rs
      - expand.rs
      - mod.rs
      - offers.rs
      - rules.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      entries:
      - ".:"
      - axis.rs
      - calibrate.rs
      - capability.rs
      - known.rs
      - matching.rs
      - measure.rs
      - measure_cpu.rs
      - measure_cuda.rs
      - measure_mock.rs
      - metrics.rs
      - mod.rs
      - pareto.rs
      - pool.rs
      - probe.rs
      - probe_cpu.rs
      - probe_cuda.rs
      - profile.rs
      - stats.rs
      - store.rs
      - workload.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
      entries:
      - ".:"
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue
      entries:
      - ".:"
      - mod.rs
      - node.rs
      - queue.rs
      - state.rs
      - task.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
      entries:
      - ".:"
      - error.rs
      - mapping.rs
      - mod.rs
      - slots.rs
      - sync.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
      entries:
      - ".:"
      - device.rs
      - error.rs
      - exec.rs
      - lifecycle.rs
      - mod.rs
      - resource.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize
      entries:
      - ".:"
      - error.rs
      - mod.rs
      - modes.rs
      - normalize.rs
```
/answer cc1b-7 allow
Text:
 Rund 130 Quelldateien. Ich lese die Modulstruktur per Outline (Docstrings/Typen), beginnend mit der Wurzel und den kleineren Ebenen.
Control Request:
```yaml
id: cc1b-8
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/backend.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/failpoint.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/telemetry.rs
  reason: Wurzelebene des resident-Moduls erfassen
```
/answer cc1b-8 allow
ReasoningToken: 591
Control Request:
```yaml
id: cc1b-9
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
        signature: pub mod mock;
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
      - id: model
        signature: pub mod model;
      - id: tokenize
        signature: pub mod tokenize;
      - id: manager
        signature: pub mod manager;
      - id: segment
        signature: pub mod segment;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/backend.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Compute backend abstraction and the single `#[cfg(feature = \"cuda\")]` decis…"
      - id: fgO4rd|wZT8Ld
        signature: use crate::resident::subengine::CpuBackend;
      - id: ComputeBackend
        signature: "pub trait ComputeBackend: Send + Sync {"
        docstring: Backend abstraction the orchestration layer runs against.
        children:
        - id: ComputeBackend.run_batch
          signature: "fn run_batch(&self, input: &[f32]) -> Vec<f32>;"
      - id: create_backend
        signature: "pub fn create_backend() -> Box<dyn ComputeBackend> {"
        docstring: "Constructs the active backend: CPU without `cuda`; with `cuda`, attempts the CU…"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: ModuleDoc
        signature: //! Error taxonomy.
      - id: gaWICu|jY2QN9
        signature: use thiserror::Error;
      - id: IsRetryable
        signature: "pub trait IsRetryable {"
        docstring: "Classifies an error as retryable (transient) vs. fatal, for backoff decisions."
        children:
        - id: IsRetryable.is_retryable
          signature: fn is_retryable(&self) -> bool;
      - id: IndexError
        signature: "pub enum IndexError {"
        docstring: Index/scope layer errors.
      - id: impl_IsRetryable_for_IndexError
        signature: "impl IsRetryable for IndexError {"
        children:
        - id: impl_IsRetryable_for_IndexError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: ManagerError
        signature: "pub enum ManagerError {"
        docstring: Index-manager layer errors. The facade maps them to public variants or retries;…
      - id: impl_IsRetryable_for_ManagerError
        signature: "impl IsRetryable for ManagerError {"
        children:
        - id: impl_IsRetryable_for_ManagerError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: PrecisionError
        signature: "pub enum PrecisionError {"
        docstring: Precision/calibration layer errors.
      - id: impl_IsRetryable_for_PrecisionError
        signature: "impl IsRetryable for PrecisionError {"
        children:
        - id: impl_IsRetryable_for_PrecisionError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: PlannerError
        signature: "pub enum PlannerError {"
        docstring: Planner/task-expansion layer errors.
      - id: impl_IsRetryable_for_PlannerError
        signature: "impl IsRetryable for PlannerError {"
        children:
        - id: impl_IsRetryable_for_PlannerError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: CapabilityError
        signature: "pub enum CapabilityError {"
        docstring: Capability/profile-matching layer errors.
      - id: ConfigError
        signature: "pub enum ConfigError {"
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
      - id: gVatFA|OrOCFr
        signature: pub use crate::resident::subengine::DeviceError;
      - id: ResidentError
        signature: "pub enum ResidentError {"
        docstring: "Top-level resident engine error, wrapping every layer error via `#[from]`."
      - id: impl_IsRetryable_for_ResidentError
        signature: "impl IsRetryable for ResidentError {"
        children:
        - id: impl_IsRetryable_for_ResidentError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/failpoint.rs
      nodes:
      - id: ModuleDoc
        signature: //! Named fault-injection points at synchronisation boundaries.
      - id: imp
        signature: "mod imp {"
        children:
        - id: imp.2Cm4Tq|ThJiiw
          signature: use std::collections::HashMap;
        - id: imp.vvlj8j|L38rUw
          signature: "static ARMED: LazyLock<Mutex<HashMap<(&'static str, u64), u32>>> ="
        - id: imp.Armed
          signature: "pub struct Armed(&'static str, u64);"
          docstring: Disarms the point on drop.
        - id: imp.impl_Drop_for_Armed
          signature: "impl Drop for Armed {"
          children:
          - id: imp.impl_Drop_for_Armed.drop
            signature: "fn drop(&mut self) {"
        - id: imp.arm
          signature: "pub fn arm(name: &'static str, key: u64, times: u32) -> Armed {"
          docstring: "Makes the next `times` hits of `(name, key)` fire."
        - id: imp.hit
          signature: "pub fn hit(name: &'static str, key: u64) -> bool {"
      - id: 5LllGF|Ny8PeE
        signature: "#[cfg(any(test, feature = \"mock\"))]"
      - id: hit
        signature: "pub fn hit(_name: &'static str, _key: u64) -> bool {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
      nodes:
      - id: ModuleDoc
        signature: //! Mock/test-double conventions.
      - id: Clock
        signature: "pub trait Clock: Send + Sync {"
        docstring: Example of a narrow trait a hot-path component depends on generically.
        children:
        - id: Clock.now_millis
          signature: fn now_millis(&self) -> u64;
      - id: FakeClock
        signature: "pub struct FakeClock {"
        docstring: "Hand-written deterministic fake, per the convention above."
      - id: impl_Clock_for_FakeClock
        signature: "impl Clock for FakeClock {"
        children:
        - id: impl_Clock_for_FakeClock.now_millis
          signature: "fn now_millis(&self) -> u64 {"
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: MockBackend
        signature: pub struct MockBackend;
        docstring: "Identity backend: sufficient for orchestration tests that do not care about num…"
      - id: impl_ComputeBackend_for_MockBackend
        signature: "impl ComputeBackend for MockBackend {"
        children:
        - id: impl_ComputeBackend_for_MockBackend.run_batch
          signature: "fn run_batch(&self, input: &[f32]) -> Vec<f32> {"
      - id: Measure
        signature: "pub trait Measure {"
        docstring: Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
        children:
        - id: Measure.sample
          signature: fn sample(&mut self) -> f64;
      - id: NoiseModel
        signature: "pub enum NoiseModel {"
        docstring: Noise models for calibration-stability tests.
      - id: FakeMeasure
        signature: "pub struct FakeMeasure {"
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
        signature: "pub mod sync {"
        docstring: "`std::sync` vs. `loom::sync` shim for CAS-protocol modules: swap the"
      - id: sync_2
        signature: "pub mod sync {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/telemetry.rs
      nodes:
      - id: ModuleDoc
        signature: //! Tracing conventions and init.
      - id: k3B3JK|9oFaU5
        signature: use tracing::Span;
      - id: 6qwbAj|88i4m1
        signature: /// Stable span/event field names (verbatim everywhere so filters and JSON pipe…
      - id: init_tracing
        signature: "pub fn init_tracing() {"
        docstring: "Initializes the global subscriber from `RUST_LOG`, defaulting to `info`."
      - id: init_tracing_non_blocking
        signature: "pub fn init_tracing_non_blocking<W>(writer: W, json: bool) -> WorkerGuard where…"
        docstring: "Builds a non-blocking, optionally JSON-formatted subscriber from `RUST_LOG` (de…"
      - id: task_span
        signature: "pub fn task_span(task_id: &str) -> Span {"
        docstring: "Creates the task span: one per task, carrying [`FIELD_TASK_ID`]. Parent of reso…"
      - id: resource_span
        signature: "pub fn resource_span(task_id: &str, resource_kind: &str) -> Span {"
        docstring: "Creates the resource span: one per leased resource (device/index), nested under…"
      - id: manager_cycle_span
        signature: "pub fn manager_cycle_span(manager_cycle: u64) -> Span {"
        docstring: "Creates the manager-cycle span: one per communication slot / scan cycle, carryi…"
      - id: for_blocking
        signature: "pub fn for_blocking(span: &Span) -> Span {"
        docstring: "Clones `span` for use inside a `spawn_blocking`/plain-thread closure, per the p…"
      - id: tests
        signature: mod tests;
```
/answer cc1b-9 allow
Control Request:
```yaml
id: cc1b-a
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  reason: Moduldoku und Re-Exporte der Wurzel lesen
```
/answer cc1b-a allow
Text:
 Module-Docstrings sind die dichteste Informationsquelle. Ich sammle sie je Ebene.
Control Request:
```yaml
id: cc1b-b
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize
    pattern: ^//!
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    pattern: ^//!
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/index
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
    pattern: ^//!
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  reason: "Modulkopf-Dokumentation (//!) der Ebenen queue, op, tokenize, memory, index, segment lesen"
```
/answer cc1b-b allow
ReasoningToken: 124
Control Request:
```yaml
id: cc1b-c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize
      matches:
      - path: mod.rs
        matches:
        - //! Queue & task primitives.
        - //!
        - "//! Single singly-linked, lock-free, append-only list (CAS on `tail.next`)."
        - "//! Unlink is logical: finished nodes (`Done`/`Cancelled`) stay in the chain and are"
        - "//! skipped by `Queue::iter_live`; no mutex guards the chain, memory is reclaimed when the"
        - "//! queue drops. Order equals age. No version, no snapshot,"
        - //! no lock on the chain itself. Priority/resource/model/mode are metadata in
        - "//! `params`, opaque to the queue."
        - "//! Instantiated multiple times: one task queue, three op queues (CPU/GPU/"
        - //! shared). This module provides the generic `Queue`/`Node` building block
        - //! plus the task-side `ClaimState`/`TaskState` state machine.
        - "//! Operation, requires(), Value::Host|Device."
        - //! One shared CPU tokenizer. Token ids reach the device only as `u32` inputs of the encoder;
        - //! there is deliberately no device-side tokenizer or id-to-address gather.
      - path: queue.rs
        matches:
        - "//! Generic lock-free queue: sentinel head, CAS append, lazy logical unlink."
      - path: node.rs
        matches:
        - "//! Append-only chain node. Public API hides the sentinel used by [`super::Queue`]"
        - //! as a stable head anchor so the head pointer itself never needs a CAS.
      - path: task.rs
        matches:
        - //! Task-queue instantiation notes.
        - //!
        - "//! The task queue is `Queue<P, D, O, E>` where `P`/`D` are the task's"
        - "//! params/payload types and `O`/`E` are its output/error types, all supplied"
        - "//! by the caller (concrete types land in later packages). `TaskState<O, E>`"
        - "//! carries exactly one `Result<O, E>`, set once by the claim winner."
        - //! Op-queues (CPU/GPU/shared) reuse the same `Queue`/`Node` with their own
        - //! `OpParams`/value payload; completion is tracked via the task's op counter
        - "//! (processor/op layer), not through `TaskState::result`. The shared op-queue is created"
        - //! empty and carries no scheduling logic yet; a subengine checks its own
        - //! queue before falling back to shared.
      - path: state.rs
        matches:
        - //! Claim state machine and result cell shared by every queue node.
      - path: requirements.rs
        matches:
        - //! Resource requirements of an operation.
      - path: lease.rs
        matches:
        - //! RAII lease on a device region; pinned regions are not evicted.
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      matches:
      - path: mod.rs
        matches:
        - "//! Region/Layout/DType, the `MemoryManager` contract, and per-class device arenas."
      - path: cpu.rs
        matches:
        - "//! Host-RAM arenas for the CPU path; no streams, no staging."
      - path: layout.rs
        matches:
        - "//! Tensor layout in bytes; views change layout only, never data."
      - path: budget.rs
        matches:
        - //! Arena classes and the VRAM budget.
      - path: wrap_gate.rs
        matches:
        - "//! Prototype gate: can an arena sub-range back a Candle tensor without Candle freeing it?"
      - path: region.rs
        matches:
        - //! Pointer-free memory handle; resolution to a device address is backend-internal.
      - path: arena.rs
        matches:
        - "//! Per-class arena allocator with stream-ordered release, shared by every memory backend."
      - path: cuda.rs
        matches:
        - "//! CUDA device arenas: one backing allocation per device-resident class, pinned staging, and"
        - //! region-addressed copies. Device pointers never leave this module.
      - path: error.rs
        matches:
        - //! Memory-layer errors.
      - path: freelist.rs
        matches:
        - "//! Backend-agnostic offset allocator: first-fit over a coalescing free map."
      - path: dtype.rs
        matches:
        - //! Engine-owned element type; backend dtypes are converted at the edge only.
      - path: staging.rs
        matches:
        - //! Ring of staging slots; a slot is reused only after the event of its last transfer completed.
      - path: manager.rs
        matches:
        - //! `MemoryManager` contract plus a GPU-free bookkeeping implementation.
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/index
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
      matches:
      - path: mod.rs
        matches:
        - "//! Index trait, Scope, AccessMethod, Stamp."
        - //!
        - "//! The contract is independent of the core: an index reports changes and"
        - //! delivers structure and stamps; it knows no geometry and resolves no scope.
        - "//! Write order is binding: data first, then stamp, then signal."
        - //! `read` may block on page faults and disk I/O; callers use a blocking pool.
        - "//! Index segments inside a subengine: slot geometry, immutable segments published as"
        - "//! generation manifests, and deferred release behind leases and stream events."
      - path: parquet_io.rs
        matches:
        - "//! Parquet-backed `Index`: stamps are scanned without decoding vectors, vectors and"
        - "//! payload are decoded on first use, and every write replaces the file atomically."
      - path: stamp.rs
        matches:
        - "//! Stamp generation and the fallback chain for file-backed indexes:"
        - //! stamp -> size+mtime -> content hash.
      - path: payload.rs
        matches:
        - //! Per-entry back-reference to the source text and its integrity checksum.
      - path: schema.rs
        matches:
        - "//! Arrow schema, footer header keys and schema versioning of the persisted index."
      - path: memory.rs
        matches:
        - //! Reference in-memory index; builds without `cuda`/`candle`.
      - path: slots.rs
        matches:
        - "//! Valid bitmap, per-slot metadata and stream-ordered upsert."
      - path: sync.rs
        matches:
        - //! Segment change events and publication ordered behind the copy that fills the segment.
      - path: error.rs
        matches:
        - //! Segment-layer errors.
      - path: mapping.rs
        matches:
        - "//! Reverse mapping from kernel results to entries, and generation-aware resolution."
```
/answer cc1b-c allow
Control Request:
```yaml
id: cc1b-d
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
    pattern: ^//!
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
    pattern: ^//!
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
    pattern: ^//!
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  reason: "Modulkopf-Dokumentation für kernel, manager, precision lesen"
```
/answer cc1b-d allow
Control Request:
```yaml
id: cc1b-e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
      matches:
      - path: mod.rs
        matches:
        - "//! Kernel abstraction: segment descriptors, hits, the `Kernel` trait and the scorers built on it."
        - //!
        - "//! The kernel knows only segments, offsets and spans. Mapping an address back to an entry is the"
        - "//! manager's job (`entry_of`, `entry_of_fixed`); the public API carries no pointers."
      - path: maxsim.rs
        matches:
        - //! Late-interaction scoring (ColBERT-style MaxSim) behind one op; the implementation is
        - "//! swappable (CPU SIMD, Candle tile chain, fused kernel)."
      - path: slots.rs
        matches:
        - //! In-memory `SlotSource` for CPU scoring and tests.
      - path: golden.rs
        matches:
        - //! Golden comparison of a candidate scorer against a reference scorer.
        - //!
        - "//! Exact equality is never required: scores may drift within a tolerance and ranks may swap only"
        - //! where the reference scores are within that tolerance of each other.
      - path: cpu_simd.rs
        matches:
        - //! CPU scoring primitives; loops are shaped for autovectorisation (8-wide accumulators).
      - path: launcher.rs
        matches:
        - //! Launch indirection between scorers and lanes.
        - //!
        - //! `Direct` submits the job to the lane unchanged. `Capture` is reserved for graph replay; the
        - "//! key already carries everything a graph cache needs, so scorers need no change when it lands."
      - path: topk.rs
        matches:
        - "//! Bounded top-k selection with deterministic tie-breaking, plus the CPU merge of slice-local results."
      - path: registry.rs
        matches:
        - "//! Lazily populated kernel table keyed by `(name, dtype, sm, variant)`."
      - path: cosine.rs
        matches:
        - "//! Dense scoring path: cosine similarity as the dot product of L2-normalised vectors."
      - path: cuda.rs
        matches:
        - "//! Real CUDA kernels / scorer: NVRTC-compiled sources behind the kernel registry."
      - path: catalog.rs
        matches:
        - "//! Capability values for which a scoring kernel exists, independent of the `cuda` feature."
      - path: mock.rs
        matches:
        - //! Deterministic hand-written kernel fake for GPU-free tests.
      - path: slice.rs
        matches:
        - "//! Work-slice scheduling: bounded launches sized from observed throughput, with yield points only"
        - //! at slice boundaries.
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      matches:
      - path: mod.rs
        matches:
        - "//! StreamedIndexManager, ScopeResolver, update queue, residency."
        - //!
        - "//! # Consistency model"
        - "//! - The index is the source of truth. VRAM is a derived, discardable mirror. On divergence the"
        - //!   index wins.
        - "//! - Identity is `(EntryId, Stamp)`. Stamps are compared with `!=` only, never with `<`."
        - //! - Reconciliation is state-based. The `watch` signal only wakes the scan
        - "//!   ([`sync::SyncScanner::scan`]). Only resident entries are compared."
        - "//! - Write order in the index is data, then stamp, then signal. Read order on load is stamp,"
        - "//!   data, stamp again ([`sync::load_entry`])."
        - "//! - Segment tables are copy-on-write with a root pointer swap ([`swap`]). A retired version is"
        - //!   freed once all readers have reached the new epoch. Mapping mutations follow the table in
        - "//!   the slot gap ([`slot`])."
        - "//! - Last writer wins. [`apply`](crate::resident::index::Index::apply) is serialized."
      - path: doorbell.rs
        matches:
        - //! Doorbell between persistent-kernel batches and the host mutation gap.
        - //!
        - //! The device publishes the number of finished batches (release); the host polls (acquire) and
        - "//! mutates only between that and its acknowledgement, which the device checks before it starts"
        - "//! the next batch. The host never waits for the kernel itself, only for a batch event."
        - "//! Open: visibility of the mapped cells across PCIe (posted writes, `__threadfence_system`) is"
        - //! not yet verified on the target hardware.
      - path: advice.rs
        matches:
        - //! Page-cache hints for mmap-backed index data. Hints only; the kernel may ignore them.
      - path: upload.rs
        matches:
        - "//! Gather path of the load strategy: scattered entries are packed on the host into one pinned"
        - //! staging slot and leave for the device as a single copy.
      - path: swap.rs
        matches:
        - "//! Mutation discipline of the index manager: copy-on-write segment tables, a single-writer"
        - //! 8-byte root pointer and epoch-gated reclamation. The kernel is never blocked or locked out.
      - path: blocking.rs
        matches:
        - "//! Index reads that may page-fault or hit disk run on the blocking pool, never on an async"
        - "//! worker, after the page cache has been hinted."
      - path: sync.rs
        matches:
        - //! State-based reconciliation between the index and a resident mirror. The change signal only
        - //! wakes the scanner; correctness comes from comparing the stamps of resident entries.
      - path: transfer.rs
        matches:
        - "//! Device-side execution of load plans. Entries are packed on the host, leave as one copy per"
        - //! staging slot and are reported as segments whose slot bindings follow through the update queue.
      - path: arena.rs
        matches:
        - "//! Index-segment use of the shared arena allocator: aligned segments, copy-on-write headroom"
        - //! and epoch-gated release of replaced versions. No allocator of its own.
      - path: residency.rs
        matches:
        - "//! Load strategy: how a resolved entry set reaches the device (whole segments, gather, or"
        - "//! zero-copy), aggregation of concurrent requests, and coalescing of identical loads."
      - path: test/flat.rs
        matches:
        - "//! Index with contiguous backing storage, like a mapped file."
      - path: build.rs
        matches:
        - "//! Index build path: the kernel appends segments to a range granted up front and never allocates."
        - //! Embedding writes and scoring hits travel on separate rings; writes are always registered first.
      - path: tracker.rs
        matches:
        - "//! Mapping between index entries and resident slots, with the stamp known per entry."
      - path: scope.rs
        matches:
        - "//! Scope resolution: the manager turns a task scope into a compact entry set."
      - path: evict.rs
        matches:
        - "//! Eviction for device and host mirrors: idle expiry, watermark hysteresis and cost-aware"
        - //! victim choice. Eviction is explicit; summaries and pinned entries are never victims.
      - path: streamed.rs
        matches:
        - //! Index manager skeleton and its CPU and GPU variants. The index is attached lazily; until then
        - //! no resource is held and no subscription exists.
      - path: seqlock.rs
        matches:
        - "//! Seqlock over word-granular atomics, for reading index data that another writer may modify"
        - //! concurrently (shared mappings). Plain reads of such memory would be a data race.
      - path: runtime.rs
        matches:
        - "//! Owner of one index operation: ties the manager, the table swap, the mapping store and the"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      matches:
      - path: mod.rs
        matches:
        - //! CapabilitySet/Profile.
      - path: probe.rs
        matches:
        - "//! Capability probing: what a subengine's hardware and kernel set can offer."
        - //! Probing is optimistic and measures nothing; unusable combinations are rejected by calibration.
      - path: pool.rs
        matches:
        - //! Dedicated low-priority threads for calibration and CPU kernels; kept apart from the tokio
        - //! runtime so measurements never occupy its workers.
      - path: known.rs
        matches:
        - "//! Well-known axes and values for the target hardware (GTX 1660, sm_75)."
        - "//! bf16 is deliberately absent: it has no native path before sm_80."
      - path: probe_cpu.rs
        matches:
        - //! CPU capability probe with an injectable feature source.
      - path: measure_cpu.rs
        matches:
        - //! Real CPU measurement of the fp32 scoring path. Only the axes this workload reacts to are
        - "//! read from a combination: `batch` (queries per pass), `threads`, and the operand/scoring"
        - "//! type, which must be fp32 because the host kernels exist for nothing else."
      - path: measure_mock.rs
        matches:
        - "//! Scripted fake of the measurement: seeded noise per combination, no device needed."
      - path: measure_cuda.rs
        matches:
        - //! Real CUDA measurement of the scoring path (`cosine_dot`).
        - //!
        - "//! Time comes from CUDA events on the stream, so launch overhead is excluded. Each timing"
        - "//! sample flushes L2 first and covers a window of back-to-back launches, split into groups that"
        - //! stay within the slice budget because a running kernel cannot be preempted. Only the axes
        - "//! this workload reacts to are read from a combination: `batch` (queries per launch) and the"
        - //! document type from `scoring` (falling back to `operands`).
      - path: metrics.rs
        matches:
        - //! Deviation metrics against the fp32 reference and plausibility checks.
      - path: axis.rs
        matches:
        - //! Extensible axis and value identifiers.
      - path: stats.rs
        matches:
        - "//! Robust statistics for timing series. Load only ever slows a run down, so estimates take"
        - //! the median or the minimum and spread ignores the slowest fifth of the samples.
      - path: store.rs
        matches:
        - //! Persistence of calibration results per model x subengine.
        - //!
        - "//! A stored record is compared with the current system by its fingerprint:"
        - "//! equal is `Calibrated`, different is `Stale` (old result stays usable while a"
        - "//! background run replaces it), no file is `Missing`."
      - path: pareto.rs
        matches:
        - "//! Pareto front and arc-length positions over (time, deviation) measurements."
      - path: probe_cuda.rs
        matches:
        - //! CUDA capability probe. The hardware table is pure and testable without a GPU; only reading
        - //! the device requires the `cuda` feature.
      - path: measure.rs
        matches:
        - //! Measurement abstraction between the calibrator and a concrete compute path.
      - path: capability.rs
        matches:
        - "//! Capability sets: axes mapped to the values a provider offers or a task requires."
      - path: profile.rs
        matches:
        - "//! Unnamed profiles: a position on the fast-to-precise scale plus a configuration."
      - path: matching.rs
        matches:
        - //! Maps a task's precision wish onto the nearest active profile of one subengine.
        - //!
        - "//! Scale: `0.0` is fastest, `1.0` is most precise. The scale is relative to the"
        - "//! subengine: `0.5` denotes different accuracy levels on CPU and GPU."
      - path: calibrate.rs
        matches:
        - "//! Calibrator: enumerates combinations of a capability set, measures them interleaved against"
        - "//! the fp32 reference, rejects invalid ones and keeps the Pareto front over (time, deviation)."
        - //!
        - "//! Scale: position 0 is the fastest, 1 the most precise combination. Calibration is best effort"
        - "//! and never blocks: caps end a measurement with the best intermediate state marked unstable."
      - path: workload.rs
        matches:
        - "//! Synthetic scoring workload shared by the real measurements: duplicate-free unit vectors"
        - //! and an fp32 host reference.
```
/answer cc1b-e allow
Control Request:
```yaml
id: cc1b-f
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    pattern: ^//!
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
    pattern: ^//!
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/model
    pattern: ^//!
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  reason: "Modulkopf-Dokumentation für facade, processor, planner, subengine, model lesen"
```
/answer cc1b-f allow
Control Request:
```yaml
id: cc1b-10
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      matches:
      - path: mod.rs
        matches:
        - //! Public resident facade API.
      - path: extensions.rs
        matches:
        - "//! Open, type-keyed parameter extensions carried unchanged through queue and planner."
      - path: report.rs
        matches:
        - //! Raw similarity scores plus the facts an outer layer needs to decide on follow-up tasks.
        - //! The engine decides nothing here; thresholds stay outside.
      - path: test/addon.rs
        matches:
        - //! Reference cascade built only on the public submit API and parameter extensions.
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
      matches:
      - path: mod.rs
        matches:
        - "//! Orchestration: task queue + op queues."
        - //! Stateless task expansion.
      - path: rules.rs
        matches:
        - "//! Rule set: pipeline data plus the explicit expander registry."
      - path: config.rs
        matches:
        - //! Per-model pipeline data; logic stays in the expanders.
      - path: expand.rs
        matches:
        - "//! Expansion rules: one task becomes an ordered operation chain."
      - path: adapter.rs
        matches:
        - "//! Hub adapter: plans a task once and hands the chain over one operation at a time."
      - path: offers.rs
        matches:
        - //! Which resource kinds each engine side can serve.
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/model
      matches:
      - path: mod.rs
        matches:
        - "//! Subengine resource lifecycle, pull capacity reporting, device handles and execution lanes."
        - "//! Model resources: specs, registry, encoder contract and loaded-encoder resources."
      - path: device.rs
        matches:
        - "//! Compute backends, device handle and CUDA execution lanes."
      - path: exec.rs
        matches:
        - "//! Execution contexts: lanes, events, and per-lane parameter and scratch storage."
        - //!
        - //! A lane is an in-order execution queue. On the CPU it is a dedicated worker thread; on CUDA it
        - //! is a prioritised stream. Cross-lane ordering is expressed only through events.
      - path: encoder.rs
        matches:
        - "//! Backbone-agnostic encode pipeline: tokens -> hidden states -> pooling -> slot."
      - path: pooling.rs
        matches:
        - //! Host reference for masked mean pooling and L2 normalisation.
      - path: head.rs
        matches:
        - //! Optional linear projection applied to token embeddings (late-interaction checkpoints).
      - path: candle_adapter.rs
        matches:
        - //! The only place where candle tensors and the BERT implementation are used.
      - path: slot.rs
        matches:
        - "//! Writes encoder output into a slot: L2-normalised and converted to the slot dtype."
      - path: batch.rs
        matches:
        - //! Token-budget batching with length bucketing.
```
/answer cc1b-10 allow
Text:
 Für Dateien ohne Moduldoku (processor, facade, subengine, planner, op, tokenize, manager/slot+flight) brauche ich die Typ-Outlines.
Control Request:
```yaml
id: cc1b-11
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/hub.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/worker.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/ops.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/starved.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/completion.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/completion_cuda.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/config.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/error.rs
  reason: Typen und Docstrings der Processor-Ebene erfassen
```
/answer cc1b-11 allow
ReasoningToken: 410
Control Request:
```yaml
id: cc1b-12
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Orchestration: task queue + op queues."
      - id: completion
        signature: mod completion;
      - id: completion_cuda
        signature: mod completion_cuda;
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
        signature: mod tests;
      - id: flBf5A|OTJLmF
        signature: "pub use completion::{"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/hub.rs
      nodes:
      - id: mo8wep|Hrrsil
        signature: "use std::sync::atomic::{AtomicBool, Ordering};"
      - id: Expansion
        signature: "pub struct Expansion {"
      - id: Expander
        signature: "pub trait Expander: Send + Sync {"
        docstring: Task-to-operation expansion; implemented by the planner.
        children:
        - id: Expander.first_resource
          signature: "fn first_resource(&self, _meta: &TaskMeta) -> Option<ResourceKind> {"
          docstring: "Resource the task's first operation needs, used to serve starved hints first."
        - id: Expander.expand
          signature: "fn expand(     &self,     meta: &TaskMeta,     payload: Payload,     ctx: &Arc<…"
        - id: Expander.expand_hinted
          signature: "fn expand_hinted(     &self,     meta: &TaskMeta,     payload: Payload,     ctx…"
          docstring: "Like [`Expander::expand`], with the starved hint that triggered the expansion."
      - id: Step
        signature: "pub enum Step {"
      - id: Engine
        signature: "pub trait Engine: Send + Sync {"
        docstring: Subengine side of the pull model. Each method does at most one unit of work.
        children:
        - id: Engine.harvest
          signature: "fn harvest(&self, _hub: &Hub) -> bool {"
          docstring: Takes over finished device work and returns results to their tasks.
        - id: Engine.feed
          signature: "fn feed(&self, _hub: &Hub) -> bool {"
          docstring: Synchronises indexes and launches work onto a free lane.
        - id: Engine.step
          signature: "fn step(&self, hub: &Hub) -> Step;"
          docstring: Claims and runs operations.
      - id: Wakeups
        signature: "pub struct Wakeups {"
      - id: impl_Wakeups
        signature: "impl Wakeups {"
        children:
        - id: impl_Wakeups.new
          signature: "pub fn new() -> Self {"
        - id: impl_Wakeups.relays
          signature: "pub fn relays(&self) -> Vec<Arc<Notify>> {"
          docstring: Relays for the task queue so every submit wakes both worker groups.
        - id: impl_Wakeups.side
          signature: "pub fn side(&self, side: Side) -> &Arc<Notify> {"
      - id: impl_Default_for_Wakeups
        signature: "impl Default for Wakeups {"
        children:
        - id: impl_Default_for_Wakeups.default
          signature: "fn default() -> Self {"
      - id: RegionCheck
        signature: pub type RegionCheck = Arc<dyn Fn(Region) -> bool + Send + Sync>;
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
          signature: "pub fn set_region_check(&self, check: RegionCheck) {"
          docstring: Enables the debug-build staleness assertion for device handles at claim time.
        - id: impl_Hub.is_shutdown
          signature: "pub(super) fn is_shutdown(&self) -> bool {"
        - id: impl_Hub.begin_shutdown
          signature: "pub(super) fn begin_shutdown(&self) {"
        - id: impl_Hub.claim
          signature: "pub fn claim(     &self,     side: Side,     accepts: &dyn Fn(&OpParams) -> boo…"
          docstring: A pick of a resource kind satisfies that kind's starved hint.
        - id: impl_Hub.complete_op
          signature: "pub fn complete_op(&self, job: OpJob, followups: Vec<Arc<OperationNode>>) {"
          docstring: Closes `job` successfully after registering and enqueueing its follow-up operat…
        - id: impl_Hub.fail_op
          signature: "pub fn fail_op(&self, job: OpJob, err: ResidentError) {"
        - id: impl_Hub.enqueue
          signature: "fn enqueue(     &self,     run: &Arc<TaskRun>,     ops: Vec<Arc<OperationNode>>…"
          docstring: "Counts first, then enqueues, so the task's open-op counter never dips to zero e…"
        - id: impl_Hub.expand_if_needed
          signature: "pub(super) fn expand_if_needed(&self, side: Side) -> bool {"
          docstring: Expands one queued task when hinted or when the op buffer ran below its waterli…
        - id: impl_Hub.expand_one
          signature: "fn expand_one(&self, hint: StarvedSnapshot) -> bool {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/worker.rs
      nodes:
      - id: H4BeJh|AF0WTW
        signature: "use std::panic::{catch_unwind, AssertUnwindSafe};"
      - id: Processor
        signature: "pub struct Processor {"
      - id: impl_Processor
        signature: "impl Processor {"
        children:
        - id: impl_Processor.start
          signature: "pub fn start(     cfg: WorkerConfig,     tasks: Arc<TaskQueue>,     wakeups: Wa…"
          docstring: '`wakeups.relays()` must be the relays of `tasks` so submissions wake the worker…'
        - id: impl_Processor.hub
          signature: "pub fn hub(&self) -> &Arc<Hub> {"
        - id: impl_Processor.shutdown
          signature: "pub fn shutdown(&mut self) {"
      - id: impl_Drop_for_Processor
        signature: "impl Drop for Processor {"
        children:
        - id: impl_Drop_for_Processor.drop
          signature: "fn drop(&mut self) {"
      - id: spawn
        signature: "fn spawn(     hub: &Arc<Hub>,     side: Side,     index: usize,     engine: Arc…"
      - id: run
        signature: "fn run(hub: &Hub, side: Side, engine: &dyn Engine) {"
      - id: pass
        signature: "pub(super) fn pass(hub: &Hub, side: Side, engine: &dyn Engine) -> bool {"
        docstring: "One prioritised pass: device results, device feeding, host work, then expansion."
      - id: lower_priority
        signature: "fn lower_priority(nice: i32) {"
      - id: lower_priority_2
        signature: "fn lower_priority(_nice: i32) {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/ops.rs
      nodes:
      - id: fb1s3C|M4aGfc
        signature: "use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};"
      - id: Collect
        signature: "pub type Collect = Box<dyn FnOnce() -> Result<Output, TaskError> + Send>;"
      - id: TaskRun
        signature: "pub struct TaskRun {"
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
          signature: "pub fn expire_if_due(&self) -> bool {"
          docstring: Fails the task with a timeout once its deadline has passed; returns whether it …
        - id: impl_TaskRun.complete_once
          signature: "fn complete_once(&self, result: impl FnOnce() -> Result<Output, TaskError>) {"
        - id: impl_TaskRun.finish
          signature: "pub fn finish(&self) {"
          docstring: Completes the task with its collected result.
        - id: impl_TaskRun.op_done
          signature: "pub fn op_done(&self) {"
          docstring: Closes one operation; the last one of a healthy task completes it.
        - id: impl_TaskRun.skip
          signature: "pub fn skip(&self) {"
          docstring: Closes an operation that is dropped unexecuted because the task already failed.
        - id: impl_TaskRun.fail
          signature: "pub fn fail(&self, err: ResidentError) {"
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
        signature: "pub enum Side {"
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
          signature: "fn claim(     &self,     accepts: &dyn Fn(&OpParams) -> bool,     max_skip: usi…"
          docstring: "Claims the oldest acceptable job, skipping at most `max_skip` foreign entries."
      - id: OpQueues
        signature: "pub struct OpQueues {"
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
          signature: "pub fn claim(     &self,     side: Side,     accepts: &dyn Fn(&OpParams) -> boo…"
          docstring: "Own queue first, then shared."
        - id: impl_OpQueues.queued
          signature: "pub fn queued(&self, side: Side) -> usize {"
        - id: impl_OpQueues.total_queued
          signature: "pub fn total_queued(&self) -> usize {"
      - id: testutil
        signature: "pub(crate) mod testutil {"
        children:
        - id: testutil.dCtZYA|LDZ8RQ
          signature: use super::*;
        - id: testutil.node
          signature: "pub fn node(     task: &Arc<TaskCtx>,     resource: ResourceKind,     engine: E…"
        - id: testutil.run
          signature: "pub fn run(id: u64) -> (Arc<TaskRun>, Arc<TaskState<Output, TaskError>>) {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
      nodes:
      - id: I6MAMl|9uLxLO
        signature: "use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering};"
      - id: ResourceState
        signature: "pub enum ResourceState {"
      - id: impl_ResourceState
        signature: "impl ResourceState {"
        children:
        - id: impl_ResourceState.code
          signature: "fn code(self) -> u8 {"
        - id: impl_ResourceState.from_code
          signature: "fn from_code(v: u8) -> Self {"
      - id: ResourceSlot
        signature: "pub struct ResourceSlot {"
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
          signature: "pub fn begin_load(&self, now_ms: u64) -> bool {"
          docstring: Only the caller that wins this transition performs the load.
        - id: impl_ResourceSlot.finish_load
          signature: "pub fn finish_load(&self, ok: bool, now_ms: u64, backoff_ms: u64) {"
        - id: impl_ResourceSlot.enter
          signature: "pub fn enter(&self) -> bool {"
          docstring: Registers an operation in flight; fails unless the resource is `Ready`.
        - id: impl_ResourceSlot.leave
          signature: "pub fn leave(&self, now_ms: u64) {"
        - id: impl_ResourceSlot.in_flight
          signature: "pub fn in_flight(&self) -> usize {"
        - id: impl_ResourceSlot.try_evict
          signature: "pub fn try_evict(&self, now_ms: u64, ttl_ms: u64) -> bool {"
          docstring: "`Ready -> Evicting` only when idle, unleased and past the TTL."
        - id: impl_ResourceSlot.finish_evict
          signature: "pub fn finish_evict(&self) {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/starved.rs
      nodes:
      - id: PaFWtS|puswEo
        signature: "use std::sync::atomic::{AtomicU64, Ordering};"
      - id: XOiMUZ|1rkWwz
        signature: "const KINDS: usize = 6;"
      - id: slot
        signature: "fn slot(kind: ResourceKind) -> usize {"
      - id: StarvedSnapshot
        signature: pub struct StarvedSnapshot(u32);
      - id: impl_StarvedSnapshot
        signature: "impl StarvedSnapshot {"
        children:
        - id: impl_StarvedSnapshot.contains
          signature: "pub fn contains(self, kind: ResourceKind) -> bool {"
        - id: impl_StarvedSnapshot.is_empty
          signature: "pub fn is_empty(self) -> bool {"
      - id: StarvedSet
        signature: "pub struct StarvedSet {"
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
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/completion.rs
      nodes:
      - id: nsWaJn|jkdY1h
        signature: "use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};"
      - id: LaneId
        signature: pub struct LaneId(pub u32);
      - id: LaneOutcome
        signature: "pub enum LaneOutcome {"
      - id: PollState
        signature: "pub enum PollState {"
      - id: Completion
        signature: "pub trait Completion: Send + Sync {"
        docstring: Backend-specific completion detection. The push path (host callback) calls
        children:
        - id: Completion.arm
          signature: "fn arm(&self, lane: LaneId);"
          docstring: Called after the lane's work has been enqueued on its stream.
        - id: Completion.poll
          signature: "fn poll(&self, lane: LaneId) -> PollState;"
      - id: hxhXVc|yFNBJX
        signature: "const IDLE: u8 = 0;"
      - id: LaneBoard
        signature: "pub struct LaneBoard {"
        docstring: Lane lifecycle `Idle -> Reserved -> Launched -> Completing -> Idle`. Callback a…
      - id: impl_LaneBoard
        signature: "impl LaneBoard {"
        children:
        - id: impl_LaneBoard.new
          signature: "pub fn new(lanes: usize, wake: Arc<Notify>) -> Self {"
        - id: impl_LaneBoard.lanes
          signature: "pub fn lanes(&self) -> usize {"
        - id: impl_LaneBoard.reserve
          signature: "pub fn reserve(&self) -> Option<LaneId> {"
        - id: impl_LaneBoard.abort
          signature: "pub fn abort(&self, lane: LaneId) {"
          docstring: Returns a reserved lane that was never launched.
        - id: impl_LaneBoard.launch
          signature: "pub fn launch(&self, lane: LaneId, completion: &dyn Completion) {"
        - id: impl_LaneBoard.signal
          signature: "pub fn signal(&self, lane: LaneId) {"
          docstring: "Callback entry point: no CUDA calls, no blocking."
        - id: impl_LaneBoard.is_idle
          signature: "pub fn is_idle(&self, lane: LaneId) -> bool {"
        - id: impl_LaneBoard.in_flight
          signature: "pub fn in_flight(&self) -> usize {"
        - id: impl_LaneBoard.reap
          signature: "pub fn reap(     &self,     completion: &dyn Completion,     mut on_done: impl …"
          docstring: Takes over every finished lane exactly once and frees it after `on_done` return…
      - id: ManualCompletion
        signature: "pub struct ManualCompletion {"
        docstring: Test double whose lanes finish only when told to.
      - id: impl_ManualCompletion
        signature: "impl ManualCompletion {"
        children:
        - id: impl_ManualCompletion.new
          signature: "pub fn new(lanes: usize) -> Self {"
        - id: impl_ManualCompletion.finish
          signature: "pub fn finish(&self, lane: LaneId, outcome: LaneOutcome) {"
      - id: impl_Completion_for_ManualCompletion
        signature: "impl Completion for ManualCompletion {"
        children:
        - id: impl_Completion_for_ManualCompletion.arm
          signature: "fn arm(&self, lane: LaneId) {"
        - id: impl_Completion_for_ManualCompletion.poll
          signature: "fn poll(&self, lane: LaneId) -> PollState {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/completion_cuda.rs
      nodes:
      - id: J1dzam|yNoX35
        signature: use std::ffi::c_void;
      - id: PolledCompletion
        signature: "pub struct PolledCompletion {"
        docstring: Detects completion by querying a timing-free event recorded behind the lane's w…
      - id: impl_PolledCompletion
        signature: "impl PolledCompletion {"
        children:
        - id: impl_PolledCompletion.new
          signature: "pub fn new(stream: Arc<CudaStream>, lanes: usize) -> Self {"
      - id: impl_Completion_for_PolledCompletion
        signature: "impl Completion for PolledCompletion {"
        children:
        - id: impl_Completion_for_PolledCompletion.arm
          signature: "fn arm(&self, lane: LaneId) {"
        - id: impl_Completion_for_PolledCompletion.poll
          signature: "fn poll(&self, lane: LaneId) -> PollState {"
      - id: Callback
        signature: "struct Callback {"
      - id: trampoline
        signature: "unsafe extern \"C\" fn trampoline(arg: *mut c_void) {"
      - id: HostFnCompletion
        signature: "pub struct HostFnCompletion {"
        docstring: "Push path: a host function at the end of the stream signals the lane board. The…"
      - id: impl_HostFnCompletion
        signature: "impl HostFnCompletion {"
        children:
        - id: impl_HostFnCompletion.new
          signature: "pub fn new(stream: Arc<CudaStream>, board: Arc<LaneBoard>) -> Self {"
      - id: impl_Completion_for_HostFnCompletion
        signature: "impl Completion for HostFnCompletion {"
        children:
        - id: impl_Completion_for_HostFnCompletion.arm
          signature: "fn arm(&self, lane: LaneId) {"
        - id: impl_Completion_for_HostFnCompletion.poll
          signature: "fn poll(&self, lane: LaneId) -> PollState {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/config.rs
      nodes:
      - id: CC3VUs|YgKIjp
        signature: use std::time::Duration;
      - id: WorkerConfig
        signature: "pub struct WorkerConfig {"
      - id: impl_WorkerConfig
        signature: "impl WorkerConfig {"
        children:
        - id: impl_WorkerConfig.new
          signature: "pub fn new(cpu_workers: usize, gpu_workers: usize) -> Self {"
        - id: impl_WorkerConfig.with_min_op_buffer
          signature: "pub fn with_min_op_buffer(mut self, n: usize) -> Self {"
        - id: impl_WorkerConfig.with_candle_rayon_threads
          signature: "pub fn with_candle_rayon_threads(mut self, n: usize) -> Self {"
        - id: impl_WorkerConfig.with_cpu_nice
          signature: "pub fn with_cpu_nice(mut self, nice: i32) -> Self {"
        - id: impl_WorkerConfig.with_starved_ttl
          signature: "pub fn with_starved_ttl(mut self, ttl: Duration) -> Self {"
        - id: impl_WorkerConfig.with_task_timeout
          signature: "pub fn with_task_timeout(mut self, timeout: Duration) -> Self {"
      - id: impl_Default_for_WorkerConfig
        signature: "impl Default for WorkerConfig {"
        children:
        - id: impl_Default_for_WorkerConfig.default
          signature: "fn default() -> Self {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/error.rs
      nodes:
      - id: eRggY0|54nLj5
        signature: use std::sync::Arc;
      - id: OrchestrationError
        signature: "pub enum OrchestrationError {"
      - id: impl_IsRetryable_for_OrchestrationError
        signature: "impl IsRetryable for OrchestrationError {"
        children:
        - id: impl_IsRetryable_for_OrchestrationError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: tests
        signature: mod tests;
```
/answer cc1b-12 allow
Control Request:
```yaml
id: cc1b-13
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/handle.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/blocking.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/report.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/extensions.rs
  reason: Typen und Docstrings der Fassadenebene erfassen
```
/answer cc1b-13 allow
Control Request:
```yaml
id: cc1b-14
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Public resident facade API.
      - id: blocking
        signature: mod blocking;
      - id: error
        signature: mod error;
      - id: extensions
        signature: mod extensions;
      - id: handle
        signature: mod handle;
      - id: params
        signature: mod params;
      - id: report
        signature: mod report;
      - id: resident
        signature: mod resident;
      - id: task
        signature: mod task;
      - id: addon_tests
        signature: mod addon_tests;
      - id: h0TGLs|GjXY1L
        signature: pub use blocking::ResidentBlocking;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
      nodes:
      - id: RY1QMi|Bs72Bv
        signature: use std::collections::HashMap;
      - id: ResidentConfig
        signature: "pub struct ResidentConfig {"
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
        signature: struct Unplanned;
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
        signature: "pub struct ResidentBuilder {"
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
          signature: "pub fn engines(     mut self,     cpu: Arc<dyn Engine>,     gpu: Option<Arc<dyn…"
          docstring: GPU workers are only started when a GPU engine is supplied.
        - id: impl_ResidentBuilder.init
          signature: "pub fn init(self) -> Result<Resident, ConfigError> {"
          docstring: Starts the processor and its worker threads; model and device resources are
      - id: Residency
        signature: "pub enum Residency {"
      - id: IndexInfo
        signature: "pub struct IndexInfo {"
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
          signature: "pub fn add_index(&self, index: Arc<dyn Index + Send + Sync>) -> IndexId {"
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
          signature: "pub fn shutdown(self) -> Result<(), OrchestrationError> {"
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
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
      nodes:
      - id: jAY2Rz|M8Kowd
        signature: use super::error::TaskError;
      - id: TaskKind
        signature: "pub enum TaskKind {"
      - id: TaskMeta
        signature: "pub struct TaskMeta {"
      - id: Payload
        signature: "pub enum Payload {"
      - id: TaskQueue
        signature: "pub type TaskQueue = Queue<TaskMeta, Payload, Output, TaskError>;"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/handle.rs
      nodes:
      - id: mnkbih|naCgoF
        signature: "use std::future::{Future, IntoFuture};"
      - id: State
        signature: "pub(crate) type State = TaskState<Output, TaskError>;"
      - id: TaskHandle
        signature: "pub struct TaskHandle {"
        docstring: Dropping an unfinished handle cancels the task if no worker has claimed it yet.
      - id: impl_TaskHandle
        signature: "impl TaskHandle {"
        children:
        - id: impl_TaskHandle.new
          signature: "pub(crate) fn new(state: Arc<State>, id: u64) -> Self {"
        - id: impl_TaskHandle.id
          signature: "pub fn id(&self) -> u64 {"
        - id: impl_TaskHandle.try_get
          signature: "pub fn try_get(&self) -> Option<Result<Output, TaskError>> {"
        - id: impl_TaskHandle.wait_timeout
          signature: "pub async fn wait_timeout(self, limit: Duration) -> Result<Output, TaskError> {"
          docstring: "On timeout the handle is dropped, which cancels the task if still queued."
        - id: impl_TaskHandle.resolve
          signature: "async fn resolve(self) -> Result<Output, TaskError> {"
      - id: impl_IntoFuture_for_TaskHandle
        signature: "impl IntoFuture for TaskHandle {"
        children:
        - id: impl_IntoFuture_for_TaskHandle.6dxsRi|fcDSqY
          signature: "type Output = Result<Output, TaskError>;"
        - id: impl_IntoFuture_for_TaskHandle.into_future
          signature: "fn into_future(self) -> Self::IntoFuture {"
      - id: impl_Drop_for_TaskHandle
        signature: "impl Drop for TaskHandle {"
        children:
        - id: impl_Drop_for_TaskHandle.drop
          signature: "fn drop(&mut self) {"
      - id: impl_std__fmt__Debug_for_TaskHandle
        signature: "impl std::fmt::Debug for TaskHandle {"
        children:
        - id: impl_std__fmt__Debug_for_TaskHandle.fmt
          signature: "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
      nodes:
      - id: an6syr|7wsnLF
        signature: "use crate::resident::index::{EntryId, Scope};"
      - id: jTwlTq|iUpnXF
        signature: "const DEFAULT_PRECISION: f32 = 1.0;"
      - id: TextRole
        signature: "pub enum TextRole {"
      - id: ScorerKind
        signature: "pub enum ScorerKind {"
        docstring: "Scoring method of a similarity task. Rerank is not a task kind of its own: it i…"
      - id: impl_ScorerKind
        signature: "impl ScorerKind {"
        children:
        - id: impl_ScorerKind.label
          signature: "pub fn label(&self) -> &'static str {"
      - id: TextInput
        signature: "pub struct TextInput {"
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
        signature: "pub struct VectorInput {"
      - id: impl_VectorInput
        signature: "impl VectorInput {"
        children:
        - id: impl_VectorInput.new
          signature: "pub fn new(vectors: Vec<Vec<f32>>) -> Self {"
        - id: impl_VectorInput.vectors
          signature: "pub fn vectors(&self) -> &[Vec<f32>] {"
      - id: PairInput
        signature: "pub struct PairInput {"
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
        signature: "pub struct EmbeddingParams {"
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
        - id: impl_EmbeddingParams.with_ext
          signature: "pub fn with_ext<T: Any + Send + Sync>(mut self, value: T) -> Self {"
        - id: impl_EmbeddingParams.ext
          signature: "pub fn ext(&self) -> &Extensions {"
      - id: impl_Default_for_EmbeddingParams
        signature: "impl Default for EmbeddingParams {"
        children:
        - id: impl_Default_for_EmbeddingParams.default
          signature: "fn default() -> Self {"
      - id: SimilarityParams
        signature: "pub struct SimilarityParams {"
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
        - id: impl_SimilarityParams.scorer
          signature: "pub fn scorer(mut self, scorer: ScorerKind) -> Self {"
        - id: impl_SimilarityParams.index
          signature: "pub fn index(&self) -> &IndexId {"
        - id: impl_SimilarityParams.get_top_k
          signature: "pub fn get_top_k(&self) -> usize {"
        - id: impl_SimilarityParams.get_scope
          signature: "pub fn get_scope(&self) -> &Scope {"
        - id: impl_SimilarityParams.get_scorer
          signature: "pub fn get_scorer(&self) -> &ScorerKind {"
        - id: impl_SimilarityParams.get_precision
          signature: "pub fn get_precision(&self) -> f32 {"
        - id: impl_SimilarityParams.with_ext
          signature: "pub fn with_ext<T: Any + Send + Sync>(mut self, value: T) -> Self {"
        - id: impl_SimilarityParams.ext
          signature: "pub fn ext(&self) -> &Extensions {"
      - id: InferenceParams
        signature: "pub struct InferenceParams {"
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
        - id: impl_InferenceParams.with_ext
          signature: "pub fn with_ext<T: Any + Send + Sync>(mut self, value: T) -> Self {"
        - id: impl_InferenceParams.ext
          signature: "pub fn ext(&self) -> &Extensions {"
      - id: impl_Default_for_InferenceParams
        signature: "impl Default for InferenceParams {"
        children:
        - id: impl_Default_for_InferenceParams.default
          signature: "fn default() -> Self {"
      - id: Output
        signature: "pub enum Output {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/blocking.rs
      nodes:
      - id: HjDKXC|1fAV7I
        signature: use std::future::IntoFuture;
      - id: ResidentBlocking
        signature: "pub struct ResidentBlocking {"
        docstring: Owns a private runtime; must not be used from inside another tokio runtime.
      - id: impl_ResidentBlocking
        signature: "impl ResidentBlocking {"
        children:
        - id: impl_ResidentBlocking.new
          signature: "pub fn new(inner: Resident) -> io::Result<Self> {"
        - id: impl_ResidentBlocking.resident
          signature: "pub fn resident(&self) -> &Resident {"
        - id: impl_ResidentBlocking.embedding
          signature: "pub fn embedding(     &self,     input: TextInput,     params: EmbeddingParams,…"
        - id: impl_ResidentBlocking.similarity
          signature: "pub fn similarity(     &self,     input: VectorInput,     params: SimilarityPar…"
        - id: impl_ResidentBlocking.inference
          signature: "pub fn inference(     &self,     input: PairInput,     params: InferenceParams,…"
        - id: impl_ResidentBlocking.wait
          signature: "fn wait(&self, handle: TaskHandle) -> Result<Output, TaskError> {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/error.rs
      nodes:
      - id: K8Urgv|1aS1gX
        signature: use std::sync::Arc;
      - id: TaskError
        signature: "pub enum TaskError {"
        docstring: Cloneable so one outcome can be delivered to several waiters.
      - id: impl_From_ResidentError__for_TaskError
        signature: "impl From<ResidentError> for TaskError {"
        children:
        - id: impl_From_ResidentError__for_TaskError.from
          signature: "fn from(err: ResidentError) -> Self {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/report.rs
      nodes:
      - id: ModuleDoc
        signature: //! Raw similarity scores plus the facts an outer layer needs to decide on foll…
      - id: jqiuOA|egf3CK
        signature: use super::params::SimilarityParams;
      - id: RankedHit
        signature: "pub struct RankedHit {"
      - id: SimilarityReport
        signature: "pub struct SimilarityReport {"
      - id: impl_SimilarityReport
        signature: "impl SimilarityReport {"
        children:
        - id: impl_SimilarityReport.build
          signature: "pub fn build(     task_id: u64,     params: &SimilarityParams,     candidates: …"
          docstring: '`hits` may be unordered and longer than `top_k`; NaN scores count as discarded.'
        - id: impl_SimilarityReport.count
          signature: "pub fn count(&self) -> usize {"
        - id: impl_SimilarityReport.s1
          signature: "pub fn s1(&self) -> Option<f32> {"
        - id: impl_SimilarityReport.sk
          signature: "pub fn sk(&self) -> Option<f32> {"
        - id: impl_SimilarityReport.mean
          signature: "pub fn mean(&self) -> Option<f32> {"
          docstring: Mean over the returned scores.
        - id: impl_SimilarityReport.std
          signature: "pub fn std(&self) -> Option<f32> {"
          docstring: Population standard deviation over the returned scores.
        - id: impl_SimilarityReport.gap_top2
          signature: "pub fn gap_top2(&self) -> Option<f32> {"
          docstring: '`s1 - s2`; a small gap means the order of the leaders is unreliable.'
        - id: impl_SimilarityReport.gap_to_k
          signature: "pub fn gap_to_k(&self) -> Option<f32> {"
          docstring: "`s1 - sk`: spread of the whole result list."
        - id: impl_SimilarityReport.z_top1
          signature: "pub fn z_top1(&self) -> Option<f32> {"
          docstring: Distance of `s1` from the mean in standard deviations; `None` for a flat distri…
        - id: impl_SimilarityReport.softmax
          signature: "pub fn softmax(&self, temperature: f32) -> Option<Vec<f32>> {"
          docstring: "Softmax over the returned scores. e5 scores live on a narrow band, so a tempera…"
        - id: impl_SimilarityReport.top1_mass
          signature: "pub fn top1_mass(&self, temperature: f32) -> Option<f32> {"
          docstring: Softmax probability mass of the best hit.
        - id: impl_SimilarityReport.entropy
          signature: "pub fn entropy(&self, temperature: f32) -> Option<f32> {"
          docstring: Shannon entropy (nats) of the softmax distribution; 0 means one clear winner.
        - id: impl_SimilarityReport.elbow
          signature: "pub fn elbow(&self) -> Option<usize> {"
          docstring: "Index of the knee of the sorted score curve: the point farthest below the chord…"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/extensions.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Open, type-keyed parameter extensions carried unchanged through queue and p…"
      - id: 0tsMET|wNULgX
        signature: "use std::any::{Any, TypeId};"
      - id: Extensions
        signature: "pub struct Extensions {"
        docstring: "One value per type. Values are shared on clone, so derived tasks copy parameter…"
      - id: impl_Extensions
        signature: "impl Extensions {"
        children:
        - id: impl_Extensions.new
          signature: "pub fn new() -> Self {"
        - id: impl_Extensions.insert
          signature: "pub fn insert<T: Any + Send + Sync>(&mut self, value: T) -> &mut Self {"
          docstring: Replaces an existing value of the same type.
        - id: impl_Extensions.get
          signature: "pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {"
        - id: impl_Extensions.remove
          signature: "pub fn remove<T: Any + Send + Sync>(&mut self) -> bool {"
        - id: impl_Extensions.len
          signature: "pub fn len(&self) -> usize {"
        - id: impl_Extensions.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
      - id: impl_fmt__Debug_for_Extensions
        signature: "impl fmt::Debug for Extensions {"
        children:
        - id: impl_fmt__Debug_for_Extensions.fmt
          signature: "fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {"
      - id: tests
        signature: mod tests;
```
/answer cc1b-14 allow
Control Request:
```yaml
id: cc1b-15
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/lifecycle.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/resource.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
  reason: Typen und Docstrings von subengine und op erfassen
```
/answer cc1b-15 allow
Control Request:
```yaml
id: cc1b-16
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Subengine resource lifecycle, pull capacity reporting, device handles and e…"
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: CpuBackend
        signature: pub struct CpuBackend;
        docstring: "CPU backend; default without `cuda`, and the fallback target if CUDA init fails."
      - id: impl_ComputeBackend_for_CpuBackend
        signature: "impl ComputeBackend for CpuBackend {"
        children:
        - id: impl_ComputeBackend_for_CpuBackend.run_batch
          signature: "fn run_batch(&self, input: &[f32]) -> Vec<f32> {"
      - id: device
        signature: mod device;
      - id: exec
        signature: mod exec;
      - id: error
        signature: mod error;
      - id: lifecycle
        signature: mod lifecycle;
      - id: resource
        signature: mod resource;
      - id: wLnUuf|YSWcPL
        signature: pub use device::*;
      - id: SubengineKind
        signature: "pub enum SubengineKind {"
      - id: Subengine
        signature: "pub trait Subengine: Send + Sync {"
        docstring: Resources and execution capacity of one compute substrate. Work arrives through…
        children:
        - id: Subengine.kind
          signature: fn kind(&self) -> SubengineKind;
        - id: Subengine.device
          signature: fn device(&self) -> &DeviceHandle;
        - id: Subengine.capacity
          signature: fn capacity(&self) -> Capacity;
        - id: Subengine.throughput
          signature: fn throughput(&self) -> f32;
          docstring: Smoothed items per second.
        - id: Subengine.batch_size
          signature: fn batch_size(&self) -> usize;
          docstring: "Throughput-derived batch size, independent of current load."
        - id: Subengine.pull_budget
          signature: "fn pull_budget(&self) -> usize {"
          docstring: Upper bound on the jobs to hand this subengine in one `Claim::claim` call.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Compute backends, device handle and CUDA execution lanes."
      - id: 77cuty|ChE8yG
        signature: use crate::resident::backend::ComputeBackend;
      - id: CudaBackend
        signature: pub struct CudaBackend;
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
      - id: BbkRhK|SOdFt8
        signature: "use std::time::{Duration, Instant};"
      - id: DeviceHandle
        signature: "pub struct DeviceHandle {"
        docstring: Identifies the compute device a subengine runs on; lanes are opened from it.
      - id: impl_std__fmt__Debug_for_DeviceHandle
        signature: "impl std::fmt::Debug for DeviceHandle {"
        children:
        - id: impl_std__fmt__Debug_for_DeviceHandle.fmt
          signature: "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {"
      - id: impl_DeviceHandle
        signature: "impl DeviceHandle {"
        children:
        - id: impl_DeviceHandle.cpu
          signature: "pub fn cpu() -> Self {"
        - id: impl_DeviceHandle.cuda
          signature: "pub fn cuda(ordinal: usize) -> Result<Self, DeviceError> {"
          docstring: "Retains the primary context of `ordinal`. The first call is expensive (0.2-2 s,…"
        - id: impl_DeviceHandle.cuda_2
          signature: "pub fn cuda(_ordinal: usize) -> Result<Self, DeviceError> {"
        - id: impl_DeviceHandle.kind
          signature: "pub fn kind(&self) -> DeviceKind {"
        - id: impl_DeviceHandle.cuda_context
          signature: "pub(crate) fn cuda_context(     &self, ) -> Option<&std::sync::Arc<cudarc::driv…"
        - id: impl_DeviceHandle.ordinal
          signature: "pub fn ordinal(&self) -> usize {"
        - id: impl_DeviceHandle.init_cost
          signature: "pub fn init_cost(&self) -> Duration {"
          docstring: "Time spent creating the context, reported to the lazy-init accounting."
        - id: impl_DeviceHandle.warmup
          signature: "pub fn warmup(&self) -> Result<Duration, DeviceError> {"
          docstring: "Forces driver-side lazy work (first allocation, stream creation) so it does not…"
        - id: impl_DeviceHandle.open_lane
          signature: "pub fn open_lane(     &self,     class: LaneClass,     cfg: LaneConfig, ) -> Re…"
      - id: Ke5I2l|scA2j5
        signature: "#[cfg(feature = \"cuda\")]"
      - id: cuda_lane
        signature: "mod cuda_lane {"
        children:
        - id: cuda_lane.CHYK8i|Km1i1Y
          signature: use std::sync::Arc;
        - id: cuda_lane.CudaLane
          signature: "pub struct CudaLane {"
          docstring: Prioritised non-blocking stream; all device work of one lane is ordered on it.
        - id: cuda_lane.event_err
          signature: "fn event_err(e: cudarc::driver::DriverError) -> DeviceError {"
        - id: cuda_lane.impl_CudaLane
          signature: "impl CudaLane {"
          children:
          - id: cuda_lane.impl_CudaLane.new
            signature: "pub(crate) fn new(     ctx: &Arc<CudaContext>,     class: LaneClass,     cfg: L…"
          - id: cuda_lane.impl_CudaLane.priority
            signature: "pub fn priority(&self) -> i32 {"
          - id: cuda_lane.impl_CudaLane.stream
            signature: "pub(crate) fn stream(&self) -> &Arc<CudaStream> {"
        - id: cuda_lane.impl_ExecContext_for_CudaLane
          signature: "impl ExecContext for CudaLane {"
          children:
          - id: cuda_lane.impl_ExecContext_for_CudaLane.class
            signature: "fn class(&self) -> LaneClass {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.device
            signature: "fn device(&self) -> DeviceKind {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.submit
            signature: "fn submit(&self, job: LaneJob) -> Result<(), DeviceError> {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.record_event
            signature: "fn record_event(&self) -> Result<EventHandle, DeviceError> {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.wait
            signature: "fn wait(&self, event: &EventHandle) -> Result<(), DeviceError> {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.synchronize
            signature: "fn synchronize(&self) -> Result<(), DeviceError> {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.param_block
            signature: "fn param_block(&mut self) -> &mut ParamBlock {"
          - id: cuda_lane.impl_ExecContext_for_CudaLane.scratch
            signature: "fn scratch(&self) -> &ScratchArena {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Execution contexts: lanes, events, and per-lane parameter and scratch stora…"
      - id: 8KLzlk|GzcaOo
        signature: "use std::panic::{catch_unwind, AssertUnwindSafe};"
      - id: LaneJob
        signature: "pub type LaneJob = Box<dyn FnOnce() -> Result<(), DeviceError> + Send + 'static…"
        docstring: Work item executed on a lane. Device work is enqueued asynchronously from withi…
      - id: LaneClass
        signature: "pub enum LaneClass {"
      - id: impl_LaneClass
        signature: "impl LaneClass {"
        children:
        - id: impl_LaneClass.priority
          signature: "pub fn priority(self) -> i32 {"
          docstring: CUDA stream priority; lower values are more urgent and `0` is the default. Prio…
      - id: LaneConfig
        signature: "pub struct LaneConfig {"
      - id: impl_LaneConfig
        signature: "impl LaneConfig {"
        children:
        - id: impl_LaneConfig.Cj9Odg|qdIpgQ
          signature: "pub const DEFAULT_PARAM_CAPACITY: usize = 4096;"
        - id: impl_LaneConfig.new
          signature: "pub fn new(scratch: Region) -> Self {"
      - id: CpuEvent
        signature: "pub(crate) struct CpuEvent {"
      - id: impl_CpuEvent
        signature: "impl CpuEvent {"
        children:
        - id: impl_CpuEvent.signal
          signature: "fn signal(&self) {"
        - id: impl_CpuEvent.wait
          signature: "fn wait(&self) {"
        - id: impl_CpuEvent.is_done
          signature: "fn is_done(&self) -> bool {"
      - id: EventRepr
        signature: "pub(crate) enum EventRepr {"
      - id: EventHandle
        signature: "pub struct EventHandle {"
        docstring: Opaque completion marker for all work submitted to a lane before it was recorde…
      - id: impl_std__fmt__Debug_for_EventHandle
        signature: "impl std::fmt::Debug for EventHandle {"
        children:
        - id: impl_std__fmt__Debug_for_EventHandle.fmt
          signature: "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {"
      - id: impl_EventHandle
        signature: "impl EventHandle {"
        children:
        - id: impl_EventHandle.from_repr
          signature: "pub(crate) fn from_repr(repr: EventRepr) -> Self {"
        - id: impl_EventHandle.repr
          signature: "pub(crate) fn repr(&self) -> &EventRepr {"
        - id: impl_EventHandle.is_complete
          signature: "pub fn is_complete(&self) -> bool {"
        - id: impl_EventHandle.synchronize
          signature: "pub fn synchronize(&self) -> Result<(), DeviceError> {"
          docstring: Blocks the calling thread until the event completes.
      - id: ParamValue
        signature: "pub trait ParamValue: Copy {"
        children:
        - id: ParamValue.D1i1th|F97p37
          signature: "const SIZE: usize;"
        - id: ParamValue.write
          signature: "fn write(self, out: &mut Vec<u8>);"
      - id: GHxFN6|KejVsY
        signature: "macro_rules! param_value {"
      - id: ParamBlock
        signature: "pub struct ParamBlock {"
        docstring: "Fixed-capacity buffer for dynamic kernel parameters, staged on the host and mir…"
      - id: impl_ParamBlock
        signature: "impl ParamBlock {"
        children:
        - id: impl_ParamBlock.new
          signature: "pub fn new(capacity: usize) -> Self {"
        - id: impl_ParamBlock.capacity
          signature: "pub fn capacity(&self) -> usize {"
        - id: impl_ParamBlock.bind
          signature: "pub fn bind(&mut self, region: Region) {"
        - id: impl_ParamBlock.region
          signature: "pub fn region(&self) -> Option<Region> {"
        - id: impl_ParamBlock.clear
          signature: "pub fn clear(&mut self) {"
        - id: impl_ParamBlock.as_bytes
          signature: "pub fn as_bytes(&self) -> &[u8] {"
        - id: impl_ParamBlock.push
          signature: "pub fn push<T: ParamValue>(&mut self, value: T) -> Result<usize, DeviceError> {"
          docstring: Appends `value` naturally aligned and returns its byte offset.
      - id: ScratchArena
        signature: "pub struct ScratchArena {"
        docstring: Bump allocator over a lane-private arena span; reset once the lane has drained.
      - id: impl_ScratchArena
        signature: "impl ScratchArena {"
        children:
        - id: impl_ScratchArena.new
          signature: "pub fn new(base: Region) -> Self {"
        - id: impl_ScratchArena.capacity
          signature: "pub fn capacity(&self) -> u64 {"
        - id: impl_ScratchArena.used
          signature: "pub fn used(&self) -> u64 {"
        - id: impl_ScratchArena.alloc
          signature: "pub fn alloc(&self, bytes: u64, align: u64) -> Result<Region, DeviceError> {"
        - id: impl_ScratchArena.reset
          signature: "pub fn reset(&self) {"
          docstring: Invalidates all regions handed out so far.
      - id: ExecContext
        signature: "pub trait ExecContext: Send {"
        docstring: "In-order execution queue of one device. Device handles, streams and pointers ne…"
        children:
        - id: ExecContext.class
          signature: fn class(&self) -> LaneClass;
        - id: ExecContext.device
          signature: fn device(&self) -> DeviceKind;
        - id: ExecContext.submit
          signature: "fn submit(&self, job: LaneJob) -> Result<(), DeviceError>;"
        - id: ExecContext.record_event
          signature: "fn record_event(&self) -> Result<EventHandle, DeviceError>;"
          docstring: Completes once everything submitted so far has finished.
        - id: ExecContext.wait
          signature: "fn wait(&self, event: &EventHandle) -> Result<(), DeviceError>;"
          docstring: Orders all later work on this lane after `event`.
        - id: ExecContext.synchronize
          signature: "fn synchronize(&self) -> Result<(), DeviceError>;"
          docstring: Blocks until the lane has drained; returns the first job failure since the last…
        - id: ExecContext.param_block
          signature: fn param_block(&mut self) -> &mut ParamBlock;
        - id: ExecContext.scratch
          signature: fn scratch(&self) -> &ScratchArena;
      - id: Msg
        signature: "enum Msg {"
      - id: CpuLane
        signature: "pub struct CpuLane {"
        docstring: "CPU lane: one worker thread, so submission order is execution order."
      - id: impl_CpuLane
        signature: "impl CpuLane {"
        children:
        - id: impl_CpuLane.new
          signature: "pub fn new(class: LaneClass, cfg: LaneConfig) -> Result<Self, DeviceError> {"
        - id: impl_CpuLane.send
          signature: "fn send(&self, msg: Msg) -> Result<(), DeviceError> {"
      - id: run_worker
        signature: "fn run_worker(rx: Receiver<Msg>, failure: &Mutex<Option<DeviceError>>) {"
      - id: impl_ExecContext_for_CpuLane
        signature: "impl ExecContext for CpuLane {"
        children:
        - id: impl_ExecContext_for_CpuLane.class
          signature: "fn class(&self) -> LaneClass {"
        - id: impl_ExecContext_for_CpuLane.device
          signature: "fn device(&self) -> DeviceKind {"
        - id: impl_ExecContext_for_CpuLane.submit
          signature: "fn submit(&self, job: LaneJob) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_CpuLane.record_event
          signature: "fn record_event(&self) -> Result<EventHandle, DeviceError> {"
        - id: impl_ExecContext_for_CpuLane.wait
          signature: "fn wait(&self, event: &EventHandle) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_CpuLane.synchronize
          signature: "fn synchronize(&self) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_CpuLane.param_block
          signature: "fn param_block(&mut self) -> &mut ParamBlock {"
        - id: impl_ExecContext_for_CpuLane.scratch
          signature: "fn scratch(&self) -> &ScratchArena {"
      - id: impl_Drop_for_CpuLane
        signature: "impl Drop for CpuLane {"
        children:
        - id: impl_Drop_for_CpuLane.drop
          signature: "fn drop(&mut self) {"
      - id: RecordingLane
        signature: "pub struct RecordingLane {"
        docstring: "Lane double for downstream tests: runs jobs inline and logs every call."
      - id: impl_RecordingLane
        signature: "impl RecordingLane {"
        children:
        - id: impl_RecordingLane.new
          signature: "pub fn new(class: LaneClass, cfg: LaneConfig) -> Self {"
        - id: impl_RecordingLane.calls
          signature: "pub fn calls(&self) -> Vec<&'static str> {"
      - id: impl_ExecContext_for_RecordingLane
        signature: "impl ExecContext for RecordingLane {"
        children:
        - id: impl_ExecContext_for_RecordingLane.class
          signature: "fn class(&self) -> LaneClass {"
        - id: impl_ExecContext_for_RecordingLane.device
          signature: "fn device(&self) -> DeviceKind {"
        - id: impl_ExecContext_for_RecordingLane.submit
          signature: "fn submit(&self, job: LaneJob) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_RecordingLane.record_event
          signature: "fn record_event(&self) -> Result<EventHandle, DeviceError> {"
        - id: impl_ExecContext_for_RecordingLane.wait
          signature: "fn wait(&self, _event: &EventHandle) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_RecordingLane.synchronize
          signature: "fn synchronize(&self) -> Result<(), DeviceError> {"
        - id: impl_ExecContext_for_RecordingLane.param_block
          signature: "fn param_block(&mut self) -> &mut ParamBlock {"
        - id: impl_ExecContext_for_RecordingLane.scratch
          signature: "fn scratch(&self) -> &ScratchArena {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/lifecycle.rs
      nodes:
      - id: 5AfduI|B2pQZe
        signature: use std::future::Future;
      - id: lUjdIX|dX0Yys
        signature: "const UNLOADED: u8 = 0;"
      - id: ResourceState
        signature: "pub enum ResourceState {"
      - id: CellConfig
        signature: "pub struct CellConfig {"
      - id: InitLimiter
        signature: pub struct InitLimiter(Arc<Semaphore>);
        docstring: Bounds the number of resources initialising concurrently across all cells.
      - id: impl_InitLimiter
        signature: "impl InitLimiter {"
        children:
        - id: impl_InitLimiter.new
          signature: "pub fn new(max_parallel: usize) -> Self {"
        - id: impl_InitLimiter.acquire
          signature: "async fn acquire(&self) -> Result<tokio::sync::SemaphorePermit<'_>, SubengineEr…"
      - id: Shared
        signature: "struct Shared {"
      - id: impl_Shared
        signature: "impl Shared {"
        children:
        - id: impl_Shared.release_lease
          signature: "fn release_lease(&self) {"
      - id: ResourceCell
        signature: "pub struct ResourceCell<R> {"
        docstring: "A lazily initialised, evictable resource."
      - id: impl_ResourceCell_R_
        signature: "impl<R: Resource> ResourceCell<R> {"
        children:
        - id: impl_ResourceCell_R_.new
          signature: "pub fn new(key: ResourceKey, cfg: CellConfig) -> Self {"
        - id: impl_ResourceCell_R_.key
          signature: "pub fn key(&self) -> &ResourceKey {"
        - id: impl_ResourceCell_R_.state
          signature: "pub fn state(&self) -> ResourceState {"
        - id: impl_ResourceCell_R_.get_or_init
          signature: "pub async fn get_or_init<F, Fut>(     &self,     limiter: &InitLimiter,     now…"
          docstring: "Leases the resource, running `init` if it is not resident. Concurrent callers s…"
        - id: impl_ResourceCell_R_.evict_idle
          signature: "pub fn evict_idle(&self, now_ms: u64) -> bool {"
          docstring: "Unloads the resource when unleased, idle past the TTL and not vetoed by the res…"
      - id: ResourceLease
        signature: "pub struct ResourceLease<R> {"
      - id: impl_ResourceLease_R_
        signature: "impl<R> ResourceLease<R> {"
        children:
        - id: impl_ResourceLease_R_.begin_op
          signature: "pub fn begin_op(&self) -> OpGuard {"
          docstring: Marks an operation as running on the resource; eviction waits for the guard to …
      - id: impl_Deref_for_ResourceLease_R_
        signature: "impl<R> Deref for ResourceLease<R> {"
        children:
        - id: impl_Deref_for_ResourceLease_R_.iyiBZo|illfNY
          signature: type Target = R;
        - id: impl_Deref_for_ResourceLease_R_.deref
          signature: "fn deref(&self) -> &R {"
      - id: impl_Drop_for_ResourceLease_R_
        signature: "impl<R> Drop for ResourceLease<R> {"
        children:
        - id: impl_Drop_for_ResourceLease_R_.drop
          signature: "fn drop(&mut self) {"
      - id: OpGuard
        signature: "pub struct OpGuard {"
      - id: impl_Drop_for_OpGuard
        signature: "impl Drop for OpGuard {"
        children:
        - id: impl_Drop_for_OpGuard.drop
          signature: "fn drop(&mut self) {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/resource.rs
      nodes:
      - id: 2Cm4Tq|b7Fqza
        signature: use std::collections::HashMap;
      - id: ResourceKey
        signature: pub struct ResourceKey(String);
      - id: impl_ResourceKey
        signature: "impl ResourceKey {"
        children:
        - id: impl_ResourceKey.new
          signature: "pub fn new(key: impl Into<String>) -> Self {"
      - id: impl_fmt__Display_for_ResourceKey
        signature: "impl fmt::Display for ResourceKey {"
        children:
        - id: impl_fmt__Display_for_ResourceKey.fmt
          signature: "fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {"
      - id: Resource
        signature: "pub trait Resource: Send + Sync + 'static {"
        children:
        - id: Resource.key
          signature: fn key(&self) -> ResourceKey;
        - id: Resource.init_cost_hint
          signature: "fn init_cost_hint(&self) -> u64 {"
          docstring: Relative cost of re-initialising after eviction; lets callers prefer cheap vict…
        - id: Resource.evictable
          signature: "fn evictable(&self, _now_ms: u64) -> bool {"
          docstring: Veto for resource-specific reasons beyond lease and TTL checks.
      - id: Ewma
        signature: "pub struct Ewma {"
        docstring: Exponentially weighted moving average; the first sample primes the value.
      - id: impl_Ewma
        signature: "impl Ewma {"
        children:
        - id: impl_Ewma.new
          signature: "pub fn new(alpha: f32) -> Self {"
        - id: impl_Ewma.observe
          signature: "pub fn observe(&mut self, sample: f32) -> f32 {"
        - id: impl_Ewma.get
          signature: "pub fn get(&self) -> Option<f32> {"
      - id: PullConfig
        signature: "pub struct PullConfig {"
      - id: impl_Default_for_PullConfig
        signature: "impl Default for PullConfig {"
        children:
        - id: impl_Default_for_PullConfig.default
          signature: "fn default() -> Self {"
      - id: PullController
        signature: "pub struct PullController {"
        docstring: Derives the batch size to pull from observed throughput; the clamp bounds load …
      - id: impl_PullController
        signature: "impl PullController {"
        children:
        - id: impl_PullController.new
          signature: "pub fn new(cfg: PullConfig) -> Self {"
        - id: impl_PullController.observe
          signature: "pub fn observe(&self, items: usize, elapsed_ms: f32) {"
        - id: impl_PullController.throughput
          signature: "pub fn throughput(&self) -> f32 {"
          docstring: Items per second; zero until the first observation.
        - id: impl_PullController.batch_size
          signature: "pub fn batch_size(&self) -> usize {"
      - id: Capacity
        signature: "pub struct Capacity {"
      - id: impl_Capacity
        signature: "impl Capacity {"
        children:
        - id: impl_Capacity.free
          signature: "pub fn free(&self) -> usize {"
      - id: ClaimFilter
        signature: "pub struct ClaimFilter {"
        docstring: Restricts which pending work a subengine is willing to take.
      - id: Claim
        signature: "pub trait Claim<J> {"
        docstring: "Implemented by the processor: hands out at most `budget` jobs matching `filter`."
        children:
        - id: Claim.claim
          signature: "fn claim(&self, filter: &ClaimFilter, budget: usize) -> Vec<J>;"
      - id: coalesce
        signature: "pub fn coalesce<T, K: Eq + Hash + Clone>(     items: impl IntoIterator<Item = T…"
        docstring: "Groups items by key, keeping first-seen group order and item order within each …"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/error.rs
      nodes:
      - id: HrcLHg|pZ5pMk
        signature: use thiserror::Error;
      - id: SubengineError
        signature: "pub enum SubengineError {"
        docstring: Subengine resource lifecycle errors.
      - id: impl_IsRetryable_for_SubengineError
        signature: "impl IsRetryable for SubengineError {"
        children:
        - id: impl_IsRetryable_for_SubengineError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: DeviceError
        signature: "pub enum DeviceError {"
        docstring: Device and execution-lane errors.
      - id: impl_IsRetryable_for_DeviceError
        signature: "impl IsRetryable for DeviceError {"
        children:
        - id: impl_IsRetryable_for_DeviceError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
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
        signature: "pub enum OpKind {"
      - id: OpParams
        signature: "pub struct OpParams {"
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
        signature: "pub struct OpDesc {"
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
      - id: ScoredHit
        signature: "pub struct ScoredHit {"
        docstring: "One resolved hit: manager-level entry, its score, staleness and the source back…"
      - id: ScoredHits
        signature: "pub struct ScoredHits {"
        docstring: Result of a similarity chain before the facade ranks and summarises it.
      - id: HostData
        signature: "pub enum HostData {"
      - id: Value
        signature: "pub enum Value {"
        docstring: Op input/output; never carries a tensor or raw pointer.
      - id: TaskCtx
        signature: "pub struct TaskCtx {"
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
          signature: "pub fn add_ops(&self, n: usize) {"
          docstring: Registers `n` follow-up ops. Must run before they are enqueued and before the
        - id: impl_TaskCtx.open_ops
          signature: "pub fn open_ops(&self) -> usize {"
        - id: impl_TaskCtx.is_cancelled
          signature: "pub fn is_cancelled(&self) -> bool {"
        - id: impl_TaskCtx.error
          signature: "pub fn error(&self) -> Option<&ResidentError> {"
        - id: impl_TaskCtx.finish_op
          signature: "pub fn finish_op(&self) -> bool {"
          docstring: Returns `true` when this call closed the last open op of a non-failed task.
        - id: impl_TaskCtx.fail
          signature: "pub fn fail(&self, err: ResidentError) -> bool {"
          docstring: Cancels the task; only the first error is kept. Returns `true` if this call was…
      - id: Successor
        signature: "pub trait Successor: Send + Sync {"
        docstring: Builds the operation that consumes a finished operation's output.
        children:
        - id: Successor.follow
          signature: "fn follow(&self, output: Value, task: &Arc<TaskCtx>) -> Vec<Arc<OperationNode>>;"
          docstring: Empty when the chain ends; the implementation then keeps `output` as the task r…
      - id: OperationNode
        signature: "pub struct OperationNode {"
      - id: impl_OperationNode
        signature: "impl OperationNode {"
        children:
        - id: impl_OperationNode.new
          signature: "pub fn new(     op: Arc<dyn Op>,     input: Value,     materialize: bool,     t…"
        - id: impl_OperationNode.with_successor
          signature: "pub fn with_successor(mut self, successor: Arc<dyn Successor>) -> Self {"
        - id: impl_OperationNode.set_output
          signature: "pub fn set_output(&self, value: Value) -> Result<(), Value> {"
          docstring: Sets the output once; a second call returns the rejected value.
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
      nodes:
      - id: ModuleDoc
        signature: //! Resource requirements of an operation.
      - id: 2iXFMc|Cm8Kf6
        signature: "use crate::resident::memory::{DeviceKind, Region};"
      - id: ModelId
        signature: pub struct ModelId(pub String);
      - id: IndexId
        signature: pub struct IndexId(pub String);
      - id: DeviceId
        signature: pub struct DeviceId(pub u32);
      - id: ResourceKind
        signature: "pub enum ResourceKind {"
      - id: EngineSelector
        signature: "pub enum EngineSelector {"
      - id: impl_EngineSelector
        signature: "impl EngineSelector {"
        children:
        - id: impl_EngineSelector.accepts
          signature: "pub fn accepts(self, dev: DeviceKind) -> bool {"
      - id: Affinity
        signature: "pub enum Affinity {"
        docstring: Placement hint; inactive in the scheduler for now.
      - id: Requirements
        signature: "pub struct Requirements {"
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
          signature: "pub fn satisfied_by(&self, dev: DeviceKind, available: &CapabilitySet) -> bool {"
          docstring: Affinity is deliberately ignored.
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/lease.rs
      nodes:
      - id: ModuleDoc
        signature: //! RAII lease on a device region; pinned regions are not evicted.
      - id: 07O91Q|1abKmK
        signature: "use std::sync::atomic::{AtomicUsize, Ordering};"
      - id: PinCount
        signature: pub struct PinCount(AtomicUsize);
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
        signature: "pub struct Lease {"
      - id: DeviceHandle
        signature: pub type DeviceHandle = Lease;
      - id: impl_Lease
        signature: "impl Lease {"
        children:
        - id: impl_Lease.new
          signature: "pub fn new(     region: Region,     pin: Arc<PinCount>,     release: Option<Unb…"
          docstring: Increments the pin count; `release` receives the region on drop.
        - id: impl_Lease.region
          signature: "pub fn region(&self) -> Region {"
      - id: impl_Drop_for_Lease
        signature: "impl Drop for Lease {"
        children:
        - id: impl_Drop_for_Lease.drop
          signature: "fn drop(&mut self) {"
      - id: tests
        signature: mod tests;
```
/answer cc1b-16 allow
Text:
 Jetzt die öffentlichen Typdeklarationen der übrigen Ebenen (kompakter als volle Outlines).
Control Request:
```yaml
id: cc1b-17
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/model
    pattern: ^pub(\(crate\))? (struct|enum|trait|type|fn)
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    - '**/tests.rs'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    pattern: ^pub(\(crate\))? (struct|enum|trait|type|fn)
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/index
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
    pattern: ^pub(\(crate\))? (struct|enum|trait|type|fn)
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  reason: "Öffentliche Typen von planner, tokenize, model, memory, index, segment erfassen"
```
/answer cc1b-17 allow
ReasoningToken: 269
Control Request:
```yaml
id: cc1b-18
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/model
      matches:
      - path: rules.rs
        matches:
        - "pub struct RuleSet {"
      - path: mod.rs
        matches:
        - "pub struct Planner {"
        - "pub struct SharedTokenizer {"
        - "pub enum Pooling {"
        - "pub enum Normalize {"
        - "pub struct ComputeDtypes {"
        - "pub struct Prefixes {"
        - "pub struct HeadSpec {"
        - "pub struct ModelPaths {"
        - "pub struct ModelSpec {"
        - "pub struct TensorRef {"
        - "pub trait RegionIo: Send + Sync {"
        - "pub struct EncodeEvent {"
        - "pub trait Encoder: Send + Sync {"
        - pub type EncoderFactory = Box<
        - "pub struct ModelRegistry {"
        - "pub struct LoadedEncoder {"
      - path: config.rs
        matches:
        - "pub enum Pooling {"
        - "pub enum PipelineDtype {"
        - "pub enum ScorerKind {"
        - "pub struct Prefixes {"
        - "pub struct PipelineConfig {"
      - path: expand.rs
        matches:
        - "pub enum TaskSpec<'a> {"
        - "pub fn first_resource(kind: TaskKind) -> ResourceKind {"
        - "pub struct PlannedOp {"
        - "pub struct ExpandContext<'a> {"
        - "pub trait PlanExpander: Send + Sync {"
      - path: adapter.rs
        matches:
        - "pub struct PlannerExpander {"
      - path: offers.rs
        matches:
        - "pub struct EngineOffers {"
      - path: normalize.rs
        matches:
        - "pub trait TextNormalizer: Send + Sync {"
        - pub struct Identity;
      - path: modes.rs
        matches:
        - "pub enum EncodeMode {"
        - "pub struct ColbertScheme {"
        - "pub enum ModeScheme {"
      - path: error.rs
        matches:
        - "pub enum TokenizeError {"
        - "pub enum ModelError {"
      - path: encoder.rs
        matches:
        - "pub trait Backbone: Send + Sync + 'static {"
        - "pub struct PooledEncoder<B: Backbone> {"
      - path: pooling.rs
        matches:
        - pub fn mean_pool(
        - "pub fn l2_normalize_rows(rows: &mut [f32], dim: usize) -> usize {"
      - path: head.rs
        matches:
        - "pub struct ProjectionHead {"
      - path: candle_adapter.rs
        matches:
        - "pub struct CandleBert {"
        - pub fn bert_factory(
      - path: slot.rs
        matches:
        - "pub fn encode_bytes(values: &[f32], dtype: DType) -> Result<Vec<u8>, ModelError> {"
        - pub fn write_slot(
      - path: batch.rs
        matches:
        - "pub struct BatchConfig {"
        - "pub struct Batch {"
        - "pub fn plan_batches(seqs: &[Vec<u32>], cfg: &BatchConfig) -> Vec<Batch> {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      matches:
      - path: cpu.rs
        matches:
        - "pub struct CpuMemoryManager {"
      - path: layout.rs
        matches:
        - "pub struct Layout {"
      - path: budget.rs
        matches:
        - "pub enum ArenaKind {"
        - "pub struct ArenaSizes {"
        - "pub struct Budget {"
        - "pub trait BudgetProbe: Send + Sync {"
        - pub fn check_fit(
      - path: region.rs
        matches:
        - pub struct ArenaId(pub u32);
        - "pub struct Region {"
      - path: arena.rs
        matches:
        - "pub struct ArenaAllocator {"
        - "pub struct ArenaSet {"
        - "pub trait DeviceMemory: Send + Sync {"
      - path: cuda.rs
        matches:
        - pub type PinnedStaging = StagingRing<PinnedHostSlice<u8>>;
        - "pub struct CudaProbe {"
        - "pub struct CudaMemoryManager {"
      - path: error.rs
        matches:
        - "pub enum TransferDir {"
        - "pub enum MemoryError {"
      - path: freelist.rs
        matches:
        - "pub struct FreeList {"
      - path: dtype.rs
        matches:
        - pub struct UnsupportedDType(pub &'static str);
        - "pub enum DType {"
      - path: staging.rs
        matches:
        - "pub struct StagingRing<S> {"
      - path: manager.rs
        matches:
        - "pub enum DeviceKind {"
        - "pub fn compute_dtype(dev: DeviceKind) -> DType {"
        - "pub trait MemoryManager {"
        - "pub struct BookkeepingArena {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/index
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
      matches:
      - path: mod.rs
        matches:
        - pub struct EntryId(u64);
        - pub struct NodeId(pub u64);
        - pub struct CellId(pub u64);
        - pub struct Stamp(u64);
        - "pub enum AccessMethod {"
        - "pub enum Scope {"
        - "pub struct EntrySet {"
        - "pub enum EntryData<'a> {"
        - "pub struct SummaryRef(pub Arc<[u8]>);"
        - "pub enum HashAlgo {"
        - "pub struct IndexHeader {"
        - "pub trait Index {"
        - "pub trait Fence: Send + Sync {"
        - "pub enum SlotClass {"
        - "pub struct SegmentView {"
        - "pub struct Retired {"
        - "pub struct Segment {"
        - "pub struct SegmentTableEntry {"
        - "pub struct SegmentTable {"
        - "pub struct Manifest {"
        - "pub struct GenerationLease {"
        - "pub struct SegmentStore {"
      - path: parquet_io.rs
        matches:
        - "pub enum Access {"
        - "pub struct IndexOptions {"
        - "pub struct ParquetIndex {"
      - path: stamp.rs
        matches:
        - pub struct StampCounter(AtomicU64);
        - "pub struct FileStamp {"
        - "pub enum Verdict {"
        - "pub fn content_hash(bytes: &[u8]) -> u64 {"
      - path: payload.rs
        matches:
        - "pub struct Payload {"
      - path: schema.rs
        matches:
        - "pub fn arrow_schema(dim: usize) -> SchemaRef {"
        - "pub fn check_fields(found: &Schema, expected: &Schema) -> Result<(), IndexError> {"
        - "pub struct SchemaHeader {"
        - "pub fn tokenizer_fingerprint(tokenizer_json: &[u8]) -> String {"
      - path: memory.rs
        matches:
        - "pub struct InMemoryIndex {"
      - path: slots.rs
        matches:
        - "pub struct ValidBitmap {"
        - "pub struct SlotMeta {"
        - "pub struct SlotWrite {"
        - pub fn encode_slot(
      - path: sync.rs
        matches:
        - "pub enum SegmentEventKind {"
        - "pub struct SegmentEvent {"
        - "pub struct SegmentBus {"
        - "pub struct Publisher {"
      - path: error.rs
        matches:
        - "pub enum SegmentError {"
        - "pub(crate) enum MappingError {"
      - path: mapping.rs
        matches:
        - "pub struct EntryRef {"
        - "pub struct SegmentHit {"
        - "pub struct SlotMap {"
        - "pub enum Resolution {"
```
/answer cc1b-18 allow
Control Request:
```yaml
id: cc1b-19
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
    pattern: ^pub(\(crate\))? (struct|enum|trait|type|fn)
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
    pattern: ^pub(\(crate\))? (struct|enum|trait|type|fn)
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
    pattern: ^pub(\(crate\))? (struct|enum|trait|type|fn)
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  reason: "Öffentliche Typen von kernel, manager, precision erfassen"
```
/answer cc1b-19 allow
Control Request:
```yaml
id: cc1b-1a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
      matches:
      - path: mod.rs
        matches:
        - "pub struct RegionSpan {"
        - pub struct OffsetRef(pub RegionSpan);
        - "pub enum SegmentLayout {"
        - "pub struct SegmentDesc {"
        - "pub struct KernelHit {"
        - "pub struct WrittenRange {"
        - "pub struct BatchDesc {"
        - pub struct BatchId(u64);
        - "pub enum KernelOutput {"
        - "pub trait Kernel: Send + Sync {"
        - "pub fn entry_of_fixed(base: u64, stride: u32, count: u32, addr: u64) -> Option<usize> {"
        - "pub fn entry_of(offsets: &[u64], addr: u64) -> Option<usize> {"
        - "pub struct SliceRange {"
        - "pub struct QueryBatch {"
        - "pub trait SlotSource: Send + Sync {"
        - "pub trait SlotScorer: Send + Sync {"
        - "pub trait Scorer: Send + Sync {"
        - "pub struct TopKScorer<S> {"
        - pub fn score_ranges(
      - path: maxsim.rs
        matches:
        - "pub struct MaxSimOp {"
      - path: slots.rs
        matches:
        - "pub struct InMemorySlots {"
        - "pub(crate) fn check_range(source: &dyn SlotSource, range: SliceRange) -> Result<(), KernelError> {"
      - path: golden.rs
        matches:
        - "pub struct Tolerance {"
        - "pub enum Deviation {"
        - pub fn compare(
        - pub fn check(
      - path: cpu_simd.rs
        matches:
        - "pub fn dot(a: &[f32], b: &[f32]) -> f32 {"
        - "pub fn maxsim(query: &[f32], doc: &[f32], dim: usize) -> f32 {"
        - "pub fn normalize(v: &mut [f32]) {"
        - pub(crate) fn scan_slots(
        - "pub struct CpuMaxSim {"
      - path: launcher.rs
        matches:
        - "pub enum LaunchMode {"
        - "pub struct LaunchKey {"
        - "pub fn shape_bucket(extent: u32) -> u32 {"
        - "pub trait Launcher: Send + Sync {"
        - pub struct DirectLauncher;
        - "pub fn launcher_for(mode: LaunchMode) -> Result<Arc<dyn Launcher>, KernelError> {"
      - path: topk.rs
        matches:
        - "pub struct TopK {"
        - "pub fn merge(parts: impl IntoIterator<Item = Vec<KernelHit>>, k: usize) -> Vec<KernelHit> {"
      - path: registry.rs
        matches:
        - "pub struct KernelKey {"
        - "pub trait KernelLoader<F>: Send + Sync {"
        - "pub struct KernelRegistry<F> {"
      - path: cosine.rs
        matches:
        - "pub struct CosineSlots {"
        - pub type CosineScorer = TopKScorer<CosineSlots>;
        - "pub fn cosine_scorer(source: Arc<dyn SlotSource>, k: usize) -> CosineScorer {"
        - "pub fn cosine_scorer_exact(source: Arc<dyn SlotSource>, k: usize) -> CosineScorer {"
      - path: cuda.rs
        matches:
        - "pub struct NvrtcLoader {"
        - "pub struct CudaKernels {"
        - "pub(crate) fn launch_cosine<T: DeviceRepr>("
        - "pub(crate) fn launch_maxsim<T: DeviceRepr>("
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      matches:
      - path: doorbell.rs
        matches:
        - "pub struct Gap {"
        - "pub struct Doorbell {"
      - path: advice.rs
        matches:
        - "pub enum Hint {"
        - "pub trait AdviceSink: Send + Sync {"
        - "pub struct MmapAdvice {"
      - path: upload.rs
        matches:
        - "pub trait StagingBuffer: Send {"
        - "pub trait UploadPort: Send + Sync {"
        - "pub struct Placement {"
        - "pub struct StagedGather<S> {"
        - "pub struct Prepared<'a, S: StagingBuffer> {"
      - path: swap.rs
        matches:
        - "pub struct RootPointer {"
        - "pub struct SegmentTable {"
        - "pub struct EpochTracker {"
        - "pub trait TableSink: Send + Sync {"
        - "pub trait TableSwap {"
        - "pub struct Snapshot {"
        - "pub struct BatchGuard<'a> {"
        - "pub struct SwapManager<'a, 'b> {"
      - path: blocking.rs
        matches:
        - "pub struct BlockingReads {"
      - path: sync.rs
        matches:
        - "pub struct ScanOutcome {"
        - "pub struct SyncScanner {"
        - "pub struct Loaded<'a> {"
        - "pub fn load_entry(index: &dyn Index, entry: EntryId) -> Option<Loaded<'_>> {"
      - path: transfer.rs
        matches:
        - "pub trait RegionSource: Send + Sync {"
        - "pub trait MappedHost: Send + Sync {"
        - "pub struct LoadedSegment {"
        - "pub trait SegmentSink: Send + Sync {"
        - "pub struct LoadedQueue {"
        - "pub struct GatherTransfer<S: StagingBuffer> {"
      - path: arena.rs
        matches:
        - "pub struct CowBudget {"
        - "pub struct ArenaBudget {"
        - "pub struct IndexArena<'a> {"
        - "pub struct CowTicket<'a, 'b> {"
        - "pub struct WritableRegion {"
      - path: residency.rs
        matches:
        - pub type SegmentId = u64;
        - "pub trait LoadGeometry: Send + Sync {"
        - "pub struct UniformGeometry {"
        - "pub enum Access {"
        - "pub enum LoadPlan {"
        - "pub struct LoadPolicy {"
        - pub fn plan_load(
        - "pub trait LoadTransfer: Send + Sync {"
        - "pub struct LoadOutcome {"
        - "pub struct Aggregated<K> {"
        - "pub struct LoadAggregator<K> {"
        - "pub struct ResidencyLoader<K> {"
      - path: test/flat.rs
        matches:
        - "pub(crate) struct FlatIndex {"
      - path: build.rs
        matches:
        - "pub enum BuildRing {"
        - "pub struct WriteCursor {"
        - "pub fn exclusive_prefix_sum(base: u64, lens: &[u64]) -> Vec<u64> {"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      matches:
      - path: probe.rs
        matches:
        - "pub trait CapabilityProbe: Send + Sync {"
        - "pub struct ProbeConfig {"
        - "pub(crate) fn powers_of_two(from: u32, to: u32) -> Vec<u32> {"
        - "pub fn intersect_with_kernels(set: &CapabilitySet, kernels: &CapabilitySet) -> CapabilitySet {"
        - "pub struct KernelFilteredProbe<P> {"
        - "pub(crate) fn numeric_axis(set: &mut CapabilitySet, axis: &AxisId, values: impl IntoIterator<Item = u32>) {"
        - pub fn make_probe(
        - "pub struct FakeCapabilityProbe {"
      - path: pool.rs
        matches:
        - "pub fn lower_current_thread_priority() -> bool {"
        - pub struct JobHandle<T>(Receiver<T>);
        - "pub struct LowPriorityPool {"
      - path: known.rs
        matches:
        - "pub fn gpu_sm75() -> CapabilitySet {"
        - "pub fn cpu() -> CapabilitySet {"
      - path: probe_cpu.rs
        matches:
        - "pub struct CpuFeatures {"
        - "pub trait CpuFeatureSource: Send + Sync {"
        - pub struct HostCpu;
        - "pub struct FixedCpu {"
        - "pub struct CpuProbe {"
      - path: measure_cpu.rs
        matches:
        - "pub struct CpuScoringMeasure {"
      - path: measure_mock.rs
        matches:
        - "pub struct ManualClock {"
        - "pub struct FakeCombo {"
        - "pub struct FakeMeasure {"
      - path: measure_cuda.rs
        matches:
        - "pub struct CudaScoringMeasure {"
      - path: metrics.rs
        matches:
        - "pub struct Plausibility {"
        - "pub fn cosine_deviation(a: &[f32], reference: &[f32]) -> f64 {"
        - "pub fn relative_error(a: &[f32], reference: &[f32]) -> f64 {"
        - "pub fn top_k_overlap(a: &[f32], reference: &[f32], k: usize) -> f64 {"
        - pub fn judge_rows(
        - "pub fn is_duplicate_free<T: Hash + Eq>(items: &[T]) -> bool {"
      - path: stats.rs
        matches:
        - "pub enum Spread {"
        - "pub enum Estimator {"
        - "pub fn median(values: &[f64]) -> f64 {"
        - "pub fn estimate(values: &[f64], how: Estimator) -> f64 {"
        - "pub fn spread(values: &[f64], kind: Spread) -> f64 {"
      - path: store.rs
        matches:
        - "pub enum StoreState {"
        - "pub struct GpuIdentity {"
        - "pub struct Fingerprint {"
        - "pub struct StoredProfile {"
        - "pub struct RejectedCombo {"
        - "pub struct PrunedCombo {"
        - "pub struct CalibrationRecord {"
        - "pub trait CalibrationStore: Send + Sync {"
        - "pub fn state(record: Option<&CalibrationRecord>, current: &Fingerprint) -> StoreState {"
        - "pub struct Resolution {"
        - "pub fn resolve(store: &dyn CalibrationStore, current: &Fingerprint) -> Resolution {"
        - pub fn recalibrate_in_background<F>(
        - "pub struct FileCalibrationStore {"
      - path: pareto.rs
        matches:
        - "pub fn pareto(pts: Vec<(f64, f64)>) -> Vec<(f64, f64)> {"
        - "pub fn pareto_by<T>(mut items: Vec<T>, key: impl Fn(&T) -> (f64, f64)) -> Vec<T> {"
        - "pub fn positions(front: &[(f64, f64)], log_time: bool) -> Vec<f64> {"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer cc1b-1a allow
Text:
 Restliche Typen der abgeschnittenen Ebenen nachladen.
Control Request:
```yaml
id: cc1b-1b
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
    pattern: ^pub(\(crate\))? (struct|enum|trait|type|fn)
    include:
    - streamed.rs
    - runtime.rs
    - slot.rs
    - flight.rs
    - scope.rs
    - tracker.rs
    - evict.rs
    - seqlock.rs
    - mod.rs
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
    pattern: ^pub(\(crate\))? (struct|enum|trait|type|fn)
    include:
    - catalog.rs
    - mock.rs
    - slice.rs
    - maxsim.rs
    - error.rs
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
    pattern: ^pub(\(crate\))? (struct|enum|trait|type|fn)
    include:
    - calibrate.rs
    - capability.rs
    - profile.rs
    - matching.rs
    - axis.rs
    - measure.rs
    - workload.rs
    - probe_cuda.rs
    - measure_cuda.rs
    - mod.rs
    exclude:
    - '**/test/**'
    limit: 50
  reason: "Fehlende Typdeklarationen für manager, kernel, precision nachladen"
```
/answer cc1b-1b allow
ReasoningToken: 188
Control Request:
```yaml
id: cc1b-1c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      matches:
      - path: tracker.rs
        matches:
        - "pub struct SlotRef {"
        - "pub struct Resident {"
        - "pub struct ResidencyTracker {"
      - path: scope.rs
        matches:
        - "pub trait CellSelector: Send + Sync {"
        - "pub struct ResolveCtx {"
        - "pub trait ScopeResolver: Send + Sync {"
        - pub struct DuplicateResolver(pub AccessMethod);
        - "pub struct ResolverRegistry {"
        - pub struct RangeResolver;
        - pub struct SubtreeResolver;
        - pub struct PrefixResolver;
        - pub struct CentroidResolver;
      - path: evict.rs
        matches:
        - "pub enum Tier {"
        - "pub enum EvictReason {"
        - "pub struct Eviction<K> {"
        - "pub struct Occupancy {"
        - "pub struct EvictPolicy {"
        - "pub struct Pin {"
        - "pub struct Evictor<K> {"
      - path: streamed.rs
        matches:
        - pub type SharedIndex = Arc<dyn Index + Send + Sync>;
        - "pub trait IndexManager: Send + Sync {"
        - "pub struct StreamedIndexManager<B: ?Sized> {"
        - "pub enum ReadPattern {"
        - "pub struct CpuIndexManager<B: ?Sized> {"
        - "pub struct SegmentKey {"
        - "pub struct GpuIndexManager<B: ?Sized> {"
        - "pub(crate) fn index_error(e: &ManagerError) -> IndexError {"
      - path: seqlock.rs
        matches:
        - "pub struct Seqlock<'a> {"
      - path: runtime.rs
        matches:
        - "pub struct IndexRuntime<'a, 'b, B: ?Sized> {"
      - path: slot.rs
        matches:
        - "pub struct Hit {"
        - "pub struct AddressMap {"
        - "pub struct MappingStore {"
        - "pub enum Update {"
        - "pub struct UpdateQueue {"
        - "pub trait RangeSource {"
        - pub(crate) fn run_slot_at(
        - pub fn run_slot(
      - path: flight.rs
        matches:
        - "pub type FlightResult<T> = Result<Arc<T>, Arc<ManagerError>>;"
        - "pub struct SingleFlight<K, T> {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
      matches:
      - path: maxsim.rs
        matches:
        - "pub struct MaxSimOp {"
      - path: error.rs
        matches:
        - "pub enum KernelError {"
      - path: catalog.rs
        matches:
        - "pub fn implemented_capabilities(kind: SubengineKind) -> CapabilitySet {"
      - path: mock.rs
        matches:
        - "pub enum Call {"
        - "pub struct MockKernel {"
      - path: slice.rs
        matches:
        - "pub enum StopReason {"
        - "pub struct SliceOutcome {"
        - "pub struct SliceSizer {"
        - pub fn run_sliced<E>(
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      matches:
      - path: measure_cuda.rs
        matches:
        - "pub struct CudaScoringMeasure {"
      - path: probe_cuda.rs
        matches:
        - "pub fn has_tensor_cores(major: i32, name: &str) -> bool {"
        - "pub fn cuda_capabilities(major: i32, minor: i32, cfg: &ProbeConfig) -> CapabilitySet {"
        - "pub struct CudaProbe {"
      - path: measure.rs
        matches:
        - "pub struct Cfg {"
        - "pub enum Validity {"
        - "pub struct DeviationReport {"
        - "pub enum Sample {"
        - "pub trait Measure {"
        - "pub struct MeasureResult {"
        - "pub struct WindowTuning {"
        - "pub trait YieldCheck: Send + Sync {"
        - pub struct NeverYield;
        - "pub trait Clock: Send + Sync {"
        - "pub struct SystemClock {"
        - pub fn make_measure(
      - path: capability.rs
        matches:
        - "pub struct CapabilitySet(BTreeMap<AxisId, BTreeSet<ValueId>>);"
      - path: profile.rs
        matches:
        - "pub struct ProfileConfig(BTreeMap<AxisId, ValueId>);"
        - "pub struct Profile {"
        - "pub struct ProfilingConfig {"
        - "pub fn derive_profiles(front: &[ParetoPoint], cfg: &ProfilingConfig) -> Vec<Profile> {"
      - path: matching.rs
        matches:
        - "pub fn match_profile(precision: f32, profiles: &[Profile]) -> Result<&Profile, PrecisionError> {"
        - "pub struct ProfileSelector {"
      - path: calibrate.rs
        matches:
        - "pub struct HardCap {"
        - "pub struct CalibratorConfig {"
        - "pub struct CalibrationTarget {"
        - "pub enum Stage {"
        - "pub enum Outcome {"
        - "pub struct Entry {"
        - "pub struct ParetoPoint {"
        - "pub struct CalibrationReport {"
        - "pub struct Calibrator<M: Measure> {"
      - path: workload.rs
        matches:
        - "pub struct WorkloadSpec {"
        - "pub struct ScoringWorkload {"
```
/answer cc1b-1c allow
Control Request:
```yaml
id: cc1b-1d
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/expand.rs
    max_line: 120
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize/mod.rs
    max_line: 80
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/slot.rs
    max_line: 30
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/flight.rs
    max_line: 30
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/runtime.rs
    max_line: 40
  reason: "Planner- und Tokenizer-Köpfe sowie Header von manager/slot, flight, runtime lesen, um deren Rolle zu klären"
```
/answer cc1b-1d allow
Control Request:
```yaml
id: cc1b-1e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
      checksum: 1649bf6a55f03da44e9bea1831776554794655e36594c9c4de1721072edd8ff7
      content: |
        //! Stateless task expansion.

        mod adapter;
        mod config;
        mod expand;
        mod offers;
        mod rules;

        use std::sync::Arc;

        use arc_swap::ArcSwap;

        use crate::resident::error::PlannerError;
        use crate::resident::processor::StarvedSnapshot;

        pub use adapter::PlannerExpander;
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
        #[path = "test/mod.rs"]
        mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/expand.rs
      checksum: 168c1f4d98c5fdf014e967cdb2ebd99a25f1f1bac67d3f7fcd42a96374331198
      content: |
        //! Expansion rules: one task becomes an ordered operation chain.
        use crate::resident::error::PlannerError;
        use crate::resident::facade::{
            EmbeddingParams, InferenceParams, Payload, ScorerKind, SimilarityParams, TaskKind,
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
                let engines: Vec<EngineSelector> = self
                    .0
                    .iter()
                    .map(|(_, p)| p.requires.engine)
                    .collect();
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
                chain
                    .push(
                        OpKind::Tokenize,
                        OpParams::new(ctx.requires(ResourceKind::Tokenizer, tokenizer)),
                    );
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize/mod.rs
      checksum: 55f1e3921c8dba2386d6a807a0e493204aae5f548ff42e612198ff4c542ded62
      content: |
        //! One shared CPU tokenizer. Token ids reach the device only as `u32` inputs of the encoder;
        //! there is deliberately no device-side tokenizer or id-to-address gather.
        mod error;
        mod modes;
        mod normalize;
        use std::sync::Arc;
        use tokenizers::{Tokenizer, TruncationParams};
        use crate::resident::model::{plan_batches, Batch, BatchConfig, ModelSpec};
        use crate::resident::subengine::{Resource, ResourceKey};
        pub use error::TokenizeError;
        pub use modes::{ColbertScheme, EncodeMode, ModeScheme};
        pub use normalize::{Identity, TextNormalizer};
        pub struct SharedTokenizer {
            key: ResourceKey,
            tokenizer: Tokenizer,
            scheme: ModeScheme,
            normalizer: Option<Arc<dyn TextNormalizer>>,
            pad_id: u32,
        }
        impl SharedTokenizer {
            /// Loads `tokenizer.json` next to the model's config file.
            pub fn from_spec(spec: &ModelSpec) -> Result<Self, TokenizeError> {
                let path = spec.paths.config.with_file_name("tokenizer.json");
                let tokenizer = Tokenizer::from_file(&path)
                    .map_err(|e| TokenizeError::Load {
                        reason: format!("{}: {e}", path.display()),
                    })?;
                Self::new(
                    format!("tokenizer:{}", path.display()),
                    tokenizer,
                    ModeScheme::Prefix(spec.prefixes.clone()),
                    spec.max_seq_len,
                )
            }
            /// Padding is left to the batcher; truncation to `max_len` keeps the closing special token.
            pub fn new(
                id: impl Into<String>,
                mut tokenizer: Tokenizer,
                scheme: ModeScheme,
                max_len: usize,
            ) -> Result<Self, TokenizeError> {
                let pad_id = tokenizer
                    .get_padding()
                    .map(|p| p.pad_id)
                    .or_else(|| tokenizer.token_to_id("<pad>"))
                    .or_else(|| tokenizer.token_to_id("[PAD]"))
                    .ok_or_else(|| TokenizeError::Load {
                        reason: "no padding token".into(),
                    })?;
                tokenizer.with_padding(None);
                tokenizer
                    .with_truncation(
                        Some(TruncationParams {
                            max_length: max_len,
                            ..Default::default()
                        }),
                    )
                    .map_err(|e| TokenizeError::Load {
                        reason: e.to_string(),
                    })?;
                Ok(Self {
                    key: ResourceKey::new(id),
                    tokenizer,
                    scheme,
                    normalizer: None,
                    pad_id,
                })
            }
            pub fn with_normalizer(mut self, normalizer: Arc<dyn TextNormalizer>) -> Self {
                self.normalizer = Some(normalizer);
                self
            }
            pub fn pad_id(&self) -> u32 {
                self.pad_id
            }
            /// Unpadded id sequences in input order.
            #[tracing::instrument(
                level = "debug",
                skip_all,
                fields(?mode, n_texts = texts.len(), truncated = tracing::field::Empty)
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/slot.rs
      checksum: 1082a7352eaab8f24ac6682528d0c9cb25a383dbf0374fd247a0a382443a239a
      content: |
        //! Communication slot between kernel batches. Phases run in a fixed order: kernel writes are
        //! registered with the index, results are resolved, and only then is the mapping mutated.
        use std::collections::HashMap;
        use std::sync::Arc;
        use parking_lot::Mutex;
        use crate::resident::error::ManagerError;
        use crate::resident::index::{EntryId, Index, Stamp};
        use crate::resident::kernel::{KernelHit, WrittenRange};
        use crate::resident::manager::swap::Snapshot;
        use crate::resident::manager::tracker::{ResidencyTracker, SlotRef};
        use crate::resident::segment::mapping::{EntryRef, SlotMap};
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct Hit {
            pub entry: EntryId,
            pub score: f32,
            pub outdated: bool,
        }
        #[derive(Debug)]
        struct MappedSegment {
            segment: u32,
            map: SlotMap,
            bounds: std::ops::Range<u64>,
        }
        struct Located {
            slot: SlotRef,
            span: std::ops::Range<u64>,
            entry: Option<EntryId>,
        }
        /// Arena address -> entry. The segment set is copy-on-write: a clone is the stable snapshot of a
        /// batch, later `add_segment`/`remove_segment` calls do not reach it. Slot bindings live in the
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/flight.rs
      checksum: 0eadf7a05debd1fd13903cf2a4f867b241e069044381ff0bd6f00bc83b3b8c40
      content: |
        //! Single-flight coalescing: concurrent requests for one key share one load and its outcome,
        //! including its error.
        use std::collections::HashMap;
        use std::future::Future;
        use std::hash::Hash;
        use std::sync::Arc;
        use futures::future::{BoxFuture, FutureExt, Shared};
        use parking_lot::Mutex;
        use crate::resident::error::ManagerError;
        pub type FlightResult<T> = Result<Arc<T>, Arc<ManagerError>>;
        type SharedLoad<T> = Shared<BoxFuture<'static, FlightResult<T>>>;
        struct Flight<T> {
            id: u64,
            load: SharedLoad<T>,
            waiters: usize,
        }
        struct Inner<K, T> {
            next_id: u64,
            flights: HashMap<K, Flight<T>>,
        }
        pub struct SingleFlight<K, T> {
            inner: Mutex<Inner<K, T>>,
        }
        impl<K, T> Default for SingleFlight<K, T> {
            fn default() -> Self {
                Self {
                    inner: Mutex::new(Inner {
                        next_id: 0,
                        flights: HashMap::new(),
                    }),
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/runtime.rs
      checksum: ba5b32d10d96f031589a4da0f63ce6e43258aee57dfce5ed3affa24bfbd8be33
      content: |
        //! Owner of one index operation: ties the manager, the table swap, the mapping store and the
        //! queue of loaded segments together and drives them in the order the mutation discipline needs.
        use std::sync::Arc;
        use std::time::Instant;
        use parking_lot::Mutex;
        use crate::resident::backend::ComputeBackend;
        use crate::resident::error::{IndexError, ManagerError};
        use crate::resident::index::EntrySet;
        use crate::resident::kernel::{KernelHit, WrittenRange};
        use crate::resident::manager::arena::IndexArena;
        use crate::resident::manager::evict::{Eviction, Occupancy};
        use crate::resident::manager::residency::{Access, LoadOutcome};
        use crate::resident::manager::slot::{Hit, MappingStore, RangeSource};
        use crate::resident::manager::streamed::{index_error, GpuIndexManager, SegmentKey};
        use crate::resident::manager::swap::{Snapshot, SwapManager, TableSwap};
        use crate::resident::manager::sync::ScanOutcome;
        use crate::resident::manager::transfer::LoadedQueue;
        /// `loaded` must be the sink of the manager's transfer: every segment the transfer reports is
        /// published here, so no caller has to remember to do it.
        pub struct IndexRuntime<'a, 'b, B: ?Sized> {
            manager: GpuIndexManager<B>,
            arena: &'a IndexArena<'b>,
            swap: SwapManager<'a, 'b>,
            store: MappingStore,
            loaded: Arc<LoadedQueue>,
            /// Table and mapping epochs advance together, so structural changes are serialized.
            structural: Mutex<()>,
        }
        impl<'a, 'b, B: ComputeBackend + ?Sized> IndexRuntime<'a, 'b, B> {
            pub fn new(
                manager: GpuIndexManager<B>,
                arena: &'a IndexArena<'b>,
                swap: SwapManager<'a, 'b>,
                loaded: Arc<LoadedQueue>,
            ) -> Self {
                Self {
                    manager,
                    arena,
                    swap,
                    store: MappingStore::new(),
```
/answer cc1b-1e allow
Text:
 Ein paar Verifikationen: Features, `Fence`-Zuordnung (index vs. segment), Planner-Regeln, Test-Layout und vorhandene README-Konventionen im Repo.
Control Request:
```yaml
id: cc1b-1f
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^\\[features\\]|^(cuda|mock|default|candle)\\s*="
    include:
    - Cargo.toml
    exclude:
    - target/**
    limit: 20
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/index
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
    pattern: pub trait Fence|^pub struct (SegmentView|SegmentStore)|^pub (struct|enum|trait) (Index|SlotClass)\b
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 10
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: README
    include:
    - README*.md
    exclude:
    - target/**
    - node_modules/**
    - .git/**
    limit: 10
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
    pattern: "OpKind::[A-Za-z]+"
    include:
    - expand.rs
    exclude:
    - '**/test/**'
    limit: 50
  reason: "Feature-Flags, Zuordnung von Fence/Segment-Typen, vorhandene READMEs und Op-Ketten der Planner-Regeln prüfen"
```
/answer cc1b-1f allow
ReasoningToken: 515
Control Request:
```yaml
id: cc1b-20
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-transformers/Cargo.toml
        matches:
        - "[features]"
        - "cuda = ["
        - "default = []"
      - path: Cargo.toml
        matches:
        - "[features]"
        - "default = []"
        - "candle = [\"dep:candle-core\", \"dep:candle-nn\", \"dep:candle-transformers\"]"
        - "cuda = [\"candle\", \"dep:cudarc\", \"candle-core/cuda\", \"candle-nn/cuda\", \"candle-transformers/cuda\"]"
        - "mock = [\"dep:rand\"]"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/index
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
      matches:
      - path: mod.rs
        matches:
        - "pub trait Index {"
        - "pub trait Fence: Send + Sync {"
        - "pub enum SlotClass {"
        - "pub struct SegmentView {"
        - "pub struct SegmentStore {"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: claude-code/markdown/remark/node_modules/debug/README.md
        matches:
        - "Become a sponsor and get your logo on our README on Github with a link to your site. [[Become a sponsor](https://opencollective.com/debug#sponsor)]"
      - path: codegen/README.md
        matches:
        - "└── README.md                    # This file"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
      matches:
      - path: expand.rs
        matches:
        - "OpKind::Tokenize,"
        - "chain.push(OpKind::Embed, embed);"
        - "OpKind::Pool,"
        - "OpKind::L2Norm,"
        - "OpKind::WriteIndex,"
        - "chain.push(OpKind::Similarity, score);"
        - "chain.push(OpKind::TopK, top);"
        - "OpKind::Infer,"
```
/answer cc1b-20 allow
Control Request:
```yaml
id: cc1b-21
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
    start: pub enum OpKind
    end: pub struct OpParams
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
    start: pub enum ResourceKind
    end: pub enum Affinity
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/mod.rs
    max_line: 40
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
    start: pub enum Residency
    end: pub struct IndexInfo
  reason: "OpKind, ResourceKind, Segment-Moduldoku, Task-Typen und Residency-Enum lesen"
```
/answer cc1b-21 allow
Control Request:
```yaml
id: cc1b-22
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/mod.rs
      checksum: 837d1e4801a4d3e1d8e26c4429febab7f5db562a2034f105ba1d44ddf82b97bc
      content: |-
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
        pub struct OpParams
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
      checksum: ee849c14f08d872069a130da56fd118e3962c50cdb8b9bd459933069175248a8
      content: |-
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
                    (self, dev), (EngineSelector::Either, _) | (EngineSelector::Cpu,
                    DeviceKind::Cpu) | (EngineSelector::Gpu, DeviceKind::Gpu)
                )
            }
        }
        /// Placement hint; inactive in the scheduler for now.
        #[non_exhaustive]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum Affinity
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/mod.rs
      checksum: 676b9ceadde62218c6c545e372f411e8824bf30eedfbb28c14b45facca832ac0
      content: |
        //! Index segments inside a subengine: slot geometry, immutable segments published as
        //! generation manifests, and deferred release behind leases and stream events.

        pub mod error;
        pub mod mapping;
        pub mod slots;
        pub mod sync;

        use std::sync::Arc;

        use arc_swap::ArcSwap;
        use parking_lot::Mutex;
        use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

        pub use error::SegmentError;
        pub use mapping::{EntryRef, Resolution, SegmentHit, SlotMap};
        pub use slots::{encode_slot, SlotMeta, SlotWrite, ValidBitmap};
        pub use sync::{Publisher, SegmentBus, SegmentEvent, SegmentEventKind};

        use crate::resident::failpoint;
        use crate::resident::kernel::{SegmentDesc, SegmentLayout};
        use crate::resident::memory::dtype::DType;
        use crate::resident::memory::region::Region;
        use crate::resident::op::{Lease, PinCount};
        use crate::resident::subengine::EventHandle;

        use slots::SlotWriter;

        /// Completion marker of device work that may still touch a segment.
        pub trait Fence: Send + Sync {
            fn is_complete(&self) -> bool;
        }

        impl Fence for EventHandle {
            fn is_complete(&self) -> bool {
                EventHandle::is_complete(self)
            }
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
      checksum: 2013098691a181d064c46869a5d6d73e17905eabc7fd45cfc0886688c37800a8
      content: |
        use super::error::TaskError;
        use super::params::{
            EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, VectorInput,
        };
        use crate::resident::queue::Queue;

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum TaskKind {
            Embedding,
            Similarity,
            Inference,
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct TaskMeta {
            pub id: u64,
            pub kind: TaskKind,
        }

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

        pub type TaskQueue = Queue<TaskMeta, Payload, Output, TaskError>;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
      checksum: eb13dc6ec541953ad7d09c800f7505b2a778fc87d7f440a17199883849b5635b
      content: |-
        pub enum Residency {
            NotLoaded,
            Loaded,
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub struct IndexInfo
```
/answer cc1b-22 allow
Text:
 Für die Abhängigkeitsrichtungen („Uses / Used by“) verifiziere ich die `use`-Beziehungen zwischen den Ebenen anhand des Codes.
Control Request:
```yaml
id: cc1b-23
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/index
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  reason: "Importbeziehungen von queue, memory, op, index, subengine, kernel, segment zu anderen resident-Modulen prüfen"
```
/answer cc1b-23 allow
ReasoningToken: 2709
Control Request:
```yaml
id: cc1b-24
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue
      matches:
      - path: queue.rs
        matches:
        - use crate::resident::failpoint;
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory
      matches:
      - path: cpu.rs
        matches:
        - use crate::resident::subengine::EventHandle;
      - path: arena.rs
        matches:
        - use crate::resident::error::IndexError;
        - use crate::resident::subengine::EventHandle;
        - "use crate::resident::memory::{ArenaId, Region};"
        - "use crate::resident::subengine::{CpuLane, EventHandle, ExecContext, LaneClass, LaneConfig};"
      - path: cuda.rs
        matches:
        - "use crate::resident::subengine::{DeviceHandle, EventHandle};"
      - path: error.rs
        matches:
        - use crate::resident::error::IsRetryable;
      - path: test/cuda.rs
        matches:
        - "use crate::resident::subengine::{LaneClass, LaneConfig};"
        - use crate::resident::manager::arena::IndexArena;
      - path: test/staging.rs
        matches:
        - use crate::resident::memory::arena::testing::Gate;
      - path: staging.rs
        matches:
        - use crate::resident::subengine::EventHandle;
      - path: manager.rs
        matches:
        - "use crate::resident::error::{IndexError, ManagerError};"
        - use crate::resident::failpoint;
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      matches:
      - path: mod.rs
        matches:
        - use crate::resident::error::ResidentError;
        - "pub source: Option<crate::resident::index::payload::Payload>,"
      - path: requirements.rs
        matches:
        - "use crate::resident::memory::{DeviceKind, Region};"
        - use crate::resident::precision::CapabilitySet;
      - path: test/mod.rs
        matches:
        - use crate::resident::error::PlannerError;
        - use crate::resident::memory::region::ArenaId;
        - use crate::resident::memory::Region;
      - path: test/lease.rs
        matches:
        - use crate::resident::memory::region::ArenaId;
      - path: lease.rs
        matches:
        - use crate::resident::memory::Region;
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/index
      matches:
      - path: parquet_io.rs
        matches:
        - use crate::resident::error::IndexError;
      - path: test/parquet_io.rs
        matches:
        - use crate::resident::memory::DType;
        - use crate::resident::model::ModelSpec;
        - "(\"embedding.pooling\", Box::new(|h| h.pooling = crate::resident::model::Pooling::PerToken)),"
        - "(\"embedding.normalize\", Box::new(|h| h.normalize = crate::resident::model::Normalize::None)),"
      - path: schema.rs
        matches:
        - use crate::resident::error::IndexError;
        - use crate::resident::memory::DType;
        - "use crate::resident::model::{ModelSpec, Normalize, Pooling};"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
      matches:
      - path: mod.rs
        matches:
        - use crate::resident::backend::ComputeBackend;
      - path: error.rs
        matches:
        - use crate::resident::error::IsRetryable;
      - path: test/device.rs
        matches:
        - "use crate::resident::memory::{ArenaId, Region};"
      - path: test/exec.rs
        matches:
        - use crate::resident::memory::ArenaId;
      - path: device.rs
        matches:
        - use crate::resident::backend::ComputeBackend;
        - "pub fn try_new() -> Result<Self, crate::resident::error::IndexError> {"
        - "Err(crate::resident::error::IndexError::TransferFailed {"
        - use crate::resident::memory::DeviceKind;
        - "use crate::resident::subengine::{"
        - "use crate::resident::subengine::{EventHandle, LaneJob, ParamBlock, ScratchArena};"
        - use crate::resident::subengine::exec::EventRepr;
      - path: exec.rs
        matches:
        - "use crate::resident::memory::{DeviceKind, Region};"
        - use crate::resident::subengine::DeviceError;
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
      matches:
      - path: mod.rs
        matches:
        - use crate::resident::error::IndexError;
        - use crate::resident::subengine::ExecContext;
      - path: maxsim.rs
        matches:
        - use crate::resident::subengine::ExecContext;
        - "dtype: crate::resident::memory::DType,"
        - use crate::resident::kernel::slots::check_range;
        - "use crate::resident::kernel::{"
        - "use crate::resident::memory::{DType, F16_MASK_VALUE};"
      - path: golden.rs
        matches:
        - use crate::resident::subengine::ExecContext;
      - path: cpu_simd.rs
        matches:
        - use crate::resident::memory::F16_MASK_VALUE;
      - path: launcher.rs
        matches:
        - "use crate::resident::memory::{DType, Region};"
        - "use crate::resident::subengine::{ExecContext, LaneClass, LaneJob};"
      - path: registry.rs
        matches:
        - use crate::resident::memory::DType;
        - "use crate::resident::subengine::{Resource, ResourceKey};"
      - path: cuda.rs
        matches:
        - use crate::resident::memory::DType;
      - path: error.rs
        matches:
        - "use crate::resident::error::{IndexError, IsRetryable};"
        - "impl From<crate::resident::subengine::DeviceError> for KernelError {"
        - "fn from(e: crate::resident::subengine::DeviceError) -> Self {"
        - use crate::resident::subengine::DeviceError;
      - path: test/maxsim.rs
        matches:
        - use crate::resident::kernel::InMemorySlots;
        - "use crate::resident::memory::{ArenaId, Region};"
        - "use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};"
        - use crate::resident::memory::DType;
      - path: test/launcher.rs
        matches:
        - use crate::resident::memory::ArenaId;
        - "use crate::resident::subengine::{LaneConfig, RecordingLane};"
      - path: test/cosine.rs
        matches:
        - use crate::resident::kernel::cpu_simd::normalize;
        - "use crate::resident::kernel::{score_ranges, InMemorySlots, Scorer};"
        - "use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};"
        - "use crate::resident::memory::{ArenaId, Region};"
      - path: test/cuda.rs
        matches:
        - "use crate::resident::kernel::cpu_simd::{dot, maxsim};"
        - use crate::resident::subengine::DeviceHandle;
        - crate::resident::memory::F16_MASK_VALUE
      - path: catalog.rs
        matches:
        - use crate::resident::memory::DType;
        - "use crate::resident::precision::known::{FP16, FP32, OPERANDS, SCORING};"
        - "use crate::resident::precision::{CapabilitySet, ValueId};"
        - use crate::resident::subengine::SubengineKind;
      - path: mock.rs
        matches:
        - use crate::resident::error::IndexError;
      - path: slice.rs
        matches:
        - use crate::resident::subengine::Ewma;
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
      matches:
      - path: mod.rs
        matches:
        - use crate::resident::failpoint;
        - "use crate::resident::kernel::{SegmentDesc, SegmentLayout};"
        - use crate::resident::memory::dtype::DType;
        - use crate::resident::memory::region::Region;
        - "use crate::resident::op::{Lease, PinCount};"
        - use crate::resident::subengine::EventHandle;
        - "pub fn hit(&self, kernel: crate::resident::kernel::KernelHit) -> Result<SegmentHit, SegmentError> {"
        - use crate::resident::memory::region::ArenaId;
      - path: slots.rs
        matches:
        - use crate::resident::index::EntryId;
        - "use crate::resident::memory::dtype::{DType, F16_MASK_VALUE};"
        - use crate::resident::memory::region::Region;
      - path: sync.rs
        matches:
        - "use crate::resident::subengine::{ExecContext, LaneJob};"
      - path: error.rs
        matches:
        - use crate::resident::error::IsRetryable;
        - use crate::resident::memory::dtype::DType;
        - use crate::resident::subengine::DeviceError;
      - path: test/sync.rs
        matches:
        - "use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};"
      - path: test/mapping.rs
        matches:
        - let done = crate::resident::segment::mock::ManualFence::new();
        - let store = crate::resident::segment::SegmentStore::new();
        - "let seg = crate::resident::segment::testutil::segment(&store, 1, 10, 0, 2);"
        - "let old_seg = crate::resident::segment::testutil::segment(&store, 1, 10, 0, 4);"
        - "let new_seg = crate::resident::segment::testutil::segment(&store, 1, 11, 1 << 16, 4);"
        - "let seg = crate::resident::segment::testutil::segment(&store, 1, 10, 0, 4);"
      - path: mapping.rs
        matches:
        - use crate::resident::index::EntryId;
        - "use crate::resident::kernel::{entry_of, entry_of_fixed};"
```
/answer cc1b-24 allow
Control Request:
```yaml
id: cc1b-25
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/model
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    - tests.rs
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    pattern: "crate::resident::[a-z_]+"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
    pattern: crate::resident::(memory|model|tokenize|planner|processor|facade|subengine|precision|op|queue|backend)
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
    pattern: crate::resident::(memory|model|tokenize|planner|processor|facade|subengine|kernel|op|queue|backend|manager|index|segment)
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 50
  reason: "Importbeziehungen von model, tokenize, planner, processor, facade, manager, precision prüfen"
```
/answer cc1b-25 allow
Control Request:
```yaml
id: cc1b-26
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/model
      matches:
      - path: mod.rs
        matches:
        - use crate::resident::memory::layout::Layout;
        - "use crate::resident::memory::{CpuMemoryManager, DType, DeviceKind, MemoryError, Region};"
        - "use crate::resident::subengine::{EventHandle, ExecContext, Resource, ResourceKey};"
      - path: encoder.rs
        matches:
        - "use crate::resident::memory::{DType, DeviceKind};"
        - use crate::resident::subengine::ExecContext;
      - path: error.rs
        matches:
        - use crate::resident::error::IsRetryable;
        - use crate::resident::memory::MemoryError;
        - use crate::resident::subengine::DeviceError;
      - path: test/mod.rs
        matches:
        - "use crate::resident::memory::{ArenaKind, ArenaSizes, DeviceMemory};"
        - "use crate::resident::subengine::{CpuLane, LaneClass, LaneConfig, RecordingLane};"
      - path: test/candle_adapter.rs
        matches:
        - use crate::resident::memory::layout::Layout;
        - "use crate::resident::memory::{"
        - "use crate::resident::model::{"
        - "use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};"
      - path: candle_adapter.rs
        matches:
        - use crate::resident::memory::DeviceKind;
      - path: slot.rs
        matches:
        - use crate::resident::memory::DType;
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize
      matches:
      - path: mod.rs
        matches:
        - "use crate::resident::model::{plan_batches, Batch, BatchConfig, ModelSpec};"
        - "use crate::resident::subengine::{Resource, ResourceKey};"
      - path: modes.rs
        matches:
        - use crate::resident::model::Prefixes;
      - path: error.rs
        matches:
        - use crate::resident::error::IsRetryable;
      - path: test/mod.rs
        matches:
        - use crate::resident::memory::DType;
        - "ModeScheme::Prefix(crate::resident::model::Prefixes {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner
      matches:
      - path: rules.rs
        matches:
        - use crate::resident::error::PlannerError;
        - use crate::resident::facade::TaskKind;
        - use crate::resident::op::ModelId;
        - use crate::resident::processor::StarvedSnapshot;
      - path: mod.rs
        matches:
        - use crate::resident::error::PlannerError;
        - use crate::resident::processor::StarvedSnapshot;
      - path: config.rs
        matches:
        - use crate::resident::error::PlannerError;
        - use crate::resident::facade::TextRole;
      - path: expand.rs
        matches:
        - use crate::resident::error::PlannerError;
        - "use crate::resident::facade::{"
        - "use crate::resident::op::{"
        - use crate::resident::processor::StarvedSnapshot;
      - path: test/mod.rs
        matches:
        - "use crate::resident::facade::{"
        - "use crate::resident::op::{EngineSelector, IndexId, ModelId, Op, OpKind, ResourceKind};"
        - use crate::resident::processor::StarvedSet;
      - path: test/adapter.rs
        matches:
        - "use crate::resident::facade::{"
        - use crate::resident::index::payload::Payload as SourceRef;
        - "use crate::resident::op::{IndexId, OpKind, ScoredHit, ScoredHits};"
        - use crate::resident::planner::RuleSet;
        - "use crate::resident::processor::{"
        - "use crate::resident::queue::{Queue, TaskState};"
        - use crate::resident::memory::arena::DeviceMemory;
        - use crate::resident::memory::cuda::CudaMemoryManager;
        - "use crate::resident::memory::{ArenaKind, ArenaSizes};"
        - use crate::resident::subengine::DeviceHandle;
        - "input: crate::resident::facade::PairInput::new(\"q\", \"c\"),"
        - "params: InferenceParams::new().model(crate::resident::op::ModelId(\"nope\".into())),"
      - path: adapter.rs
        matches:
        - "use crate::resident::error::{PlannerError, ResidentError};"
        - "use crate::resident::facade::{"
        - use crate::resident::index::payload::Payload as SourceRef;
        - use crate::resident::index::EntryId;
        - use crate::resident::manager::slot::Hit;
        - "use crate::resident::op::{"
        - "use crate::resident::processor::{Collect, Expander, Expansion, StarvedSnapshot};"
      - path: offers.rs
        matches:
        - use crate::resident::error::PlannerError;
        - "use crate::resident::op::{EngineSelector, ResourceKind};"
        - use crate::resident::processor::StarvedSnapshot;
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
      matches:
      - path: worker.rs
        matches:
        - use crate::resident::facade::TaskQueue;
      - path: resource.rs
        matches:
        - use crate::resident::op::PinCount;
      - path: hub.rs
        matches:
        - use crate::resident::error::ResidentError;
        - "use crate::resident::facade::{Payload, TaskError, TaskMeta, TaskQueue};"
        - use crate::resident::memory::Region;
        - "use crate::resident::op::{OpParams, OperationNode, ResourceKind, TaskCtx, Value};"
        - use crate::resident::queue::ClaimState;
        - use crate::resident::telemetry;
      - path: error.rs
        matches:
        - "use crate::resident::error::{IsRetryable, ResidentError};"
        - "use crate::resident::op::{EngineSelector, ResourceKind};"
      - path: test/resource.rs
        matches:
        - "use crate::resident::memory::{ArenaId, Region};"
        - use crate::resident::op::Lease;
      - path: test/error.rs
        matches:
        - use crate::resident::error::IndexError;
      - path: test/ops.rs
        matches:
        - use crate::resident::op::ResourceKind;
        - "run.fail(crate::resident::error::PlannerError::UnknownModel { model: \"m\".into() }.into());"
        - "run.fail(crate::resident::error::PlannerError::UnknownModel { model: \"a\".into() }.into());"
        - "run.fail(crate::resident::error::PlannerError::UnknownModel { model: \"b\".into() }.into());"
      - path: ops.rs
        matches:
        - use crate::resident::error::ResidentError;
        - "use crate::resident::facade::{Output, TaskError};"
        - "use crate::resident::op::{EngineSelector, OpParams, OperationNode, TaskCtx};"
        - "use crate::resident::queue::{Queue, TaskState};"
        - "use crate::resident::op::{Op, OpDesc, OpKind, Requirements, ResourceKind, Value, HostData};"
      - path: starved.rs
        matches:
        - use crate::resident::op::ResourceKind;
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      matches:
      - path: mod.rs
        matches:
        - pub use crate::resident::processor::WorkerConfig;
      - path: resident.rs
        matches:
        - "use crate::resident::error::{ConfigError, PlannerError, ResidentError};"
        - use crate::resident::index::Index;
        - "use crate::resident::op::{IndexId, ModelId, TaskCtx};"
        - "use crate::resident::processor::{"
        - "use crate::resident::queue::{Queue, TaskState};"
        - use crate::resident::telemetry::task_span;
      - path: params.rs
        matches:
        - "use crate::resident::index::{EntryId, Scope};"
        - "use crate::resident::op::{IndexId, ModelId};"
      - path: report.rs
        matches:
        - use crate::resident::index::payload::Payload as SourceRef;
        - use crate::resident::index::EntryId;
        - use crate::resident::manager::slot::Hit;
      - path: error.rs
        matches:
        - use crate::resident::error::ResidentError;
      - path: test/resident.rs
        matches:
        - use crate::resident::facade::Output;
        - let blocking = crate::resident::facade::ResidentBlocking::new(r).unwrap();
        - let id = r.add_index(Arc::new(crate::resident::index::memory::InMemoryIndex::new()));
      - path: test/report.rs
        matches:
        - use crate::resident::index::payload::Payload as SourceRef;
        - "use crate::resident::index::{EntryId, Scope};"
        - use crate::resident::manager::slot::Hit;
        - "use crate::resident::op::{IndexId, ModelId};"
        - use crate::resident::facade::ScorerKind;
      - path: test/addon.rs
        matches:
        - "use crate::resident::index::{EntryId, EntrySet, Scope};"
        - "use crate::resident::op::{IndexId, ModelId};"
        - use crate::resident::processor::WorkerConfig;
      - path: task.rs
        matches:
        - use crate::resident::queue::Queue;
      - path: handle.rs
        matches:
        - "use crate::resident::queue::{ClaimState, TaskState};"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      matches:
      - path: doorbell.rs
        matches:
        - "use crate::resident::subengine::{DeviceError, EventHandle};"
        - "use crate::resident::subengine::{CpuLane, ExecContext, LaneClass, LaneConfig};"
      - path: upload.rs
        matches:
        - use crate::resident::memory::error::MemoryError;
        - use crate::resident::memory::region::Region;
        - use crate::resident::memory::StagingRing;
        - use crate::resident::subengine::EventHandle;
        - use crate::resident::memory::error::TransferDir;
        - use crate::resident::memory::cuda::CudaMemoryManager;
        - "use crate::resident::subengine::{CudaLane, ExecContext};"
      - path: swap.rs
        matches:
        - use crate::resident::memory::Region;
        - "use crate::resident::memory::{ArenaAllocator, ArenaKind, DeviceKind};"
        - "arena: crate::resident::memory::ArenaId(0),"
        - use crate::resident::subengine::DeviceHandle;
      - path: transfer.rs
        matches:
        - "use crate::resident::memory::error::{MemoryError, TransferDir};"
        - use crate::resident::memory::Region;
        - "impl MappedHost for crate::resident::memory::cuda::CudaMemoryManager {"
        - "use crate::resident::memory::{budget::ArenaKind, DeviceMemory};"
        - "crate::resident::memory::DeviceMemory::free(self, region)"
      - path: arena.rs
        matches:
        - "use crate::resident::memory::{ArenaAllocator, MemoryError, Region};"
        - use crate::resident::subengine::EventHandle;
      - path: residency.rs
        matches:
        - "use crate::resident::memory::error::{MemoryError, TransferDir};"
        - use crate::resident::precision::known::DENSITY;
        - "use crate::resident::precision::{ProfileConfig, ValueId};"
      - path: test/upload.rs
        matches:
        - use crate::resident::memory::region::ArenaId;
        - "use crate::resident::subengine::{CpuLane, ExecContext, LaneClass, LaneConfig};"
        - use crate::resident::memory::arena::DeviceMemory;
        - use crate::resident::memory::cuda::CudaMemoryManager;
        - "use crate::resident::memory::{ArenaKind, ArenaSizes};"
        - "use crate::resident::subengine::{CudaLane, DeviceHandle};"
      - path: test/transfer.rs
        matches:
        - use crate::resident::backend::ComputeBackend;
        - use crate::resident::memory::region::ArenaId;
        - "use crate::resident::memory::{ArenaAllocator, ArenaKind, DeviceKind, StagingRing};"
        - "use crate::resident::subengine::{CpuLane, ExecContext, LaneClass, LaneConfig};"
        - use crate::resident::subengine::EventHandle;
        - use crate::resident::memory::cuda::CudaMemoryManager;
        - use crate::resident::memory::arena::DeviceMemory;
        - use crate::resident::memory::ArenaSizes;
        - "use crate::resident::subengine::{CudaLane, DeviceHandle};"
      - path: test/arena.rs
        matches:
        - use crate::resident::memory::arena::testing::Gate;
        - "use crate::resident::memory::{ArenaKind, DeviceKind};"
      - path: test/build.rs
        matches:
        - "use crate::resident::memory::{ArenaKind, DeviceKind};"
        - use crate::resident::memory::ArenaAllocator;
        - use crate::resident::subengine::DeviceHandle;
      - path: test/runtime.rs
        matches:
        - use crate::resident::backend::ComputeBackend;
        - "use crate::resident::memory::error::{MemoryError, TransferDir};"
        - use crate::resident::memory::region::ArenaId;
        - "use crate::resident::memory::{"
        - "use crate::resident::subengine::{"
      - path: build.rs
        matches:
        - use crate::resident::memory::Region;
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      matches:
      - path: probe.rs
        matches:
        - use crate::resident::kernel::catalog::implemented_capabilities;
        - use crate::resident::memory::DeviceKind;
        - "use crate::resident::subengine::{DeviceHandle, SubengineKind};"
      - path: probe_cpu.rs
        matches:
        - use crate::resident::subengine::SubengineKind;
      - path: measure_cpu.rs
        matches:
        - use crate::resident::kernel::cpu_simd::dot;
      - path: measure_cuda.rs
        matches:
        - "use crate::resident::kernel::cuda::{launch_cosine, CudaKernels, COSINE_DOT};"
        - use crate::resident::memory::DType;
        - use crate::resident::subengine::DeviceHandle;
      - path: store.rs
        matches:
        - "use crate::resident::subengine::{DeviceHandle, SubengineKind};"
      - path: test/probe.rs
        matches:
        - use crate::resident::kernel::catalog::implemented_capabilities;
      - path: test/measure_cuda.rs
        matches:
        - use crate::resident::subengine::SubengineKind;
      - path: probe_cuda.rs
        matches:
        - "use crate::resident::subengine::{DeviceHandle, SubengineKind};"
      - path: measure.rs
        matches:
        - use crate::resident::subengine::DeviceHandle;
        - use crate::resident::memory::DeviceKind;
      - path: calibrate.rs
        matches:
        - use crate::resident::subengine::SubengineKind;
      - path: workload.rs
        matches:
        - "use crate::resident::kernel::cpu_simd::{dot, normalize};"
```
/answer cc1b-26 allow
Text:
 Noch klären: Wer implementiert die Engine-/Subengine-Traits konkret, und wie ist das Test-Layout?
Control Request:
```yaml
id: cc1b-27
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: "impl(<[^>]*>)? (Engine|Subengine|Expander|Index|IndexManager|Kernel|Backbone|Completion|MemoryManager|DeviceMemory) for"
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    - '**/tests.rs'
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: resident
    include:
    - lib.rs
    - main.rs
    exclude:
    - target/**
    limit: 10
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/test
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/test
    pattern: ^//!
    include:
    - '*.rs'
    limit: 20
  reason: Implementierungen der zentralen Traits lokalisieren und Wurzel-Testordner prüfen
```
/answer cc1b-27 allow
ReasoningToken: 205
Control Request:
```yaml
id: cc1b-28
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/processor/tests.rs
        matches:
        - "impl Expander for TestExpander {"
        - "impl Engine for TestEngine {"
        - "impl Expander for GpuExpander {"
      - path: resident/processor/completion_cuda.rs
        matches:
        - "impl Completion for PolledCompletion {"
        - "impl Completion for HostFnCompletion {"
      - path: resident/processor/completion.rs
        matches:
        - "impl Completion for ManualCompletion {"
      - path: resident/facade/resident.rs
        matches:
        - "impl Expander for Unplanned {"
        - "impl Engine for NoEngine {"
      - path: resident/index/parquet_io.rs
        matches:
        - "impl Index for ParquetIndex {"
      - path: resident/index/memory.rs
        matches:
        - "impl Index for InMemoryIndex {"
      - path: resident/kernel/mock.rs
        matches:
        - "impl Kernel for MockKernel {"
      - path: resident/manager/test/flat.rs
        matches:
        - "impl Index for FlatIndex {"
      - path: resident/manager/test/sync.rs
        matches:
        - "impl Index for Probe {"
      - path: resident/manager/test/scope.rs
        matches:
        - "impl Index for FakeIndex {"
      - path: resident/manager/streamed.rs
        matches:
        - "impl<B: ComputeBackend + ?Sized> IndexManager for StreamedIndexManager<B> {"
        - "impl<B: ComputeBackend + ?Sized> IndexManager for CpuIndexManager<B> {"
        - "impl<B: ComputeBackend + ?Sized> IndexManager for GpuIndexManager<B> {"
      - path: resident/model/test/mod.rs
        matches:
        - "impl Backbone for FakeBackbone {"
        - "impl Backbone for Failing {"
        - "impl Backbone for ZeroBackbone {"
      - path: resident/model/candle_adapter.rs
        matches:
        - "impl Backbone for CandleBert {"
      - path: resident/memory/cpu.rs
        matches:
        - "impl DeviceMemory for CpuMemoryManager {"
      - path: resident/memory/arena.rs
        matches:
        - "impl MemoryManager for ArenaAllocator {"
      - path: resident/memory/cuda.rs
        matches:
        - "impl DeviceMemory for CudaMemoryManager {"
      - path: resident/memory/manager.rs
        matches:
        - "impl MemoryManager for BookkeepingArena {"
      - path: resident/planner/test/adapter.rs
        matches:
        - "impl Engine for Scripted {"
      - path: resident/planner/adapter.rs
        matches:
        - "impl Expander for PlannerExpander {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: src/lib.rs
        matches:
        - pub mod resident;
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/test
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/test
      matches:
      - path: flat.rs
        matches:
        - "//! Index with contiguous backing storage, like a mapped file."
```
/answer cc1b-28 allow
Control Request:
```yaml
id: cc1b-29
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/test
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/test
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test
    recursive: false
  reason: "Test-Layout prüfen, um es korrekt in den READMEs zu beschreiben"
```
/answer cc1b-29 allow
Control Request:
```yaml
id: cc1b-2a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test
      entries:
      - ".:"
      - error.rs
      - telemetry.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/test
      entries:
      - ".:"
      - advice.rs
      - arena.rs
      - blocking.rs
      - build.rs
      - evict.rs
      - flat.rs
      - flight.rs
      - residency.rs
      - runtime.rs
      - scope.rs
      - slot.rs
      - streamed.rs
      - sync.rs
      - tracker.rs
      - transfer.rs
      - upload.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/test
      entries:
      - ".:"
      - mapping.rs
      - mod.rs
      - slots.rs
      - sync.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test
      entries:
      - ".:"
      - axis.rs
      - calibrate.rs
      - capability.rs
      - known.rs
      - matching.rs
      - measure_cpu.rs
      - measure_cuda.rs
      - metrics.rs
      - pareto.rs
      - pool.rs
      - probe.rs
      - probe_cpu.rs
      - probe_cuda.rs
      - profile.rs
      - stats.rs
      - store.rs
      - workload.rs
```
/answer cc1b-2a allow
Thinking:
Im Produktivcode gibt es keine konkrete `Engine`-/`Subengine`-Implementierung – das halte ich in der Doku fest. Jetzt lese ich noch die kleinen Planner-Dateien und Kerntypen, um sie genau zu beschreiben.


Control Request:
```yaml
id: cc1b-2b
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/offers.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/adapter.rs
    max_line: 90
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/config.rs
    max_line: 80
  reason: "Planner-Interna (Regeln, Offers, Adapter, Pipeline-Config) für eine korrekte Beschreibung lesen"
```
/answer cc1b-2b allow
ReasoningToken: 656
Control Request:
```yaml
id: cc1b-2c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/rules.rs
      checksum: 403d2f5b0ac0c3e278ffc4ac5254b9662dc56cb3d066f84baecf1d73f4f1de6c
      content: |
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
                    expanders
                        .insert(
                            (model.clone(), TaskKind::Embedding),
                            Box::new(EmbeddingExpander),
                        );
                    expanders
                        .insert(
                            (model.clone(), TaskKind::Similarity),
                            Box::new(SimilarityExpander),
                        );
                    expanders
                        .insert(
                            (model.clone(), TaskKind::Inference),
                            Box::new(InferenceExpander),
                        );
                }
                Ok(Self {
                    default_model,
                    pipelines,
                    expanders,
                })
            }
            pub fn from_toml(src: &str) -> Result<Self, PlannerError> {
                let raw: RawRules = toml::from_str(src)
                    .map_err(|e| PlannerError::InvalidConfig {
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
                    return Err(PlannerError::UnknownModel {
                        model: model.0,
                    });
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
                expander
                    .expand(
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/offers.rs
      checksum: 0f93f7b8e0af913b9bed863fc75c818012a449dcfb073b625e2c22e43aa0dc6a
      content: |
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/adapter.rs
      checksum: b5b2a107f21f154bd79100882185c467886c3452aa13c6b2b96f6c18a7d29a0f
      content: |
        //! Hub adapter: plans a task once and hands the chain over one operation at a time.
        use std::collections::HashMap;
        use std::sync::Arc;
        use parking_lot::Mutex;
        use crate::resident::error::{PlannerError, ResidentError};
        use crate::resident::facade::{
            Output, Payload, SimilarityParams, SimilarityReport, TaskError, TaskKind, TaskMeta,
        };
        use crate::resident::index::payload::Payload as SourceRef;
        use crate::resident::index::EntryId;
        use crate::resident::manager::slot::Hit;
        use crate::resident::op::{
            HostData, OperationNode, ResourceKind, ScoredHits, Successor, TaskCtx, Value,
        };
        use crate::resident::processor::{Collect, Expander, Expansion, StarvedSnapshot};
        use super::expand::{first_resource, PlannedOp, TaskSpec};
        use super::{EngineOffers, Planner};
        pub struct PlannerExpander {
            planner: Arc<Planner>,
            offers: EngineOffers,
        }
        impl PlannerExpander {
            pub fn new(planner: Arc<Planner>, offers: EngineOffers) -> Self {
                Self { planner, offers }
            }
        }
        /// Planned operations plus the host result of the last one.
        struct Chain {
            ops: Vec<PlannedOp>,
            result: Mutex<Option<HostData>>,
        }
        struct Step {
            chain: Arc<Chain>,
            index: usize,
        }
        impl Chain {
            fn node(
                self: &Arc<Self>,
                index: usize,
                input: Value,
                task: &Arc<TaskCtx>,
            ) -> Arc<OperationNode> {
                let planned = &self.ops[index];
                let node = OperationNode::new(
                    Arc::new(planned.desc.clone()),
                    input,
                    planned.materialize,
                    task.clone(),
                );
                Arc::new(node.with_successor(Arc::new(Step { chain: self.clone(), index })))
            }
        }
        impl Successor for Step {
            fn follow(&self, output: Value, task: &Arc<TaskCtx>) -> Vec<Arc<OperationNode>> {
                let next = self.index + 1;
                if next < self.chain.ops.len() {
                    return vec![self.chain.node(next, output, task)];
                }
                if let Value::Host(data) = output {
                    *self.chain.result.lock() = Some(data);
                }
                Vec::new()
            }
        }
        fn invalid(reason: impl Into<String>) -> TaskError {
            ResidentError::from(PlannerError::InvalidConfig {
                    reason: reason.into(),
                })
                .into()
        }
        fn report_of(
            task_id: u64,
            params: &SimilarityParams,
            scored: ScoredHits,
        ) -> SimilarityReport {
            let sources: HashMap<EntryId, SourceRef> = scored
                .hits
                .iter()
                .filter_map(|h| Some((EntryId::new(h.entry), h.source.clone()?)))
                .collect();
            let hits = scored
                .hits
                .iter()
                .map(|h| Hit {
                    entry: EntryId::new(h.entry),
                    score: h.score,
                    outdated: h.outdated,
                })
                .collect();
            SimilarityReport::build(
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/config.rs
      error: The requested range selects more than 70% of the file. Read the whole file instead (omit the range parameters) and rely on the checksum-based conditional read to detect unchanged content.
```
/answer cc1b-2c allow
Control Request:
```yaml
id: cc1b-2d
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model/mod.rs
    max_line: 140
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/streamed.rs
    max_line: 70
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
    start: pub enum Output
    end: zzzz_no_end
  reason: "Modell-Spec/Registry, IndexManager-Trait und Output-Typ der Fassade lesen"
```
/answer cc1b-2d allow
Control Request:
```yaml
id: cc1b-2e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model/mod.rs
      checksum: dc09c3ca07237f902acebec7c6c2e2521ea53ab1c0a08e9436da5a80ae10e756
      content: |
        //! Model resources: specs, registry, encoder contract and loaded-encoder resources.
        mod batch;
        #[cfg(feature = "candle")]
        pub(crate) mod candle_adapter;
        mod encoder;
        mod error;
        mod head;
        mod pooling;
        mod slot;
        use std::collections::HashMap;
        use std::path::{Path, PathBuf};
        use std::sync::Arc;
        use parking_lot::Mutex;
        pub use batch::{plan_batches, Batch, BatchConfig, BUCKETS};
        pub use encoder::{Backbone, PooledEncoder};
        pub use error::ModelError;
        pub use head::ProjectionHead;
        pub use pooling::{l2_normalize_rows, mean_pool};
        pub use slot::{encode_bytes, write_slot};
        use crate::resident::memory::layout::Layout;
        use crate::resident::memory::{CpuMemoryManager, DType, DeviceKind, MemoryError, Region};
        use crate::resident::subengine::{EventHandle, ExecContext, Resource, ResourceKey};
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Pooling {
            /// One vector per sequence: mean over unmasked tokens.
            MeanMasked,
            /// One vector per token.
            PerToken,
        }
        /// Scaling applied to every produced vector.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Normalize {
            /// Unit Euclidean length; a dot product then equals the cosine.
            L2,
            None,
        }
        /// Backbone element type per device class; independent of the storage dtype.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct ComputeDtypes {
            pub cpu: DType,
            pub gpu: DType,
        }
        impl ComputeDtypes {
            pub fn for_device(&self, device: DeviceKind) -> DType {
                match device {
                    DeviceKind::Cpu => self.cpu,
                    DeviceKind::Gpu => self.gpu,
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct Prefixes {
            pub query: String,
            pub passage: String,
        }
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct HeadSpec {
            pub weight_name: String,
            pub in_dim: usize,
            pub out_dim: usize,
            pub bias: bool,
        }
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct ModelPaths {
            pub config: PathBuf,
            pub weights: Vec<PathBuf>,
        }
        /// Plain description of a model; owns no resources.
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct ModelSpec {
            pub model_id: String,
            pub arch_id: String,
            pub dim: usize,
            pub pooling: Pooling,
            pub normalize: Normalize,
            pub prefixes: Prefixes,
            /// Element type of the produced embeddings (storage).
            pub dtype: DType,
            /// Element type of the backbone forward pass per device class.
            pub compute: ComputeDtypes,
            pub max_seq_len: usize,
            pub head: Option<HeadSpec>,
            pub paths: ModelPaths,
        }
        impl ModelSpec {
            /// `intfloat/multilingual-e5-small` checkout in `dir`.
            pub fn e5_small(dir: &Path, dtype: DType) -> Self {
                Self {
                    model_id: "intfloat/multilingual-e5-small".into(),
                    arch_id: "bert".into(),
                    normalize: Normalize::L2,
                    compute: ComputeDtypes {
                        cpu: DType::F32,
                        gpu: DType::F32,
                    },
                    dim: 384,
                    pooling: Pooling::MeanMasked,
                    prefixes: Prefixes {
                        query: "query: ".into(),
                        passage: "passage: ".into(),
                    },
                    dtype,
                    max_seq_len: 512,
                    head: None,
                    paths: ModelPaths {
                        config: dir.join("config.json"),
                        weights: vec![dir.join("model.safetensors")],
                    },
                }
            }
            /// Rejects configurations that would silently degrade embeddings.
            pub fn validate(&self) -> Result<(), ModelError> {
                let invalid = |reason: String| Err(ModelError::Incompatible { reason });
                if self.dim == 0 || self.max_seq_len == 0 {
                    return invalid(
                        format!(
                            "dim ({}) and max_seq_len ({}) must be positive", self.dim, self
                            .max_seq_len
                        ),
                    );
                }
                for (name, prefix) in [
                    ("query", &self.prefixes.query),
                    ("passage", &self.prefixes.passage),
                ] {
                    if !prefix.is_empty() && !prefix.ends_with(' ') {
                        return invalid(
                            format!("{name} prefix `{prefix}` must end with a space"),
                        );
                    }
                }
                Ok(())
            }
        }
        /// A region together with its layout; the only way tensors cross the encoder API.
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct TensorRef {
            pub region: Region,
            pub layout: Layout,
        }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/streamed.rs
      checksum: 941b990253a33819549781f10435d8b081d6e21cc071f28a24254dcaf532431c
      content: |
        //! Index manager skeleton and its CPU and GPU variants. The index is attached lazily; until then
        //! no resource is held and no subscription exists.
        use std::collections::BTreeMap;
        use std::future::Future;
        use std::ops::Range;
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;
        use std::time::Instant;
        use parking_lot::Mutex;
        use tokio::runtime::Handle;
        use tokio::sync::OnceCell;
        use tracing::Instrument;
        use crate::resident::backend::ComputeBackend;
        use crate::resident::error::{IndexError, ManagerError};
        use crate::resident::index::{EntryId, EntrySet, Index, Scope};
        use crate::resident::kernel::{KernelHit, WrittenRange};
        use crate::resident::manager::advice::{AdviceSink, Hint};
        use crate::resident::manager::blocking::BlockingReads;
        use crate::resident::manager::evict::{EvictPolicy, Eviction, Evictor, Occupancy, Tier};
        use crate::resident::manager::residency::{Access, LoadOutcome, ResidencyLoader};
        use crate::resident::manager::scope::{ResolveCtx, ResolverRegistry};
        use crate::resident::manager::slot::{
            run_slot_at, Hit, MappingStore, RangeSource, UpdateQueue,
        };
        use crate::resident::manager::slot::Update;
        use crate::resident::manager::swap::{Snapshot, TableSwap};
        use crate::resident::manager::transfer::{LoadedSegment, RegionSource};
        use crate::resident::manager::sync::{ScanOutcome, SyncScanner};
        use crate::resident::manager::tracker::{ResidencyTracker, SlotRef};
        use crate::resident::memory::Region;
        pub type SharedIndex = Arc<dyn Index + Send + Sync>;
        /// Interface shared by the CPU and GPU variants.
        pub trait IndexManager: Send + Sync {
            /// `false` when an index is already attached; the first one stays.
            fn attach_index(&self, index: SharedIndex) -> bool;
            fn is_active(&self) -> bool;
        }
        pub struct StreamedIndexManager<B: ?Sized> {
            engine: Arc<B>,
            index_id: String,
            index: OnceCell<SharedIndex>,
            residency: Mutex<ResidencyTracker>,
        }
        impl<B: ComputeBackend + ?Sized> StreamedIndexManager<B> {
            pub fn new(engine: Arc<B>, index_id: impl Into<String>) -> Self {
                Self {
                    engine,
                    index_id: index_id.into(),
                    index: OnceCell::new(),
                    residency: Mutex::new(ResidencyTracker::new()),
                }
            }
            pub fn engine(&self) -> &B {
                &self.engine
            }
            pub fn index_id(&self) -> &str {
                &self.index_id
            }
            pub fn index(&self) -> Result<&SharedIndex, IndexError> {
                self.index
                    .get()
                    .ok_or_else(|| IndexError::IndexUnavailable {
                        scope: self.index_id.clone(),
                    })
            }
            /// Runs `init` on first use only; concurrent callers wait for the same initialization and a
            /// failed one leaves the manager inactive.
            pub async fn activate<F, Fut>(&self, init: F) -> Result<&SharedIndex, IndexError>
            where
                F: FnOnce() -> Fut,
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
      error: End marker not found in file.
```
/answer cc1b-2e allow
Control Request:
```yaml
id: cc1b-2f
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/config.rs
  reason: Planner-Pipeline-Konfiguration vollständig lesen
```
/answer cc1b-2f allow
ReasoningToken: 99
Control Request:
```yaml
id: cc1b-30
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
  name: Output
  reason: Definition des Ergebnistyps der Fassade nachsehen
```
/answer cc1b-30 allow
Control Request:
```yaml
id: cc1b-31
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
      nodes:
      - id: Output
        code: |
          #[derive(Clone, Debug, PartialEq)]
          #[non_exhaustive]
          pub enum Output {
              Embeddings(Vec<Vec<f32>>),
              Hits(Vec<(EntryId, f32)>),
              Similarity(SimilarityReport),
              Scores(Vec<f32>),
          }
```
/answer cc1b-31 allow
ReasoningToken: 2126
Control Request:
```yaml
id: cc1b-32
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project
    pattern: resident
    recursive: true
  - path: /home/user/xyan/xy.ai.workbench/docs
    pattern: resident
    recursive: true
  reason: "Prüfen, wo resident.md (Basiskonzept) und weitere Konzeptdokumente liegen, damit Verweise korrekt sind"
```
/answer cc1b-32 deny vermeide Verweise wo möglich, diese können schnell veralten. Das sollte kein Problem sein, da jede README nur so gering übergreifend wie möglich sein soll.
Text:
 Verstanden – ich verzichte auf Verweise auf die Projektdokumente und halte Querverweise minimal. Noch kurz die Feature-Gates prüfen (damit die „Features“-Angaben stimmen).
ReasoningToken: 1514
Control Request:
```yaml
id: cc1b-33
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "cfg\\((any\\()?(test, )?feature = \"(cuda|candle|mock)\"[^\\]]*\\)\\]\\s*$"
    include:
    - mod.rs
    - backend.rs
    - completion_cuda.rs
    exclude:
    - '**/test/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: ^(pub(\(crate\))? )?mod (cuda|candle_adapter|completion_cuda|measure_cuda|probe_cuda|wrap_gate|mock|golden)\b
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    limit: 20
  reason: Feature-Gating der cuda-/candle-/mock-abhängigen Module verifizieren
```
/answer cc1b-33 allow
Control Request:
```yaml
id: cc1b-34
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: mod.rs
        matches:
        - "#[cfg(any(test, feature = \"mock\"))]"
      - path: segment/mod.rs
        matches:
        - "#[cfg(any(test, feature = \"mock\"))]"
      - path: precision/mod.rs
        matches:
        - "#[cfg(any(test, feature = \"mock\"))]"
      - path: kernel/mod.rs
        matches:
        - "#[cfg(any(test, feature = \"mock\"))]"
      - path: subengine/mod.rs
        matches:
        - "#[cfg(any(test, feature = \"mock\"))]"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: mod.rs
        matches:
        - pub mod mock;
      - path: segment/mod.rs
        matches:
        - "pub mod mock {"
      - path: processor/mod.rs
        matches:
        - mod completion_cuda;
      - path: precision/mod.rs
        matches:
        - mod measure_cuda;
        - mod probe_cuda;
      - path: kernel/mod.rs
        matches:
        - pub mod cuda;
        - pub mod golden;
        - pub mod mock;
      - path: model/mod.rs
        matches:
        - pub(crate) mod candle_adapter;
      - path: memory/mod.rs
        matches:
        - pub mod cuda;
        - mod wrap_gate;
      - path: memory/dtype.rs
        matches:
        - "pub(crate) mod candle_adapter {"
```
/answer cc1b-34 allow
Control Request:
```yaml
id: cc1b-35
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "#\\[cfg\\(.*feature = \"(cuda|candle|mock)\""
    include:
    - mod.rs
    - backend.rs
    exclude:
    - '**/test/**'
    intent: survey
    limit: 50
  reason: Alle feature-gegateten Stellen in mod.rs/backend.rs auflisten
```
/answer cc1b-35 allow
ReasoningToken: 138
Control Request:
```yaml
id: cc1b-36
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: mod.rs
        matches:
        - "#[cfg(any(test, feature = \"mock\"))]"
      - path: segment/mod.rs
        matches:
        - "#[cfg(any(test, feature = \"mock\"))]"
      - path: processor/mod.rs
        matches:
        - "#[cfg(feature = \"cuda\")]"
      - path: precision/mod.rs
        matches:
        - "#[cfg(feature = \"cuda\")]"
        - "#[cfg(any(test, feature = \"mock\"))]"
      - path: kernel/mod.rs
        matches:
        - "#[cfg(any(test, feature = \"mock\"))]"
      - path: subengine/mod.rs
        matches:
        - "#[cfg(any(test, feature = \"mock\"))]"
      - path: model/mod.rs
        matches:
        - "#[cfg(feature = \"candle\")]"
      - path: backend.rs
        matches:
        - "//! Compute backend abstraction and the single `#[cfg(feature = \"cuda\")]` decision point."
      - path: memory/mod.rs
        matches:
        - "#[cfg(feature = \"cuda\")]"
        - "#[cfg(all(test, feature = \"cuda\"))]"
```
/answer cc1b-36 allow
Control Request:
```yaml
id: cc1b-37
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/backend.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/mod.rs
    max_line: 40
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
    max_line: 40
  reason: Backend-Auswahl und Feature-Gates von memory/kernel exakt nachlesen
```
/answer cc1b-37 allow
Control Request:
```yaml
id: cc1b-38
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/backend.rs
      checksum: 31227fdc9138ff518abc2cf1aea0151660f359e5469f63d9192d097a1792847c
      content: |
        //! Compute backend abstraction and the single `#[cfg(feature = "cuda")]` decision point.
        //!
        //! Everything outside `create_backend` runs against the `ComputeBackend` trait and stays
        //! feature-independent; only this factory consults the feature; the CUDA adapter modules (`kernel::cuda`,
        //! `subengine::device`) always compile and gate any toolkit-specific code internally.

        use crate::resident::subengine::CpuBackend;
        use crate::resident::subengine::CudaBackend;

        /// Backend abstraction the orchestration layer runs against.
        pub trait ComputeBackend: Send + Sync {
            fn run_batch(&self, input: &[f32]) -> Vec<f32>;
        }

        /// Constructs the active backend: CPU without `cuda`; with `cuda`, attempts the CUDA backend
        /// and falls back to CPU if GPU initialization fails.
        pub fn create_backend() -> Box<dyn ComputeBackend> {
            if cfg!(feature = "cuda") {
                if let Ok(backend) = CudaBackend::try_new() {
                    return Box::new(backend);
                }
            }
            Box::new(CpuBackend)
        }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      checksum: 5d9852afebc6991bcfe8ae0af7182a36527940ab212e0d504716674632d9d4bb
      content: |
        //! Kernel abstraction: segment descriptors, hits, the `Kernel` trait and the scorers built on it.
        //!
        //! The kernel knows only segments, offsets and spans. Mapping an address back to an entry is the
        //! manager's job (`entry_of`, `entry_of_fixed`); the public API carries no pointers.
        pub mod catalog;
        pub mod cosine;
        pub mod cpu_simd;
        pub mod cuda;
        pub mod error;
        pub mod golden;
        pub mod launcher;
        pub mod maxsim;
        pub mod registry;
        pub mod slice;
        pub mod slots;
        pub mod topk;
        pub use error::KernelError;
        pub use slice::{run_sliced, SliceOutcome, SliceSizer, StopReason};
        pub use slots::InMemorySlots;
        pub use topk::TopK;
        #[cfg(any(test, feature = "mock"))]
        pub mod mock;
        use std::time::Duration;
        use crate::resident::error::IndexError;
        use crate::resident::subengine::ExecContext;
        /// Address-agnostic arena span; pointer resolution happens only inside a backend.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct RegionSpan {
            pub offset: u64,
            pub len: u64,
        }
        /// Offset table (`n + 1` prefix sums) living in an arena span.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct OffsetRef(pub RegionSpan);
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum SegmentLayout {
            Fixed { stride: u32 },
            Variable { offsets: OffsetRef },
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/mod.rs
      error: The requested range selects more than 70% of the file. Read the whole file instead (omit the range parameters) and rely on the checksum-based conditional read to detect unchanged content.
```
/answer cc1b-38 allow
Text:
 Zwei Unklarheiten vor dem Schreiben: wie `segment` und `manager` zusammenhängen, und die Init-Semantik der Fassade.
Control Request:
```yaml
id: cc1b-39
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: resident::segment
    include:
    - '*.rs'
    exclude:
    - '**/test/**'
    - segment/**
    limit: 20
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    pattern: Starts the processor
    include:
    - resident.rs
    limit: 3
  reason: Nutzung des segment-Moduls außerhalb seiner selbst und Init-Docstring der Fassade prüfen
```
/answer cc1b-39 allow
ReasoningToken: 9420
Control Request:
```yaml
id: cc1b-3a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: segment/test/mapping.rs
        matches:
        - let done = crate::resident::segment::mock::ManualFence::new();
        - let store = crate::resident::segment::SegmentStore::new();
        - "let seg = crate::resident::segment::testutil::segment(&store, 1, 10, 0, 2);"
        - "let old_seg = crate::resident::segment::testutil::segment(&store, 1, 10, 0, 4);"
        - "let new_seg = crate::resident::segment::testutil::segment(&store, 1, 11, 1 << 16, 4);"
        - "let seg = crate::resident::segment::testutil::segment(&store, 1, 10, 0, 4);"
      - path: error.rs
        matches:
        - "Segment(#[from] crate::resident::segment::SegmentError),"
      - path: manager/doorbell.rs
        matches:
        - use crate::resident::segment::testutil::region;
      - path: manager/transfer.rs
        matches:
        - use crate::resident::segment::mapping::SlotMap;
      - path: manager/build.rs
        matches:
        - "use crate::resident::segment::mapping::{EntryRef, SlotMap};"
      - path: manager/slot.rs
        matches:
        - "use crate::resident::segment::mapping::{EntryRef, SlotMap};"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      matches:
      - path: resident.rs
        matches:
        - /// Starts the processor and its worker threads; model and device resources are
```
/answer cc1b-3a allow
Control Request:
```yaml
id: cc1b-3b
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
    start: /// Starts the processor
    end: pub enum Residency
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/expand.rs
    min_line: 120
    max_line: 330
  reason: Init-Verhalten und die Similarity-/Inference-Ketten des Planners exakt nachlesen
```
/answer cc1b-3b allow
Control Request:
```yaml
id: cc1b-3c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
      checksum: 790987e92ebb8be96492b665610cc5ebbe1ba9f58b234362a03c8dfe61c7b771
      content: |-
        /// Starts the processor and its worker threads; model and device resources are
            /// created by the first task, not here.
            pub fn init(self) -> Result<Resident, ConfigError> {
                let model = self.model.ok_or_else(|| ConfigError::MissingField {
                    field: "model".into(),
                })?;
                let mut workers = self.workers;
                if let Some(n) = self.min_op_buffer {
                    workers.min_op_buffer = n;
                }
                if workers.min_op_buffer == 0 {
                    return Err(ConfigError::InvalidValue {
                        field: "min_op_buffer".into(),
                        reason: "must be at least 1".into(),
                    });
                }
                let wakeups = Wakeups::new();
                let queue: Arc<TaskQueue> = Arc::new(Queue::new(wakeups.relays()));
                let processor = Processor::start(
                    workers.clone(),
                    queue.clone(),
                    wakeups,
                    self.expander.unwrap_or_else(|| Arc::new(Unplanned)),
                    self.cpu.unwrap_or_else(|| Arc::new(NoEngine)),
                    self.gpu,
                )
                .map_err(|e| ConfigError::InvalidValue {
                    field: "workers".into(),
                    reason: e.to_string(),
                })?;
                Ok(Resident {
                    config: ResidentConfig { model, workers },
                    queue,
                    processor,
                    next_task: AtomicU64::new(0),
                    next_index: AtomicU64::new(0),
                    indexes: Mutex::new(HashMap::new()),
                })
            }
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum Residency
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/expand.rs
      checksum: 5490d36a4d1ea34f55b40eaa28cc37ce58c8eb4c812ee27511f9f7349cd5587b
      content: |2
                    );
                let mut embed = OpParams::new(ctx.requires(ResourceKind::Encoder, encoder));
                embed.dim = ctx.pipeline.dim;
                chain.push(OpKind::Embed, embed);
                if ctx.pipeline.pooling != Pooling::None {
                    chain
                        .push(
                            OpKind::Pool,
                            OpParams::new(ctx.requires(ResourceKind::Encoder, encoder)),
                        );
                }
                if ctx.pipeline.normalize {
                    chain
                        .push(
                            OpKind::L2Norm,
                            OpParams::new(ctx.requires(ResourceKind::Encoder, encoder)),
                        );
                }
                if let Some(index) = params.target_index() {
                    let engine = ctx.engine(ResourceKind::Index)?;
                    chain
                        .push(
                            OpKind::WriteIndex,
                            OpParams::new(
                                ctx
                                    .requires(ResourceKind::Index, engine)
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
                if let ScorerKind::CrossEncoder(_) = params.get_scorer() {
                    return Err(PlannerError::ScorerUnavailable {
                        scorer: params.get_scorer().label().into(),
                    });
                }
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
                chain
                    .push(
                        OpKind::Tokenize,
                        OpParams::new(ctx.requires(ResourceKind::Tokenizer, tokenizer)),
                    );
                chain
                    .push(
                        OpKind::Infer,
                        OpParams::new(ctx.requires(ResourceKind::CrossEncoder, cross)),
                    );
                Ok(chain.finish())
            }
        }
```
/answer cc1b-3c allow
Text:
 Ich habe jetzt ein belastbares Bild aller 14 Modulebenen. Hier die READMEs – zuerst die Wurzel, dann je Ebene eine.
ReasoningToken: 5201
Control Request:
```yaml
id: cc1b-3d
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/README.md
    mode: replace
    content: |
      # resident

      Warm embedding and similarity engine of the `rag` crate. Models, tokenizer, kernels and index mirrors are created lazily on first use and then stay resident until an idle timeout evicts them.

      The module is a **substrate**: tokenise → embed → store → score. Chunking, query formulation, reranking policy, cascades and result quality belong to the layers above the facade.

      ## Design in brief

      - **Pull, not push.** The facade only enqueues tasks. Workers expand them into operations; CPU and GPU subengines claim the operations whose requirements they can serve. Load balances itself by real progress.
      - **Task ≠ operation.** A task is a user request (host data + parameters). An operation is one step on one resource (tokenizer, encoder, scorer, index) with immutable parameters and a flag saying whether the result returns to the host or stays on the device.
      - **The index is the source of truth.** It lives in RAM and knows no engine, kernel or profile. GPU memory is a derived, discardable mirror kept in sync by the index manager. Stale results are an accepted variable; consistency is never bought with locks.
      - **Kernels are blind to entries.** They see segments, offsets and spans and return (address, score). Mapping back to entries is the manager's job.
      - **Precision is a wish.** Callers ask for 0 (fast) … 1 (precise); each subengine maps the wish to its own calibrated profiles. Bit-exact reproducibility is not a goal.
      - **No silent fallbacks.** Unsupported scopes, missing capabilities and unavailable scorers fail explicitly.
      - **Extensions instead of core features.** Iteration, cascades and recursion are addons that enqueue ordinary tasks through the public API.

      ## Layers

      | Layer | Modules | Responsibility |
      |---|---|---|
      | Public API | `facade` | Configure, start, submit tasks, collect results |
      | Orchestration | `processor`, `planner` | Queues, workers, backpressure · task → operation chain |
      | Execution resources | `subengine`, `model`, `tokenize`, `kernel`, `precision` | What a CPU/GPU side owns and runs · calibration and profiles |
      | Index and data placement | `index`, `segment`, `manager`, `memory` | Index contract · device-side layout · RAM ↔ VRAM mirroring · arenas |
      | Primitives | `queue`, `op` | Lock-free queue and task state · operation vocabulary |
      | Cross-cutting | files in this directory | Backend choice, errors, tracing, fault injection, test doubles |

      Layers describe responsibility, not a strict import order. Some pairs reference each other's basic types (for example `memory` and `subengine` for events and lanes, `processor` and `planner` for the facade's task types).

      ## Status

      Building blocks exist for every layer. Neither the processor's engine interface nor the subengine trait has a production implementation in this tree yet: the facade installs placeholders (tasks fail fast) and only tests wire fake engines. Whoever composes the real CPU/GPU engines also supplies the planner's engine offers.

      ## Request path

      1. `facade` puts the task into the task queue and returns an awaitable handle.
      2. A `processor` worker runs a prioritised pass: take over finished device work, feed devices, run host work, expand new tasks (only when hinted or the operation buffer runs low).
      3. Expansion goes through the `planner`: rules per model and task kind yield an ordered operation chain, each operation with its requirements.
      4. Operations land in the CPU, GPU or shared operation queue; an engine claims matching ones and runs them on a lane against leased resources.
      5. A finished operation builds its successor. The last result completes the task state, which resolves the handle.

      Similarity additionally: the manager resolves the scope to an entry set and makes it resident, the kernel scores segments, and hits travel back as kernel hit (address) → segment hit (slot) → manager hit (entry, outdated flag) → scored hit (with source back-reference) → facade report.

      ## Same name, different thing

      | Name | Where | Meaning |
      |---|---|---|
      | Pooling | `model`, `planner` | encoder-side mode vs. pipeline-config setting |
      | ScorerKind | `facade`, `planner` | task parameter vs. pipeline-config default |
      | Payload | `facade`, `index` | task input bundle vs. per-entry source back-reference |
      | Resident | `facade`, `manager` | the public engine handle vs. a loaded-entry record |
      | DeviceHandle | `subengine`, `op` | device/context identity vs. alias of the region lease |
      | ResourceState | `processor`, `subengine` | state set of a synchronous cell vs. of an async lazy-init cell |
      | Lease | `op`, `subengine`, `segment` | region pin vs. resource lease vs. generation lease |
      | SegmentTable | `segment`, `manager` | published generation table vs. copy-on-write table behind a root pointer |
      | Access | `index`, `manager` | file access mode vs. load access pattern |
      | Resolution | `segment`, `precision` | hit-resolution outcome vs. calibration-store lookup |
      | Clock, Measure | `mock.rs`, `precision` | generic test doubles vs. the calibrator's own measurement abstraction |
      | Engine, Subengine, IndexManager | `processor`, `subengine`, `manager` | pull-side worker interface · resource and capacity of a CPU/GPU side · mirror of one index |

      ## Where to look

      | Task | Start in |
      |---|---|
      | New or changed task parameter | `facade/params.rs`, then `planner/expand.rs` if the chain changes |
      | How a task becomes operations | `planner` |
      | New model | `model` (spec, registry), `tokenize` (mode scheme), planner rules |
      | Scheduling, backpressure, worker loop | `processor` |
      | New scoring implementation | `kernel` |
      | Index format, stamps, scope | `index` |
      | Loading, eviction, sync to the device | `manager` |
      | Arenas, budget, staging | `memory` |
      | Calibration, profiles, precision matching | `precision` |

      ## Files in this directory

      | File | Role |
      |---|---|
      | `mod.rs` | Declares all submodules; `mock` only with `test` or feature `mock` |
      | `backend.rs` | Compute backend abstraction and the single place that decides CPU vs. CUDA; falls back to CPU if GPU init fails |
      | `error.rs` | One error enum per layer plus a top-level wrapper; shared retryable/fatal classification |
      | `telemetry.rs` | Tracing initialisation and the span hierarchy (task → resource, manager cycle) with stable field names |
      | `failpoint.rs` | Named fault-injection points at synchronisation boundaries; inert outside `test`/`mock` |
      | `mock.rs` | Conventions and shared test doubles (clock, backend, noisy measurement, loom shim for CAS protocols) |

      ## Conventions

      - **Features:** `cuda` (real CUDA paths; the CUDA adapter modules always compile and gate toolkit code internally), `candle` (Candle adapter in `model`), `mock` (test doubles outside tests).
      - **Tests:** in a `test/` directory next to the sources, one file per covered source file. Exceptions are noted in the module READMEs.
      - **Boundaries:** modules depend on narrow traits so hot-path logic runs against hand-written fakes without a GPU.
      - **These READMEs:** reference files and principal types, never functions. Renaming a function does not make them stale; adding, removing or moving a file does.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/README.md
    mode: replace
    content: |
      # facade

      Public API of the engine: configure, start, submit tasks, collect results.

      ## Concept

      - **Two-step start.** A builder collects configuration (model, worker counts, planner, CPU engine, optional GPU engine); init only starts the worker threads. Model and device resources are created by the first task, so an idle engine holds nothing. GPU workers start only if a GPU engine is supplied.
      - **Three task kinds:** embedding (text → vectors, optionally written to an index), similarity (vectors against an index scope → scored hits), inference (query/chunk pairs for a cross-encoder). Rerank is not a task kind; it is a similarity task with another scorer on a narrowed scope.
      - **Parameters** are typed per kind and carry a precision wish (default: precise) plus an open, type-keyed extension map that travels unchanged through queue and planner. Addons use only this and the normal submit path, so the tasks they derive are indistinguishable from application tasks. `test/addon.rs` is the reference cascade.
      - **Results** arrive through an awaitable handle. Dropping an unfinished handle cancels the task if no worker has claimed it. The blocking wrapper owns a private runtime and must not be used from inside another tokio runtime.
      - **Similarity returns facts, not decisions.** Raw scores plus descriptive statistics (count, gaps, spread, entropy, elbow) let the caller decide on follow-up tasks. Thresholds stay outside.
      - **Indexes are registered, not loaded.** The first task that touches an index triggers loading.
      - **Fail fast without wiring.** Without a supplied planner or engine, placeholders make tasks fail instead of waiting forever.
      - Errors reaching waiters are cloneable so one outcome can serve several waiters.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Public surface and re-exports |
      | `resident.rs` | Configuration, builder, the engine handle, index registry, shutdown, placeholders |
      | `blocking.rs` | Synchronous wrapper around the engine handle |
      | `task.rs` | Task kinds, task metadata, payload, task queue type |
      | `params.rs` | Inputs, per-kind parameters, result type |
      | `handle.rs` | Awaitable task handle with cancel-on-drop |
      | `extensions.rs` | Type-keyed parameter extensions |
      | `report.rs` | Similarity report: ranked hits and derived score statistics |
      | `error.rs` | Task-level error delivered to waiters |

      Tests: `test/` (`addon.rs` doubles as documentation of the extension mechanism).
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/README.md
    mode: replace
    content: |
      # queue

      Generic lock-free queue and the claim state machine of its nodes. Instantiated four times by the engine: the task queue and the CPU, GPU and shared operation queues.

      ## Concept

      - Singly linked, append-only; append is a CAS on the tail, no lock guards the chain. Order equals age.
      - Unlinking is logical: finished nodes stay in the chain and are skipped by iteration. Memory is reclaimed only when the queue drops.
      - The queue is blind to meaning. Priority, resource, model and mode are metadata in the node's parameters.
      - A node is claimed by exactly one winner (queued → taken → finished). Cancel only works while a node is still queued. The result cell is set once, by the claim winner.
      - Operation queues reuse the same building block; their completion is tracked by the task's operation counter, not by the node result.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Module overview and re-exports |
      | `queue.rs` | The queue: sentinel head, CAS append, iteration over live nodes |
      | `node.rs` | Chain node (parameters, payload, link) |
      | `state.rs` | Claim state machine and the result cell |
      | `task.rs` | Notes on how task and operation queues are instantiated (no code of its own) |
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/README.md
    mode: replace
    content: |
      # op

      The vocabulary between planner and subengines: what an operation is, what it needs and what flows between operations.

      ## Concept

      - **Operation parameters are immutable and hashable.** They serve as the pre-claim filter (does this engine qualify?) and as a cache/graph key.
      - **Requirements** state resource kind, allowed engine side (CPU, GPU, either), optionally model, index and required capabilities. Affinity exists as a field but is deliberately ignored when matching.
      - **The chain is implicit.** Each operation carries a successor that builds the next one from its output; an empty answer ends the chain and the output becomes the task result.
      - **A value is host data or a device handle, never a tensor or raw pointer.** Whether an output is brought back to the host is decided by the planner and executed by the operation.
      - **Task context** is the state shared by all operations of one task: open-operation counter, first error, cancellation. Follow-ups are counted before they are enqueued so the counter cannot reach zero early.
      - **Leases** pin a device region while an operation uses it; pinned regions are not evicted.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Operation kinds, parameters, description, values, scored-hit results, task context, operation node, successor contract |
      | `requirements.rs` | Resource kind, engine selector, model/index/device ids, affinity, requirements and their matching |
      | `lease.rs` | RAII region lease and pin counter |
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    mode: replace
    content: |
      # processor

      Orchestration: owns the worker threads, the operation queues, wake-ups, backpressure and per-task completion. It decides *when* work happens, not *what* the work is.

      ## Concept

      - **One worker pool, two sides.** Workers belong to the CPU or the GPU side. Each runs a prioritised pass, re-evaluated after every operation: (1) take over finished device work, (2) feed devices (sync indexes, launch on a free lane), (3) claim and run host work, (4) expand new tasks. GPU precedence limits itself: feeding needs a free lane, expansion needs a hint or an empty queue.
      - **The two sides are interfaces.** An engine interface (three entry points: take over, feed, claim-and-run) is implemented by the CPU/GPU engines; an expander interface turns a task into an operation chain and is implemented by the planner's adapter. The processor knows neither Candle nor kernels.
      - **Backpressure by starvation.** An engine iteration that finds a free resource but nothing to pick marks a decaying per-kind hint. Expansion runs only for a hint or when the operation buffer is below its minimum. No fixed fill threshold.
      - **Queues.** CPU, GPU and shared operation queues. Shared is wired in but never filled by routing; an engine checks its own queue first, then shared.
      - **Task completion.** A counter of open operations; the last one completes the task. The first error cancels the task, remaining operations are dropped at claim. A deadline fails overdue tasks.
      - **GPU completion.** A lane board tracks each lane (idle → reserved → launched → completing). Completion arrives by a host callback (push, no CUDA calls inside) or by polling an event; each lane is taken over exactly once.
      - **Resource cell.** A synchronous CAS lifecycle cell (unloaded → loading → ready → evicting) with in-flight count, pins and TTL eviction.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Overview and re-exports |
      | `hub.rs` | Shared coordination point handed to engines: engine and expander interfaces, wake-ups, claim, finish/fail, expansion trigger |
      | `worker.rs` | Processor start/shutdown and the prioritised worker pass |
      | `ops.rs` | Per-task run bookkeeping, operation jobs, the three operation queues and routing |
      | `starved.rs` | Decaying per-kind demand hints |
      | `resource.rs` | Synchronous lifecycle cell for lazily initialised resources |
      | `completion.rs` | Lane board and completion-detection interface, manual double for tests |
      | `completion_cuda.rs` | Callback-based and poll-based completion on CUDA streams (feature `cuda`) |
      | `config.rs` | Worker counts, minimum operation buffer, hint TTL, task timeout, priority and thread limits |
      | `error.rs` | Orchestration errors |

      Tests: `test/` per file; `tests.rs` holds scenario tests of the whole processor with fake engines.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/README.md
    mode: replace
    content: |
      # planner

      Stateless expansion of a task into an ordered chain of operations. Same inputs, same operations; no queues, no workers, no notification.

      ## Concept

      - **Inputs:** the task's parameters (never its data), the rules, the engine offers (which resource kinds the CPU and GPU side can serve) and the current starved hint.
      - **Rules = data + code.** Per-model pipeline data (prefixes, pooling, normalisation, maximum sequence length, dimension, dtype, scorer) can be loaded from TOML and is validated. Logic lives in one expander per model × task kind; the registry can be overridden. Rules sit behind an atomic swap, so a reload never disturbs a running expansion.
      - **Chains.** Embedding: tokenise → embed → pool (unless disabled) → L2-normalise (if configured) → write to index (if a target index is set). Similarity: score → top-k. Inference: tokenise → infer. A similarity task with a cross-encoder scorer is currently rejected.
      - **Side choice.** GPU preferred when both sides offer a kind; CPU when only it does. A starved hint widens the choice to either side. The hint only picks between equivalent variants; it never changes the operation set.
      - **Materialisation.** A result stays on the device only when the next operation runs on the GPU; otherwise it returns to the host.
      - **Adapter.** The processor-facing adapter plans a task once, hands out the chain one operation at a time through successors and converts the final host data into the facade's result type (similarity → report).

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | The planner: rule snapshot, reload, expansion entry |
      | `rules.rs` | Rule set: pipelines per model, expander registry, TOML loading, model selection |
      | `config.rs` | Per-model pipeline data (planner-side types, see root README on name clashes) |
      | `expand.rs` | Task view, planned operation, expander contract and the three standard expanders |
      | `offers.rs` | Which resource kinds each side offers; side choice |
      | `adapter.rs` | Bridge to the processor's expander interface; result conversion |

      Tests: `test/` (`mod.rs` for expansion, `adapter.rs` for the bridge).
  reason: "Wurzel-README sowie READMEs für facade, queue, op, processor und planner anlegen"
```
/answer cc1b-3d allow
Text:
 Kurze Kontrolle, ob die Sonderzeichen korrekt geschrieben wurden, bevor ich die restlichen Ebenen anlege.
ReasoningToken: 181
Control Request:
```yaml
id: cc1b-3e
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/README.md
    max_line: 8
  reason: "Prüfen, dass Unicode-Escapes (→, ≠, ·) korrekt als Zeichen im Dateiinhalt stehen"
```
/answer cc1b-3e allow
Control Request:
```yaml
id: cc1b-3f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/README.md
      checksum: ad07d9219196b788fa7638789b4cf2b77e18538f77797cad00a8f3604131bbc6
      content: |
        # op

        The vocabulary between planner and subengines: what an operation is, what it needs and what flows between operations.

        ## Concept

        - **Operation parameters are immutable and hashable.** They serve as the pre-claim filter (does this engine qualify?) and as a cache/graph key.
        - **Requirements** state resource kind, allowed engine side (CPU, GPU, either), optionally model, index and required capabilities. Affinity exists as a field but is deliberately ignored when matching.
```
/answer cc1b-3f allow
Text:
 Passt. Jetzt die restlichen neun Ebenen.
Control Request:
```yaml
id: cc1b-40
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/README.md
    mode: replace
    content: |
      # subengine

      What a compute side (CPU or GPU) owns and how it is handled generically: device handle, execution lanes, resource lifecycle and pull-capacity reporting. Not the work itself and not the queues.

      ## Concept

      - **Invariants.** No tensors or raw pointers in public signatures (regions plus layouts only). All device work runs on a lane, never on the default stream. Candle is an adapter at exactly one place (`model`), not a dependency of this layer.
      - **Lane = in-order execution queue.** On the CPU a dedicated worker thread; on CUDA a prioritised non-blocking stream. Ordering across lanes is expressed only through events. Each lane has a parameter block for dynamic kernel arguments and a bump-allocated scratch span that is reset once the lane has drained.
      - **Device handle.** The first CUDA context creation is expensive (seconds), so lazy initialisation must budget for it; a warm-up forces driver-side lazy work out of the first real task.
      - **Resource lifecycle** (async cell): unloaded → loading → resident → leased(n) → idle → evicting, plus failed. Initialisation runs once for concurrent callers, a limiter bounds parallel initialisations, and a failed init can be retried. Eviction needs zero leases and zero running operations and an expired idle time; VRAM resources idle much longer than host ones. A resource may veto eviction.
      - **Pull sizing.** Batch size is derived from smoothed observed throughput within clamps. Throughput measured here steers only the batch size, not the split between CPU and GPU.
      - **Claim contract.** The processor implements a claim interface (filter + budget → jobs); a subengine reports free capacity and pulls.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Subengine contract (kind, device, capacity, throughput, batch size, pull budget), CPU backend |
      | `device.rs` | Backends, device handle, CUDA lane (CUDA parts gated inside) |
      | `exec.rs` | Lane contract, lane classes and config, events, parameter block, scratch arena, CPU lane, recording lane for tests |
      | `lifecycle.rs` | Lazily initialised, evictable resource cell with leases and init limiter |
      | `resource.rs` | Resource identity and eviction hints, smoothing, pull controller, capacity, claim filter and interface, grouping helper |
      | `error.rs` | Lifecycle and device/lane errors |

      Tests: `test/` per file; a recording lane double is part of `exec.rs`.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/README.md
    mode: replace
    content: |
      # memory

      Engine-owned memory model: pointer-free handles, layouts, dtypes, per-class arenas, staging and the VRAM budget.

      ## Concept

      - **Region = handle, not pointer.** Arena, offset, length and generation; the generation invalidates handles after evict/reload. A device address exists only inside a backend (`cuda.rs`); nothing else sees it.
      - **Arenas by lifetime class.** Long-lived (weights), streamed and evictable (index segments), short-lived (per-lane scratch), staging (pinned host memory, GPU only). The CPU path has host arenas without streams or staging.
      - **One allocator for all backends.** An offset allocator (first-fit over a coalescing free map) sits behind a shared per-class arena set. Backends supply only the raw device memory and copies. No device malloc/free during operation.
      - **Stream-ordered reuse.** A region is released, and a staging slot reused, only after the event of its last user has completed. This lives here, not in the operations.
      - **Budget is measured.** VRAM is probed at runtime rather than assumed; arena sizes are checked against it, including headroom for copy-on-write peaks.
      - **Engine-owned dtype.** Backend dtypes are converted at the edge only. The default compute dtype per device kind is defined here (GPU f16, CPU f32 by design intent; model specs can override).
      - **Layout** describes shape, strides and dtype in bytes; views change layout, never data.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Overview, re-exports |
      | `region.rs` | Region and arena id |
      | `layout.rs` | Byte layout of tensors |
      | `dtype.rs` | Element type and mask constants; Candle conversion at the edge |
      | `freelist.rs` | Backend-agnostic offset allocator |
      | `arena.rs` | Per-class arena allocator and arena set, device-memory trait it runs on |
      | `manager.rs` | Memory-manager contract, device kinds, GPU-free bookkeeping implementation |
      | `budget.rs` | Arena classes, sizes, VRAM budget and probe, fit check |
      | `staging.rs` | Staging ring with event-guarded slot reuse |
      | `cpu.rs` | Host-RAM implementation |
      | `cuda.rs` | CUDA implementation: backing allocations, pinned staging, region-addressed copies (feature `cuda`) |
      | `wrap_gate.rs` | Prototype gate: can an arena sub-range back a Candle tensor without Candle freeing it |
      | `error.rs` | Memory errors, transfer direction |

      Tests: `test/` per file; CUDA tests need feature `cuda`.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/model/README.md
    mode: replace
    content: |
      # model

      Model resources of the compute side: description, registry, encoder contract and the loaded encoder as an evictable resource.

      ## Concept

      - **Spec owns nothing.** A model spec is plain data: ids, dimension, pooling, normalisation, query/passage prefixes, storage dtype, compute dtype per device class, maximum sequence length, optional projection head, file paths. Validation rejects configurations that would silently degrade embeddings (for example a prefix without trailing space). A preset exists for `intfloat/multilingual-e5-small`.
      - **Registry.** Architecture id → factory. A new model is a registry entry plus a spec.
      - **Backbone vs. pipeline.** A backbone only turns tokens into hidden states. The shared pipeline around it is backbone-agnostic: tokens → hidden states → pooling → optional projection → normalisation → write into the destination slot (L2-normalised, converted to the slot dtype). Candle types appear only in `candle_adapter.rs` (feature `candle`).
      - **Pooling.** Masked mean gives one vector per sequence (dense); per-token keeps all vectors (late interaction). Host reference versions exist for comparison.
      - **Batching.** Token budget instead of item count, length buckets, padding to a common length inside a batch.
      - **Loaded encoder** = architecture + weights on one device, a lifecycle resource that can be leased and evicted. Output is written straight into a region; the source tensor must stay alive until the write event.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Spec, paths, prefixes, dtypes, tensor reference, region I/O contract, encoder contract, registry, loaded-encoder resource |
      | `encoder.rs` | Backbone contract and the backbone-agnostic encode pipeline |
      | `candle_adapter.rs` | The only Candle/BERT code; factory for the registry (feature `candle`) |
      | `pooling.rs` | Host reference: masked mean pooling, L2 normalisation |
      | `head.rs` | Optional linear projection of token embeddings |
      | `slot.rs` | Writes encoder output into a slot with normalisation and dtype conversion |
      | `batch.rs` | Token-budget batching with length buckets |
      | `error.rs` | Model errors |

      Tests: `test/` per file with fake backbones.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize/README.md
    mode: replace
    content: |
      # tokenize

      The one shared CPU tokenizer. Token ids reach the device only as `u32` encoder input.

      ## Concept

      - **Single instance, CPU only.** It is a CPU resource (lifecycle-managed like others) shared by all models that use the same vocabulary. There is deliberately no device-side tokenizer and no id-to-address gather.
      - **Mode scheme.** Text is encoded as query or passage. A scheme decides what that means: textual prefixes for dense models, or marker tokens with fixed-length mask padding (queries) and truncation to the document maximum (documents) for ColBERT-style models.
      - **Truncation yes, padding no.** Sequences are truncated to the maximum length (closing special token kept). Padding to a common length is left to the batcher, so one tokenizer call can feed several batches.
      - **Normalisation is pluggable.** A text-normaliser hook (default: identity) runs before tokenisation, always on the CPU.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | The shared tokenizer: loading from a model spec, encoding, batch production |
      | `modes.rs` | Encode mode and mode schemes (prefix, ColBERT-style) |
      | `normalize.rs` | Normaliser hook and identity default |
      | `error.rs` | Tokenizer errors |

      Tests: `test/mod.rs`.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/README.md
    mode: replace
    content: |
      # kernel

      Scoring compute. A kernel receives segment descriptors and spans and returns (address, score) pairs. It knows nothing about entries, indexes or models.

      ## Concept

      - **Segments, not entries.** A segment has a base, a count and a layout: fixed stride (entry boundaries by arithmetic) or variable (offset table of prefix sums). A candidate set is simply a variable layout. The kernel never traverses and never communicates during a run. Mapping an address back to an entry is the manager's job; mod.rs only provides the arithmetic helpers for both layouts.
      - **Scorers are swappable behind one operation.** Dense path: cosine as the dot product of L2-normalised vectors (an exact variant exists). Late interaction: MaxSim, implementable as CPU SIMD loop, as a tile chain on Candle ops, or as a fused kernel. Padding is masked on the document side; query mask tokens count.
      - **Top-k is bounded and deterministic.** Slice-local reduction with tie-breaking, merged on the CPU.
      - **Work slices.** Launches are bounded and sized from observed throughput; yielding is possible only at slice boundaries because a running GPU kernel cannot be preempted.
      - **Launch indirection.** A launcher sits between scorer and lane: direct submit now; graph capture is reserved and keyed so scorers need no change when it arrives.
      - **Registry.** Lazily populated kernel table keyed by name, dtype, SM architecture and variant; the CUDA loader compiles at runtime. `cuda.rs` and the registry always compile; toolkit-specific code is gated inside.
      - **Catalog.** States which capability values have an implemented scoring kernel, independent of the `cuda` feature. Precision probing intersects with it.
      - **Golden comparison.** A candidate scorer is judged against a reference within a tolerance: scores may drift, ranks may swap only where reference scores are that close. Exact equality is never required.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Segment descriptors, hits, written ranges, kernel contract, scorer interfaces, address-to-entry arithmetic |
      | `cosine.rs` | Dense scoring path |
      | `maxsim.rs` | Late-interaction scoring operation |
      | `cpu_simd.rs` | CPU primitives shaped for auto-vectorisation |
      | `cuda.rs` | NVRTC loader and real CUDA scoring kernels |
      | `registry.rs` | Kernel table and loader contract |
      | `topk.rs` | Bounded top-k and merge of slice-local results |
      | `slice.rs` | Slice sizing and sliced execution |
      | `launcher.rs` | Launch modes and launch key |
      | `slots.rs` | In-memory slot source for CPU scoring and tests |
      | `catalog.rs` | Implemented capability values per subengine kind |
      | `golden.rs` | Tolerance-based scorer comparison |
      | `mock.rs` | Deterministic kernel fake (`test` or feature `mock`) |
      | `error.rs` | Kernel errors |

      Tests: `test/` per file; CUDA tests need feature `cuda`.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/README.md
    mode: replace
    content: |
      # precision

      Capability sets, calibration, profile building, matching of a precision wish to a profile, and persistence of the results.

      ## Concept

      - **Capabilities, not profile names.** A capability set maps open axis identifiers (operand type, accumulator, scoring type, batch, tile, threads, …) to the values a provider offers or a task requires. Axes and values are extensible per hardware without touching facade or task interface. `known.rs` lists the axes for the target hardware (GTX 1660, sm_75; bf16 deliberately absent) and also the axis for the index manager's gather-versus-segment load threshold.
      - **Probe, then calibrate.** Probing is optimistic and measures nothing; its result is intersected with the kernels that exist. Unusable combinations are removed by calibration.
      - **Calibrator.** Enumerates combinations (with pruning), measures them interleaved against the fp32 reference, rejects invalid ones (NaN/Inf, degenerate output, deviation above the maximum) and keeps the Pareto front over (time, deviation). Only relations count, estimates are robust (median/minimum). Measurement stops on stability or at a hard cap; then the best intermediate state is kept and marked `unstable`. Calibration never blocks the user and runs on dedicated low-priority threads, not on tokio workers.
      - **Profiles are unnamed positions.** Their number follows from the spread of the front alone. Each has a position in [0, 1] derived from arc length on the normalised front: 0 fastest, 1 most precise. A single profile sits at 0.5.
      - **Matching.** The task's wish is clamped to [0, 1] (NaN is an error) and mapped to the nearest profile of the engine's own set; a tie goes to the more precise one. The scale is relative per subengine, and a wish is always satisfiable.
      - **Store.** Results are kept per model × subengine with a fingerprint of the system. Equal fingerprint = calibrated, different = stale (old result stays usable while a background run replaces it), no file = missing.
      - **Measurement is an interface** with CPU, CUDA and scripted implementations; a shared synthetic workload and fp32 reference serve the real ones.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Overview, re-exports |
      | `axis.rs` | Open axis and value identifiers |
      | `capability.rs` | Capability sets |
      | `known.rs` | Well-known axes and values for the target hardware |
      | `probe.rs` | Probe interface, kernel intersection, probe configuration |
      | `probe_cpu.rs` | CPU probe with injectable feature source |
      | `probe_cuda.rs` | CUDA probe; hardware table testable without GPU |
      | `measure.rs` | Measurement abstraction, validity, deviation report, clock |
      | `measure_cpu.rs` | Real CPU scoring measurement |
      | `measure_cuda.rs` | Real CUDA scoring measurement (event-timed, sliced) |
      | `measure_mock.rs` | Scripted fake measurement |
      | `workload.rs` | Synthetic scoring workload and fp32 reference |
      | `metrics.rs` | Deviation metrics, plausibility checks |
      | `stats.rs` | Robust estimators and spread |
      | `calibrate.rs` | The calibrator: caps, stages, outcomes, report |
      | `pareto.rs` | Pareto front and arc-length positions |
      | `profile.rs` | Profile type, configuration, derivation from the front |
      | `matching.rs` | Wish → profile selection |
      | `store.rs` | Fingerprint, records, store interface and file store, background recalibration |
      | `pool.rs` | Low-priority thread pool for calibration and CPU kernels |

      Tests: `test/` per file; CUDA measurement and probe tests need feature `cuda`.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/README.md
    mode: replace
    content: |
      # index

      The index contract and its implementations. An index is the RAM-resident source of truth for vectors; it reports changes and delivers structure, nothing more.

      ## Concept

      - **Independent of the engine.** An index knows no engine, profile, kernel or geometry. The only hard binding to a model is model id and dimension in its header. Persistence format and timing belong to the index object, not to the engine.
      - **Own model per index.** Whole files, sliding windows, AST nodes, chunks or cascading structures are index decisions. An entry is a vector range of variable length (dense = 1 vector, multi-vector = many); entries may overlap.
      - **Stamps.** Each entry carries the value of a global monotonic counter at its last write. Stamps are unique across removal and re-creation of an id and are compared with `!=` only. File-backed indexes without a counter fall back to size+mtime, then to a content hash, treating timestamps within the clock resolution as unsafe. Stamps live in their own column so a scan never pages in vectors.
      - **Change signalling.** A level signal without content (a watch value carrying a watermark). Lost or merged signals are harmless because consumers reconcile by comparing stamps. Binding write order: data, then stamp, then signal.
      - **Structure only.** The index can list children of a node, members of a cell and summary entries (for example a centroid table), and declares which access methods it supports. It does not interpret distance, radius, top-k or prefix match and does not resolve scopes; the manager does.
      - **Writers.** Applying a write is serialised and returns the stamp of the write; last writer wins.
      - **Lazy and possibly blocking.** Reads may hit page faults or disk; callers use a blocking pool.
      - **Source back-reference.** Each entry can point to its source (target file, character offsets, checksum), so an outer layer can fetch the original text, for example for a cross-encoder.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Ids, stamp, access methods, scope, entry sets and data, header, the index contract |
      | `memory.rs` | Reference in-memory index; builds without `cuda`/`candle` |
      | `parquet_io.rs` | Parquet-backed index: stamps scanned without decoding vectors, lazy vector/payload decoding, atomic file replacement on every write |
      | `schema.rs` | Arrow schema, footer header keys, schema versioning and compatibility check (model, dimension, pooling, prefixes, dtype, tokenizer fingerprint) |
      | `stamp.rs` | Stamp counter and the file-stamp fallback chain |
      | `payload.rs` | Source back-reference and checksum |

      Changing the model means a new schema version and re-embedding.

      Tests: `test/` per file.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/README.md
    mode: replace
    content: |
      # segment

      How index data is laid out and published inside a subengine: slot geometry, immutable segments, generation manifests, deferred release and the mapping from kernel results back to entries.

      ## Concept

      - **Slots.** A slot has fixed size (maximum tokens × dimension × dtype). Slot classes by token length keep the kernel lookup-free for variable-length entries. Per slot: number of valid tokens (padding is masked with the dtype's mask value, not zero) and a valid bit.
      - **Upsert never writes in place.** A new slot is filled, then the old slot's valid bit is cleared. The valid bit is set only after the filling copy has completed (stream-ordered).
      - **Segments are immutable.** A change publishes a new generation manifest. An old generation is released only when no lease references it and every fence (device event that may still touch it) has completed.
      - **Reverse mapping.** A kernel hit (segment generation, slot, score) is resolved to an entry reference. Hits from generations that were retired meanwhile are dropped or remapped.
      - **Change events.** Segment added/changed events are published on a bus, ordered behind the copy that fills the segment.
      - The manager uses the slot map and entry reference for its own address-based resolution; the manager decides policy, this module provides geometry and generation safety.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Fence contract, slot classes, segment view, segments and tables, manifests, generation leases, store; `mock`/test helpers behind `test` or feature `mock` |
      | `slots.rs` | Valid bitmap, slot metadata, stream-ordered slot write |
      | `mapping.rs` | Slot map, entry reference, generation-aware resolution of hits |
      | `sync.rs` | Segment events, bus and publisher |
      | `error.rs` | Segment errors |

      Tests: `test/` per file.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/README.md
    mode: replace
    content: |
      # manager

      The index manager: keeps a resident mirror of an index in engine memory and resolves kernel results back to entries. It is a component of the CPU/GPU engine, neither of the facade nor of the index.

      ## Concept

      - **Attached lazily.** The manager is built with an engine; the index is attached on first use. Until then it holds no resource and no subscription. The CPU variant is nearly trivial (RAM is primary; only residency hints for mapped files and idle unloading). The GPU variant does the real work below.
      - **Consistency model** (summarised in `mod.rs`): the index wins on divergence; identity is entry id plus stamp; reconciliation is state-based, the change signal only wakes the scan, and only resident entries are compared. On load the order is stamp, data, stamp again. The manager records the stamp returned by its own writes, which suppresses echoes.
      - **Never block the kernel.** The manager mutates only in gaps (batch end or communication slot). Larger structures are copy-on-write: build the new version, swap one 8-byte root pointer, free the old version once all readers have reached the new epoch.
      - **Communication slot has a fixed order:** register what the kernel wrote with the index; resolve results (address → entry); only then apply queued updates, which change the mapping. Result states: removed in the index → dropped; update pending → flagged outdated.
      - **Scope resolution.** Pluggable resolver per access method (range, subtree, prefix, centroid). An index must declare the method *and* a resolver must exist, otherwise the scope is unsupported; there is no fallback to "all".
      - **Load strategy.** Dense scopes load whole segments, sparse sets are gathered into one pinned staging slot and copied once, tiny sets may be read zero-copy. Concurrent loads of the same range are coalesced (single flight, error shared by all waiters); tasks against one index are aggregated over the union of their scopes.
      - **Eviction.** Summary structures live longest, pinned entries are never victims, idle expiry plus watermark hysteresis, cost-aware victim choice; VRAM idles longer than RAM.
      - **Index build path.** The manager grants a writable range up front; the kernel appends into it and never allocates. Embedding writes and scoring hits travel on separate rings; writes are always registered first, so indexing during search is safe.
      - Reads that may fault or hit disk run on the blocking pool after a page-cache hint. Seqlock-style reads protect against concurrent writers in shared mappings.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Overview and the consistency model |
      | `streamed.rs` | Manager skeleton, CPU and GPU variants, lazy index attachment |
      | `runtime.rs` | Owner of one index operation: ties manager, swap, mapping and loaded queue together in the required order |
      | `scope.rs` | Scope resolvers and their registry |
      | `residency.rs` | Load plans (segments/gather/zero-copy), aggregation, loader |
      | `flight.rs` | Single-flight coalescing |
      | `evict.rs` | Tiers, policy, victim choice |
      | `tracker.rs` | Entry ↔ slot mapping with known stamps |
      | `sync.rs` | State-based scan and stamp-ordered entry load |
      | `slot.rs` | Communication slot, address map, mapping store, update queue |
      | `swap.rs` | Copy-on-write tables, root pointer, epoch tracking, batch guard |
      | `arena.rs` | Index-segment use of the shared arena allocator: alignment, copy-on-write headroom, epoch-gated release |
      | `transfer.rs` | Device-side execution of load plans, loaded-segment queue |
      | `upload.rs` | Gather path into pinned staging |
      | `build.rs` | Index build path, build rings, write cursor |
      | `doorbell.rs` | Handshake between persistent-kernel batches and the host mutation gap |
      | `advice.rs` | Page-cache hints for mapped index data |
      | `blocking.rs` | Blocking-pool reads |
      | `seqlock.rs` | Seqlock reads over word-granular atomics |

      Tests: `test/` per file; `test/flat.rs` is a shared index double with contiguous backing storage.
  reason: "READMEs für subengine, memory, model, tokenize, kernel, precision, index, segment und manager anlegen"
```
/answer cc1b-40 allow
Text:
 Zwei Aussagen prüfe ich noch am Code, weil ich sie aus dem Konzept statt aus der Quelle abgeleitet habe (Default-Compute-Dtype in `memory`, Valid-Bit-Reihenfolge in `segment`).
Control Request:
```yaml
id: cc1b-41
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/manager.rs
  name: compute_dtype
  reason: Tatsächliches Verhalten der Default-Compute-Dtype-Funktion prüfen
```
/answer cc1b-41 allow
ReasoningToken: 694
Control Request:
```yaml
id: cc1b-42
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/manager.rs
      nodes:
      - id: compute_dtype
        code: |
          pub fn compute_dtype(dev: DeviceKind) -> DType {
              match dev {
                  DeviceKind::Gpu => DType::F16,
                  DeviceKind::Cpu => DType::F32,
              }
          }
```
/answer cc1b-42 allow
Control Request:
```yaml
id: cc1b-43
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
    pattern: valid|Valid
    include:
    - slots.rs
    - sync.rs
    exclude:
    - '**/test/**'
    limit: 15
  reason: Reihenfolge von Slot-Befüllung und Valid-Bit im segment-Modul verifizieren
```
/answer cc1b-43 allow
Control Request:
```yaml
id: cc1b-44
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment
      matches:
      - path: slots.rs
        matches:
        - "//! Valid bitmap, per-slot metadata and stream-ordered upsert."
        - "pub struct ValidBitmap {"
        - "impl ValidBitmap {"
        - "pub valid: bool,"
        - "valid: true,"
        - self.valid.set(slot);
        - self.valid.clear(old);
        - "w.meta[old as usize].valid = false;"
        - self.valid.clear(slot);
        - "w.meta[slot as usize].valid = false;"
        - "pub fn is_valid(&self, slot: u32) -> bool {"
        - self.valid.get(slot)
        - "pub fn valid_count(&self) -> u32 {"
        - self.valid.count()
        - "pub fn valid_words(&self) -> Vec<u64> {"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer cc1b-44 allow
ReasoningToken: 191
Control Request:
```yaml
id: cc1b-45
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/slots.rs
  node_type: function_item|impl_item
  regexp: upsert|Upsert|commit|complete|event|fence
  reason: "Upsert-/Commit-Logik im segment-Modul lesen, um die Valid-Bit-Aussage zu belegen"
```
/answer cc1b-45 allow
Control Request:
```yaml
id: cc1b-46
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/slots.rs
      nodes: []
```
/answer cc1b-46 allow
Control Request:
```yaml
id: cc1b-47
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/slots.rs
    start: pub struct SlotWrite
    max_line: 200
  reason: Slot-Schreib- und Upsert-Semantik in slots.rs direkt prüfen
```
/answer cc1b-47 allow
Control Request:
```yaml
id: cc1b-48
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/slots.rs
      checksum: 431c8b441af18b98f9a1ce22a1001144312166f2b83b1258e04fb33a75f48a97
      content: |
        pub struct SlotWrite {
            slot: u32,
            entry: EntryRef,
            tokens: u16,
            centroid: Option<u32>,
            region: Region,
        }

        impl SlotWrite {
            pub fn slot(&self) -> u32 {
                self.slot
            }

            pub fn region(&self) -> Region {
                self.region
            }

            pub fn tokens(&self) -> u16 {
                self.tokens
            }
        }

        pub(super) struct SlotWriter {
            meta: Vec<SlotMeta>,
            next_free: u32,
            seq: u64,
            by_entry: HashMap<EntryId, u32>,
            pending: VecDeque<(SlotWrite, Arc<dyn Fence>)>,
        }

        impl SlotWriter {
            pub(super) fn new(slots: u32) -> Self {
                Self {
                    meta: vec![SlotMeta::default(); slots as usize],
                    next_free: 0,
                    seq: 0,
                    by_entry: HashMap::new(),
                    pending: VecDeque::new(),
                }
            }

            pub(super) fn take_pending_fences(&mut self) -> Vec<Arc<dyn Fence>> {
                self.pending.drain(..).map(|(_, f)| f).collect()
            }
        }

        impl Segment {
            pub fn begin_write(
                &self,
                entry: EntryRef,
                tokens: u32,
                centroid: Option<u32>,
            ) -> Result<SlotWrite, SegmentError> {
                let max = self.view.class.max_tokens();
                let tokens16 = u16::try_from(tokens)
                    .ok()
                    .filter(|_| tokens <= max)
                    .ok_or(SegmentError::TokenOverflow { tokens, max })?;
                let mut w = self.writer.lock();
                if w.next_free >= self.view.slots {
                    return Err(SegmentError::SegmentFull { slots: self.view.slots });
                }
                let slot = w.next_free;
                w.next_free += 1;
                Ok(SlotWrite {
                    slot,
                    entry,
                    tokens: tokens16,
                    centroid,
                    region: self.view.slot_region(slot)?,
                })
            }

            /// Makes the slot visible once `ready` (the event of its data transfer) has completed.
            /// Commits apply in submission order.
            pub fn commit_after(&self, write: SlotWrite, ready: Arc<dyn Fence>) {
                self.writer.lock().pending.push_back((write, ready));
            }

            pub fn commit(&self, write: SlotWrite, ready: &dyn Fence) -> Result<(), SegmentError> {
                if !ready.is_complete() {
                    return Err(SegmentError::NotReady);
                }
                let mut w = self.writer.lock();
                self.apply(&mut w, write);
                Ok(())
            }

            /// Applies queued commits whose transfers finished; returns how many became visible.
            pub fn settle(&self) -> usize {
                let mut w = self.writer.lock();
                let mut applied = 0;
                while w.pending.front().is_some_and(|(_, f)| f.is_complete()) {
                    let (write, _) = w.pending.pop_front().expect("front checked");
                    self.apply(&mut w, write);
                    applied += 1;
                }
                applied
            }

            fn apply(&self, w: &mut SlotWriter, write: SlotWrite) {
                let slot = write.slot;
                let _ = self.map.bind(slot, write.entry);
                w.seq += 1;
                w.meta[slot as usize] = SlotMeta {
                    valid: true,
                    token_count: write.tokens,
                    gen: w.seq,
                    centroid: write.centroid,
                };
                self.valid.set(slot);
                if let Some(old) = w.by_entry.insert(write.entry.entry, slot) {
                    if old != slot {
                        self.valid.clear(old);
                        w.meta[old as usize].valid = false;
                        tracing::trace!(segment_gen = self.view.gen, slot = old, "tombstone");
                    }
                }
                tracing::trace!(segment_gen = self.view.gen, slot, "upsert");
            }

            /// Hides the entry's slot; its data stays until the segment is rebuilt.
            pub fn tombstone(&self, entry: EntryId) -> bool {
                let mut w = self.writer.lock();
                let Some(slot) = w.by_entry.remove(&entry) else {
                    return false;
                };
                self.valid.clear(slot);
                w.meta[slot as usize].valid = false;
                tracing::trace!(segment_gen = self.view.gen, slot, "tombstone");
```
/answer cc1b-48 allow
Thinking:
Ich korrigiere zwei Punkte: Bei `segment` werden Slots per Commit angehängt statt wirklich unveränderlich zu sein, und bei `memory` ist der tatsächliche Default-Compute-Dtype GPU f16/CPU f32.


ReasoningToken: 589
Control Request:
```yaml
id: cc1b-49
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/segment/README.md
    mode: replace
    content: |
      # segment

      How index data is laid out and published inside a subengine: slot geometry, segments published as generation manifests, deferred release and the mapping from kernel results back to entries.

      ## Concept

      - **Slots.** A slot has fixed size (maximum tokens × dimension × dtype). Slot classes by token length keep the kernel lookup-free for variable-length entries. Per slot: number of valid tokens (padding is masked with the dtype's mask value, not zero), a valid bit and an optional centroid code.
      - **Upsert never writes in place.** An entry's data goes into a new slot. The slot becomes visible only after the event of its data transfer has completed; commits apply in submission order. In the same step the entry's previous slot is invalidated. Removed entries are tombstoned; their data stays until the segment is rebuilt.
      - **Generations.** A segment is published through a generation manifest. A replaced generation is released only when no lease references it and every fence (device event that may still touch it) has completed.
      - **Reverse mapping.** A kernel hit (segment generation, slot, score) is resolved to an entry reference. Hits from generations that were retired meanwhile are dropped or remapped.
      - **Change events.** Segment added/changed events are published on a bus, ordered behind the copy that fills the segment.
      - **Relation to `manager`.** The manager reuses the slot map and entry reference for its own address-based resolution. The manager decides policy; this module provides geometry and generation safety.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Fence contract, slot classes, segment view, segments and tables, manifests, generation leases, store; `mock`/test helpers behind `test` or feature `mock` |
      | `slots.rs` | Valid bitmap, slot metadata, begin/commit of slot writes, tombstones |
      | `mapping.rs` | Slot map, entry reference, generation-aware resolution of hits |
      | `sync.rs` | Segment events, bus and publisher |
      | `error.rs` | Segment errors |

      Tests: `test/` per file.
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/memory/README.md
    mode: replace
    content: |
      # memory

      Engine-owned memory model: pointer-free handles, layouts, dtypes, per-class arenas, staging and the VRAM budget.

      ## Concept

      - **Region = handle, not pointer.** Arena, offset, length and generation; the generation invalidates handles after evict/reload. A device address exists only inside a backend (`cuda.rs`); nothing else sees it.
      - **Arenas by lifetime class.** Long-lived (weights), streamed and evictable (index segments), short-lived (per-lane scratch), staging (pinned host memory, GPU only). The CPU path has host arenas without streams or staging.
      - **One allocator for all backends.** An offset allocator (first-fit over a coalescing free map) sits behind a shared per-class arena set. Backends supply only the raw device memory and copies. Each device-resident class is one backing allocation, so there is no device malloc/free during operation.
      - **Stream-ordered reuse.** A region is released, and a staging slot reused, only after the event of its last user has completed. This lives here, not in the operations.
      - **Budget is measured.** VRAM is probed at runtime rather than assumed; arena sizes are checked against it.
      - **Engine-owned dtype.** Backend dtypes are converted at the edge only. A helper gives the default compute dtype per device kind (GPU f16, CPU f32); a model spec carries its own compute dtypes per device class.
      - **Layout** describes shape, strides and dtype in bytes; views change layout, never data.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | Overview, re-exports |
      | `region.rs` | Region and arena id |
      | `layout.rs` | Byte layout of tensors |
      | `dtype.rs` | Element type and mask constants; Candle conversion at the edge |
      | `freelist.rs` | Backend-agnostic offset allocator |
      | `arena.rs` | Per-class arena allocator and arena set, device-memory trait it runs on |
      | `manager.rs` | Memory-manager contract, device kinds, default compute dtype, GPU-free bookkeeping implementation |
      | `budget.rs` | Arena classes, sizes, VRAM budget and probe, fit check |
      | `staging.rs` | Staging ring with event-guarded slot reuse |
      | `cpu.rs` | Host-RAM implementation |
      | `cuda.rs` | CUDA implementation: backing allocations, pinned staging, region-addressed copies (feature `cuda`) |
      | `wrap_gate.rs` | Prototype gate: can an arena sub-range back a Candle tensor without Candle freeing it |
      | `error.rs` | Memory errors, transfer direction |

      Tests: `test/` per file; CUDA tests need feature `cuda`.
  reason: segment- und memory-README an die im Code verifizierten Fakten anpassen
```
/answer cc1b-49 allow
Text:
 Noch eine Stelle in `tokenize`, an der ich mehr behauptet habe, als der Code belegt („shared by all models with the same vocabulary“, „one call can feed several batches“). Ich korrigiere sie.
ReasoningToken: 471
Control Request:
```yaml
id: cc1b-4a
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/tokenize/README.md
    mode: replace
    content: |
      # tokenize

      The one shared CPU tokenizer. Token ids reach the device only as `u32` encoder input.

      ## Concept

      - **Single instance, CPU only.** It is a CPU resource (keyed like other lifecycle resources) built from a model spec. There is deliberately no device-side tokenizer and no id-to-address gather.
      - **Mode scheme.** Text is encoded as query or passage. A scheme decides what that means: textual prefixes for dense models, or marker tokens with fixed-length mask padding (queries) and truncation to the document maximum (documents) for ColBERT-style models.
      - **Truncation yes, padding no.** Sequences are truncated to the maximum length (closing special token kept). Padding to a common length belongs to the batching in `model`; the tokenizer can hand its sequences to that batching.
      - **Normalisation is pluggable.** A text-normaliser hook (default: identity) runs before tokenisation, always on the CPU.

      ## Files

      | File | Role |
      |---|---|
      | `mod.rs` | The shared tokenizer: loading from a model spec, encoding, batch production |
      | `modes.rs` | Encode mode and mode schemes (prefix, ColBERT-style) |
      | `normalize.rs` | Normaliser hook and identity default |
      | `error.rs` | Tokenizer errors |

      Tests: `test/mod.rs`.
  reason: Nicht belegte Aussagen im tokenize-README entfernen
```
/answer cc1b-4a allow
Text:
 Abschlusskontrolle: Vollständigkeit der 15 READMEs und ob versehentlich Funktionsnamen (`name()`) in den Texten stehen.
Control Request:
```yaml
id: cc1b-4b
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: README\.md$
  reason: "Prüfen, dass für alle 15 Ebenen eine README existiert"
```
/answer cc1b-4b allow
ReasoningToken: 81
Control Request:
```yaml
id: cc1b-4c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".:"
      - README.md
      - "./facade:"
      - README.md
      - "./index:"
      - README.md
      - "./kernel:"
      - README.md
      - "./manager:"
      - README.md
      - "./memory:"
      - README.md
      - "./model:"
      - README.md
      - "./op:"
      - README.md
      - "./planner:"
      - README.md
      - "./precision:"
      - README.md
      - "./processor:"
      - README.md
      - "./queue:"
      - README.md
      - "./segment:"
      - README.md
      - "./subengine:"
      - README.md
      - "./tokenize:"
      - README.md
```
/answer cc1b-4c allow
Control Request:
```yaml
id: cc1b-4d
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "[a-z_]+\\(\\)|fn [a-z_]+|`[a-z_]+::[a-z_]+`"
    include:
    - README.md
    limit: 20
  reason: "Sicherstellen, dass keine Funktionsnamen in den READMEs stehen (Umbenennungs-Robustheit)"
```
/answer cc1b-4d allow
Control Request:
```yaml
id: cc1b-4e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
```
/answer cc1b-4e allow
Result Stats: id=3da6f405-9126-4575-ae25-c66a27d28d06, total: 265166, in: 54, out: 61056, reason: 0, read: 3534059, write: 204056, start: 1793135548161, end: 1793135559086, time: 10855, total: 478525, io: 0, control: 70, sent: 0, recv: 269
Ich habe 15 README-Dateien angelegt: eine für `resident/` und je eine für die 14 Unterverzeichnisse. Die Texte sind auf Englisch und stammen aus dem Code. Gebaut oder getestet habe ich nichts, weil nur Markdown dazukam.

**Aufbau**
- **Wurzel-`README.md`:** Sie enthält die Projektkonzeption einmal. Dazu kommen die Schichtenübersicht, der Ablauf einer Anfrage und eine Tabelle „Wo fange ich an?“. Die Wurzel-README beschreibt außerdem ihre eigenen Dateien und die Konventionen (Features, Tests).
- **Modul-READMEs:** `facade`, `processor`, `planner`, `queue`, `op`, `subengine`, `memory`, `model`, `tokenize`, `kernel`, `precision`, `index`, `segment` und `manager`. Jede hat Rolle und Konzept des Moduls sowie eine Dateitabelle (Datei → Aufgabe). Die Konzeptpunkte betreffen nur das jeweilige Modul.

**Zu deinen Vorgaben**
- **Funktionsnamen:** Es stehen keine in den Texten. Eine Suche nach Funktionsnamen blieb leer. Sie nennen nur Dateinamen und wenige zentrale Typen. Veralten können sie also nur durch hinzugefügte, gelöschte oder verschobene Dateien.
- **Verweise:** Die Konzeptdokumente in `project/resident` sind nirgends verlinkt. Das hast du bei meiner Verzeichnisabfrage dort bemängelt. Querverweise zwischen Modulen gibt es nur in der Wurzel-README.

**Auslegung eines Satzes**
Deinen Satz „Die jede Umbenennung einer function soll eine Änderung benötigen“ habe ich als „soll keine Änderung benötigen“ gelesen. Dazu passt das „jedoch“ vor „Dateinamen sind final“. Falls du das Gegenteil meintest, sag Bescheid. Dann müssten die Funktionsnamen rein.

**Was aus dem Code kam und nicht aus den Konzeptdokumenten**
- **Keine produktive Engine-Implementierung:** Es gibt außerhalb von `test/` weder eine `Engine`- noch eine `Subengine`-Implementierung. Die Fassade setzt Platzhalter ein, bei denen Tasks sofort fehlschlagen. Die Bausteine sind vorhanden, ihre Zusammensetzung zu echten CPU-/GPU-Engines fehlt noch. Das steht im Abschnitt „Status“ der Wurzel-README.
- **Namensdubletten:** Mehrere Namen bedeuten je nach Modul etwas anderes, zum Beispiel `Pooling`, `ScorerKind`, `Payload`, `Resident`, `DeviceHandle`, `ResourceState`, `Lease` und `Resolution`. Die Wurzel-README listet sie in einer Tabelle, weil sie die Einordnung erschweren.
- **Widersprüche zum Konzept:**
  - Der Planner lehnt Similarity mit Cross-Encoder derzeit ab.
  - Segmente sind nicht schlechthin unveränderlich: Die Slots darin werden per Commit angehängt.
  - Der Default-Compute-Dtype (GPU f16, CPU f32) ist im Code festgelegt.

Die Tabelle der Namensdubletten und der „Status“-Absatz sind die Stellen, die am ehesten veralten. Falls dir das zu viel Querverweis ist, kann ich beide entfernen.

Die Dateien liegen in `/home/user/xyan/xy.ai.workbench/rag/src/resident/`.
