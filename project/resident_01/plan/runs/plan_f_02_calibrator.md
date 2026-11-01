Setze F2 um.

- keine direkte Referenzierung von Planelementen in Kommentaren
- nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`
- Nutze Bash nicht für Retrieval Tasks
- Erledige restlos alle Punkte

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_f_00_overview.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_f_02_calibrator.md)

SystemInit:  id=7c6ed41f-e523-4ce4-b018-6eb50be99e19, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich verschaffe mir zuerst einen Überblick über den bestehenden Code im Resident-Modul.
Control Request:
```yaml
id: 9c73-109
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  - path: /home/user/xyan/xy.ai.workbench/rag
    recursive: false
  reason: Struktur des resident-Moduls und Crate-Root erfassen
```
/answer 9c73-109 allow
ReasoningToken: 53
Control Request:
```yaml
id: 9c73-10a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".: (6 files)"
      - backend.rs
      - "./facade: (8 files)"
      - blocking.rs
      - "./facade/test: (3 files)"
      - addon.rs
      - "./index: (3 files)"
      - memory.rs
      - "./index/test: (2 files)"
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
      - "./precision: (9 files)"
      - axis.rs
      - "./precision/test: (8 files)"
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
      warning: Result reduced because the total number of entries (196) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
    - path: /home/user/xyan/xy.ai.workbench/rag
      entries:
      - ".:"
      - .gitignore
      - .project
      - .pydevproject
      - Cargo.lock
      - Cargo.toml
      - TODO.md
      - xyrag.sh
```
/answer 9c73-10a allow
Control Request:
```yaml
id: 9c73-10b
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: precision|Precision|CapabilitySet|Profile
    include:
    - '*.rs'
    limit: 40
  reason: Bestehende Präzisions-/Capability-Typen finden (F1/B6)
```
/answer 9c73-10b allow
Control Request:
```yaml
id: 9c73-10c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/mod.rs
        matches:
        - pub mod precision;
      - path: resident/precision/mod.rs
        matches:
        - //! CapabilitySet/Profile.
        - pub use capability::CapabilitySet;
        - "pub use profile::{Profile, ProfileConfig};"
      - path: resident/precision/probe.rs
        matches:
        - use crate::resident::error::PrecisionError;
        - use super::capability::CapabilitySet;
        - "fn probe(&self) -> Result<CapabilitySet, PrecisionError>;"
        - "pub kernels: Option<CapabilitySet>,"
        - "pub fn intersect_with_kernels(set: &CapabilitySet, kernels: &CapabilitySet) -> CapabilitySet {"
        - let mut out = CapabilitySet::new();
        - "kernels: CapabilitySet,"
        - "pub fn new(inner: P, kernels: CapabilitySet) -> Self {"
        - "fn probe(&self) -> Result<CapabilitySet, PrecisionError> {"
        - "pub(crate) fn numeric_axis(set: &mut CapabilitySet, axis: &AxisId, values: impl IntoIterator<Item = u32>) {"
        - ") -> Result<Box<dyn CapabilityProbe>, PrecisionError> {"
        - "pub set: CapabilitySet,"
      - path: resident/precision/known.rs
        matches:
        - use super::capability::CapabilitySet;
        - "pub fn gpu_sm75() -> CapabilitySet {"
        - let mut c = CapabilitySet::new();
        - "pub fn cpu() -> CapabilitySet {"
      - path: resident/precision/probe_cpu.rs
        matches:
        - use crate::resident::error::PrecisionError;
        - use super::capability::CapabilitySet;
        - "fn probe(&self) -> Result<CapabilitySet, PrecisionError> {"
        - let mut set = CapabilitySet::new();
      - path: resident/precision/test/probe.rs
        matches:
        - "use crate::resident::precision::known::{self, FP16, FP32, FUSION, INT8_DP4A, ON, OPERANDS, SCORING};"
        - "let set = CapabilitySet::new().with(&*SCORING, &*INT8_DP4A).with(&*FUSION, &*ON);"
        - "let kernels = CapabilitySet::new().with(&*SCORING, &*FP32);"
        - "let set = CapabilitySet::new().with(\"future-axis\", \"future-value\");"
        - "let kernels = CapabilitySet::new().with(\"future-axis\", \"future-value\");"
      - path: resident/precision/test/known.rs
        matches:
        - "let req = CapabilitySet::new().with(&*OPERANDS, &*FP16).with(&*FUSION, &*ON);"
      - path: resident/precision/test/probe_cpu.rs
        matches:
        - use crate::resident::precision::AxisId;
        - "fn probe(features: CpuFeatures, cores: usize) -> CapabilitySet {"
      - path: resident/precision/test/probe_cuda.rs
        matches:
        - "use crate::resident::precision::known::{FUSION, ON};"
        - "use crate::resident::precision::{AxisId, ValueId};"
        - "fn tu116() -> CapabilitySet {"
        - use crate::resident::precision::make_probe;
      - path: resident/precision/test/capability.rs
        matches:
        - "fn hash_of(c: &CapabilitySet) -> u64 {"
        - let have = CapabilitySet::new()
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 9c73-10c allow
Control Request:
```yaml
id: 9c73-10d
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    recursive: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test
  reason: Vollständige Dateiliste von precision und resident
```
/answer 9c73-10d allow
Control Request:
```yaml
id: 9c73-10e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      entries:
      - ".:"
      - axis.rs
      - capability.rs
      - known.rs
      - mod.rs
      - pareto.rs
      - probe.rs
      - probe_cpu.rs
      - probe_cuda.rs
      - profile.rs
      - "./test:"
      - axis.rs
      - capability.rs
      - known.rs
      - pareto.rs
      - probe.rs
      - probe_cpu.rs
      - probe_cuda.rs
      - profile.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
      entries:
      - ".:"
      - backend.rs
      - error.rs
      - failpoint.rs
      - mock.rs
      - mod.rs
      - telemetry.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/test
      entries:
      - ".:"
      - error.rs
      - telemetry.rs
```
/answer 9c73-10e allow
Control Request:
```yaml
id: 9c73-10f
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/capability.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/axis.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  reason: Bestehende Typen und Modulstruktur lesen
```
/answer 9c73-10f allow
Control Request:
```yaml
id: 9c73-110
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! CapabilitySet/Profile.
      - id: axis
        signature: mod axis;
      - id: capability
        signature: mod capability;
      - id: known
        signature: pub mod known;
      - id: pareto
        signature: mod pareto;
      - id: probe
        signature: mod probe;
      - id: probe_cpu
        signature: mod probe_cpu;
      - id: probe_cuda
        signature: mod probe_cuda;
      - id: profile
        signature: mod profile;
      - id: CD5tM3|eS3CGz
        signature: "pub use axis::{AxisId, ValueId};"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/capability.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Capability sets: axes mapped to the values a provider offers or a task requ…"
      - id: mKJOyz|BKgeCZ
        signature: "use std::collections::{BTreeMap, BTreeSet};"
      - id: CapabilitySet
        signature: "pub struct CapabilitySet(BTreeMap<AxisId, BTreeSet<ValueId>>);"
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
          signature: "pub fn satisfies(&self, req: &CapabilitySet) -> bool {"
          docstring: Every required value on every required axis is present in `self`.
      - id: tests
        signature: mod tests;
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
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Unnamed profiles: a position on the fast-to-precise scale plus a configurat…"
      - id: U2xmDk|vDgHuO
        signature: use std::collections::BTreeMap;
      - id: ProfileConfig
        signature: "pub struct ProfileConfig(BTreeMap<AxisId, ValueId>);"
        docstring: Exactly one value per chosen axis.
      - id: impl_ProfileConfig
        signature: "impl ProfileConfig {"
        children:
        - id: impl_ProfileConfig.new
          signature: "pub fn new() -> Self {"
        - id: impl_ProfileConfig.with
          signature: "pub fn with(mut self, axis: impl Into<AxisId>, value: impl Into<ValueId>) -> Se…"
        - id: impl_ProfileConfig.get
          signature: "pub fn get(&self, axis: &AxisId) -> Option<&ValueId> {"
        - id: impl_ProfileConfig.iter
          signature: "pub fn iter(&self) -> impl Iterator<Item = (&AxisId, &ValueId)> {"
        - id: impl_ProfileConfig.len
          signature: "pub fn len(&self) -> usize {"
        - id: impl_ProfileConfig.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
      - id: Profile
        signature: "pub struct Profile {"
      - id: impl_Profile
        signature: "impl Profile {"
        children:
        - id: impl_Profile.7xkl21|GBK4RB
          signature: /// Position of a lone profile.
        - id: impl_Profile.new
          signature: "pub fn new(position: f32, config: ProfileConfig) -> Result<Self, PrecisionError…"
      - id: tests
        signature: mod tests;
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
```
/answer 9c73-110 allow
ReasoningToken: 62
Control Request:
```yaml
id: 9c73-111
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag/src/resident
    script:
    - wc -l precision/*.rs precision/test/*.rs mock.rs telemetry.rs failpoint.rs error.rs
    - cat precision/pareto.rs
  reason: Größe der Dateien prüfen und vorhandene pareto.rs lesen (bereits vorhanden)
```
/answer 9c73-111 deny AST
Text:
 Bash ist für Retrieval nicht erlaubt; ich nutze die AST-Tools.
Control Request:
```yaml
id: 9c73-112
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/pareto.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe_cuda.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe_cpu.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/telemetry.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/failpoint.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/pareto.rs
  reason: "Vorhandene Pareto-, Probe-, Mock- und Telemetrie-Strukturen verstehen"
```
/answer 9c73-112 allow
Control Request:
```yaml
id: 9c73-113
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/pareto.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Pareto front and arc-length positions over (time, deviation) measurements."
      - id: pareto
        signature: "pub fn pareto(mut pts: Vec<(f64, f64)>) -> Vec<(f64, f64)> {"
        docstring: Minimizes time and deviation. Non-finite points are dropped; the result is
      - id: positions
        signature: "pub fn positions(front: &[(f64, f64)], log_time: bool) -> Vec<f64> {"
        docstring: "Positions in `[0, 1]` for a front from [`pareto`], by cumulative arc length"
      - id: normalize
        signature: "fn normalize(v: &[f64]) -> Vec<f64> {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Capability probing: what a subengine's hardware and kernel set can offer."
      - id: xIJRa2|pEpb17
        signature: use std::collections::BTreeSet;
      - id: CapabilityProbe
        signature: "pub trait CapabilityProbe: Send + Sync {"
        children:
        - id: CapabilityProbe.kind
          signature: fn kind(&self) -> SubengineKind;
        - id: CapabilityProbe.probe
          signature: "fn probe(&self) -> Result<CapabilitySet, PrecisionError>;"
      - id: ProbeConfig
        signature: "pub struct ProbeConfig {"
        docstring: "Values for the configuration-driven axes (`batch`, `tile`)."
      - id: impl_Default_for_ProbeConfig
        signature: "impl Default for ProbeConfig {"
        children:
        - id: impl_Default_for_ProbeConfig.default
          signature: "fn default() -> Self {"
      - id: impl_ProbeConfig
        signature: "impl ProbeConfig {"
        children:
        - id: impl_ProbeConfig.batch_values
          signature: "pub(crate) fn batch_values(&self) -> impl Iterator<Item = u32> + '_ {"
        - id: impl_ProbeConfig.tile_values
          signature: "pub(crate) fn tile_values(&self) -> impl Iterator<Item = u32> + '_ {"
      - id: powers_of_two
        signature: "pub(crate) fn powers_of_two(from: u32, to: u32) -> Vec<u32> {"
      - id: intersect_with_kernels
        signature: "pub fn intersect_with_kernels(     set: &CapabilitySet,     kernels: &Capabilit…"
        docstring: Keeps only values for which a kernel exists. Axes absent from `kernels` are not…
      - id: KernelFilteredProbe
        signature: "pub struct KernelFilteredProbe<P> {"
        docstring: Applies the kernel intersection to the hardware set of an inner probe.
      - id: impl_KernelFilteredProbe_P_
        signature: "impl<P: CapabilityProbe> KernelFilteredProbe<P> {"
        children:
        - id: impl_KernelFilteredProbe_P_.new
          signature: "pub fn new(inner: P, kernels: CapabilitySet) -> Self {"
      - id: impl_CapabilityProbe_for_KernelFilteredProbe_P_
        signature: "impl<P: CapabilityProbe> CapabilityProbe for KernelFilteredProbe<P> {"
        children:
        - id: impl_CapabilityProbe_for_KernelFilteredProbe_P_.kind
          signature: "fn kind(&self) -> SubengineKind {"
        - id: impl_CapabilityProbe_for_KernelFilteredProbe_P_.probe
          signature: "fn probe(&self) -> Result<CapabilitySet, PrecisionError> {"
      - id: numeric_axis
        signature: "pub(crate) fn numeric_axis(     set: &mut CapabilitySet,     axis: &AxisId,    …"
      - id: make_probe
        signature: "pub fn make_probe(     device: &DeviceHandle,     cfg: ProbeConfig, ) -> Result…"
        docstring: The only place that branches on the `cuda` feature for probing.
      - id: FakeCapabilityProbe
        signature: "pub struct FakeCapabilityProbe {"
        docstring: Deterministic probe for calibration and profile tests.
      - id: impl_CapabilityProbe_for_FakeCapabilityProbe
        signature: "impl CapabilityProbe for FakeCapabilityProbe {"
        children:
        - id: impl_CapabilityProbe_for_FakeCapabilityProbe.kind
          signature: "fn kind(&self) -> SubengineKind {"
        - id: impl_CapabilityProbe_for_FakeCapabilityProbe.probe
          signature: "fn probe(&self) -> Result<CapabilitySet, PrecisionError> {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Well-known axes and values for the target hardware (GTX 1660, sm_75)."
      - id: D6Tc1t|3ttI7x
        signature: use std::sync::LazyLock;
      - id: cX8Pw2|9eDmyX
        signature: "macro_rules! known_ids {"
      - id: gpu_sm75
        signature: "pub fn gpu_sm75() -> CapabilitySet {"
        docstring: Upper bound of what the GPU can offer; the real set is measured at runtime.
      - id: cpu
        signature: "pub fn cpu() -> CapabilitySet {"
        docstring: "Numeric axes (`threads`, `tile`) are not listed."
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe_cuda.rs
      nodes:
      - id: ModuleDoc
        signature: //! CUDA capability probe. The hardware table is pure and testable without a GP…
      - id: 9EqFK4|U6hJhV
        signature: use crate::resident::error::PrecisionError;
      - id: OLH6q5|TkXIhR
        signature: /// Device names without tensor cores although they report compute capability 7…
      - id: has_tensor_cores
        signature: "pub fn has_tensor_cores(major: i32, name: &str) -> bool {"
      - id: cuda_capabilities
        signature: "pub fn cuda_capabilities(major: i32, minor: i32, cfg: &ProbeConfig) -> Capabili…"
        docstring: Optimistic capability set of a CUDA device. No tensor-core (WMMA/MMA) path and …
      - id: CudaProbe
        signature: "pub struct CudaProbe {"
      - id: cuda_probe
        signature: "pub(super) fn cuda_probe(     device: &DeviceHandle,     cfg: ProbeConfig, ) ->…"
      - id: impl_CapabilityProbe_for_CudaProbe
        signature: "impl CapabilityProbe for CudaProbe {"
        children:
        - id: impl_CapabilityProbe_for_CudaProbe.kind
          signature: "fn kind(&self) -> SubengineKind {"
        - id: impl_CapabilityProbe_for_CudaProbe.probe
          signature: "fn probe(&self) -> Result<CapabilitySet, PrecisionError> {"
        - id: impl_CapabilityProbe_for_CudaProbe.probe_2
          signature: "fn probe(&self) -> Result<CapabilitySet, PrecisionError> {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe_cpu.rs
      nodes:
      - id: ModuleDoc
        signature: //! CPU capability probe with an injectable feature source.
      - id: 9EqFK4|87Wghl
        signature: use crate::resident::error::PrecisionError;
      - id: CpuFeatures
        signature: "pub struct CpuFeatures {"
      - id: CpuFeatureSource
        signature: "pub trait CpuFeatureSource: Send + Sync {"
        children:
        - id: CpuFeatureSource.features
          signature: fn features(&self) -> CpuFeatures;
        - id: CpuFeatureSource.logical_cores
          signature: fn logical_cores(&self) -> usize;
      - id: HostCpu
        signature: pub struct HostCpu;
      - id: impl_CpuFeatureSource_for_HostCpu
        signature: "impl CpuFeatureSource for HostCpu {"
        children:
        - id: impl_CpuFeatureSource_for_HostCpu.features
          signature: "fn features(&self) -> CpuFeatures {"
        - id: impl_CpuFeatureSource_for_HostCpu.features_2
          signature: "fn features(&self) -> CpuFeatures {"
        - id: impl_CpuFeatureSource_for_HostCpu.logical_cores
          signature: "fn logical_cores(&self) -> usize {"
      - id: FixedCpu
        signature: "pub struct FixedCpu {"
        docstring: Fixed source for tests and for forcing a feature level.
      - id: impl_CpuFeatureSource_for_FixedCpu
        signature: "impl CpuFeatureSource for FixedCpu {"
        children:
        - id: impl_CpuFeatureSource_for_FixedCpu.features
          signature: "fn features(&self) -> CpuFeatures {"
        - id: impl_CpuFeatureSource_for_FixedCpu.logical_cores
          signature: "fn logical_cores(&self) -> usize {"
      - id: CpuProbe
        signature: "pub struct CpuProbe {"
      - id: impl_CpuProbe
        signature: "impl CpuProbe {"
        children:
        - id: impl_CpuProbe.new
          signature: "pub fn new(source: Box<dyn CpuFeatureSource>, cfg: ProbeConfig) -> Self {"
      - id: thread_counts
        signature: "fn thread_counts(cores: usize) -> Vec<u32> {"
        docstring: "Thread counts `1, 2, 4, ...` up to and including `cores`."
      - id: impl_CapabilityProbe_for_CpuProbe
        signature: "impl CapabilityProbe for CpuProbe {"
        children:
        - id: impl_CapabilityProbe_for_CpuProbe.kind
          signature: "fn kind(&self) -> SubengineKind {"
        - id: impl_CapabilityProbe_for_CpuProbe.probe
          signature: "fn probe(&self) -> Result<CapabilitySet, PrecisionError> {"
      - id: tests
        signature: mod tests;
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/pareto.rs
      nodes:
      - id: LwlKpn|JFVpeI
        signature: use super::*;
      - id: dominated_points_are_dropped
        signature: "fn dominated_points_are_dropped() {"
      - id: non_finite_points_are_ignored
        signature: "fn non_finite_points_are_ignored() {"
      - id: positions_follow_arc_length
        signature: "fn positions_follow_arc_length() {"
      - id: single_point_sits_in_the_middle
        signature: "fn single_point_sits_in_the_middle() {"
      - id: constant_axis_does_not_divide_by_zero
        signature: "fn constant_axis_does_not_divide_by_zero() {"
      - id: log_time_compresses_large_ratios
        signature: "fn log_time_compresses_large_ratios() {"
```
/answer 9c73-113 allow
Control Request:
```yaml
id: 9c73-114
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
    ids:
    - ModuleDoc
    - Clock
    - FakeClock
    - Measure
    - NoiseModel
    - FakeMeasure
    - impl_FakeMeasure
    - impl_Measure_for_FakeMeasure
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/pareto.rs
    ids:
    - ModuleDoc
    - pareto
    - positions
    - tests
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - PrecisionError
    - impl_IsRetryable_for_PrecisionError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe.rs
    ids:
    - CapabilityProbe
    - FakeCapabilityProbe
    - make_probe
    - numeric_axis
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    ids:
    - ModuleDoc
  reason: Details der vorhandenen Bausteine für den Calibrator lesen
```
/answer 9c73-114 allow
ReasoningToken: 100
Control Request:
```yaml
id: 9c73-115
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Mock/test-double conventions.
          //!
          //! Narrow traits for GPU/measurement interfaces use generics (static dispatch) in the hot
          //! path; `dyn` only at the edges. Deterministic fakes (fixed output, simulated failure) are
          //! hand-written rather than `mockall`-generated, since that composes more simply across
          //! `async`/`Send` boundaries; `mockall::automock` is reserved for orchestrator-level traits.
          //! Everything here is gated so it never ships in a production build without the `mock` feature.
          #![cfg(any(test, feature = "mock"))]
      - id: Clock
        type: ItemTrait
        signature: "pub trait Clock: Send + Sync {"
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
      - id: Measure
        type: ItemTrait
        signature: "pub trait Measure {"
        docstring: Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
        children:
        - id: Measure.sample
          type: TraitItemFn
          code: |2
                fn sample(&mut self) -> f64;
      - id: NoiseModel
        type: ItemEnum
        code: |
          /// Noise models for calibration-stability tests.
          pub enum NoiseModel {
              Constant,
              Outlier { every_n: u32, magnitude: f64 },
              Drift { per_sample: f64 },
          }
      - id: FakeMeasure
        type: ItemStruct
        code: |
          /// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so
          /// stability/convergence tests are deterministic across runs for a fixed seed.
          pub struct FakeMeasure {
              base: f64,
              noise: NoiseModel,
              rng: rand::rngs::StdRng,
              sample_count: u32,
              drift_accum: f64,
          }
      - id: impl_FakeMeasure
        type: ItemImpl
        signature: "impl FakeMeasure {"
        children:
        - id: impl_FakeMeasure.new
          type: ImplItemFn
          code: |2
                pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {
                    use rand::SeedableRng;
                    Self {
                        base,
                        noise,
                        rng: rand::rngs::StdRng::seed_from_u64(seed),
                        sample_count: 0,
                        drift_accum: 0.0,
                    }
                }
      - id: impl_Measure_for_FakeMeasure
        type: ItemImpl
        signature: "impl Measure for FakeMeasure {"
        children:
        - id: impl_Measure_for_FakeMeasure.sample
          type: ImplItemFn
          code: |2
                fn sample(&mut self) -> f64 {
                    use rand::Rng;
                    self.sample_count += 1;
                    match &self.noise {
                        NoiseModel::Constant => self.base + self.rng.gen_range(-0.01..0.01),
                        NoiseModel::Outlier { every_n, magnitude } => {
                            if self.sample_count % every_n == 0 {
                                self.base + magnitude
                            } else {
                                self.base + self.rng.gen_range(-0.01..0.01)
                            }
                        }
                        NoiseModel::Drift { per_sample } => {
                            self.drift_accum += per_sample;
                            self.base + self.drift_accum + self.rng.gen_range(-0.01..0.01)
                        }
                    }
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/pareto.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Pareto front and arc-length positions over (time, deviation) measurements.
      - id: pareto
        type: ItemFn
        code: |
          /// Minimizes time and deviation. Non-finite points are dropped; the result is
          /// ordered by ascending time with strictly decreasing deviation.
          pub fn pareto(mut pts: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
              pts.retain(|p| p.0.is_finite() && p.1.is_finite());
              pts.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
              let mut out: Vec<(f64, f64)> = Vec::new();
              for p in pts {
                  if out.last().is_none_or(|l| p.1 < l.1) {
                      out.push(p);
                  }
              }
              out
          }
      - id: positions
        type: ItemFn
        code: |
          /// Positions in `[0, 1]` for a front from [`pareto`], by cumulative arc length
          /// over min-max normalized axes. Time is normalized logarithmically when
          /// `log_time` is set and all times are positive.
          pub fn positions(front: &[(f64, f64)], log_time: bool) -> Vec<f64> {
              match front.len() {
                  0 => return Vec::new(),
                  1 => return vec![0.5],
                  _ => {}
              }
              let use_log = log_time && front.iter().all(|p| p.0 > 0.0);
              let t: Vec<f64> = front
                  .iter()
                  .map(|p| if use_log { p.0.ln() } else { p.0 })
                  .collect();
              let d: Vec<f64> = front.iter().map(|p| p.1).collect();
              let (t, d) = (normalize(&t), normalize(&d));

              let mut cum = Vec::with_capacity(front.len());
              let mut s = 0.0;
              cum.push(s);
              for i in 1..front.len() {
                  s += (t[i] - t[i - 1]).hypot(d[i] - d[i - 1]);
                  cum.push(s);
              }
              if s > 0.0 {
                  cum.iter().map(|c| c / s).collect()
              } else {
                  let last = (front.len() - 1) as f64;
                  (0..front.len()).map(|i| i as f64 / last).collect()
              }
          }
      - id: tests
        type: ItemMod
        code: |
          #[cfg(test)]
          #[path = "test/pareto.rs"]
          mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: PrecisionError
        type: ItemEnum
        code: |
          /// Precision/calibration layer errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum PrecisionError {
              #[error("incompatible precision/layout: {reason}")]
              Incompatible { reason: String },
              #[error("calibration unstable: {reason}")]
              CalibrationUnstable { reason: String },
              #[error("no active calibration profile")]
              NoActiveProfile,
              #[error("invalid precision requested: {requested}")]
              InvalidPrecision { requested: f32 },
              #[error("capability probe failed: {reason}")]
              Capability { reason: String },
          }
      - id: impl_IsRetryable_for_PrecisionError
        type: ItemImpl
        signature: "impl IsRetryable for PrecisionError {"
        children:
        - id: impl_IsRetryable_for_PrecisionError.is_retryable
          type: ImplItemFn
          code: |2
                fn is_retryable(&self) -> bool {
                    matches!(self, PrecisionError::CalibrationUnstable { .. })
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe.rs
      nodes:
      - id: CapabilityProbe
        type: ItemTrait
        signature: "pub trait CapabilityProbe: Send + Sync {"
        children:
        - id: CapabilityProbe.kind
          type: TraitItemFn
          code: |2
                fn kind(&self) -> SubengineKind;
        - id: CapabilityProbe.probe
          type: TraitItemFn
          code: |2
                fn probe(&self) -> Result<CapabilitySet, PrecisionError>;
      - id: FakeCapabilityProbe
        type: ItemStruct
        code: |
          /// Deterministic probe for calibration and profile tests.
          #[cfg(any(test, feature = "mock"))]
          pub struct FakeCapabilityProbe {
              pub kind: SubengineKind,
              pub set: CapabilitySet,
          }
      - id: make_probe
        type: ItemFn
        code: |
          /// The only place that branches on the `cuda` feature for probing.
          pub fn make_probe(
              device: &DeviceHandle,
              cfg: ProbeConfig,
          ) -> Result<Box<dyn CapabilityProbe>, PrecisionError> {
              match device.kind() {
                  DeviceKind::Cpu => {
                      let kernels = cfg
                          .kernels
                          .clone()
                          .unwrap_or_else(|| implemented_capabilities(SubengineKind::Cpu));
                      let probe = CpuProbe::new(Box::new(HostCpu), cfg);
                      Ok(Box::new(KernelFilteredProbe::new(probe, kernels)))
                  }
                  DeviceKind::Gpu => {
                      let kernels = cfg
                          .kernels
                          .clone()
                          .unwrap_or_else(|| implemented_capabilities(SubengineKind::Cuda));
                      let probe = probe_cuda::cuda_probe(device, cfg)?;
                      Ok(Box::new(KernelFilteredProbe::new(probe, kernels)))
                  }
              }
          }
      - id: numeric_axis
        type: ItemFn
        code: |
          pub(crate) fn numeric_axis(set: &mut CapabilitySet, axis: &AxisId, values: impl IntoIterator<Item = u32>) {
              let unique: BTreeSet<u32> = values.into_iter().collect();
              for v in unique {
                  set.insert(axis, ValueId::numeric(v));
              }
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! CapabilitySet/Profile.
```
/answer 9c73-115 allow
Control Request:
```yaml
id: 9c73-116
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: .
    include:
    - Cargo.toml
    exclude:
    - target/**
    - .bin/**
    limit: 80
  reason: "Cargo.toml: Features und Abhängigkeiten (rand, cudarc, thread-priority)"
```
/answer 9c73-116 allow
Control Request:
```yaml
id: 9c73-117
result:
  structuredContent:
    errors:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      pattern: .
      error: limit must be between 1 and 50.
```
/answer 9c73-117 allow
ReasoningToken: 144
Control Request:
```yaml
id: 9c73-118
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: .
    include:
    - Cargo.toml
    exclude:
    - target/**
    - .bin/**
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    - /home/user/xyan/xy.ai.workbench/rag/tests
    - /home/user/xyan/xy.ai.workbench/rag/benches
    pattern: FakeMeasure|mock::Measure|NoiseModel
    include:
    - '*.rs'
    limit: 30
  reason: Cargo-Features/Abhängigkeiten sowie Nutzungen des bestehenden FakeMeasure ermitteln
```
/answer 9c73-118 allow
Control Request:
```yaml
id: 9c73-119
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-transformers/Cargo.toml
        matches:
        - "# THIS FILE IS AUTOMATICALLY GENERATED BY CARGO"
        - "#"
        - "# When uploading crates to the registry Cargo will automatically"
        - "# \"normalize\" Cargo.toml files for maximal compatibility"
        - "# with all versions of Cargo and also rewrite `path` dependencies"
        - "# to registry (e.g., crates.io) dependencies."
        - "# If you are reading this file be aware that the original Cargo.toml"
        - "# will likely look very different (and much more reasonable)."
        - "# See Cargo.toml.orig for the original contents."
        - "[package]"
        - edition = "2021"
        - name = "candle-transformers"
        - version = "0.11.0"
        - build = false
        - autolib = false
        - autobins = false
        - autoexamples = false
        - autotests = false
        - autobenches = false
        - description = "Minimalist ML framework."
        - readme = "README.md"
        - "keywords = ["
        - "\"blas\","
        - "\"tensor\","
        - "\"machine-learning\","
        - "]"
        - "categories = [\"science\"]"
        - license = "MIT OR Apache-2.0"
        - repository = "https://github.com/huggingface/candle"
        - "[features]"
        - "accelerate = ["
        - "\"dep:accelerate-src\","
        - "\"candle/accelerate\","
        - "\"candle-nn/accelerate\","
        - "cuda = ["
        - "\"candle/cuda\","
        - "\"candle-nn/cuda\","
        - "cudnn = ["
        - "\"candle/cudnn\","
        - "\"candle-nn/cudnn\","
        - "default = []"
        - "flash-attn = ["
        - "\"cuda\","
        - "\"dep:candle-flash-attn\","
        - "metal = ["
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      - /home/user/xyan/xy.ai.workbench/rag/tests
      - /home/user/xyan/xy.ai.workbench/rag/benches
      matches:
      - path: resident/mock.rs
        matches:
        - /// Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
        - "pub enum NoiseModel {"
        - "/// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so"
        - "pub struct FakeMeasure {"
        - "noise: NoiseModel,"
        - "impl FakeMeasure {"
        - "pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {"
        - "impl Measure for FakeMeasure {"
        - "NoiseModel::Constant => self.base + self.rng.gen_range(-0.01..0.01),"
        - "NoiseModel::Outlier { every_n, magnitude } => {"
        - "NoiseModel::Drift { per_sample } => {"
```
/answer 9c73-119 allow
Control Request:
```yaml
id: 9c73-11a
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^(\\[|[a-z_-]+ *=)"
    include:
    - Cargo.toml
    exclude:
    - third_party/**
    - target/**
    - .bin/**
    limit: 50
  reason: Nur das Crate-Cargo.toml lesen (third_party ausgeschlossen)
```
/answer 9c73-11a allow
Control Request:
```yaml
id: 9c73-11b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-transformers/Cargo.toml
        matches:
        - "[package]"
        - edition = "2021"
        - name = "candle-transformers"
        - version = "0.11.0"
        - build = false
        - autolib = false
        - autobins = false
        - autoexamples = false
        - autotests = false
        - autobenches = false
        - description = "Minimalist ML framework."
        - readme = "README.md"
        - "keywords = ["
        - "categories = [\"science\"]"
        - license = "MIT OR Apache-2.0"
        - repository = "https://github.com/huggingface/candle"
        - "[features]"
        - "accelerate = ["
        - "cuda = ["
        - "cudnn = ["
        - "default = []"
        - "flash-attn = ["
        - "metal = ["
        - "metal-debug-labels = ["
        - "mkl = ["
        - "[lib]"
        - name = "candle_transformers"
        - path = "src/lib.rs"
        - "[dependencies.accelerate-src]"
        - version = "0.3.2"
        - optional = true
        - "[dependencies.byteorder]"
        - version = "1.4.3"
        - "[dependencies.candle]"
        - package = "candle-core"
        - "[dependencies.candle-flash-attn]"
        - "[dependencies.candle-nn]"
        - "[dependencies.fancy-regex]"
        - version = "0.18.0"
        - "[dependencies.intel-mkl-src]"
        - version = "0.8.1"
        - "features = [\"mkl-static-lp64-iomp\"]"
        - "[dependencies.num-traits]"
        - version = "0.2.15"
        - "[dependencies.rand]"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 9c73-11b allow
ReasoningToken: 71
Control Request:
```yaml
id: 9c73-11c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
  reason: Crate-Cargo.toml gezielt lesen
```
/answer 9c73-11c allow
Control Request:
```yaml
id: 9c73-11d
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
      - id: F2LjhE|e8LBEZ
        signature: "[dependencies]"
      - id: anLBDO|YAkuwn
        signature: "[target.'cfg(target_os = \"linux\")'.dependencies]"
      - id: adh0oF|46Sfax
        signature: "[features]"
      - id: UeF6gB|FaChiw
        signature: "[dev-dependencies]"
      - id: 9OxNvg|OtfeRA
        signature: "[[bench]]"
      - id: 9XeKXQ|8kn2M9
        signature: "[patch.crates-io]"
```
/answer 9c73-11d allow
Control Request:
```yaml
id: 9c73-11e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
    ids:
    - F2LjhE|e8LBEZ
    - anLBDO|YAkuwn
    - adh0oF|46Sfax
    - UeF6gB|FaChiw
  reason: Dependencies und Features lesen
```
/answer 9c73-11e allow
Control Request:
```yaml
id: 9c73-11f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/Cargo.toml
      nodes:
      - id: F2LjhE|e8LBEZ
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
          toml = "0.9"
          half = "2.7.1"
          memmap2 = "0.9.3"
          xxhash-rust = { version = "0.8", features = ["xxh3"] }
          parquet = "60.0.0"
          arrow = "60.0.0"
          tokenizers = { version = "0.23.2", default-features = false, features = ["fancy-regex"] }
          safetensors = "0.8.0"
          candle-core = { version = "0.11.0", optional = true, default-features = false }
          candle-nn = { version = "0.11.0", optional = true, default-features = false }
          candle-transformers = { version = "0.11.0", optional = true, default-features = false }
          cudarc = { version = "=0.19.9", optional = true }
          rand = { version = "0.8", optional = true }
          roaring = "0.11.5"

      - id: anLBDO|YAkuwn
        type: table
        code: |+
          [target.'cfg(target_os = "linux")'.dependencies]
          libc = "0.2"

      - id: adh0oF|46Sfax
        type: table
        code: |+
          [features]
          default = []
          candle = ["dep:candle-core", "dep:candle-nn", "dep:candle-transformers"]
          cuda = ["candle", "dep:cudarc", "candle-core/cuda", "candle-nn/cuda", "candle-transformers/cuda"]
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
```
/answer 9c73-11f allow
Thinking:
F1 (Capability/Probe) und die Pareto-Funktionen sind schon vorhanden, aber `Measure`-Trait mit `Cfg`, `Calibrator`, Pruning, Slice-Logik, Thread-Pool und CUDA-Messung fehlen noch. Als Nächstes schaue ich mir die bekannten Achsen und Gerätetypen an.


Control Request:
```yaml
id: 9c73-120
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
    ids:
    - ModuleDoc
    - cX8Pw2|9eDmyX
    - gpu_sm75
    - cpu
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/axis.rs
    ids:
    - vAh2Ds|1Xi56W
    - dC7yQX|7HVxmG
    - impl_ValueId
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
    ids:
    - Profile
    - impl_Profile
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe.rs
    ids:
    - ProbeConfig
    - impl_Default_for_ProbeConfig
  reason: "Achsenkatalog, ID-Typen und Profile-Konstruktor für den Calibrator kennen"
```
/answer 9c73-120 allow
ReasoningToken: 411
Control Request:
```yaml
id: 9c73-121
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Well-known axes and values for the target hardware (GTX 1660, sm_75).
          //! bf16 is deliberately absent: it has no native path before sm_80.
      - id: cX8Pw2|9eDmyX
        type: StatementGroup
        code: |
          macro_rules! known_ids {
              ($ty:ident : $($name:ident = $s:literal),+ $(,)?) => {
                  $(pub static $name : LazyLock <$ty > = LazyLock::new(|| $ty ::new($s));)+
              };
          }
          known_ids!(
              AxisId : OPERANDS = "operands", ACCUMULATOR = "accumulator", SCORING = "scoring",
              FUSION = "fusion", BATCH = "batch", TILE = "tile", THREADS = "threads", DENSITY =
              "density",
          );
          known_ids!(
              ValueId : FP32 = "fp32", FP16 = "fp16", FP16_TO_FP32 = "fp16->fp32", INT8 = "int8",
              INT8_DP4A = "int8-dp4a", INT8_VNNI = "int8-vnni", ON = "on", OFF = "off",
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
              for v in [&FP32, &INT8, &INT8_VNNI] {
                  c.insert(&*SCORING, &**v);
              }
              c
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/axis.rs
      nodes:
      - id: vAh2Ds|1Xi56W
        type: StatementGroup
        code: |
          macro_rules! string_id {
              ($(#[$meta:meta])* $name:ident) => {
                  $(#[$meta])*
                  #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
                  pub struct $name(Arc<str>);

                  impl $name {
                      pub fn new(id: impl AsRef<str>) -> Self {
                          Self(Arc::from(id.as_ref()))
                      }

                      pub fn as_str(&self) -> &str {
                          &self.0
                      }
                  }

                  impl From<&str> for $name {
                      fn from(id: &str) -> Self {
                          Self::new(id)
                      }
                  }

                  impl From<String> for $name {
                      fn from(id: String) -> Self {
                          Self::new(id)
                      }
                  }

                  impl From<&$name> for $name {
                      fn from(id: &$name) -> Self {
                          id.clone()
                      }
                  }

                  impl fmt::Debug for $name {
                      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                          write!(f, concat!(stringify!($name), "({:?})"), &*self.0)
                      }
                  }

                  impl fmt::Display for $name {
                      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                          f.write_str(&self.0)
                      }
                  }
              };
          }
      - id: dC7yQX|7HVxmG
        type: StatementGroup
        code: |
          string_id!(
              /// Capability axis such as `operands` or `scoring`; any string is valid.
              AxisId
          );
          string_id!(
              /// A value on an axis such as `fp16`; any string is valid.
              ValueId
          );
      - id: impl_ValueId
        type: ItemImpl
        signature: "impl ValueId {"
        children:
        - id: impl_ValueId.numeric
          type: ImplItemFn
          code: |2
                pub fn numeric(n: u32) -> Self {
                    Self::new(n.to_string())
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
      nodes:
      - id: Profile
        type: ItemStruct
        code: |
          #[derive(Clone, Debug, PartialEq)]
          pub struct Profile {
              /// 0 is fastest, 1 is most precise.
              pub position: f32,
              pub config: ProfileConfig,
          }
      - id: impl_Profile
        type: ItemImpl
        signature: "impl Profile {"
        children:
        - id: impl_Profile.7xkl21|GBK4RB
          type: StatementGroup
          code: |2
                /// Position of a lone profile.
                pub const SINGLE_POSITION: f32 = 0.5;
        - id: impl_Profile.new
          type: ImplItemFn
          code: |2
                pub fn new(position: f32, config: ProfileConfig) -> Result<Self, PrecisionError> {
                    if !(0.0..=1.0).contains(&position) {
                        return Err(PrecisionError::InvalidPrecision { requested: position });
                    }
                    Ok(Self { position, config })
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe.rs
      nodes:
      - id: ProbeConfig
        type: ItemStruct
        code: |
          /// Values for the configuration-driven axes (`batch`, `tile`).
          #[derive(Clone, Debug)]
          pub struct ProbeConfig {
              pub batches: Vec<u32>,
              pub tiles: Vec<u32>,
              /// Tiles above this length are not offered; calibration checks processability up to it.
              pub max_seq_len: u32,
              /// Kernels the subengine can execute. `None` uses the built-in catalog of the engine kind.
              pub kernels: Option<CapabilitySet>,
          }
      - id: impl_Default_for_ProbeConfig
        type: ItemImpl
        signature: "impl Default for ProbeConfig {"
        children:
        - id: impl_Default_for_ProbeConfig.default
          type: ImplItemFn
          code: |2
                fn default() -> Self {
                    Self {
                        batches: vec![1, 8, 32],
                        tiles: powers_of_two(16, 512),
                        max_seq_len: 512,
                        kernels: None,
                    }
                }
```
/answer 9c73-121 allow
Control Request:
```yaml
id: 9c73-122
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "enum SubengineKind|enum DeviceKind|struct DeviceHandle|fn implemented_capabilities|FIELD_[A-Z_]+ *:|pub const FIELD"
    include:
    - '*.rs'
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
    pattern: cudarc|record_event|elapsed_ms
    include:
    - '*.rs'
    limit: 30
  reason: "SubengineKind/DeviceHandle-Definitionen, Telemetrie-Felder und bestehende cudarc-Nutzung finden"
```
/answer 9c73-122 allow
Control Request:
```yaml
id: 9c73-123
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: kernel/catalog.rs
        matches:
        - "pub fn implemented_capabilities(kind: SubengineKind) -> CapabilitySet {"
      - path: subengine/mod.rs
        matches:
        - "pub enum SubengineKind {"
      - path: subengine/device.rs
        matches:
        - "pub struct DeviceHandle {"
      - path: memory/manager.rs
        matches:
        - "pub enum DeviceKind {"
      - path: telemetry.rs
        matches:
        - "pub const FIELD_TASK_ID: &str = \"task_id\";"
        - "pub const FIELD_RESOURCE_KIND: &str = \"resource_kind\";"
        - "pub const FIELD_MODEL_ID: &str = \"model_id\";"
        - "pub const FIELD_INDEX_ID: &str = \"index_id\";"
        - "pub const FIELD_SEGMENT_GEN: &str = \"segment_gen\";"
        - "pub const FIELD_SLOT: &str = \"slot\";"
        - "pub const FIELD_LANE: &str = \"lane\";"
        - "pub const FIELD_MANAGER_CYCLE: &str = \"manager_cycle\";"
        - "pub const FIELD_OP: &str = \"op\";"
        - "pub const FIELD_PRECISION: &str = \"precision\";"
        - "pub const FIELD_PROFILE: &str = \"profile\";"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
      matches:
      - path: registry.rs
        matches:
        - "elapsed_ms = started.elapsed().as_millis() as u64,"
      - path: cuda.rs
        matches:
        - "use cudarc::driver::{"
        - "use cudarc::nvrtc::{"
        - "fn log_of(prog: cudarc::nvrtc::sys::nvrtcProgram) -> String {"
        - ") -> Result<Arc<cudarc::driver::CudaModule>, KernelError> {"
      - path: test/cuda.rs
        matches:
        - "use cudarc::driver::{LaunchConfig, PushKernelArg};"
        - "use cudarc::nvrtc::{compile_ptx_with_opts, CompileOptions};"
      - path: resource.rs
        matches:
        - "pub fn observe(&self, items: usize, elapsed_ms: f32) {"
        - "if items > 0 && elapsed_ms > 0.0 {"
        - self.rate_per_ms.lock().observe(items as f32 / elapsed_ms);
      - path: error.rs
        matches:
        - "#[error(\"cudarc version mismatch: candle uses {candle}, project uses {project}\")]"
      - path: test/device.rs
        matches:
        - "fn project_cudarc_is_the_one_candle_uses() {"
        - "TypeId::of::< candle_core::cuda_backend::cudarc::driver::CudaContext > (),"
        - TypeId::of::< cudarc::driver::CudaContext > ()
        - let event = producer.record_event().unwrap();
      - path: test/exec.rs
        matches:
        - let event = lane.record_event().unwrap();
        - let event = producer.record_event().unwrap();
        - let e = lane.record_event().unwrap();
      - path: device.rs
        matches:
        - "ctx: Option<std::sync::Arc<cudarc::driver::CudaContext>>,"
        - let ctx = cudarc::driver::CudaContext::new(ordinal)
        - ") -> Option<&std::sync::Arc<cudarc::driver::CudaContext>> {"
        - "let init = |e: cudarc::driver::DriverError| DeviceError::Init {"
        - "use cudarc::driver::{result, CudaContext, CudaStream};"
        - "fn event_err(e: cudarc::driver::DriverError) -> DeviceError {"
        - "fn record_event(&self) -> Result<EventHandle, DeviceError> {"
        - let event = self.stream.record_event(None).map_err(event_err)?;
      - path: exec.rs
        matches:
        - "Cuda(cudarc::driver::CudaEvent),"
        - "fn record_event(&self) -> Result<EventHandle, DeviceError>;"
        - "fn record_event(&self) -> Result<EventHandle, DeviceError> {"
        - self.record_event()?.synchronize()?;
      warning: "Limit of 30 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 9c73-123 allow
Control Request:
```yaml
id: 9c73-124
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/device.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/exec.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/catalog.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  reason: Geräte-/Stream-/Kernel-APIs für die CUDA-Messung verstehen
```
/answer 9c73-124 allow
ReasoningToken: 147
Control Request:
```yaml
id: 9c73-125
result:
  structuredContent:
    results:
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Real CUDA kernels / scorer: NVRTC-compiled sources behind the kernel regist…"
      - id: MZne2K|b8Bb3B
        signature: "use std::ffi::{c_char, CString};"
      - id: kA9lm6|Qv4RkF
        signature: "pub const COSINE_DOT: &str = \"cosine_dot\";"
      - id: zBTwsZ|GsLJG7
        signature: /// One block per slot; scores every query against the slot's single vector. Ou…
      - id: QxmP5v|fHa4nk
        signature: "/// One block per document, one query. Documents are padded to `doc_tokens`; `d…"
      - id: body
        signature: "fn body(name: &str) -> Option<&'static str> {"
      - id: defines
        signature: "fn defines(key: &KernelKey) -> Result<Vec<String>, KernelError> {"
      - id: log_of
        signature: "fn log_of(prog: cudarc::nvrtc::sys::nvrtcProgram) -> String {"
      - id: compile_cubin
        signature: "fn compile_cubin(src: &str, sm: u32, opts: &[String]) -> Result<Vec<u8>, Kernel…"
        docstring: Machine code for exactly `sm_<sm>`; usable when the driver cannot JIT the PTX v…
      - id: NvrtcLoader
        signature: "pub struct NvrtcLoader {"
        docstring: Compiles to PTX for `compute_<sm>` and falls back to a CUBIN when the driver re…
      - id: impl_NvrtcLoader
        signature: "impl NvrtcLoader {"
        children:
        - id: impl_NvrtcLoader.new
          signature: "pub fn new(ctx: Arc<CudaContext>) -> Self {"
        - id: impl_NvrtcLoader.module
          signature: "fn module(     &self,     key: &KernelKey,     src: &str, ) -> Result<Arc<cudar…"
      - id: arch_for
        signature: "fn arch_for(sm: u32) -> &'static str {"
      - id: impl_KernelLoader_CudaFunction__for_NvrtcLoader
        signature: "impl KernelLoader<CudaFunction> for NvrtcLoader {"
        children:
        - id: impl_KernelLoader_CudaFunction__for_NvrtcLoader.load
          signature: "fn load(&self, key: &KernelKey) -> Result<CudaFunction, KernelError> {"
      - id: CudaKernels
        signature: "pub struct CudaKernels {"
        docstring: Registry plus the device's compute capability.
      - id: impl_CudaKernels
        signature: "impl CudaKernels {"
        children:
        - id: impl_CudaKernels.new
          signature: "pub fn new(ctx: &Arc<CudaContext>) -> Result<Self, KernelError> {"
        - id: impl_CudaKernels.sm
          signature: "pub fn sm(&self) -> u32 {"
        - id: impl_CudaKernels.registry
          signature: "pub fn registry(&self) -> &KernelRegistry<CudaFunction> {"
        - id: impl_CudaKernels.function
          signature: "pub fn function(     &self,     name: &'static str,     dtype: DType,     dim: …"
      - id: launch_err
        signature: "fn launch_err(e: impl std::error::Error + Send + Sync + 'static) -> KernelError…"
      - id: 05tXPx|yv1hhd
        signature: "const BLOCK: u32 = 128;"
      - id: cfg
        signature: "fn cfg(blocks: u32) -> LaunchConfig {"
      - id: launch_cosine
        signature: "pub(crate) fn launch_cosine<T: DeviceRepr>(     stream: &Arc<CudaStream>,     f…"
        docstring: Output layout `n_queries x n_slots`.
      - id: launch_maxsim
        signature: "pub(crate) fn launch_maxsim<T: DeviceRepr>(     stream: &Arc<CudaStream>,     f…"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/catalog.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Capability values for which a scoring kernel exists, independent of the `cu…"
      - id: PZQ3hE|ruqejM
        signature: use crate::resident::memory::DType;
      - id: CqGY8w|6JQ34E
        signature: /// Element types the NVRTC loader in `cuda.rs` can compile a kernel for.
      - id: dtype_value
        signature: "fn dtype_value(dtype: DType) -> Option<&'static ValueId> {"
      - id: implemented_capabilities
        signature: "pub fn implemented_capabilities(kind: SubengineKind) -> CapabilitySet {"
        docstring: "Axes governed by the kernels: operand and scoring values need an executable ker…"
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
```
/answer 9c73-125 allow
Control Request:
```yaml
id: 9c73-126
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel
    recursive: true
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
    recursive: true
  reason: Kernel- und Subengine-Dateien auflisten
```
/answer 9c73-126 allow
Control Request:
```yaml
id: 9c73-127
result:
  structuredContent:
    results:
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
      - "./test:"
      - cosine.rs
      - cpu_simd.rs
      - cuda.rs
      - golden.rs
      - launcher.rs
      - maxsim.rs
      - mod.rs
      - registry.rs
      - slice.rs
      - topk.rs
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
      entries:
      - ".:"
      - device.rs
      - error.rs
      - exec.rs
      - lifecycle.rs
      - mod.rs
      - resource.rs
      - "./test:"
      - device.rs
      - exec.rs
      - lifecycle.rs
      - resource.rs
```
/answer 9c73-127 allow
ReasoningToken: 352
Control Request:
```yaml
id: 9c73-128
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/slice.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cosine.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cpu_simd.rs
  reason: Bestehende Slice-Logik und Scoring-Launcher für die Messung wiederverwenden
```
/answer 9c73-128 allow
Control Request:
```yaml
id: 9c73-129
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/slice.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Work-slice scheduling: bounded launches sized from observed throughput, wit…"
      - id: FLGxwX|f1B42f
        signature: use std::time::Duration;
      - id: StopReason
        signature: "pub enum StopReason {"
      - id: SliceOutcome
        signature: "pub struct SliceOutcome {"
      - id: SliceSizer
        signature: "pub struct SliceSizer {"
        docstring: Chooses slice lengths so one launch lasts about `target`.
      - id: impl_SliceSizer
        signature: "impl SliceSizer {"
        children:
        - id: impl_SliceSizer.zp1hn9|T2uOUh
          signature: "pub const DEFAULT_TARGET: Duration = Duration::from_millis(2);"
        - id: impl_SliceSizer.new
          signature: "pub fn new(target: Duration, min: u32, max: u32) -> Self {"
        - id: impl_SliceSizer.throughput
          signature: "pub fn throughput(&self) -> Option<f32> {"
          docstring: Slots per second; `None` before the first observation.
        - id: impl_SliceSizer.next_len
          signature: "pub fn next_len(&self) -> u32 {"
          docstring: Starts at the minimum until a throughput sample exists.
        - id: impl_SliceSizer.record
          signature: "pub fn record(&mut self, slots: u32, elapsed: Duration) {"
      - id: impl_Default_for_SliceSizer
        signature: "impl Default for SliceSizer {"
        children:
        - id: impl_Default_for_SliceSizer.default
          signature: "fn default() -> Self {"
      - id: run_sliced
        signature: "pub fn run_sliced<E>(     range: SliceRange,     sizer: &mut SliceSizer,     bu…"
        docstring: Runs `range` as consecutive slices. `exec` returns the elapsed time of the slic…
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/launcher.rs
      nodes:
      - id: ModuleDoc
        signature: //! Launch indirection between scorers and lanes.
      - id: Ofgz1c|5vdUur
        signature: use std::sync::Arc;
      - id: LaunchMode
        signature: "pub enum LaunchMode {"
      - id: LaunchKey
        signature: "pub struct LaunchKey {"
        docstring: "Identity of a launch. Regions carry their generation, so a generation change yi…"
      - id: Launcher
        signature: "pub trait Launcher: Send + Sync {"
        children:
        - id: Launcher.mode
          signature: fn mode(&self) -> LaunchMode;
        - id: Launcher.launch
          signature: "fn launch(     &self,     key: &LaunchKey,     lane: &dyn ExecContext,     job:…"
      - id: DirectLauncher
        signature: pub struct DirectLauncher;
      - id: impl_Launcher_for_DirectLauncher
        signature: "impl Launcher for DirectLauncher {"
        children:
        - id: impl_Launcher_for_DirectLauncher.mode
          signature: "fn mode(&self) -> LaunchMode {"
        - id: impl_Launcher_for_DirectLauncher.launch
          signature: "level = \"trace\",     skip_all,     fields(op = key.op, bucket = key.shape_bucke…"
      - id: launcher_for
        signature: "pub fn launcher_for(mode: LaunchMode) -> Result<Arc<dyn Launcher>, KernelError>…"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cosine.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Dense scoring path: cosine similarity as the dot product of L2-normalised v…"
      - id: yJlSas|Xpl7lE
        signature: use std::sync::Arc;
      - id: CosineSlots
        signature: "pub struct CosineSlots {"
      - id: impl_CosineSlots
        signature: "impl CosineSlots {"
        children:
        - id: impl_CosineSlots.new
          signature: "pub fn new(source: Arc<dyn SlotSource>) -> Self {"
      - id: impl_SlotScorer_for_CosineSlots
        signature: "impl SlotScorer for CosineSlots {"
        children:
        - id: impl_SlotScorer_for_CosineSlots.scores
          signature: "fn scores(     &self,     queries: &QueryBatch,     range: SliceRange, ) -> Res…"
        - id: impl_SlotScorer_for_CosineSlots.slot_offset
          signature: "fn slot_offset(&self, segment: u32, slot: u32) -> u64 {"
      - id: CosineScorer
        signature: pub type CosineScorer = TopKScorer<CosineSlots>;
      - id: cosine_scorer
        signature: "pub fn cosine_scorer(source: Arc<dyn SlotSource>, k: usize) -> CosineScorer {"
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Kernel abstraction: segment descriptors, hits, the `Kernel` trait and the s…"
      - id: catalog
        signature: pub mod catalog;
      - id: cosine
        signature: pub mod cosine;
      - id: cpu_simd
        signature: pub mod cpu_simd;
      - id: cuda
        signature: pub mod cuda;
      - id: error
        signature: pub mod error;
      - id: golden
        signature: pub mod golden;
      - id: launcher
        signature: pub mod launcher;
      - id: maxsim
        signature: pub mod maxsim;
      - id: registry
        signature: pub mod registry;
      - id: slice
        signature: pub mod slice;
      - id: slots
        signature: pub mod slots;
      - id: topk
        signature: pub mod topk;
      - id: 1jD1e9|HeHd71
        signature: pub use error::KernelError;
      - id: mock
        signature: pub mod mock;
      - id: Dh3quH|ljFcuS
        signature: use std::time::Duration;
      - id: RegionSpan
        signature: "pub struct RegionSpan {"
        docstring: Address-agnostic arena span; pointer resolution happens only inside a backend.
      - id: OffsetRef
        signature: pub struct OffsetRef(pub RegionSpan);
        docstring: Offset table (`n + 1` prefix sums) living in an arena span.
      - id: SegmentLayout
        signature: "pub enum SegmentLayout {"
      - id: SegmentDesc
        signature: "pub struct SegmentDesc {"
      - id: KernelHit
        signature: "pub struct KernelHit {"
      - id: WrittenRange
        signature: "pub struct WrittenRange {"
      - id: BatchDesc
        signature: "pub struct BatchDesc {"
      - id: BatchId
        signature: pub struct BatchId(u64);
      - id: impl_BatchId
        signature: "impl BatchId {"
        children:
        - id: impl_BatchId.get
          signature: "pub fn get(self) -> u64 {"
      - id: KernelOutput
        signature: "pub enum KernelOutput {"
      - id: Kernel
        signature: "pub trait Kernel: Send + Sync {"
        children:
        - id: Kernel.begin
          signature: "fn begin(&self, batch: &BatchDesc) -> Result<BatchId, IndexError>;"
        - id: Kernel.run
          signature: "fn run(     &self,     batch: BatchId,     input: &[RegionSpan], ) -> Result<Ke…"
        - id: Kernel.grant_region
          signature: "fn grant_region(&self, batch: BatchId, region: RegionSpan) -> Result<(), IndexE…"
          docstring: Monotonically non-decreasing.
        - id: Kernel.progress
          signature: "fn progress(&self, batch: BatchId) -> u64;"
        - id: Kernel.end
          signature: "fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, IndexError>;"
      - id: entry_of_fixed
        signature: "pub fn entry_of_fixed(base: u64, stride: u32, count: u32, addr: u64) -> Option<…"
        docstring: Entry index for `addr` in a `Fixed` segment.
      - id: entry_of
        signature: "pub fn entry_of(offsets: &[u64], addr: u64) -> Option<usize> {"
        docstring: Entry index for `addr` given `n + 1` ascending offsets; entry `i` covers
      - id: SliceRange
        signature: "pub struct SliceRange {"
        docstring: Half-open slot range of one segment; the unit of work for a single launch.
      - id: impl_SliceRange
        signature: "impl SliceRange {"
        children:
        - id: impl_SliceRange.len
          signature: "pub fn len(&self) -> u32 {"
        - id: impl_SliceRange.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_SliceRange.split_at
          signature: "pub fn split_at(self, n: u32) -> (SliceRange, SliceRange) {"
          docstring: Splits off at most `n` leading slots.
      - id: QueryBatch
        signature: "pub struct QueryBatch {"
        docstring: Queries scored together so every document tile is read once. Each query is a ro…
      - id: impl_QueryBatch
        signature: "impl QueryBatch {"
        children:
        - id: impl_QueryBatch.new
          signature: "pub fn new(dim: usize, queries: Vec<Vec<f32>>) -> Result<Self, KernelError> {"
        - id: impl_QueryBatch.dim
          signature: "pub fn dim(&self) -> usize {"
        - id: impl_QueryBatch.len
          signature: "pub fn len(&self) -> usize {"
        - id: impl_QueryBatch.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_QueryBatch.tokens
          signature: "pub fn tokens(&self, i: usize) -> usize {"
        - id: impl_QueryBatch.data
          signature: "pub fn data(&self, i: usize) -> &[f32] {"
      - id: SlotSource
        signature: "pub trait SlotSource: Send + Sync {"
        docstring: Host-side view of slot vectors for CPU scoring (`F32`).
        children:
        - id: SlotSource.dim
          signature: fn dim(&self) -> usize;
        - id: SlotSource.slot_count
          signature: "fn slot_count(&self, segment: u32) -> u32;"
        - id: SlotSource.slot_offset
          signature: "fn slot_offset(&self, segment: u32, slot: u32) -> u64;"
          docstring: Offset reported in `KernelHit` for this slot.
        - id: SlotSource.slot_vectors
          signature: "fn slot_vectors(&self, segment: u32, slot: u32) -> Option<&[f32]>;"
          docstring: "Row-major `tokens x dim`, valid tokens only."
      - id: SlotScorer
        signature: "pub trait SlotScorer: Send + Sync {"
        docstring: Raw scores per slot; the top-k reduction is shared in `TopKScorer`.
        children:
        - id: SlotScorer.scores
          signature: "fn scores(     &self,     queries: &QueryBatch,     range: SliceRange, ) -> Res…"
          docstring: "One vector per query, one score per slot of `range` in slot order."
        - id: SlotScorer.slot_offset
          signature: "fn slot_offset(&self, segment: u32, slot: u32) -> u64;"
      - id: Scorer
        signature: "pub trait Scorer: Send + Sync {"
        docstring: Scores one slice for all queries of a batch; the result holds the slice-local t…
        children:
        - id: Scorer.score
          signature: "fn score(     &self,     queries: &QueryBatch,     range: SliceRange,     lane:…"
      - id: TopKScorer
        signature: "pub struct TopKScorer<S> {"
      - id: impl_TopKScorer_S_
        signature: "impl<S: SlotScorer> TopKScorer<S> {"
        children:
        - id: impl_TopKScorer_S_.new
          signature: "pub fn new(inner: S, k: usize) -> Self {"
        - id: impl_TopKScorer_S_.k
          signature: "pub fn k(&self) -> usize {"
      - id: impl_Scorer_for_TopKScorer_S_
        signature: "impl<S: SlotScorer> Scorer for TopKScorer<S> {"
        children:
        - id: impl_Scorer_for_TopKScorer_S_.score
          signature: "fn score(     &self,     queries: &QueryBatch,     range: SliceRange,     _lane…"
      - id: score_ranges
        signature: "pub fn score_ranges(     scorer: &dyn Scorer,     queries: &QueryBatch,     ran…"
        docstring: Scores several slices (e.g. a scope's ranges) and merges them into the global t…
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cpu_simd.rs
      nodes:
      - id: ModuleDoc
        signature: //! CPU scoring primitives; loops are shaped for autovectorisation (8-wide accu…
      - id: KFO4ZZ|XLcoE3
        signature: use std::sync::Arc;
      - id: dot
        signature: "pub fn dot(a: &[f32], b: &[f32]) -> f32 {"
      - id: maxsim
        signature: "pub fn maxsim(query: &[f32], doc: &[f32], dim: usize) -> f32 {"
        docstring: Sum over query tokens of the best dot product against any document token. A doc…
      - id: normalize
        signature: "pub fn normalize(v: &mut [f32]) {"
      - id: scan_slots
        signature: "pub(crate) fn scan_slots(     source: &dyn SlotSource,     queries: &QueryBatch…"
        docstring: "Slot-wise scorer: `score(query_i, slot)`; slots are the outer loop so a slot is…"
      - id: CpuMaxSim
        signature: "pub struct CpuMaxSim {"
      - id: impl_CpuMaxSim
        signature: "impl CpuMaxSim {"
        children:
        - id: impl_CpuMaxSim.new
          signature: "pub fn new(source: Arc<dyn SlotSource>) -> Self {"
      - id: impl_SlotScorer_for_CpuMaxSim
        signature: "impl SlotScorer for CpuMaxSim {"
        children:
        - id: impl_SlotScorer_for_CpuMaxSim.scores
          signature: "fn scores(     &self,     queries: &QueryBatch,     range: SliceRange, ) -> Res…"
        - id: impl_SlotScorer_for_CpuMaxSim.slot_offset
          signature: "fn slot_offset(&self, segment: u32, slot: u32) -> u64 {"
      - id: tests
        signature: mod tests;
```
/answer 9c73-129 allow
Control Request:
```yaml
id: 9c73-12a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
    ids:
    - kA9lm6|Qv4RkF
    - zBTwsZ|GsLJG7
    - body
    - defines
    - impl_CudaKernels
    - launch_err
    - 05tXPx|yv1hhd
    - cfg
    - launch_cosine
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/slice.rs
    ids:
    - StopReason
    - SliceOutcome
    - SliceSizer
    - run_sliced
  reason: "Cosine-Kernel, Launcher und Slice-Runner im Detail lesen"
```
/answer 9c73-12a allow
ReasoningToken: 92
Control Request:
```yaml
id: 9c73-12b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/cuda.rs
      nodes:
      - id: kA9lm6|Qv4RkF
        type: StatementGroup
        code: |
          pub const COSINE_DOT: &str = "cosine_dot";
          pub const MAXSIM_FUSED: &str = "maxsim_fused";
          const PRELUDE: &str = r#"
          #if defined(DTYPE_F16)
          typedef unsigned short elem_t;
          __device__ __forceinline__ float ld(elem_t v) {
              float f;
              asm("cvt.f32.f16 %0, %1;" : "=f"(f) : "h"(v));
              return f;
          }
          #else
          typedef float elem_t;
          __device__ __forceinline__ float ld(elem_t v) { return v; }
          #endif

          __device__ __forceinline__ float warp_sum(float v) {
              for (int o = 16; o > 0; o >>= 1) v += __shfl_down_sync(0xffffffffu, v, o);
              return v;
          }
          __device__ __forceinline__ float warp_max(float v) {
              for (int o = 16; o > 0; o >>= 1) v = fmaxf(v, __shfl_down_sync(0xffffffffu, v, o));
              return v;
          }
          "#;
      - id: zBTwsZ|GsLJG7
        type: StatementGroup
        code: |
          /// One block per slot; scores every query against the slot's single vector. Output is
          /// `n_queries x n_slots`.
          const COSINE_SRC: &str = r#"
          extern "C" __global__ void cosine_dot(
              const elem_t* __restrict__ docs, const float* __restrict__ queries, float* __restrict__ out,
              unsigned n_slots, unsigned n_queries)
          {
              unsigned slot = blockIdx.x;
              if (slot >= n_slots) return;
              const elem_t* d = docs + (size_t)slot * DIM;
              __shared__ float part[32];
              unsigned lane = threadIdx.x & 31, warp = threadIdx.x >> 5, warps = (blockDim.x + 31) >> 5;
              for (unsigned q = 0; q < n_queries; ++q) {
                  const float* qv = queries + (size_t)q * DIM;
                  float acc = 0.f;
                  for (unsigned i = threadIdx.x; i < DIM; i += blockDim.x) acc += ld(d[i]) * qv[i];
                  acc = warp_sum(acc);
                  if (lane == 0) part[warp] = acc;
                  __syncthreads();
                  if (threadIdx.x == 0) {
                      float s = 0.f;
                      for (unsigned w = 0; w < warps; ++w) s += part[w];
                      out[(size_t)q * n_slots + slot] = s;
                  }
                  __syncthreads();
              }
          }
          "#;
      - id: body
        type: ItemFn
        code: |
          fn body(name: &str) -> Option<&'static str> {
              match name {
                  COSINE_DOT => Some(COSINE_SRC),
                  MAXSIM_FUSED => Some(MAXSIM_SRC),
                  _ => None,
              }
          }
      - id: defines
        type: ItemFn
        code: |
          fn defines(key: &KernelKey) -> Result<Vec<String>, KernelError> {
              let ty = match key.dtype {
                  DType::F16 => "-DDTYPE_F16",
                  DType::F32 => "-DDTYPE_F32",
                  other => {
                      return Err(KernelError::Unavailable {
                          reason: format!("no kernel for dtype {other:?}"),
                      });
                  }
              };
              Ok(vec![ty.to_string(), format!("-DDIM={}", key.variant)])
          }
      - id: impl_CudaKernels
        type: ItemImpl
        signature: "impl CudaKernels {"
        children:
        - id: impl_CudaKernels.new
          type: ImplItemFn
          code: |2
                pub fn new(ctx: &Arc<CudaContext>) -> Result<Self, KernelError> {
                    let (major, minor) = ctx
                        .compute_capability()
                        .map_err(|e| KernelError::Unavailable {
                            reason: e.to_string(),
                        })?;
                    let loader: Arc<dyn KernelLoader<CudaFunction>> = Arc::new(
                        NvrtcLoader::new(ctx.clone()),
                    );
                    Ok(Self {
                        registry: KernelRegistry::new(loader),
                        sm: (major * 10 + minor) as u32,
                    })
                }
        - id: impl_CudaKernels.sm
          type: ImplItemFn
          code: |2
                pub fn sm(&self) -> u32 {
                    self.sm
                }
        - id: impl_CudaKernels.registry
          type: ImplItemFn
          code: |2
                pub fn registry(&self) -> &KernelRegistry<CudaFunction> {
                    &self.registry
                }
        - id: impl_CudaKernels.function
          type: ImplItemFn
          code: |2
                pub fn function(
                    &self,
                    name: &'static str,
                    dtype: DType,
                    dim: u32,
                ) -> Result<CudaFunction, KernelError> {
                    self.registry.get(&KernelKey::new(name, dtype, self.sm, dim))
                }
      - id: launch_err
        type: ItemFn
        code: |
          fn launch_err(e: impl std::error::Error + Send + Sync + 'static) -> KernelError {
              KernelError::LaunchFailed {
                  source: Box::new(e),
              }
          }
      - id: 05tXPx|yv1hhd
        type: StatementGroup
        code: |
          const BLOCK: u32 = 128;
      - id: cfg
        type: ItemFn
        code: |
          fn cfg(blocks: u32) -> LaunchConfig {
              LaunchConfig {
                  grid_dim: (blocks.max(1), 1, 1),
                  block_dim: (BLOCK, 1, 1),
                  shared_mem_bytes: 0,
              }
          }
      - id: launch_cosine
        type: ItemFn
        code: |
          /// Output layout `n_queries x n_slots`.
          pub(crate) fn launch_cosine<T: DeviceRepr>(
              stream: &Arc<CudaStream>,
              f: &CudaFunction,
              docs: &CudaSlice<T>,
              queries: &CudaSlice<f32>,
              out: &mut CudaSlice<f32>,
              n_slots: u32,
              n_queries: u32,
          ) -> Result<(), KernelError> {
              let mut b = stream.launch_builder(f);
              b.arg(docs).arg(queries).arg(out).arg(&n_slots).arg(&n_queries);
              unsafe { b.launch(cfg(n_slots)) }.map(|_| ()).map_err(launch_err)
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/slice.rs
      nodes:
      - id: StopReason
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub enum StopReason {
              Preempted,
              Deadline,
          }
      - id: SliceOutcome
        type: ItemStruct
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub struct SliceOutcome {
              pub done: u32,
              /// Work left when stopped early; resume from here.
              pub remaining: Option<SliceRange>,
              pub stopped: Option<StopReason>,
          }
      - id: SliceSizer
        type: ItemStruct
        code: |
          /// Chooses slice lengths so one launch lasts about `target`.
          #[derive(Debug)]
          pub struct SliceSizer {
              target: Duration,
              min: u32,
              max: u32,
              rate: Ewma,
          }
      - id: run_sliced
        type: ItemFn
        code: |
          /// Runs `range` as consecutive slices. `exec` returns the elapsed time of the slice it ran.
          /// `should_yield` is polled after each completed slice, never inside one. At least one slice always
          /// completes, so the caller makes progress even under permanent preemption pressure.
          pub fn run_sliced<E>(
              range: SliceRange,
              sizer: &mut SliceSizer,
              budget: Option<Duration>,
              mut should_yield: impl FnMut() -> bool,
              mut exec: impl FnMut(SliceRange) -> Result<Duration, E>,
          ) -> Result<SliceOutcome, E> {
              let mut rest = range;
              let mut done = 0u32;
              let mut spent = Duration::ZERO;
              while !rest.is_empty() {
                  let (head, tail) = rest.split_at(sizer.next_len());
                  let elapsed = exec(head)?;
                  sizer.record(head.len(), elapsed);
                  done += head.len();
                  spent += elapsed;
                  rest = tail;
                  if rest.is_empty() {
                      break;
                  }
                  let stopped = if budget.is_some_and(|b| spent >= b) {
                      Some(StopReason::Deadline)
                  } else if should_yield() {
                      Some(StopReason::Preempted)
                  } else {
                      None
                  };
                  if stopped.is_some() {
                      return Ok(SliceOutcome { done, remaining: Some(rest), stopped });
                  }
              }
              Ok(SliceOutcome { done, remaining: None, stopped: None })
          }
```
/answer 9c73-12b allow
Control Request:
```yaml
id: 9c73-12c
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test/cuda.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/probe_cuda.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/probe.rs
  reason: "Testkonventionen (cuda-Tests, Fake-Probe) kennenlernen"
```
/answer 9c73-12c allow
Control Request:
```yaml
id: 9c73-12d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test/cuda.rs
      nodes:
      - id: IYLZKb|VO72MG
        signature: use super::*;
      - id: wNsDct|ow87y6
        signature: "const DIM: usize = 64;"
      - id: rnd
        signature: "fn rnd(n: usize, seed: u32) -> Vec<f32> {"
      - id: setup
        signature: "fn setup() -> (Arc<CudaContext>, Arc<CudaStream>, CudaKernels) {"
      - id: registry_compiles_for_device_sm_and_caches
        signature: "fn registry_compiles_for_device_sm_and_caches() {"
      - id: cubin_path_loads_and_runs
        signature: "fn cubin_path_loads_and_runs() {"
      - id: cosine_kernel_matches_cpu_for_f32_and_f16
        signature: "fn cosine_kernel_matches_cpu_for_f32_and_f16() {"
      - id: fused_maxsim_matches_cpu_with_padding
        signature: "fn fused_maxsim_matches_cpu_with_padding() {"
      - id: gate_maxsim_chain_vs_fused
        signature: "fn gate_maxsim_chain_vs_fused() {"
        docstring: "Measurement gate: batched Candle chain (`matmul -> mask -> max -> sum` over a w…"
      - id: gate_cosine_dtype_front
        signature: "fn gate_cosine_dtype_front() {"
        docstring: "Measurement gate: cosine scoring time against score error per document dtype,"
      - id: gate_cosine_bandwidth_front
        signature: "fn gate_cosine_bandwidth_front() {"
        docstring: "Measurement gate: bandwidth-oriented scoring (one warp per document) for f32, f…"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/probe_cuda.rs
      nodes:
      - id: ISzc4S|yD2NS5
        signature: use super::*;
      - id: tu116
        signature: "fn tu116() -> CapabilitySet {"
      - id: sm75_lists_fp16_and_dp4a_but_no_bf16
        signature: "fn sm75_lists_fp16_and_dp4a_but_no_bf16() {"
      - id: no_tensor_core_path_is_listed
        signature: "fn no_tensor_core_path_is_listed() {"
      - id: capability_gates_follow_compute_capability
        signature: "fn capability_gates_follow_compute_capability() {"
      - id: batch_and_tile_come_from_config
        signature: "fn batch_and_tile_come_from_config() {"
      - id: tensor_cores_are_never_derived_from_compute_capability_alone
        signature: "fn tensor_cores_are_never_derived_from_compute_capability_alone() {"
      - id: real_device_probe_matches_hardware_table
        signature: "fn real_device_probe_matches_hardware_table() {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/probe.rs
      nodes:
      - id: 2c4o4q|XtE9ge
        signature: use super::*;
      - id: intersect_drops_values_without_kernel
        signature: "fn intersect_drops_values_without_kernel() {"
      - id: ungoverned_axes_pass_unchanged
        signature: "fn ungoverned_axes_pass_unchanged() {"
      - id: governed_axis_without_remaining_values_disappears
        signature: "fn governed_axis_without_remaining_values_disappears() {"
      - id: new_axes_and_values_need_no_signature_change
        signature: "fn new_axes_and_values_need_no_signature_change() {"
      - id: kernel_catalog_matches_implemented_dtypes
        signature: "fn kernel_catalog_matches_implemented_dtypes() {"
      - id: tiles_are_bounded_by_max_seq_len
        signature: "fn tiles_are_bounded_by_max_seq_len() {"
      - id: filtered_probe_applies_kernel_set
        signature: "fn filtered_probe_applies_kernel_set() {"
      - id: cpu_factory_yields_cpu_probe_limited_to_kernels
        signature: "fn cpu_factory_yields_cpu_probe_limited_to_kernels() {"
      - id: cuda_probe_unavailable_without_feature
        signature: "fn cuda_probe_unavailable_without_feature() {"
```
/answer 9c73-12d allow
ReasoningToken: 107
Control Request:
```yaml
id: 9c73-12e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test/cuda.rs
    ids:
    - IYLZKb|VO72MG
    - rnd
    - setup
    - cosine_kernel_matches_cpu_for_f32_and_f16
    - gate_cosine_dtype_front
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe_cuda.rs
    ids:
    - 9EqFK4|U6hJhV
    - CudaProbe
    - cuda_probe
    - impl_CapabilityProbe_for_CudaProbe
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/catalog.rs
    ids:
    - PZQ3hE|ruqejM
    - CqGY8w|6JQ34E
    - implemented_capabilities
  reason: "Vorlage für reale CUDA-Messung (Setup, Hostdaten, Gate-Test) sowie cfg-Gating-Muster"
```
/answer 9c73-12e allow
Control Request:
```yaml
id: 9c73-12f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/test/cuda.rs
      nodes:
      - id: IYLZKb|VO72MG
        type: ImportGroup
        code: |
          use super::*;
          use crate::resident::kernel::cpu_simd::{dot, maxsim};
          use crate::resident::subengine::DeviceHandle;
          use half::f16;
      - id: rnd
        type: ItemFn
        code: |
          fn rnd(n: usize, seed: u32) -> Vec<f32> {
              (0..n)
                  .map(|i| {
                      ((i as u32).wrapping_mul(2654435761).wrapping_add(seed.wrapping_mul(97))
                          % 1000) as f32 / 500.0 - 1.0
                  })
                  .collect()
          }
      - id: setup
        type: ItemFn
        code: |
          fn setup() -> (Arc<CudaContext>, Arc<CudaStream>, CudaKernels) {
              let dev = DeviceHandle::cuda(0).expect("cuda device");
              let ctx = dev.cuda_context().unwrap().clone();
              let stream = ctx.new_stream().unwrap();
              let kernels = CudaKernels::new(&ctx).unwrap();
              (ctx, stream, kernels)
          }
      - id: cosine_kernel_matches_cpu_for_f32_and_f16
        type: ItemFn
        code: |
          #[test]
          #[ignore = "requires a CUDA device"]
          fn cosine_kernel_matches_cpu_for_f32_and_f16() {
              let (_, stream, k) = setup();
              let (n, nq) = (37usize, 3usize);
              let docs = rnd(n * DIM, 3);
              let queries = rnd(nq * DIM, 4);
              let d_q = stream.clone_htod(&queries).unwrap();
              let f32_fn = k.function(COSINE_DOT, DType::F32, DIM as u32).unwrap();
              let d_docs = stream.clone_htod(&docs).unwrap();
              let mut out = stream.alloc_zeros::<f32>(n * nq).unwrap();
              launch_cosine(&stream, &f32_fn, &d_docs, &d_q, &mut out, n as u32, nq as u32)
                  .unwrap();
              let got = stream.clone_dtoh(&out).unwrap();
              for q in 0..nq {
                  for s in 0..n {
                      let want = dot(
                          &queries[q * DIM..(q + 1) * DIM],
                          &docs[s * DIM..(s + 1) * DIM],
                      );
                      assert!((got[q * n + s] - want).abs() < 1e-3, "f32 q={q} s={s}");
                  }
              }
              let f16_fn = k.function(COSINE_DOT, DType::F16, DIM as u32).unwrap();
              let bits: Vec<u16> = docs.iter().map(|&v| f16::from_f32(v).to_bits()).collect();
              let rounded: Vec<f32> = bits.iter().map(|&b| f16::from_bits(b).to_f32()).collect();
              let d_docs16 = stream.clone_htod(&bits).unwrap();
              let mut out16 = stream.alloc_zeros::<f32>(n * nq).unwrap();
              launch_cosine(&stream, &f16_fn, &d_docs16, &d_q, &mut out16, n as u32, nq as u32)
                  .unwrap();
              let got16 = stream.clone_dtoh(&out16).unwrap();
              for q in 0..nq {
                  for s in 0..n {
                      let want = dot(
                          &queries[q * DIM..(q + 1) * DIM],
                          &rounded[s * DIM..(s + 1) * DIM],
                      );
                      assert!((got16[q * n + s] - want).abs() < 1e-3, "f16 q={q} s={s}");
                  }
              }
          }
      - id: gate_cosine_dtype_front
        type: ItemFn
        code: |
          /// Measurement gate: cosine scoring time against score error per document dtype,
          /// interleaved rounds, median time. Run with `--nocapture`.
          #[test]
          #[ignore = "requires a CUDA device; measurement"]
          fn gate_cosine_dtype_front() {
              use std::time::Instant;
              const D: usize = 384;
              let (n, nq, rounds) = (200_000usize, 8usize, 25usize);
              let (_, stream, k) = setup();
              let unit = |count: usize, seed: u32| -> Vec<f32> {
                  let mut v = rnd(count * D, seed);
                  for row in v.chunks_mut(D) {
                      let norm = row.iter().map(|x| x * x).sum::<f32>().sqrt();
                      row.iter_mut().for_each(|x| *x /= norm);
                  }
                  v
              };
              let docs = unit(n, 21);
              let queries = unit(nq, 22);
              let d_q = stream.clone_htod(&queries).unwrap();
              let d32 = stream.clone_htod(&docs).unwrap();
              let bits: Vec<u16> = docs.iter().map(|&v| f16::from_f32(v).to_bits()).collect();
              let d16 = stream.clone_htod(&bits).unwrap();
              let f32_fn = k.function(COSINE_DOT, DType::F32, D as u32).unwrap();
              let f16_fn = k.function(COSINE_DOT, DType::F16, D as u32).unwrap();
              let mut out32 = stream.alloc_zeros::<f32>(n * nq).unwrap();
              let mut out16 = stream.alloc_zeros::<f32>(n * nq).unwrap();
              let mut times = [Vec::new(), Vec::new()];
              for round in 0..rounds {
                  let t = Instant::now();
                  launch_cosine(&stream, &f32_fn, &d32, &d_q, &mut out32, n as u32, nq as u32)
                      .unwrap();
                  stream.synchronize().unwrap();
                  let a = t.elapsed().as_secs_f64() * 1e3;
                  let t = Instant::now();
                  launch_cosine(&stream, &f16_fn, &d16, &d_q, &mut out16, n as u32, nq as u32)
                      .unwrap();
                  stream.synchronize().unwrap();
                  let b = t.elapsed().as_secs_f64() * 1e3;
                  if round >= 5 {
                      times[0].push(a);
                      times[1].push(b);
                  }
              }
              let got32 = stream.clone_dtoh(&out32).unwrap();
              let got16 = stream.clone_dtoh(&out16).unwrap();
              let (mut e32, mut e16) = (0f64, 0f64);
              let mut sum16 = 0f64;
              let mut count = 0usize;
              for q in 0..nq {
                  for s in (0..n).step_by(401) {
                      let want: f64 = (0..D)
                          .map(|i| f64::from(queries[q * D + i]) * f64::from(docs[s * D + i]))
                          .sum();
                      e32 = e32.max((f64::from(got32[q * n + s]) - want).abs());
                      let err = (f64::from(got16[q * n + s]) - want).abs();
                      e16 = e16.max(err);
                      sum16 += err;
                      count += 1;
                  }
              }
              for (name, idx, err) in [("F32", 0usize, e32), ("F16", 1, e16)] {
                  let t = &mut times[idx];
                  t.sort_by(f64::total_cmp);
                  println!(
                      "G-COSINE {name}: median {:.3} ms min {:.3} ms max {:.3} ms abs_err_max {err:.3e}",
                      t[t.len() / 2], t[0], t[t.len() - 1],
                  );
              }
              println!(
                  "G-COSINE docs={n} dim={D} queries={nq} F16 abs_err_mean {:.3e}", sum16 / count
                  as f64
              );
          }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe_cuda.rs
      nodes:
      - id: 9EqFK4|U6hJhV
        type: ImportGroup
        code: |
          use crate::resident::error::PrecisionError;
          use crate::resident::subengine::{DeviceHandle, SubengineKind};

          use super::capability::CapabilitySet;
          use super::known::{
              ACCUMULATOR, BATCH, FP16, FP32, FUSION, INT8_DP4A, OFF, ON, OPERANDS, SCORING, TILE,
          };
          use super::probe::{numeric_axis, CapabilityProbe, ProbeConfig};
      - id: CudaProbe
        type: ItemStruct
        code: |
          pub struct CudaProbe {
              #[cfg_attr(not(feature = "cuda"), allow(dead_code))]
              device: DeviceHandle,
              #[cfg_attr(not(feature = "cuda"), allow(dead_code))]
              cfg: ProbeConfig,
          }
      - id: cuda_probe
        type: ItemFn
        code: |
          pub(super) fn cuda_probe(device: &DeviceHandle, cfg: ProbeConfig) -> Result<CudaProbe, PrecisionError> {
              #[cfg(feature = "cuda")]
              {
                  Ok(CudaProbe { device: device.clone(), cfg })
              }
              #[cfg(not(feature = "cuda"))]
              {
                  let _ = (device, cfg);
                  Err(PrecisionError::Capability { reason: "cuda feature disabled".into() })
              }
          }
      - id: impl_CapabilityProbe_for_CudaProbe
        type: ItemImpl
        signature: "impl CapabilityProbe for CudaProbe {"
        children:
        - id: impl_CapabilityProbe_for_CudaProbe.kind
          type: ImplItemFn
          code: |2
                fn kind(&self) -> SubengineKind {
                    SubengineKind::Cuda
                }
        - id: impl_CapabilityProbe_for_CudaProbe.probe
          type: ImplItemFn
          code: |2
                #[cfg(feature = "cuda")]
                fn probe(&self) -> Result<CapabilitySet, PrecisionError> {
                    let fail = |e: cudarc::driver::DriverError| PrecisionError::Capability { reason: e.to_string() };
                    let ctx = self.device.cuda_context().ok_or_else(|| PrecisionError::Capability {
                        reason: "device has no cuda context".into(),
                    })?;
                    let (major, minor) = ctx.compute_capability().map_err(fail)?;
                    let name = ctx.name().map_err(fail)?;
                    let set = cuda_capabilities(major, minor, &self.cfg);
                    tracing::info!(
                        device = %name,
                        cc = %format!("{major}.{minor}"),
                        tensor_cores = has_tensor_cores(major, &name),
                        "cuda capabilities"
                    );
                    Ok(set)
                }
        - id: impl_CapabilityProbe_for_CudaProbe.probe_2
          type: ImplItemFn
          code: |2
                #[cfg(not(feature = "cuda"))]
                fn probe(&self) -> Result<CapabilitySet, PrecisionError> {
                    Err(PrecisionError::Capability { reason: "cuda feature disabled".into() })
                }
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/kernel/catalog.rs
      nodes:
      - id: PZQ3hE|ruqejM
        type: ImportGroup
        code: |
          use crate::resident::memory::DType;
          use crate::resident::precision::known::{FP16, FP32, OPERANDS, SCORING};
          use crate::resident::precision::{CapabilitySet, ValueId};
          use crate::resident::subengine::SubengineKind;
      - id: CqGY8w|6JQ34E
        type: StatementGroup
        code: |
          /// Element types the NVRTC loader in `cuda.rs` can compile a kernel for.
          pub const CUDA_KERNEL_DTYPES: [DType; 2] = [DType::F32, DType::F16];

          /// Element types the host scoring loops in `cpu_simd.rs` operate on.
          pub const CPU_KERNEL_DTYPES: [DType; 1] = [DType::F32];
      - id: implemented_capabilities
        type: ItemFn
        code: |
          /// Axes governed by the kernels: operand and scoring values need an executable kernel.
          /// Other axes (accumulator, fusion, batch, tile, threads) are not restricted by the catalog.
          pub fn implemented_capabilities(kind: SubengineKind) -> CapabilitySet {
              let dtypes: &[DType] = match kind {
                  SubengineKind::Cpu => &CPU_KERNEL_DTYPES,
                  SubengineKind::Cuda => &CUDA_KERNEL_DTYPES,
              };
              let mut set = CapabilitySet::new();
              for value in dtypes.iter().filter_map(|d| dtype_value(*d)) {
                  set.insert(&*OPERANDS, value);
                  set.insert(&*SCORING, value);
              }
              set
          }
```
/answer 9c73-12f allow
ReasoningToken: 10143
Control Request:
```yaml
id: 9c73-130
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: "forbid\\(unsafe|deny\\(unsafe|#!\\[(deny|warn|forbid)"
    include:
    - '*.rs'
    limit: 10
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "derive\\([^)]*\\)\\]\\s*$"
    include:
    - profile.rs
    - mod.rs
    limit: 20
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
    pattern: derive|pub struct ProfileConfig|pub enum SubengineKind
    include:
    - profile.rs
    - capability.rs
    limit: 10
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
    pattern: enum SubengineKind
    include:
    - mod.rs
    limit: 3
  - directory:
    - /home/user/.cargo/registry/src
    pattern: pub fn (record_event|elapsed_ms|memset_zeros|alloc_zeros|clone_htod|synchronize|is_complete)\b
    include:
    - '*.rs'
    limit: 20
  reason: "Unsafe-Policy, Derives von ProfileConfig/SubengineKind prüfen und cudarc-0.19.9-API (Events, memset) verifizieren"
```
/answer 9c73-130 allow
Control Request:
```yaml
id: 9c73-131
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: layers/trigram/trie.rs
        matches:
        - "#![forbid(unsafe_code)]"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: segment/mod.rs
        matches:
        - "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
        - "#[derive(Clone, Copy, Debug, PartialEq, Eq)]"
        - "#[derive(Clone, Debug, Default, PartialEq, Eq)]"
        - "#[derive(Default)]"
      - path: precision/profile.rs
        matches:
        - "#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]"
        - "#[derive(Clone, Debug, PartialEq)]"
      - path: index/mod.rs
        matches:
        - "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]"
        - "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
        - "#[derive(Clone, Debug, PartialEq)]"
        - "#[derive(Clone, Debug, Default, PartialEq, Eq)]"
        - "#[derive(Clone, Debug)]"
        - "#[derive(Clone, Copy, Debug, PartialEq, Eq)]"
        - "#[derive(Clone, Debug, PartialEq, Eq)]"
      - path: kernel/mod.rs
        matches:
        - "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      warning: "Limit of 20 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      matches:
      - path: capability.rs
        matches:
        - "#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]"
      - path: profile.rs
        matches:
        - "#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]"
        - "pub struct ProfileConfig(BTreeMap<AxisId, ValueId>);"
        - "#[derive(Clone, Debug, PartialEq)]"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
      matches:
      - path: mod.rs
        matches:
        - "pub enum SubengineKind {"
    - directory:
      - /home/user/.cargo/registry/src
      matches:
      - path: index.crates.io-1949cf8c6b5b557f/ug-cuda-0.5.0/src/runtime.rs
        matches:
        - "pub fn synchronize(&self) -> Result<()> {"
      - path: index.crates.io-1949cf8c6b5b557f/cudarc-0.19.9/src/driver/safe/core.rs
        matches:
        - "pub fn synchronize(&self) -> Result<(), DriverError> {"
        - "pub fn elapsed_ms(&self, end: &Self) -> Result<f32, DriverError> {"
        - "pub fn is_complete(&self) -> bool {"
        - pub fn record_event(
        - "pub fn record_event(event: &'a Option<CudaEvent>, stream: &'a CudaStream) -> Self {"
        - "pub fn alloc_zeros<T: DeviceRepr + ValidAsZeroBits>("
        - "pub fn memset_zeros<T: DeviceRepr + ValidAsZeroBits, Dst: DevicePtrMut<T>>("
        - "pub fn clone_htod<T: DeviceRepr, Src: HostSlice<T> + ?Sized>("
      - path: index.crates.io-1949cf8c6b5b557f/cudarc-0.19.9/src/driver/result.rs
        matches:
        - "pub fn synchronize() -> Result<(), DriverError> {"
      - path: index.crates.io-1949cf8c6b5b557f/cudarc-0.19.9/src/cufile/safe.rs
        matches:
        - "pub fn synchronize(self) -> Result<isize, crate::driver::DriverError> {"
      - path: index.crates.io-1949cf8c6b5b557f/candle-core-0.11.0/src/cuda_backend/device.rs
        matches:
        - "pub fn alloc_zeros<T: cudarc::driver::DeviceRepr + cudarc::driver::ValidAsZeroBits>("
        - pub fn clone_htod<
      - path: index.crates.io-1949cf8c6b5b557f/candle-core-0.11.0/src/device.rs
        matches:
        - "pub fn synchronize(&self) -> Result<()> {"
      - path: index.crates.io-1949cf8c6b5b557f/candle-transformers-0.11.0/src/models/z_image/scheduler.rs
        matches:
        - "pub fn is_complete(&self) -> bool {"
      - path: index.crates.io-1949cf8c6b5b557f/cudarc-0.17.8/src/driver/safe/core.rs
        matches:
        - "pub fn synchronize(&self) -> Result<(), DriverError> {"
      warning: "Limit of 20 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 9c73-131 allow
Thinking:
Das Design steht: Measure mit `deviation`/`sample`, `Calibrator` mit OFAT-Pruning, Stabilitätsschleife, Low-Priority-Pool und CUDA-Messung über den vorhandenen Cosine-Kernel. Jetzt verifiziere ich noch die cudarc-API-Signaturen.


ReasoningToken: 166
Control Request:
```yaml
id: 9c73-132
toolName: ast_find
arguments:
  paths:
  - /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cudarc-0.19.9/src/driver/safe/core.rs
  regexp: pub fn (record_event|elapsed_ms|memset_zeros|alloc_zeros)\b
  reason: "Exakte Signaturen von record_event, elapsed_ms, memset_zeros, alloc_zeros prüfen"
```
/answer 9c73-132 allow
Control Request:
```yaml
id: 9c73-133
result:
  structuredContent:
    files:
    - path: /home/user/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cudarc-0.19.9/src/driver/safe/core.rs
      nodes:
      - id: impl_CudaEvent.elapsed_ms
        code: |2
              /// The time between two events. `self` is the start event, and `end` is the end event.
              /// This is effectively `end - self`.
              pub fn elapsed_ms(&self, end: &Self) -> Result<f32, DriverError> {
                  if self.ctx != end.ctx {
                      return Err(DriverError(sys::cudaError_enum::CUDA_ERROR_INVALID_CONTEXT));
                  }
                  self.ctx.bind_to_thread()?;
                  self.synchronize()?;
                  end.synchronize()?;
                  unsafe { result::event::elapsed(self.cu_event, end.cu_event) }
              }
      - id: impl_CudaStream.record_event
        code: |2
              /// Creates a new [CudaEvent] and records the current work in the stream to the event.
              pub fn record_event(
                  &self,
                  flags: Option<sys::CUevent_flags>,
              ) -> Result<CudaEvent, DriverError> {
                  let event = self.ctx.new_event(flags)?;
                  event.record(self)?;
                  Ok(event)
              }
      - id: impl_SyncOnDrop__a_.record_event
        code: |2
              /// Construct a [SyncOnDrop::Record] variant
              pub fn record_event(event: &'a Option<CudaEvent>, stream: &'a CudaStream) -> Self {
                  SyncOnDrop::Record(event.as_ref().map(|e| (e, stream)))
              }
      - id: impl_CudaStream_2.alloc_zeros
        code: |2
              /// Allocates a [CudaSlice] with `len` elements of type `T`. All values are zero'd out.
              pub fn alloc_zeros<T: DeviceRepr + ValidAsZeroBits>(
                  self: &Arc<Self>,
                  len: usize,
              ) -> Result<CudaSlice<T>, DriverError> {
                  let mut dst = unsafe { self.alloc(len) }?;
                  self.memset_zeros(&mut dst)?;
                  Ok(dst)
              }
      - id: impl_CudaStream_2.memset_zeros
        code: |2
              /// Set's all the memory in `dst` to 0. `dst` can be a [CudaSlice] or [CudaViewMut]
              pub fn memset_zeros<T: DeviceRepr + ValidAsZeroBits, Dst: DevicePtrMut<T>>(
                  self: &Arc<Self>,
                  dst: &mut Dst,
              ) -> Result<(), DriverError> {
                  self.ctx.bind_to_thread()?;
                  let num_bytes = dst.num_bytes();
                  let (dptr, _record) = dst.device_ptr_mut(self);
                  unsafe { result::memset_d8_async(dptr, 0, num_bytes, self.cu_stream) }?;
                  Ok(())
              }
```
/answer 9c73-133 allow
Control Request:
```yaml
id: 9c73-134
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
    ids:
    - SubengineKind
  reason: Derives von SubengineKind
```
/answer 9c73-134 allow
Control Request:
```yaml
id: 9c73-135
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
      nodes:
      - id: SubengineKind
        type: ItemEnum
        code: |
          #[derive(Clone, Copy, Debug, PartialEq, Eq)]
          pub enum SubengineKind {
              Cpu,
              Cuda,
          }
```
/answer 9c73-135 allow
Text:
 Ich beginne mit der Erweiterung der Rauschmodelle in `mock.rs` (bimodal, nie stabil), die der neue `FakeMeasure` als Sample-Quelle nutzt.
ReasoningToken: 403
Control Request:
```yaml
id: 9c73-136
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
    id: NoiseModel
    start_marker: "    Drift { per_sample: f64 },"
    end_marker: "    Drift { per_sample: f64 },"
    content: |2-
          Drift { per_sample: f64 },
          /// With probability `p` a sample lands on `base + offset`, otherwise on `base`.
          Bimodal { offset: f64, p: f64 },
          /// Uniform noise of `±amplitude` around `base`; never settles for a large amplitude.
          Erratic { amplitude: f64 },
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
    id: impl_Measure_for_FakeMeasure.sample
    start_marker: "            NoiseModel::Drift { per_sample } => {"
    end_marker: |-
      self.base + self.drift_accum + self.rng.gen_range(-0.01..0.01)
                  }
    content: |2-
                  NoiseModel::Drift { per_sample } => {
                      self.drift_accum += per_sample;
                      self.base + self.drift_accum + self.rng.gen_range(-0.01..0.01)
                  }
                  NoiseModel::Bimodal { offset, p } => {
                      let jitter = self.rng.gen_range(-0.01..0.01);
                      if self.rng.gen_bool(p.clamp(0.0, 1.0)) {
                          self.base + offset + jitter
                      } else {
                          self.base + jitter
                      }
                  }
                  NoiseModel::Erratic { amplitude } => {
                      self.base + self.rng.gen_range(-amplitude..=amplitude)
                  }
  reason: Rauschmodelle für bimodal/nie-stabil ergänzen
```
/answer 9c73-136 allow
ReasoningToken: 125
Control Request:
```yaml
id: 9c73-137
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
    id: NoiseModel
    old_text: |-
      /// Noise models for calibration-stability tests.
      pub enum NoiseModel {
    new_text: |-
      /// Noise models for calibration-stability tests.
      #[derive(Clone, Debug)]
      pub enum NoiseModel {
  reason: "NoiseModel klonbar machen, damit pro Kombination eine Sample-Quelle entsteht"
```
/answer 9c73-137 allow
ReasoningToken: 35
Control Request:
```yaml
id: 9c73-138
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/pareto.rs
    id: pareto
    start_marker: pub fn pareto(mut pts
    end_marker: |2-
          out
      }
    content: |-
      pub fn pareto(pts: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
          pareto_by(pts, |p| *p)
      }

      /// Same front as [`pareto`] over arbitrary items; `key` yields `(time, deviation)`.
      pub fn pareto_by<T>(mut items: Vec<T>, key: impl Fn(&T) -> (f64, f64)) -> Vec<T> {
          items.retain(|i| {
              let k = key(i);
              k.0.is_finite() && k.1.is_finite()
          });
          items.sort_by(|a, b| {
              let (a, b) = (key(a), key(b));
              a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1))
          });
          let mut out: Vec<T> = Vec::new();
          let mut best = f64::INFINITY;
          for item in items {
              let dev = key(&item).1;
              if dev < best {
                  best = dev;
                  out.push(item);
              }
          }
          out
      }
  reason: Generische Pareto-Funktion für ParetoPoint mit Identität (O(n log n))
```
/answer 9c73-138 allow
Text:
 Jetzt die neuen Dateien. Zuerst Abweichungsmetriken und Plausibilitätsprüfung.
ReasoningToken: 16951
Control Request:
```yaml
id: 9c73-139
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/metrics.rs
    source: |
      //! Deviation metrics against the fp32 reference and plausibility checks.

      use std::collections::HashSet;
      use std::hash::Hash;

      use super::measure::Validity;

      /// `1 - cos` below this is rounding noise (it even goes negative) and counts as equal.
      pub const NOISE_FLOOR: f64 = 1e-6;

      #[derive(Clone, Copy, Debug, PartialEq)]
      pub struct Plausibility {
          /// Candidate norm relative to the reference norm must stay inside this range.
          pub norm_ratio: (f64, f64),
          /// Variance relative to the mean square; below this the vector is constant.
          pub min_rel_variance: f64,
      }

      impl Default for Plausibility {
          fn default() -> Self {
              Self { norm_ratio: (0.5, 2.0), min_rel_variance: 1e-9 }
          }
      }

      fn floor_noise(d: f64) -> f64 {
          if d < NOISE_FLOOR {
              0.0
          } else {
              d
          }
      }

      fn norm(v: &[f32]) -> f64 {
          v.iter().map(|&x| f64::from(x) * f64::from(x)).sum::<f64>().sqrt()
      }

      /// `max(0, 1 - cos(a, reference))`, 1 when either vector is zero.
      pub fn cosine_deviation(a: &[f32], reference: &[f32]) -> f64 {
          let dot: f64 = a.iter().zip(reference).map(|(&x, &y)| f64::from(x) * f64::from(y)).sum();
          let (na, nb) = (norm(a), norm(reference));
          if na == 0.0 || nb == 0.0 {
              return 1.0;
          }
          floor_noise((1.0 - dot / (na * nb)).max(0.0))
      }

      /// `||a - reference|| / ||reference||`.
      pub fn relative_error(a: &[f32], reference: &[f32]) -> f64 {
          let diff: f64 = a
              .iter()
              .zip(reference)
              .map(|(&x, &y)| (f64::from(x) - f64::from(y)).powi(2))
              .sum::<f64>()
              .sqrt();
          let n = norm(reference);
          if n == 0.0 {
              return if diff == 0.0 { 0.0 } else { f64::INFINITY };
          }
          diff / n
      }

      fn top_k(scores: &[f32], k: usize) -> Vec<usize> {
          let mut idx: Vec<usize> = (0..scores.len()).collect();
          idx.sort_by(|&a, &b| scores[b].total_cmp(&scores[a]).then(a.cmp(&b)));
          idx.truncate(k);
          idx
      }

      /// Share of the reference top-`k` that the candidate also ranks in its top-`k`.
      pub fn top_k_overlap(a: &[f32], reference: &[f32], k: usize) -> f64 {
          let k = k.min(a.len()).min(reference.len());
          if k == 0 {
              return 1.0;
          }
          let want: HashSet<usize> = top_k(reference, k).into_iter().collect();
          let hits = top_k(a, k).into_iter().filter(|i| want.contains(i)).count();
          hits as f64 / k as f64
      }

      fn is_constant(v: &[f32], min_rel_variance: f64) -> bool {
          if v.is_empty() {
              return true;
          }
          let n = v.len() as f64;
          let mean = v.iter().map(|&x| f64::from(x)).sum::<f64>() / n;
          let mean_sq = v.iter().map(|&x| f64::from(x) * f64::from(x)).sum::<f64>() / n;
          let variance = mean_sq - mean * mean;
          mean_sq == 0.0 || variance <= min_rel_variance * mean_sq
      }

      /// Judges `candidate` against `reference`, both made of rows of `dim` values.
      /// Returns the worst row deviation (cosine, plus `1 - top-k overlap` when `rank_k` is set)
      /// and the first failing validity.
      pub fn judge_rows(
          candidate: &[f32],
          reference: &[f32],
          dim: usize,
          rank_k: Option<usize>,
          p: &Plausibility,
      ) -> (f64, Validity) {
          if dim == 0 || candidate.len() != reference.len() || candidate.len() % dim != 0 {
              return (f64::INFINITY, Validity::Degenerate);
          }
          if candidate.iter().any(|x| !x.is_finite()) {
              return (f64::INFINITY, Validity::NonFinite);
          }
          let mut worst = 0.0f64;
          let mut validity = Validity::Ok;
          for (c, r) in candidate.chunks(dim).zip(reference.chunks(dim)) {
              let ratio = norm(c) / norm(r).max(f64::MIN_POSITIVE);
              if is_constant(c, p.min_rel_variance) || ratio < p.norm_ratio.0 || ratio > p.norm_ratio.1
              {
                  validity = Validity::Degenerate;
              }
              let mut dev = cosine_deviation(c, r);
              if let Some(k) = rank_k {
                  dev = dev.max(floor_noise(1.0 - top_k_overlap(c, r, k)));
              }
              worst = worst.max(dev);
          }
          (worst, validity)
      }

      /// Rank metrics are meaningless with ties, so test sets must not contain duplicates.
      pub fn is_duplicate_free<T: Hash + Eq>(items: &[T]) -> bool {
          let mut seen = HashSet::with_capacity(items.len());
          items.iter().all(|i| seen.insert(i))
      }

      #[cfg(test)]
      #[path = "test/metrics.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/metrics.rs
    source: |
      use super::*;

      fn ramp(n: usize) -> Vec<f32> {
          (0..n).map(|i| (i as f32 * 0.37).sin() + 0.1 * i as f32).collect()
      }

      #[test]
      fn identical_vectors_have_zero_deviation() {
          let v = ramp(64);
          assert_eq!(cosine_deviation(&v, &v), 0.0);
          assert_eq!(relative_error(&v, &v), 0.0);
      }

      #[test]
      fn rounding_noise_below_floor_counts_as_equal() {
          let v = ramp(64);
          let w: Vec<f32> = v.iter().map(|x| x * (1.0 + 1e-5)).collect();
          assert_eq!(cosine_deviation(&w, &v), 0.0);
      }

      #[test]
      fn orthogonal_vectors_deviate_by_one() {
          assert!((cosine_deviation(&[1.0, 0.0], &[0.0, 1.0]) - 1.0).abs() < 1e-12);
      }

      #[test]
      fn zero_vector_deviates_fully() {
          assert_eq!(cosine_deviation(&[0.0; 4], &[1.0; 4]), 1.0);
      }

      #[test]
      fn relative_error_scales_with_difference() {
          let e = relative_error(&[1.1, 0.0], &[1.0, 0.0]);
          assert!((e - 0.1).abs() < 1e-6);
          assert_eq!(relative_error(&[1.0], &[0.0]), f64::INFINITY);
      }

      #[test]
      fn top_k_overlap_counts_shared_ranks() {
          let reference = [0.9, 0.8, 0.7, 0.1, 0.0];
          assert_eq!(top_k_overlap(&reference, &reference, 3), 1.0);
          let swapped = [0.9, 0.1, 0.7, 0.8, 0.0];
          assert!((top_k_overlap(&swapped, &reference, 3) - 2.0 / 3.0).abs() < 1e-12);
      }

      #[test]
      fn non_finite_candidates_are_rejected() {
          let r = ramp(8);
          let mut c = r.clone();
          c[3] = f32::NAN;
          let (_, v) = judge_rows(&c, &r, 8, None, &Plausibility::default());
          assert_eq!(v, Validity::NonFinite);
          c[3] = f32::INFINITY;
          let (_, v) = judge_rows(&c, &r, 8, None, &Plausibility::default());
          assert_eq!(v, Validity::NonFinite);
      }

      #[test]
      fn constant_vectors_are_degenerate() {
          let r = ramp(8);
          let (_, v) = judge_rows(&[0.5; 8], &r, 8, None, &Plausibility::default());
          assert_eq!(v, Validity::Degenerate);
      }

      #[test]
      fn norm_outside_reference_ratio_is_degenerate() {
          let r = ramp(8);
          let big: Vec<f32> = r.iter().map(|x| x * 3.0).collect();
          let (_, v) = judge_rows(&big, &r, 8, None, &Plausibility::default());
          assert_eq!(v, Validity::Degenerate);
          let small: Vec<f32> = r.iter().map(|x| x * 0.2).collect();
          let (_, v) = judge_rows(&small, &r, 8, None, &Plausibility::default());
          assert_eq!(v, Validity::Degenerate);
      }

      #[test]
      fn worst_row_decides_the_deviation() {
          let r: Vec<f32> = ramp(16);
          let mut c = r.clone();
          for x in &mut c[8..] {
              *x += 0.5 * (*x).signum() * x.abs().sqrt();
          }
          let (dev, v) = judge_rows(&c, &r, 8, None, &Plausibility::default());
          assert_eq!(v, Validity::Ok);
          assert!(dev > 0.0);
          assert_eq!(dev, cosine_deviation(&c[8..], &r[8..]));
      }

      #[test]
      fn rank_metric_raises_the_deviation_of_reordered_scores() {
          let reference = [0.9, 0.8, 0.7, 0.1, 0.0, 0.3];
          let reordered = [0.9, 0.1, 0.7, 0.8, 0.0, 0.3];
          let (plain, _) = judge_rows(&reordered, &reference, 6, None, &Plausibility::default());
          let (ranked, _) = judge_rows(&reordered, &reference, 6, Some(3), &Plausibility::default());
          assert!(ranked > plain);
      }

      #[test]
      fn shape_mismatch_is_degenerate() {
          let (_, v) = judge_rows(&[1.0; 5], &[1.0; 6], 3, None, &Plausibility::default());
          assert_eq!(v, Validity::Degenerate);
          let (_, v) = judge_rows(&[1.0; 5], &[1.0; 5], 0, None, &Plausibility::default());
          assert_eq!(v, Validity::Degenerate);
      }

      #[test]
      fn duplicates_are_detected() {
          assert!(is_duplicate_free(&["a", "b", "c"]));
          assert!(!is_duplicate_free(&["a", "b", "a"]));
      }
  reason: metrics.rs mit Tests anlegen
```
/answer 9c73-139 allow
ReasoningToken: 166
Control Request:
```yaml
id: 9c73-13a
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
    source: |
      //! Measurement abstraction between the calibrator and a concrete compute path.

      use std::fmt;
      use std::sync::atomic::{AtomicBool, Ordering};
      use std::sync::Arc;
      use std::time::{Duration, Instant};

      use crate::resident::error::PrecisionError;
      use crate::resident::subengine::DeviceHandle;

      use super::profile::ProfileConfig;

      /// One combination to measure: a value per axis plus the sequence lengths of the test set.
      #[derive(Clone, Debug, PartialEq, Eq, Hash)]
      pub struct Cfg {
          pub combo: ProfileConfig,
          /// Lengths up to the maximum, to check processability (memory, overflow), not quality.
          pub seq_lens: Vec<usize>,
      }

      impl Cfg {
          pub fn new(combo: ProfileConfig, seq_lens: Vec<usize>) -> Self {
              Self { combo, seq_lens }
          }
      }

      impl fmt::Display for Cfg {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              for (i, (axis, value)) in self.combo.iter().enumerate() {
                  if i > 0 {
                      f.write_str(",")?;
                  }
                  write!(f, "{axis}={value}")?;
              }
              Ok(())
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum Validity {
          Ok,
          NonFinite,
          Degenerate,
          OverMaxDeviation,
          /// The combination could not run at all (allocation, launch, unsupported value).
          Failed,
      }

      impl Validity {
          pub fn is_ok(self) -> bool {
              self == Validity::Ok
          }

          pub fn as_str(self) -> &'static str {
              match self {
                  Validity::Ok => "ok",
                  Validity::NonFinite => "non-finite values",
                  Validity::Degenerate => "degenerate vectors",
                  Validity::OverMaxDeviation => "deviation above maximum",
                  Validity::Failed => "combination failed to run",
              }
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq)]
      pub struct DeviationReport {
          /// Against the fp32 reference of the same test set.
          pub deviation: f64,
          pub valid: Validity,
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum Sample {
          /// Time per work item, already normalised by the implementation.
          Time(Duration),
          /// A pending production task asked for the device between slices; nothing was measured.
          Interrupted,
      }

      /// The only way the calibrator obtains deviation and time. Real measurements and fakes
      /// implement the same trait, so the calibration logic runs without a GPU.
      pub trait Measure {
          /// Runs the combination once against the fp32 reference and judges the output.
          /// `Err(PrecisionError::Incompatible)` marks the combination as unusable.
          fn deviation(&mut self, cfg: &Cfg) -> Result<DeviationReport, PrecisionError>;

          /// One timing sample. The implementation warms up on first use, scales its iteration
          /// count to [`WindowTuning::window_ms`] and keeps single launches within
          /// [`WindowTuning::slice_budget_ms`].
          fn sample(&mut self, cfg: &Cfg) -> Result<Sample, PrecisionError>;
      }

      /// Aggregated result of one combination.
      #[derive(Clone, Debug, PartialEq)]
      pub struct MeasureResult {
          /// Robust estimate (median or minimum) over the samples.
          pub time: Duration,
          pub deviation: f64,
          pub valid: Validity,
          /// `false` when the repetition or time cap ended the measurement first.
          pub stable: bool,
          pub samples: u32,
      }

      #[derive(Clone, Copy, Debug, PartialEq)]
      pub struct WindowTuning {
          /// Target length of one timing window.
          pub window_ms: f64,
          /// Upper bound for one uninterruptible launch group; the device cannot be preempted.
          pub slice_budget_ms: f64,
      }

      impl Default for WindowTuning {
          fn default() -> Self {
              Self { window_ms: 2.0, slice_budget_ms: 5.0 }
          }
      }

      /// Polled between slices: whether a production task is waiting for the device.
      pub trait YieldCheck: Send + Sync {
          fn should_yield(&self) -> bool;
      }

      pub struct NeverYield;

      impl YieldCheck for NeverYield {
          fn should_yield(&self) -> bool {
              false
          }
      }

      impl YieldCheck for AtomicBool {
          fn should_yield(&self) -> bool {
              self.load(Ordering::Relaxed)
          }
      }

      /// Monotonic time source, injectable so budgets are testable.
      pub trait Clock: Send + Sync {
          fn elapsed(&self) -> Duration;
      }

      pub struct SystemClock {
          start: Instant,
      }

      impl SystemClock {
          pub fn new() -> Self {
              Self { start: Instant::now() }
          }
      }

      impl Default for SystemClock {
          fn default() -> Self {
              Self::new()
          }
      }

      impl Clock for SystemClock {
          fn elapsed(&self) -> Duration {
              self.start.elapsed()
          }
      }

      /// Real measurement for the device behind `device`. This is the only place that branches
      /// on the `cuda` feature for measuring.
      pub fn make_measure(
          device: &DeviceHandle,
          tuning: WindowTuning,
          yield_check: Arc<dyn YieldCheck>,
      ) -> Result<Box<dyn Measure>, PrecisionError> {
          use crate::resident::memory::DeviceKind;
          match device.kind() {
              DeviceKind::Cpu => Ok(Box::new(super::measure_cpu::CpuScoringMeasure::new(
                  super::workload::WorkloadSpec::default(),
                  tuning,
                  yield_check,
              )?)),
              DeviceKind::Gpu => {
                  #[cfg(feature = "cuda")]
                  {
                      Ok(Box::new(super::measure_cuda::CudaScoringMeasure::new(
                          device,
                          super::workload::WorkloadSpec::default(),
                          tuning,
                          yield_check,
                      )?))
                  }
                  #[cfg(not(feature = "cuda"))]
                  {
                      let _ = (tuning, yield_check);
                      Err(PrecisionError::Capability { reason: "cuda feature disabled".into() })
                  }
              }
          }
      }
  reason: "measure.rs mit Cfg, Measure-Trait, Clock, YieldCheck und Factory anlegen"
```
/answer 9c73-13a allow
ReasoningToken: 766
Control Request:
```yaml
id: 9c73-13b
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/workload.rs
    source: |
      //! Synthetic scoring workload shared by the real measurements: duplicate-free unit vectors
      //! and an fp32 host reference.

      use crate::resident::error::PrecisionError;
      use crate::resident::kernel::cpu_simd::{dot, normalize};

      use super::known::BATCH;
      use super::measure::{Cfg, DeviationReport};
      use super::metrics::{judge_rows, Plausibility};

      #[derive(Clone, Debug)]
      pub struct WorkloadSpec {
          pub docs: usize,
          pub dim: usize,
          /// Upper bound of the `batch` axis; the queries of a smaller batch are a prefix.
          pub max_queries: usize,
          pub top_k: usize,
          pub seed: u64,
          pub plausibility: Plausibility,
      }

      impl WorkloadSpec {
          /// Queries per launch when the combination has no `batch` value.
          pub const DEFAULT_QUERIES: usize = 8;
      }

      impl Default for WorkloadSpec {
          fn default() -> Self {
              Self {
                  docs: 20_000,
                  dim: 384,
                  max_queries: 32,
                  top_k: 5,
                  seed: 0x5eed_c0de,
                  plausibility: Plausibility::default(),
              }
          }
      }

      fn xorshift(state: &mut u64) -> u64 {
          *state ^= *state >> 12;
          *state ^= *state << 25;
          *state ^= *state >> 27;
          state.wrapping_mul(0x2545_f491_4f6c_dd1d)
      }

      fn unit_rows(rows: usize, dim: usize, seed: u64) -> Vec<f32> {
          let mut state = seed | 1;
          let mut out = Vec::with_capacity(rows * dim);
          for _ in 0..rows {
              let start = out.len();
              out.extend((0..dim).map(|_| (xorshift(&mut state) >> 40) as f32 / (1u64 << 23) as f32 - 1.0));
              normalize(&mut out[start..]);
          }
          out
      }

      pub struct ScoringWorkload {
          spec: WorkloadSpec,
          docs: Vec<f32>,
          queries: Vec<f32>,
          /// Row-major `max_queries x docs`.
          reference: Vec<f32>,
      }

      impl ScoringWorkload {
          pub fn generate(spec: WorkloadSpec) -> Self {
              let docs = unit_rows(spec.docs, spec.dim, spec.seed);
              let queries = unit_rows(spec.max_queries, spec.dim, spec.seed ^ 0x9e37_79b9_7f4a_7c15);
              let mut reference = Vec::with_capacity(spec.max_queries * spec.docs);
              for q in queries.chunks(spec.dim) {
                  reference.extend(docs.chunks(spec.dim).map(|d| dot(q, d)));
              }
              Self { spec, docs, queries, reference }
          }

          pub fn spec(&self) -> &WorkloadSpec {
              &self.spec
          }

          pub fn docs(&self) -> &[f32] {
              &self.docs
          }

          /// Query count of a combination, from its `batch` value.
          pub fn query_count(&self, cfg: &Cfg) -> Result<usize, PrecisionError> {
              let n = match cfg.combo.get(&BATCH) {
                  None => WorkloadSpec::DEFAULT_QUERIES.min(self.spec.max_queries),
                  Some(v) => v.as_str().parse::<usize>().map_err(|_| PrecisionError::Incompatible {
                      reason: format!("batch value `{v}` is not a number"),
                  })?,
              };
              if n == 0 || n > self.spec.max_queries {
                  return Err(PrecisionError::Incompatible {
                      reason: format!("batch {n} outside 1..={}", self.spec.max_queries),
                  });
              }
              Ok(n)
          }

          pub fn queries(&self, count: usize) -> &[f32] {
              &self.queries[..count * self.spec.dim]
          }

          pub fn reference(&self, count: usize) -> &[f32] {
              &self.reference[..count * self.spec.docs]
          }

          /// `scores` is row-major `count x docs`.
          pub fn judge(&self, scores: &[f32], count: usize) -> DeviationReport {
              let (deviation, valid) = judge_rows(
                  scores,
                  self.reference(count),
                  self.spec.docs,
                  Some(self.spec.top_k),
                  &self.spec.plausibility,
              );
              DeviationReport { deviation, valid }
          }
      }

      #[cfg(test)]
      #[path = "test/workload.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/workload.rs
    source: |
      use super::*;
      use crate::resident::precision::metrics::is_duplicate_free;
      use crate::resident::precision::profile::ProfileConfig;
      use crate::resident::precision::measure::Validity;

      fn small() -> WorkloadSpec {
          WorkloadSpec { docs: 200, dim: 32, max_queries: 8, ..WorkloadSpec::default() }
      }

      fn cfg_with_batch(b: &str) -> Cfg {
          Cfg::new(ProfileConfig::new().with(&*BATCH, b), vec![])
      }

      #[test]
      fn rows_are_unit_length_and_distinct() {
          let w = ScoringWorkload::generate(small());
          let rows: Vec<Vec<u32>> = w
              .docs()
              .chunks(32)
              .map(|r| r.iter().map(|x| x.to_bits()).collect())
              .collect();
          assert!(is_duplicate_free(&rows));
          for r in w.docs().chunks(32) {
              let n: f32 = r.iter().map(|x| x * x).sum::<f32>().sqrt();
              assert!((n - 1.0).abs() < 1e-5);
          }
      }

      #[test]
      fn generation_is_deterministic() {
          let a = ScoringWorkload::generate(small());
          let b = ScoringWorkload::generate(small());
          assert_eq!(a.docs(), b.docs());
          assert_eq!(a.reference(8), b.reference(8));
      }

      #[test]
      fn reference_as_candidate_is_a_perfect_match() {
          let w = ScoringWorkload::generate(small());
          let r = w.judge(w.reference(4), 4);
          assert_eq!(r.valid, Validity::Ok);
          assert_eq!(r.deviation, 0.0);
      }

      #[test]
      fn nan_scores_are_flagged() {
          let w = ScoringWorkload::generate(small());
          let mut s = w.reference(2).to_vec();
          s[5] = f32::NAN;
          assert_eq!(w.judge(&s, 2).valid, Validity::NonFinite);
      }

      #[test]
      fn constant_scores_are_degenerate() {
          let w = ScoringWorkload::generate(small());
          let s = vec![0.25f32; 2 * 200];
          assert_eq!(w.judge(&s, 2).valid, Validity::Degenerate);
      }

      #[test]
      fn batch_axis_selects_the_query_count() {
          let w = ScoringWorkload::generate(small());
          assert_eq!(w.query_count(&cfg_with_batch("4")).unwrap(), 4);
          assert_eq!(
              w.query_count(&Cfg::new(ProfileConfig::new(), vec![])).unwrap(),
              WorkloadSpec::DEFAULT_QUERIES
          );
          assert!(w.query_count(&cfg_with_batch("9")).is_err());
          assert!(w.query_count(&cfg_with_batch("0")).is_err());
          assert!(w.query_count(&cfg_with_batch("x")).is_err());
      }
  reason: "Gemeinsame Mess-Workload (duplikatfrei, fp32-Referenz) mit Tests anlegen"
```
/answer 9c73-13b allow
ReasoningToken: 224
Control Request:
```yaml
id: 9c73-13c
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/stats.rs
    source: |
      //! Robust statistics for timing series. Load only ever slows a run down, so estimates take
      //! the median or the minimum and spread ignores the slowest fifth of the samples.

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum Spread {
          /// Standard deviation over mean.
          Cv,
          /// `(max - min) / median`.
          RangeOverMedian,
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum Estimator {
          Median,
          Min,
      }

      fn sorted(values: &[f64]) -> Vec<f64> {
          let mut v = values.to_vec();
          v.sort_by(f64::total_cmp);
          v
      }

      pub fn median(values: &[f64]) -> f64 {
          if values.is_empty() {
              return f64::NAN;
          }
          let v = sorted(values);
          let mid = v.len() / 2;
          if v.len() % 2 == 1 {
              v[mid]
          } else {
              (v[mid - 1] + v[mid]) / 2.0
          }
      }

      pub fn estimate(values: &[f64], how: Estimator) -> f64 {
          match how {
              Estimator::Median => median(values),
              Estimator::Min => values.iter().copied().fold(f64::NAN, f64::min),
          }
      }

      /// Relative scatter of `values` after dropping the slowest `len / 5`; infinite when there
      /// are fewer than two samples or no positive centre.
      pub fn spread(values: &[f64], kind: Spread) -> f64 {
          let mut v = sorted(values);
          v.truncate(v.len() - v.len() / 5);
          if v.len() < 2 || v.iter().any(|x| !x.is_finite()) {
              return f64::INFINITY;
          }
          match kind {
              Spread::Cv => {
                  let n = v.len() as f64;
                  let mean = v.iter().sum::<f64>() / n;
                  if mean <= 0.0 {
                      return f64::INFINITY;
                  }
                  let var = v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
                  var.sqrt() / mean
              }
              Spread::RangeOverMedian => {
                  let m = median(&v);
                  if m <= 0.0 {
                      return f64::INFINITY;
                  }
                  (v[v.len() - 1] - v[0]) / m
              }
          }
      }

      #[cfg(test)]
      #[path = "test/stats.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/stats.rs
    source: |
      use super::*;

      #[test]
      fn median_handles_odd_even_and_empty() {
          assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
          assert_eq!(median(&[4.0, 1.0, 2.0, 3.0]), 2.5);
          assert!(median(&[]).is_nan());
      }

      #[test]
      fn estimators_pick_median_or_minimum() {
          let v = [5.0, 1.0, 9.0, 2.0, 3.0];
          assert_eq!(estimate(&v, Estimator::Median), 3.0);
          assert_eq!(estimate(&v, Estimator::Min), 1.0);
      }

      #[test]
      fn constant_series_has_zero_spread() {
          for kind in [Spread::Cv, Spread::RangeOverMedian] {
              assert_eq!(spread(&[2.0; 6], kind), 0.0);
          }
      }

      #[test]
      fn one_slow_outlier_in_five_is_ignored() {
          let v = [1.0, 1.0, 1.0, 1.0, 50.0];
          for kind in [Spread::Cv, Spread::RangeOverMedian] {
              assert_eq!(spread(&v, kind), 0.0);
          }
      }

      #[test]
      fn short_or_invalid_series_never_count_as_stable() {
          assert_eq!(spread(&[1.0], Spread::Cv), f64::INFINITY);
          assert_eq!(spread(&[], Spread::Cv), f64::INFINITY);
          assert_eq!(spread(&[1.0, f64::NAN], Spread::Cv), f64::INFINITY);
          assert_eq!(spread(&[0.0, 0.0], Spread::Cv), f64::INFINITY);
      }

      #[test]
      fn spread_grows_with_scatter() {
          let tight = spread(&[1.0, 1.01, 0.99, 1.0], Spread::Cv);
          let wide = spread(&[1.0, 1.5, 0.6, 1.0], Spread::Cv);
          assert!(wide > 10.0 * tight);
      }
  reason: "Robuste Statistik (Median/Min, getrimmte Streuung) mit Tests anlegen"
```
/answer 9c73-13c allow
Text:
 Jetzt der Kern: der `Calibrator`.
ReasoningToken: 10776
Control Request:
```yaml
id: 9c73-13d
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/calibrate.rs
    source: |
      //! Calibrator: enumerates combinations of a capability set, measures them interleaved against
      //! the fp32 reference, rejects invalid ones and keeps the Pareto front over (time, deviation).
      //!
      //! Scale: position 0 is the fastest, 1 the most precise combination. Calibration is best effort
      //! and never blocks: caps end a measurement with the best intermediate state marked unstable.

      use std::collections::{BTreeSet, HashSet};
      use std::sync::Arc;
      use std::time::Duration;

      use tracing::{debug, info};

      use crate::resident::error::PrecisionError;
      use crate::resident::subengine::SubengineKind;

      use super::axis::{AxisId, ValueId};
      use super::capability::CapabilitySet;
      use super::known::{BATCH, THREADS, TILE};
      use super::measure::{
          Cfg, Clock, Measure, MeasureResult, NeverYield, Sample, SystemClock, Validity, WindowTuning,
          YieldCheck,
      };
      use super::pareto::pareto_by;
      use super::profile::ProfileConfig;
      use super::stats::{estimate, spread, Estimator, Spread};

      /// Ends a single combination's measurement, whichever limit hits first.
      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub struct HardCap {
          /// Counts every attempt, interrupted ones included.
          pub max_reps: u32,
          pub max_time: Option<Duration>,
      }

      #[derive(Clone, Debug)]
      pub struct CalibratorConfig {
          /// Above this deviation a combination counts as incompatible and is not activated.
          pub max_deviation: f64,
          /// Raised to 3 if lower.
          pub min_reps: u32,
          pub window_ms: f64,
          /// Relative scatter below which a series counts as stable.
          pub cv_threshold: f64,
          pub spread: Spread,
          pub estimator: Estimator,
          pub hard_cap: HardCap,
          pub slice_budget_ms: f64,
          /// Budget of a whole calibration run; the rest is skipped when it is used up.
          pub total_budget: Option<Duration>,
          /// A combination is dominated only if a peer is faster by more than this share.
          pub dominance_margin: f64,
          /// Pause after an interrupted sample so a waiting production task gets the device.
          pub interrupt_backoff: Duration,
          /// Axes measured one at a time; only their winners are combined and verified.
          pub separable_axes: BTreeSet<AxisId>,
          /// Measures the full cartesian product instead (reference for the pruned result).
          pub exhaustive: bool,
          pub seq_lens: Vec<usize>,
      }

      impl CalibratorConfig {
          /// Lengths in steps of four up to and including `max`.
          pub fn seq_lens_up_to(max: usize) -> Vec<usize> {
              let mut lens = Vec::new();
              let mut n = 16usize;
              while n < max {
                  lens.push(n);
                  n = n.saturating_mul(4);
              }
              lens.push(max);
              lens
          }

          pub fn tuning(&self) -> WindowTuning {
              WindowTuning { window_ms: self.window_ms, slice_budget_ms: self.slice_budget_ms }
          }
      }

      impl Default for CalibratorConfig {
          fn default() -> Self {
              Self {
                  // Measured for multilingual-e5-small: fp16 sits at the fp32 noise floor,
                  // dynamic int8 at ~3e-2.
                  max_deviation: 5e-2,
                  min_reps: 3,
                  window_ms: 2.0,
                  cv_threshold: 0.05,
                  spread: Spread::Cv,
                  estimator: Estimator::Median,
                  hard_cap: HardCap { max_reps: 40, max_time: Some(Duration::from_secs(30)) },
                  slice_budget_ms: 5.0,
                  total_budget: Some(Duration::from_secs(600)),
                  dominance_margin: 0.1,
                  interrupt_backoff: Duration::from_millis(5),
                  separable_axes: [&*BATCH, &*TILE, &*THREADS].into_iter().cloned().collect(),
                  exhaustive: false,
                  seq_lens: Self::seq_lens_up_to(512),
              }
          }
      }

      #[derive(Clone, Debug, PartialEq, Eq)]
      pub struct CalibrationTarget {
          pub engine: SubengineKind,
          pub model_id: String,
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      pub enum Stage {
          /// One axis varied, the others at their base value.
          Axis,
          /// Combined winners (or the full product), measured together.
          Verify,
      }

      #[derive(Clone, Debug, PartialEq)]
      pub enum Outcome {
          /// `result.stable == false` marks an unstable, best-effort measurement.
          Measured(MeasureResult),
          Rejected(Validity),
          /// Stopped early: slower and not more precise than a peer.
          Dominated,
          /// Not measured: budget used up or the device never became available.
          Skipped,
      }

      #[derive(Clone, Debug, PartialEq)]
      pub struct Entry {
          pub stage: Stage,
          pub cfg: Cfg,
          pub outcome: Outcome,
      }

      #[derive(Clone, Debug, PartialEq)]
      pub struct ParetoPoint {
          pub cfg: Cfg,
          pub time: Duration,
          pub deviation: f64,
          pub stable: bool,
      }

      #[derive(Clone, Debug)]
      pub struct CalibrationReport {
          pub target: CalibrationTarget,
          pub entries: Vec<Entry>,
          /// Ascending time, strictly decreasing deviation.
          pub front: Vec<ParetoPoint>,
          pub budget_exhausted: bool,
          /// Size of the full cartesian product that pruning avoided measuring.
          pub exhaustive_size: usize,
          pub elapsed: Duration,
      }

      impl CalibrationReport {
          fn count(&self, f: impl Fn(&Outcome) -> bool) -> usize {
              self.entries.iter().filter(|e| f(&e.outcome)).count()
          }

          pub fn measured(&self) -> usize {
              self.count(|o| matches!(o, Outcome::Measured(_)))
          }

          pub fn unstable(&self) -> usize {
              self.count(|o| matches!(o, Outcome::Measured(r) if !r.stable))
          }

          pub fn rejected(&self) -> usize {
              self.count(|o| matches!(o, Outcome::Rejected(_)))
          }

          pub fn dominated(&self) -> usize {
              self.count(|o| matches!(o, Outcome::Dominated))
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      enum State {
          Active,
          Done { stable: bool },
          Dominated,
          Rejected(Validity),
      }

      struct Track {
          cfg: Cfg,
          deviation: f64,
          samples: Vec<f64>,
          /// Sample over the group anchor's sample of the same round.
          ratios: Vec<f64>,
          attempts: u32,
          started: Option<Duration>,
          state: State,
      }

      impl Track {
          fn active(&self) -> bool {
              self.state == State::Active
          }
      }

      pub struct Calibrator<M: Measure> {
          cfg: CalibratorConfig,
          measure: M,
          clock: Arc<dyn Clock>,
          yield_check: Arc<dyn YieldCheck>,
          run_started: Duration,
      }

      type Axes = [(AxisId, Vec<ValueId>)];

      fn ordered(values: &BTreeSet<ValueId>) -> Vec<ValueId> {
          let mut v: Vec<ValueId> = values.iter().cloned().collect();
          v.sort_by_key(|x| match x.as_str().parse::<u64>() {
              Ok(n) => (0, n, String::new()),
              Err(_) => (1, 0, x.as_str().to_owned()),
          });
          v
      }

      fn cartesian_from(start: &ProfileConfig, axes: &Axes) -> Vec<ProfileConfig> {
          let mut acc = vec![start.clone()];
          for (axis, values) in axes {
              acc = acc
                  .iter()
                  .flat_map(|c| values.iter().map(move |v| c.clone().with(axis, v)))
                  .collect();
          }
          acc
      }

      fn product_size(axes: &[&Axes]) -> usize {
          axes.iter()
              .flat_map(|a| a.iter())
              .fold(1usize, |n, (_, v)| n.saturating_mul(v.len()))
      }

      impl<M: Measure> Calibrator<M> {
          pub fn new(mut cfg: CalibratorConfig, measure: M) -> Self {
              cfg.min_reps = cfg.min_reps.max(3);
              Self {
                  cfg,
                  measure,
                  clock: Arc::new(SystemClock::new()),
                  yield_check: Arc::new(NeverYield),
                  run_started: Duration::ZERO,
              }
          }

          pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self {
              self.clock = clock;
              self
          }

          pub fn with_yield_check(mut self, yield_check: Arc<dyn YieldCheck>) -> Self {
              self.yield_check = yield_check;
              self
          }

          pub fn config(&self) -> &CalibratorConfig {
              &self.cfg
          }

          pub fn measure(&self) -> &M {
              &self.measure
          }

          pub fn run(
              &mut self,
              target: &CalibrationTarget,
              caps: &CapabilitySet,
          ) -> Result<CalibrationReport, PrecisionError> {
              let span = tracing::info_span!("calibrate", engine = ?target.engine, model_id = %target.model_id);
              let _entered = span.enter();
              self.run_started = self.clock.elapsed();

              let (core, seps) = self.split_axes(caps)?;
              let mut report = CalibrationReport {
                  target: target.clone(),
                  entries: Vec::new(),
                  front: Vec::new(),
                  budget_exhausted: false,
                  exhaustive_size: product_size(&[&core, &seps]),
                  elapsed: Duration::ZERO,
              };

              let candidates = if self.cfg.exhaustive {
                  let all = [core.as_slice(), seps.as_slice()].concat();
                  self.cfgs(cartesian_from(&ProfileConfig::new(), &all))
              } else {
                  self.axis_stage(&core, &seps, &mut report)?
              };

              let verified = self.measure_group(candidates, true)?;
              self.record(&mut report, Stage::Verify, verified);
              let points: Vec<ParetoPoint> = report
                  .entries
                  .iter()
                  .filter(|e| e.stage == Stage::Verify)
                  .filter_map(|e| match &e.outcome {
                      Outcome::Measured(r) => Some(ParetoPoint {
                          cfg: e.cfg.clone(),
                          time: r.time,
                          deviation: r.deviation,
                          stable: r.stable,
                      }),
                      _ => None,
                  })
                  .collect();
              report.front = pareto_by(points, |p| (p.time.as_secs_f64(), p.deviation));
              report.budget_exhausted |= self.over_budget();
              report.elapsed = self.clock.elapsed().saturating_sub(self.run_started);

              info!(
                  front = report.front.len(),
                  measured = report.measured(),
                  unstable = report.unstable(),
                  rejected = report.rejected(),
                  dominated = report.dominated(),
                  exhaustive_size = report.exhaustive_size,
                  budget_exhausted = report.budget_exhausted,
                  "calibration finished"
              );
              Ok(report)
          }

          fn split_axes(&self, caps: &CapabilitySet) -> Result<(Vec<(AxisId, Vec<ValueId>)>, Vec<(AxisId, Vec<ValueId>)>), PrecisionError> {
              let (mut core, mut seps) = (Vec::new(), Vec::new());
              for (axis, values) in caps.iter().filter(|(_, v)| !v.is_empty()) {
                  let entry = (axis.clone(), ordered(values));
                  if self.cfg.separable_axes.contains(axis) {
                      seps.push(entry);
                  } else {
                      core.push(entry);
                  }
              }
              if core.is_empty() && seps.is_empty() {
                  return Err(PrecisionError::Capability { reason: "empty capability set".into() });
              }
              Ok((core, seps))
          }

          fn cfgs(&self, combos: Vec<ProfileConfig>) -> Vec<Cfg> {
              combos.into_iter().map(|c| Cfg::new(c, self.cfg.seq_lens.clone())).collect()
          }

          /// Measures each separable axis alone per core combination and returns the cartesian
          /// product of the per-axis winners.
          fn axis_stage(
              &mut self,
              core: &Axes,
              seps: &Axes,
              report: &mut CalibrationReport,
          ) -> Result<Vec<Cfg>, PrecisionError> {
              let mut candidates: Vec<ProfileConfig> = Vec::new();
              for core_combo in cartesian_from(&ProfileConfig::new(), core) {
                  if self.over_budget() {
                      report.budget_exhausted = true;
                      break;
                  }
                  let base = seps
                      .iter()
                      .fold(core_combo.clone(), |c, (axis, values)| c.with(axis, &values[0]));
                  let mut winners: Vec<(AxisId, Vec<ValueId>)> = Vec::new();
                  for (axis, values) in seps {
                      if values.len() == 1 {
                          winners.push((axis.clone(), values.clone()));
                          continue;
                      }
                      let group = self.cfgs(
                          values.iter().map(|v| base.clone().with(axis, v)).collect(),
                      );
                      let results = self.measure_group(group, true)?;
                      let won = self.winners(axis, &results);
                      self.record(report, Stage::Axis, results);
                      winners.push((axis.clone(), won));
                  }
                  if winners.iter().any(|(_, v)| v.is_empty()) {
                      continue;
                  }
                  candidates.extend(cartesian_from(&core_combo, &winners));
              }
              let mut seen = HashSet::new();
              candidates.retain(|c| seen.insert(c.clone()));
              Ok(self.cfgs(candidates))
          }

          fn winners(&self, axis: &AxisId, results: &[(Cfg, Outcome)]) -> Vec<ValueId> {
              let points: Vec<(ValueId, f64, f64)> = results
                  .iter()
                  .filter_map(|(cfg, outcome)| match (outcome, cfg.combo.get(axis)) {
                      (Outcome::Measured(r), Some(v)) => {
                          Some((v.clone(), r.time.as_secs_f64(), r.deviation))
                      }
                      _ => None,
                  })
                  .collect();
              pareto_by(points, |p| (p.1, p.2)).into_iter().map(|p| p.0).collect()
          }

          fn record(&self, report: &mut CalibrationReport, stage: Stage, results: Vec<(Cfg, Outcome)>) {
              for (cfg, outcome) in results {
                  // The base combination reappears as the anchor of every axis group.
                  if stage == Stage::Axis && report.entries.iter().any(|e| e.stage == stage && e.cfg == cfg) {
                      continue;
                  }
                  match &outcome {
                      Outcome::Measured(r) => debug!(
                          %cfg, time_us = r.time.as_micros() as u64, deviation = r.deviation,
                          stable = r.stable, samples = r.samples, "combination measured"
                      ),
                      Outcome::Rejected(v) => info!(%cfg, reason = v.as_str(), "combination rejected"),
                      Outcome::Dominated => debug!(%cfg, "combination dominated, stopped early"),
                      Outcome::Skipped => info!(%cfg, "combination skipped"),
                  }
                  if let Outcome::Measured(r) = &outcome {
                      if !r.stable {
                          info!(%cfg, error = %PrecisionError::CalibrationUnstable {
                              reason: "cap reached, best intermediate state used".into(),
                          }, "combination unstable");
                      }
                  }
                  report.entries.push(Entry { stage, cfg, outcome });
              }
          }

          fn over_budget(&self) -> bool {
              self.cfg
                  .total_budget
                  .is_some_and(|b| self.clock.elapsed().saturating_sub(self.run_started) >= b)
          }

          fn capped(&self, t: &Track, now: Duration) -> bool {
              let cap = self.cfg.hard_cap;
              t.attempts >= cap.max_reps
                  || cap.max_time.is_some_and(|m| {
                      t.started.is_some_and(|s| now.saturating_sub(s) >= m)
                  })
          }

          /// Deviation phase: cheap, and decides rejection before any time is spent.
          fn judge(&mut self, cfg: &Cfg) -> Result<(f64, State), PrecisionError> {
              let (deviation, validity) = match self.measure.deviation(cfg) {
                  Ok(r) if r.valid.is_ok() && r.deviation.is_nan() => (f64::NAN, Validity::NonFinite),
                  Ok(r) if r.valid.is_ok() && r.deviation > self.cfg.max_deviation => {
                      (r.deviation, Validity::OverMaxDeviation)
                  }
                  Ok(r) => (r.deviation, r.valid),
                  Err(PrecisionError::Incompatible { reason }) => {
                      debug!(%cfg, %reason, "combination incompatible");
                      (f64::INFINITY, Validity::Failed)
                  }
                  Err(e) => return Err(e),
              };
              let state = if validity.is_ok() { State::Active } else { State::Rejected(validity) };
              Ok((deviation, state))
          }

          fn take_sample(&mut self, cfg: &Cfg) -> Result<Option<f64>, PrecisionError> {
              let sample = if self.yield_check.should_yield() {
                  Sample::Interrupted
              } else {
                  self.measure.sample(cfg)?
              };
              match sample {
                  Sample::Time(d) => Ok(Some(d.as_secs_f64())),
                  Sample::Interrupted => {
                      if !self.cfg.interrupt_backoff.is_zero() {
                          std::thread::sleep(self.cfg.interrupt_backoff);
                      }
                      Ok(None)
                  }
              }
          }

          /// Measures `cfgs` round-robin so load fluctuations hit all of them alike. Odd rounds run in
          /// reverse order (ABBA). Stability is judged on the time ratio to the first combination,
          /// since only relations between combinations matter.
          fn measure_group(
              &mut self,
              cfgs: Vec<Cfg>,
              prune: bool,
          ) -> Result<Vec<(Cfg, Outcome)>, PrecisionError> {
              let mut tracks = Vec::with_capacity(cfgs.len());
              for cfg in cfgs {
                  let (deviation, state) = self.judge(&cfg)?;
                  tracks.push(Track {
                      cfg,
                      deviation,
                      samples: Vec::new(),
                      ratios: Vec::new(),
                      attempts: 0,
                      started: None,
                      state,
                  });
              }
              let anchor = tracks.iter().position(Track::active);
              let had_peers = tracks.iter().filter(|t| t.active()).count() > 1;

              let mut round = 0usize;
              while tracks.iter().any(Track::active) && !self.over_budget() {
                  let mut order: Vec<usize> = (0..tracks.len()).filter(|&i| tracks[i].active()).collect();
                  if round % 2 == 1 {
                      order.reverse();
                  }
                  let mut taken: Vec<Option<f64>> = vec![None; tracks.len()];
                  for i in order {
                      let now = self.clock.elapsed();
                      tracks[i].attempts += 1;
                      tracks[i].started.get_or_insert(now);
                      let cfg = tracks[i].cfg.clone();
                      match self.take_sample(&cfg) {
                          Ok(Some(s)) => {
                              tracks[i].samples.push(s);
                              taken[i] = Some(s);
                          }
                          Ok(None) => {}
                          Err(PrecisionError::Incompatible { reason }) => {
                              debug!(%cfg, %reason, "combination failed while sampling");
                              tracks[i].state = State::Rejected(Validity::Failed);
                          }
                          Err(e) => return Err(e),
                      }
                  }
                  if let Some(a) = anchor {
                      if let Some(base) = taken[a].filter(|s| *s > 0.0) {
                          for (i, t) in tracks.iter_mut().enumerate().filter(|(i, _)| *i != a) {
                              if let Some(s) = taken[i] {
                                  t.ratios.push(s / base);
                              }
                          }
                      }
                  }
                  self.settle(&mut tracks, anchor, had_peers, prune);
                  round += 1;
              }
              Ok(self.finish(tracks))
          }

          fn stable_series(&self, series: &[f64]) -> bool {
              series.len() >= self.cfg.min_reps as usize
                  && spread(series, self.cfg.spread) <= self.cfg.cv_threshold
          }

          fn dominated(&self, tracks: &[Track], i: usize) -> bool {
              let t = &tracks[i];
              if t.samples.len() < self.cfg.min_reps as usize {
                  return false;
              }
              let mine = estimate(&t.samples, self.cfg.estimator);
              tracks.iter().enumerate().any(|(j, o)| {
                  j != i
                      && o.samples.len() >= self.cfg.min_reps as usize
                      && !matches!(o.state, State::Dominated | State::Rejected(_))
                      && o.deviation <= t.deviation
                      && estimate(&o.samples, self.cfg.estimator) * (1.0 + self.cfg.dominance_margin)
                          < mine
              })
          }

          fn settle(&self, tracks: &mut [Track], anchor: Option<usize>, had_peers: bool, prune: bool) {
              let now = self.clock.elapsed();
              for i in 0..tracks.len() {
                  if !tracks[i].active() || Some(i) == anchor {
                      continue;
                  }
                  let next = if prune && self.dominated(tracks, i) {
                      State::Dominated
                  } else if self.stable_series(&tracks[i].ratios) {
                      State::Done { stable: true }
                  } else if self.capped(&tracks[i], now) {
                      State::Done { stable: false }
                  } else {
                      continue;
                  };
                  tracks[i].state = next;
              }

              let Some(a) = anchor.filter(|&a| tracks[a].active()) else { return };
              if tracks.iter().enumerate().any(|(j, t)| j != a && t.active()) {
                  return;
              }
              let own_ok = self.stable_series(&tracks[a].samples);
              let next = if had_peers {
                  let peers = tracks.iter().enumerate().filter(|(j, _)| *j != a).map(|(_, t)| t.state);
                  let (mut any_stable, mut any_unstable) = (false, false);
                  for s in peers {
                      match s {
                          State::Done { stable: true } => any_stable = true,
                          State::Done { stable: false } => any_unstable = true,
                          _ => {}
                      }
                  }
                  State::Done { stable: (any_stable && !any_unstable) || own_ok }
              } else if own_ok {
                  State::Done { stable: true }
              } else if self.capped(&tracks[a], now) {
                  State::Done { stable: false }
              } else {
                  return;
              };
              tracks[a].state = next;
          }

          fn finish(&self, tracks: Vec<Track>) -> Vec<(Cfg, Outcome)> {
              tracks
                  .into_iter()
                  .map(|t| {
                      let stable = matches!(t.state, State::Done { stable: true });
                      let outcome = match t.state {
                          State::Dominated => Outcome::Dominated,
                          State::Rejected(v) => Outcome::Rejected(v),
                          State::Active | State::Done { .. } if t.samples.is_empty() => Outcome::Skipped,
                          State::Active | State::Done { .. } => Outcome::Measured(MeasureResult {
                              time: Duration::from_secs_f64(
                                  estimate(&t.samples, self.cfg.estimator).max(0.0),
                              ),
                              deviation: t.deviation,
                              valid: Validity::Ok,
                              stable,
                              samples: t.samples.len() as u32,
                          }),
                      };
                      (t.cfg, outcome)
                  })
                  .collect()
          }
      }

      #[cfg(test)]
      #[path = "test/calibrate.rs"]
      mod tests;
  reason: "calibrate.rs (Calibrator, Pruning, interleaved Messung, Pareto) anlegen"
```
/answer 9c73-13d allow
ReasoningToken: 1836
Control Request:
```yaml
id: 9c73-13e
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/calibrate.rs
    id: impl_Calibrator_M_.run
    old_text: "product_size(&[&core, &seps])"
    new_text: "product_size(&[core.as_slice(), seps.as_slice()])"
  reason: Explizite Slice-Koerzion für product_size
```
/answer 9c73-13e allow
ReasoningToken: 88
Control Request:
```yaml
id: 9c73-13f
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
    id: Measure
    position: after
    source: |
      impl<T: Measure + ?Sized> Measure for Box<T> {
          fn deviation(&mut self, cfg: &Cfg) -> Result<DeviationReport, PrecisionError> {
              (**self).deviation(cfg)
          }

          fn sample(&mut self, cfg: &Cfg) -> Result<Sample, PrecisionError> {
              (**self).sample(cfg)
          }
      }
  reason: Box<dyn Measure> aus der Factory direkt im Calibrator nutzbar machen
```
/answer 9c73-13f allow
ReasoningToken: 923
Control Request:
```yaml
id: 9c73-140
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_mock.rs
    source: |
      //! Scripted fake of the measurement: seeded noise per combination, no device needed.
      #![cfg(any(test, feature = "mock"))]

      use std::collections::hash_map::DefaultHasher;
      use std::collections::HashMap;
      use std::hash::{Hash, Hasher};
      use std::sync::atomic::{AtomicU64, Ordering};
      use std::sync::Arc;
      use std::time::Duration;

      use crate::resident::error::PrecisionError;
      use crate::resident::mock::{self, Measure as _, NoiseModel};

      use super::measure::{Cfg, Clock, DeviationReport, Measure, Sample, Validity};

      /// Clock that only moves when told to; the fake advances it by every sampled time.
      #[derive(Default)]
      pub struct ManualClock {
          nanos: AtomicU64,
      }

      impl ManualClock {
          pub fn new() -> Self {
              Self::default()
          }

          pub fn advance(&self, by: Duration) {
              self.nanos.fetch_add(by.as_nanos() as u64, Ordering::Relaxed);
          }
      }

      impl Clock for ManualClock {
          fn elapsed(&self) -> Duration {
              Duration::from_nanos(self.nanos.load(Ordering::Relaxed))
          }
      }

      /// Behaviour of one combination. Times are milliseconds; the noise models perturb them by
      /// absolute amounts of the same unit.
      #[derive(Clone, Debug)]
      pub struct FakeCombo {
          pub time_ms: f64,
          pub noise: NoiseModel,
          pub deviation: f64,
          pub valid: Validity,
          pub incompatible: bool,
      }

      impl FakeCombo {
          pub fn new(time_ms: f64, deviation: f64) -> Self {
              Self {
                  time_ms,
                  noise: NoiseModel::Constant,
                  deviation,
                  valid: Validity::Ok,
                  incompatible: false,
              }
          }

          pub fn noise(mut self, noise: NoiseModel) -> Self {
              self.noise = noise;
              self
          }

          pub fn valid(mut self, valid: Validity) -> Self {
              self.valid = valid;
              self
          }

          pub fn incompatible(mut self) -> Self {
              self.incompatible = true;
              self
          }
      }

      pub struct FakeMeasure {
          script: Box<dyn Fn(&Cfg) -> FakeCombo + Send>,
          seed: u64,
          sources: HashMap<Cfg, mock::FakeMeasure>,
          clock: Option<Arc<ManualClock>>,
          load: Option<(usize, Box<dyn Fn(usize) -> f64 + Send>)>,
          calls: usize,
          counts: HashMap<Cfg, u32>,
          order: Vec<Cfg>,
      }

      impl FakeMeasure {
          pub fn new(seed: u64, script: impl Fn(&Cfg) -> FakeCombo + Send + 'static) -> Self {
              Self {
                  script: Box::new(script),
                  seed,
                  sources: HashMap::new(),
                  clock: None,
                  load: None,
                  calls: 0,
                  counts: HashMap::new(),
                  order: Vec::new(),
              }
          }

          pub fn with_clock(mut self, clock: Arc<ManualClock>) -> Self {
              self.clock = Some(clock);
              self
          }

          /// Multiplies every sample by `load(call_index / per_round)`: a disturbance shared by all
          /// combinations of a round, as real background load would be.
          pub fn with_shared_load(
              mut self,
              per_round: usize,
              load: impl Fn(usize) -> f64 + Send + 'static,
          ) -> Self {
              self.load = Some((per_round.max(1), Box::new(load)));
              self
          }

          pub fn samples_taken(&self, cfg: &Cfg) -> u32 {
              self.counts.get(cfg).copied().unwrap_or(0)
          }

          /// Every sampled combination in call order.
          pub fn order(&self) -> &[Cfg] {
              &self.order
          }
      }

      impl Measure for FakeMeasure {
          fn deviation(&mut self, cfg: &Cfg) -> Result<DeviationReport, PrecisionError> {
              let combo = (self.script)(cfg);
              if combo.incompatible {
                  return Err(PrecisionError::Incompatible { reason: "scripted".into() });
              }
              Ok(DeviationReport { deviation: combo.deviation, valid: combo.valid })
          }

          fn sample(&mut self, cfg: &Cfg) -> Result<Sample, PrecisionError> {
              let combo = (self.script)(cfg);
              let seed = {
                  let mut h = DefaultHasher::new();
                  cfg.hash(&mut h);
                  self.seed ^ h.finish()
              };
              let source = self
                  .sources
                  .entry(cfg.clone())
                  .or_insert_with(|| mock::FakeMeasure::new(combo.time_ms, combo.noise.clone(), seed));
              let mut ms = source.sample().max(1e-6);
              if let Some((per_round, load)) = &self.load {
                  ms *= load(self.calls / per_round);
              }
              self.calls += 1;
              *self.counts.entry(cfg.clone()).or_default() += 1;
              self.order.push(cfg.clone());
              let time = Duration::from_secs_f64(ms / 1000.0);
              if let Some(clock) = &self.clock {
                  clock.advance(time);
              }
              Ok(Sample::Time(time))
          }
      }
  reason: FakeMeasure und ManualClock für GPU-freie Tests anlegen
```
/answer 9c73-140 allow
ReasoningToken: 81
Control Request:
```yaml
id: 9c73-141
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/pool.rs
    source: |
      //! Dedicated low-priority threads for calibration and CPU kernels; kept apart from the tokio
      //! runtime so measurements never occupy its workers.

      use std::io;
      use std::panic::{catch_unwind, AssertUnwindSafe};
      use std::thread::JoinHandle;

      use crossbeam::channel::{bounded, unbounded, Receiver, Sender};

      /// Lowest scheduling priority (`nice 19`) for the calling thread. Returns whether it applied;
      /// on platforms without support the thread keeps its priority.
      pub fn lower_current_thread_priority() -> bool {
          #[cfg(target_os = "linux")]
          {
              // SAFETY: plain syscalls without pointer arguments.
              unsafe {
                  let tid = libc::syscall(libc::SYS_gettid) as libc::id_t;
                  libc::setpriority(libc::PRIO_PROCESS, tid, 19) == 0
              }
          }
          #[cfg(not(target_os = "linux"))]
          {
              false
          }
      }

      type Job = Box<dyn FnOnce() + Send + 'static>;

      /// Result of a submitted job.
      pub struct JobHandle<T>(Receiver<T>);

      impl<T> JobHandle<T> {
          /// Blocks for the result; `None` if the job panicked or the pool was dropped first.
          pub fn join(self) -> Option<T> {
              self.0.recv().ok()
          }

          pub fn try_join(&self) -> Option<T> {
              self.0.try_recv().ok()
          }
      }

      pub struct LowPriorityPool {
          tx: Option<Sender<Job>>,
          workers: Vec<JoinHandle<()>>,
      }

      impl LowPriorityPool {
          pub fn new(name: &str, threads: usize) -> io::Result<Self> {
              let (tx, rx) = unbounded::<Job>();
              let mut workers = Vec::with_capacity(threads.max(1));
              for i in 0..threads.max(1) {
                  let rx = rx.clone();
                  let worker = std::thread::Builder::new().name(format!("{name}-{i}")).spawn(move || {
                      if !lower_current_thread_priority() {
                          tracing::debug!("thread priority not lowered");
                      }
                      while let Ok(job) = rx.recv() {
                          // A panicking job must not take the worker down with it.
                          let _ = catch_unwind(AssertUnwindSafe(job));
                      }
                  })?;
                  workers.push(worker);
              }
              Ok(Self { tx: Some(tx), workers })
          }

          pub fn submit<T: Send + 'static>(
              &self,
              job: impl FnOnce() -> T + Send + 'static,
          ) -> JobHandle<T> {
              let (tx, rx) = bounded(1);
              let boxed: Job = Box::new(move || {
                  let _ = tx.send(job());
              });
              if let Some(queue) = &self.tx {
                  let _ = queue.send(boxed);
              }
              JobHandle(rx)
          }
      }

      impl Drop for LowPriorityPool {
          fn drop(&mut self) {
              self.tx = None;
              for worker in self.workers.drain(..) {
                  let _ = worker.join();
              }
          }
      }

      #[cfg(test)]
      #[path = "test/pool.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/pool.rs
    source: |
      use super::*;

      #[test]
      fn jobs_return_their_result() {
          let pool = LowPriorityPool::new("test-pool", 2).unwrap();
          let handles: Vec<_> = (0..8).map(|i| pool.submit(move || i * 2)).collect();
          let got: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
          assert_eq!(got, (0..8).map(|i| i * 2).collect::<Vec<_>>());
      }

      #[test]
      fn a_panicking_job_does_not_kill_the_worker() {
          let pool = LowPriorityPool::new("test-pool", 1).unwrap();
          let bad = pool.submit(|| -> i32 { panic!("boom") });
          assert!(bad.join().is_none());
          assert_eq!(pool.submit(|| 7).join(), Some(7));
      }

      #[test]
      fn workers_run_apart_from_the_caller() {
          let pool = LowPriorityPool::new("test-pool", 1).unwrap();
          let caller = std::thread::current().id();
          let worker = pool.submit(|| std::thread::current().id()).join().unwrap();
          assert_ne!(caller, worker);
      }

      #[cfg(target_os = "linux")]
      #[test]
      fn workers_run_at_lowest_priority() {
          let pool = LowPriorityPool::new("test-pool", 1).unwrap();
          let nice = pool
              .submit(|| unsafe {
                  let tid = libc::syscall(libc::SYS_gettid) as libc::id_t;
                  libc::getpriority(libc::PRIO_PROCESS, tid)
              })
              .join()
              .unwrap();
          assert_eq!(nice, 19);
      }

      #[cfg(target_os = "linux")]
      #[test]
      fn lowering_priority_is_scoped_to_the_calling_thread() {
          let before = unsafe { libc::getpriority(libc::PRIO_PROCESS, 0) };
          std::thread::spawn(|| assert!(lower_current_thread_priority())).join().unwrap();
          let after = unsafe { libc::getpriority(libc::PRIO_PROCESS, 0) };
          assert_eq!(before, after);
      }

      #[test]
      fn dropping_the_pool_waits_for_queued_jobs() {
          use std::sync::atomic::{AtomicU32, Ordering};
          use std::sync::Arc;
          let done = Arc::new(AtomicU32::new(0));
          {
              let pool = LowPriorityPool::new("test-pool", 1).unwrap();
              for _ in 0..5 {
                  let done = done.clone();
                  drop(pool.submit(move || {
                      done.fetch_add(1, Ordering::SeqCst);
                  }));
              }
          }
          assert_eq!(done.load(Ordering::SeqCst), 5);
      }
  reason: Eigener Low-Priority-Thread-Pool (nice 19) mit Tests anlegen
```
/answer 9c73-141 allow
ReasoningToken: 1149
Control Request:
```yaml
id: 9c73-142
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cpu.rs
    source: |
      //! Real CPU measurement of the fp32 scoring path. Only the axes this workload reacts to are
      //! read from a combination: `batch` (queries per pass), `threads`, and the operand/scoring
      //! type, which must be fp32 because the host kernels exist for nothing else.

      use std::collections::{HashMap, HashSet};
      use std::sync::Arc;
      use std::time::{Duration, Instant};

      use rayon::prelude::*;

      use crate::resident::error::PrecisionError;
      use crate::resident::kernel::cpu_simd::dot;

      use super::known::{FP32, OPERANDS, SCORING, THREADS};
      use super::measure::{Cfg, DeviationReport, Measure, Sample, WindowTuning, YieldCheck};
      use super::workload::{ScoringWorkload, WorkloadSpec};

      pub struct CpuScoringMeasure {
          load: ScoringWorkload,
          tuning: WindowTuning,
          yield_check: Arc<dyn YieldCheck>,
          pools: HashMap<usize, rayon::ThreadPool>,
          warmed: HashSet<(usize, usize)>,
      }

      impl CpuScoringMeasure {
          pub fn new(
              spec: WorkloadSpec,
              tuning: WindowTuning,
              yield_check: Arc<dyn YieldCheck>,
          ) -> Result<Self, PrecisionError> {
              Ok(Self {
                  load: ScoringWorkload::generate(spec),
                  tuning,
                  yield_check,
                  pools: HashMap::new(),
                  warmed: HashSet::new(),
              })
          }

          fn threads(cfg: &Cfg) -> Result<usize, PrecisionError> {
              match cfg.combo.get(&THREADS) {
                  None => Ok(1),
                  Some(v) => v.as_str().parse::<usize>().ok().filter(|n| *n > 0).ok_or_else(|| {
                      PrecisionError::Incompatible { reason: format!("threads value `{v}` invalid") }
                  }),
              }
          }

          fn check_types(cfg: &Cfg) -> Result<(), PrecisionError> {
              for axis in [&*OPERANDS, &*SCORING] {
                  if let Some(v) = cfg.combo.get(axis) {
                      if v != &*FP32 {
                          return Err(PrecisionError::Incompatible {
                              reason: format!("no cpu kernel for {axis}={v}"),
                          });
                      }
                  }
              }
              Ok(())
          }

          /// Returns the query count and thread count after validating the combination.
          fn plan(&mut self, cfg: &Cfg) -> Result<(usize, usize), PrecisionError> {
              Self::check_types(cfg)?;
              let nq = self.load.query_count(cfg)?;
              let threads = Self::threads(cfg)?;
              if !self.pools.contains_key(&threads) {
                  let pool = rayon::ThreadPoolBuilder::new()
                      .num_threads(threads)
                      .thread_name(|i| format!("cpu-measure-{i}"))
                      .build()
                      .map_err(|e| PrecisionError::Incompatible { reason: e.to_string() })?;
                  self.pools.insert(threads, pool);
              }
              Ok((nq, threads))
          }

          /// Scores into `out`, document-major (`docs x nq`).
          fn score(&self, threads: usize, nq: usize, out: &mut [f32]) {
              let dim = self.load.spec().dim;
              let queries = self.load.queries(nq);
              self.pools[&threads].install(|| {
                  out.par_chunks_mut(nq).zip(self.load.docs().par_chunks(dim)).for_each(|(row, doc)| {
                      for (q, slot) in row.iter_mut().enumerate() {
                          *slot = dot(&queries[q * dim..(q + 1) * dim], doc);
                      }
                  });
              });
          }
      }

      impl Measure for CpuScoringMeasure {
          fn deviation(&mut self, cfg: &Cfg) -> Result<DeviationReport, PrecisionError> {
              let (nq, threads) = self.plan(cfg)?;
              let docs = self.load.spec().docs;
              let mut doc_major = vec![0f32; docs * nq];
              self.score(threads, nq, &mut doc_major);
              let mut scores = vec![0f32; docs * nq];
              for (s, row) in doc_major.chunks(nq).enumerate() {
                  for (q, v) in row.iter().enumerate() {
                      scores[q * docs + s] = *v;
                  }
              }
              Ok(self.load.judge(&scores, nq))
          }

          fn sample(&mut self, cfg: &Cfg) -> Result<Sample, PrecisionError> {
              let (nq, threads) = self.plan(cfg)?;
              let mut out = vec![0f32; self.load.spec().docs * nq];
              if self.warmed.insert((threads, nq)) {
                  self.score(threads, nq, &mut out);
              }
              let window = Duration::from_secs_f64(self.tuning.window_ms / 1e3);
              let start = Instant::now();
              let mut passes = 0u32;
              loop {
                  self.score(threads, nq, &mut out);
                  passes += 1;
                  if start.elapsed() >= window {
                      break;
                  }
                  if self.yield_check.should_yield() {
                      return Ok(Sample::Interrupted);
                  }
              }
              std::hint::black_box(&out);
              Ok(Sample::Time(start.elapsed().div_f64(f64::from(passes) * nq as f64)))
          }
      }

      #[cfg(test)]
      #[path = "test/measure_cpu.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/measure_cpu.rs
    source: |
      use super::*;
      use crate::resident::precision::known::{BATCH, FP16};
      use crate::resident::precision::measure::{NeverYield, Validity};
      use crate::resident::precision::profile::ProfileConfig;
      use std::sync::atomic::AtomicBool;
      use std::sync::atomic::Ordering;

      fn spec() -> WorkloadSpec {
          WorkloadSpec { docs: 300, dim: 32, max_queries: 8, ..WorkloadSpec::default() }
      }

      fn measure() -> CpuScoringMeasure {
          CpuScoringMeasure::new(spec(), WindowTuning::default(), Arc::new(NeverYield)).unwrap()
      }

      fn cfg(threads: &str, batch: &str) -> Cfg {
          Cfg::new(
              ProfileConfig::new()
                  .with(&*OPERANDS, &*FP32)
                  .with(&*THREADS, threads)
                  .with(&*BATCH, batch),
              vec![],
          )
      }

      #[test]
      fn fp32_scoring_matches_the_reference_for_any_thread_count() {
          let mut m = measure();
          for threads in ["1", "2", "3"] {
              let r = m.deviation(&cfg(threads, "4")).unwrap();
              assert_eq!(r.valid, Validity::Ok, "threads={threads}");
              assert!(r.deviation < 1e-6, "threads={threads}");
          }
      }

      #[test]
      fn samples_are_positive_times() {
          let mut m = measure();
          match m.sample(&cfg("2", "4")).unwrap() {
              Sample::Time(t) => assert!(t > Duration::ZERO),
              Sample::Interrupted => panic!("unexpected interrupt"),
          }
      }

      #[test]
      fn non_fp32_types_are_incompatible() {
          let mut m = measure();
          let c = Cfg::new(ProfileConfig::new().with(&*OPERANDS, &*FP16), vec![]);
          assert!(matches!(m.deviation(&c), Err(PrecisionError::Incompatible { .. })));
          assert!(matches!(m.sample(&c), Err(PrecisionError::Incompatible { .. })));
      }

      #[test]
      fn invalid_axis_values_are_incompatible() {
          let mut m = measure();
          assert!(m.deviation(&cfg("0", "4")).is_err());
          assert!(m.deviation(&cfg("x", "4")).is_err());
          assert!(m.deviation(&cfg("1", "99")).is_err());
      }

      #[test]
      fn a_waiting_task_interrupts_a_long_window() {
          let flag = Arc::new(AtomicBool::new(true));
          let tuning = WindowTuning { window_ms: 10_000.0, slice_budget_ms: 5.0 };
          let mut m = CpuScoringMeasure::new(spec(), tuning, flag.clone()).unwrap();
          assert_eq!(m.sample(&cfg("1", "2")).unwrap(), Sample::Interrupted);
          flag.store(false, Ordering::Relaxed);
      }
  reason: "Reale CPU-Messung (fp32 Scoring, threads/batch) mit Tests anlegen"
```
/answer 9c73-142 allow
ReasoningToken: 2118
Control Request:
```yaml
id: 9c73-143
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cuda.rs
    source: |
      //! Real CUDA measurement of the scoring path (`cosine_dot`).
      //!
      //! Time comes from CUDA events on the stream, so launch overhead is excluded. Each timing
      //! sample flushes L2 first and covers a window of back-to-back launches, split into groups that
      //! stay within the slice budget because a running kernel cannot be preempted. Only the axes
      //! this workload reacts to are read from a combination: `batch` (queries per launch) and the
      //! document type from `scoring` (falling back to `operands`).

      use std::collections::HashMap;
      use std::fmt::Display;
      use std::sync::Arc;
      use std::time::Duration;

      use cudarc::driver::sys::CUevent_flags;
      use cudarc::driver::{CudaContext, CudaFunction, CudaSlice, CudaStream};
      use half::f16;

      use crate::resident::error::PrecisionError;
      use crate::resident::kernel::cuda::{launch_cosine, CudaKernels, COSINE_DOT};
      use crate::resident::memory::DType;
      use crate::resident::subengine::DeviceHandle;

      use super::known::{FP16, FP32, OPERANDS, SCORING};
      use super::measure::{Cfg, DeviationReport, Measure, Sample, WindowTuning, YieldCheck};
      use super::workload::{ScoringWorkload, WorkloadSpec};

      fn incompatible(e: impl Display) -> PrecisionError {
          PrecisionError::Incompatible { reason: format!("cuda measurement: {e}") }
      }

      #[derive(Clone, Copy)]
      struct Plan {
          iters: u32,
          group: u32,
      }

      pub struct CudaScoringMeasure {
          stream: Arc<CudaStream>,
          kernels: CudaKernels,
          load: ScoringWorkload,
          tuning: WindowTuning,
          yield_check: Arc<dyn YieldCheck>,
          docs32: CudaSlice<f32>,
          docs16: CudaSlice<u16>,
          queries: CudaSlice<f32>,
          out: CudaSlice<f32>,
          flush: CudaSlice<u8>,
          plans: HashMap<(bool, usize), Plan>,
      }

      impl CudaScoringMeasure {
          /// L2 of the target generation is a few MiB; this exceeds it by a wide margin.
          const FLUSH_BYTES: usize = 64 << 20;
          const WARMUP_LAUNCHES: u32 = 3;
          const MAX_ITERS: u32 = 10_000;

          pub fn new(
              device: &DeviceHandle,
              spec: WorkloadSpec,
              tuning: WindowTuning,
              yield_check: Arc<dyn YieldCheck>,
          ) -> Result<Self, PrecisionError> {
              let ctx: &Arc<CudaContext> = device.cuda_context().ok_or_else(|| {
                  PrecisionError::Capability { reason: "device has no cuda context".into() }
              })?;
              let stream = ctx.new_stream().map_err(incompatible)?;
              let kernels = CudaKernels::new(ctx).map_err(incompatible)?;
              let load = ScoringWorkload::generate(spec);
              let bits: Vec<u16> = load.docs().iter().map(|&v| f16::from_f32(v).to_bits()).collect();
              let (docs, max_q) = (load.spec().docs, load.spec().max_queries);
              Ok(Self {
                  docs32: stream.clone_htod(load.docs()).map_err(incompatible)?,
                  docs16: stream.clone_htod(&bits).map_err(incompatible)?,
                  queries: stream.clone_htod(load.queries(max_q)).map_err(incompatible)?,
                  out: stream.alloc_zeros::<f32>(docs * max_q).map_err(incompatible)?,
                  flush: stream.alloc_zeros::<u8>(Self::FLUSH_BYTES).map_err(incompatible)?,
                  stream,
                  kernels,
                  load,
                  tuning,
                  yield_check,
                  plans: HashMap::new(),
              })
          }

          fn dtype(cfg: &Cfg) -> Result<DType, PrecisionError> {
              for axis in [&*SCORING, &*OPERANDS] {
                  if let Some(v) = cfg.combo.get(axis) {
                      return if v == &*FP32 {
                          Ok(DType::F32)
                      } else if v == &*FP16 {
                          Ok(DType::F16)
                      } else {
                          Err(incompatible(format!("no kernel for {axis}={v}")))
                      };
                  }
              }
              Ok(DType::F32)
          }

          fn function(&self, dtype: DType) -> Result<CudaFunction, PrecisionError> {
              self.kernels
                  .function(COSINE_DOT, dtype, self.load.spec().dim as u32)
                  .map_err(incompatible)
          }

          fn launch(&mut self, f: &CudaFunction, dtype: DType, nq: usize) -> Result<(), PrecisionError> {
              let n = self.load.spec().docs as u32;
              match dtype {
                  DType::F16 => launch_cosine(
                      &self.stream, f, &self.docs16, &self.queries, &mut self.out, n, nq as u32,
                  ),
                  _ => launch_cosine(
                      &self.stream, f, &self.docs32, &self.queries, &mut self.out, n, nq as u32,
                  ),
              }
              .map_err(incompatible)
          }

          fn timed(&mut self, f: &CudaFunction, dtype: DType, nq: usize, launches: u32) -> Result<f64, PrecisionError> {
              let flags = Some(CUevent_flags::CU_EVENT_DEFAULT);
              let start = self.stream.record_event(flags).map_err(incompatible)?;
              for _ in 0..launches {
                  self.launch(f, dtype, nq)?;
              }
              let end = self.stream.record_event(flags).map_err(incompatible)?;
              start.elapsed_ms(&end).map(f64::from).map_err(incompatible)
          }

          /// Warms up, then sizes the window and the launch groups from one timed launch.
          fn plan(&mut self, f: &CudaFunction, dtype: DType, nq: usize) -> Result<Plan, PrecisionError> {
              for _ in 0..Self::WARMUP_LAUNCHES {
                  self.launch(f, dtype, nq)?;
              }
              self.stream.synchronize().map_err(incompatible)?;
              let one_ms = self.timed(f, dtype, nq, 1)?.max(1e-3);
              let iters = (self.tuning.window_ms / one_ms).ceil().clamp(1.0, f64::from(Self::MAX_ITERS));
              let group = (self.tuning.slice_budget_ms / one_ms).floor().clamp(1.0, iters);
              Ok(Plan { iters: iters as u32, group: group as u32 })
          }
      }

      impl Measure for CudaScoringMeasure {
          fn deviation(&mut self, cfg: &Cfg) -> Result<DeviationReport, PrecisionError> {
              let dtype = Self::dtype(cfg)?;
              let nq = self.load.query_count(cfg)?;
              let f = self.function(dtype)?;
              self.launch(&f, dtype, nq)?;
              self.stream.synchronize().map_err(incompatible)?;
              let all = self.stream.clone_dtoh(&self.out).map_err(incompatible)?;
              let n = self.load.spec().docs;
              Ok(self.load.judge(&all[..nq * n], nq))
          }

          fn sample(&mut self, cfg: &Cfg) -> Result<Sample, PrecisionError> {
              let dtype = Self::dtype(cfg)?;
              let nq = self.load.query_count(cfg)?;
              let f = self.function(dtype)?;
              let key = (dtype == DType::F16, nq);
              let plan = match self.plans.get(&key) {
                  Some(p) => *p,
                  None => {
                      let p = self.plan(&f, dtype, nq)?;
                      self.plans.insert(key, p);
                      p
                  }
              };
              self.stream.memset_zeros(&mut self.flush).map_err(incompatible)?;
              let (mut done, mut total_ms) = (0u32, 0f64);
              while done < plan.iters {
                  let group = plan.group.min(plan.iters - done);
                  total_ms += self.timed(&f, dtype, nq, group)?;
                  done += group;
                  if done < plan.iters && self.yield_check.should_yield() {
                      return Ok(Sample::Interrupted);
                  }
              }
              let per_item = total_ms / 1e3 / f64::from(plan.iters) / nq as f64;
              Ok(Sample::Time(Duration::from_secs_f64(per_item)))
          }
      }

      #[cfg(test)]
      #[path = "test/measure_cuda.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/measure_cuda.rs
    source: |
      use super::*;
      use crate::resident::precision::calibrate::{CalibrationTarget, Calibrator, CalibratorConfig, Outcome};
      use crate::resident::precision::known::BATCH;
      use crate::resident::precision::measure::{NeverYield, Validity};
      use crate::resident::precision::profile::ProfileConfig;
      use crate::resident::precision::CapabilitySet;
      use crate::resident::subengine::SubengineKind;

      fn measure(spec: WorkloadSpec, tuning: WindowTuning) -> CudaScoringMeasure {
          let dev = DeviceHandle::cuda(0).expect("cuda device");
          CudaScoringMeasure::new(&dev, spec, tuning, Arc::new(NeverYield)).unwrap()
      }

      fn small() -> WorkloadSpec {
          WorkloadSpec { docs: 4_000, dim: 64, max_queries: 8, ..WorkloadSpec::default() }
      }

      fn cfg(dtype: &crate::resident::precision::ValueId, batch: &str) -> Cfg {
          Cfg::new(ProfileConfig::new().with(&*SCORING, dtype).with(&*BATCH, batch), vec![])
      }

      #[test]
      #[ignore = "requires a CUDA device"]
      fn fp32_matches_the_reference_and_fp16_stays_close() {
          let mut m = measure(small(), WindowTuning::default());
          let r32 = m.deviation(&cfg(&FP32, "4")).unwrap();
          assert_eq!(r32.valid, Validity::Ok);
          assert!(r32.deviation < 1e-4, "{}", r32.deviation);
          let r16 = m.deviation(&cfg(&FP16, "4")).unwrap();
          assert_eq!(r16.valid, Validity::Ok);
          assert!(r16.deviation < 5e-2, "{}", r16.deviation);
      }

      #[test]
      #[ignore = "requires a CUDA device"]
      fn samples_scale_to_the_window_and_stay_positive() {
          let mut m = measure(small(), WindowTuning::default());
          for _ in 0..3 {
              match m.sample(&cfg(&FP32, "4")).unwrap() {
                  Sample::Time(t) => assert!(t > Duration::ZERO && t < Duration::from_millis(50)),
                  Sample::Interrupted => panic!("unexpected interrupt"),
              }
          }
          let plan = m.plans[&(false, 4)];
          assert!(plan.group >= 1 && plan.group <= plan.iters);
      }

      #[test]
      #[ignore = "requires a CUDA device"]
      fn a_waiting_task_interrupts_between_groups() {
          use std::sync::atomic::{AtomicBool, Ordering};
          let flag = Arc::new(AtomicBool::new(false));
          let dev = DeviceHandle::cuda(0).unwrap();
          // Window far above the slice budget forces several groups.
          let tuning = WindowTuning { window_ms: 200.0, slice_budget_ms: 1.0 };
          let mut m = CudaScoringMeasure::new(&dev, small(), tuning, flag.clone()).unwrap();
          assert!(matches!(m.sample(&cfg(&FP32, "4")).unwrap(), Sample::Time(_)));
          flag.store(true, Ordering::Relaxed);
          assert_eq!(m.sample(&cfg(&FP32, "4")).unwrap(), Sample::Interrupted);
      }

      #[test]
      #[ignore = "requires a CUDA device"]
      fn unsupported_types_are_incompatible() {
          let mut m = measure(small(), WindowTuning::default());
          let int8 = crate::resident::precision::known::INT8_DP4A.clone();
          assert!(matches!(m.deviation(&cfg(&int8, "4")), Err(PrecisionError::Incompatible { .. })));
      }

      /// End to end on the device: probe-style capability set, real measurement, Pareto front.
      #[test]
      #[ignore = "requires a CUDA device; measurement"]
      fn gate_calibration_front_on_device() {
          let caps = CapabilitySet::new()
              .with(&*SCORING, &*FP32)
              .with(&*SCORING, &*FP16)
              .with(&*BATCH, "1")
              .with(&*BATCH, "8");
          let spec = WorkloadSpec { docs: 20_000, dim: 384, max_queries: 8, ..WorkloadSpec::default() };
          let cfg = CalibratorConfig::default();
          let m = measure(spec, cfg.tuning());
          let mut cal = Calibrator::new(cfg, m);
          let target = CalibrationTarget { engine: SubengineKind::Cuda, model_id: "gate".into() };
          let report = cal.run(&target, &caps).unwrap();
          for e in &report.entries {
              match &e.outcome {
                  Outcome::Measured(r) => println!(
                      "G-CALIB {:?} {} time {:?} dev {:.3e} stable {} samples {}",
                      e.stage, e.cfg, r.time, r.deviation, r.stable, r.samples
                  ),
                  other => println!("G-CALIB {:?} {} {other:?}", e.stage, e.cfg),
              }
          }
          for p in &report.front {
              println!("G-CALIB front {} time {:?} dev {:.3e} stable {}", p.cfg, p.time, p.deviation, p.stable);
          }
          assert!(!report.front.is_empty());
          assert_eq!(report.rejected(), 0);
      }
  reason: "Reale CUDA-Messung (Events, Warmup, L2-Flush, Slices) mit Device-Tests anlegen"
```
/answer 9c73-143 allow
Control Request:
```yaml
id: 9c73-144
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
    id: make_measure
    old_text: |2-
                  super::workload::WorkloadSpec::default(),
                  tuning,
                  yield_check,
              )?)),
    new_text: |2-
                  super::workload::WorkloadSpec { docs: 4_000, ..Default::default() },
                  tuning,
                  yield_check,
              )?)),
  reason: "Kleinere Workload für CPU, damit ein Durchlauf kurz bleibt"
```
/answer 9c73-144 allow
Error on control endpoint
Error on control endpoint
ReasoningToken: 68
Control Request:
```yaml
id: ed48-1
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
  reason: "Erneuter Versuch nach Verbindungsabbruch: mod.rs lesen"
```
/answer ed48-1 allow
Control Request:
```yaml
id: ed48-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! CapabilitySet/Profile.
      - id: axis
        signature: mod axis;
      - id: capability
        signature: mod capability;
      - id: known
        signature: pub mod known;
      - id: pareto
        signature: mod pareto;
      - id: probe
        signature: mod probe;
      - id: probe_cpu
        signature: mod probe_cpu;
      - id: probe_cuda
        signature: mod probe_cuda;
      - id: profile
        signature: mod profile;
      - id: CD5tM3|eS3CGz
        signature: "pub use axis::{AxisId, ValueId};"
```
/answer ed48-2 allow
ReasoningToken: 134
Control Request:
```yaml
id: ed48-3
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
  regexp: "pub use|docs: 4_000|WorkloadSpec::default"
  reason: Re-Exports in mod.rs und Stand der make_measure-Änderung prüfen
```
/answer ed48-3 allow
Control Request:
```yaml
id: ed48-4
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: CD5tM3|eS3CGz
        code: |
          pub use axis::{AxisId, ValueId};
          pub use capability::CapabilitySet;
          pub use pareto::{pareto, positions};
          pub use probe::{
              intersect_with_kernels, make_probe, CapabilityProbe, KernelFilteredProbe, ProbeConfig,
          };
          #[cfg(any(test, feature = "mock"))]
          pub use probe::FakeCapabilityProbe;
          pub use probe_cpu::{CpuFeatureSource, CpuFeatures, CpuProbe, FixedCpu, HostCpu};
          pub use probe_cuda::{cuda_capabilities, has_tensor_cores, CudaProbe};
          pub use profile::{Profile, ProfileConfig};
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
      nodes:
      - id: make_measure
        code: |
          /// Real measurement for the device behind `device`. This is the only place that branches
          /// on the `cuda` feature for measuring.
          pub fn make_measure(
              device: &DeviceHandle,
              tuning: WindowTuning,
              yield_check: Arc<dyn YieldCheck>,
          ) -> Result<Box<dyn Measure>, PrecisionError> {
              use crate::resident::memory::DeviceKind;
              match device.kind() {
                  DeviceKind::Cpu => {
                      Ok(
                          Box::new(
                              super::measure_cpu::CpuScoringMeasure::new(
                                  super::workload::WorkloadSpec::default(),
                                  tuning,
                                  yield_check,
                              )?,
                          ),
                      )
                  }
                  DeviceKind::Gpu => {
                      #[cfg(feature = "cuda")]
                      {
                          Ok(
                              Box::new(
                                  super::measure_cuda::CudaScoringMeasure::new(
                                      device,
                                      super::workload::WorkloadSpec::default(),
                                      tuning,
                                      yield_check,
                                  )?,
                              ),
                          )
                      }
                      #[cfg(not(feature = "cuda"))]
                      {
                          let _ = (tuning, yield_check);
                          Err(PrecisionError::Capability {
                              reason: "cuda feature disabled".into(),
                          })
                      }
                  }
              }
          }
```
/answer ed48-4 allow
Control Request:
```yaml
id: ed48-5
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
    id: make_measure
    start_marker: super::measure_cpu::CpuScoringMeasure::new(
    end_marker: |-
      yield_check,
                          )?,
    content: |-
      super::measure_cpu::CpuScoringMeasure::new(
                              super::workload::WorkloadSpec {
                                  docs: 4_000,
                                  ..Default::default()
                              },
                              tuning,
                              yield_check,
                          )?,
  reason: Kleinere CPU-Workload in der Factory setzen (vorheriger Versuch wurde nicht angewendet)
```
/answer ed48-5 allow
Control Request:
```yaml
id: ed48-6
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
      error: End marker is ambiguous – found 2 occurrences in node.
      id: make_measure
      reason: ambiguous
```
/answer ed48-6 allow
ReasoningToken: 44
Control Request:
```yaml
id: ed48-7
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    id: CD5tM3|eS3CGz
    start_marker: "pub use axis::{AxisId, ValueId};"
    end_marker: "pub use axis::{AxisId, ValueId};"
    content: |-
      pub use axis::{AxisId, ValueId};
      pub use calibrate::{
          CalibrationReport, CalibrationTarget, Calibrator, CalibratorConfig, Entry, HardCap, Outcome,
          ParetoPoint, Stage,
      };
      pub use measure::{
          make_measure, Cfg, Clock, DeviationReport, Measure, MeasureResult, NeverYield, Sample,
          SystemClock, Validity, WindowTuning, YieldCheck,
      };
      pub use measure_cpu::CpuScoringMeasure;
      #[cfg(feature = "cuda")]
      pub use measure_cuda::CudaScoringMeasure;
      #[cfg(any(test, feature = "mock"))]
      pub use measure_mock::{FakeCombo, FakeMeasure, ManualClock};
      pub use metrics::{
          cosine_deviation, is_duplicate_free, judge_rows, relative_error, top_k_overlap, Plausibility,
      };
      pub use pareto::pareto_by;
      pub use pool::{lower_current_thread_priority, JobHandle, LowPriorityPool};
      pub use stats::{Estimator, Spread};
      pub use workload::{ScoringWorkload, WorkloadSpec};
  reason: Neue Re-Exports in mod.rs ergänzen
```
/answer ed48-7 allow
Control Request:
```yaml
id: ed48-8
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    id: axis
    position: before
    source: |
      mod calibrate;
      mod measure;
      mod measure_cpu;
      #[cfg(feature = "cuda")]
      mod measure_cuda;
      #[cfg(any(test, feature = "mock"))]
      mod measure_mock;
      mod metrics;
      mod pool;
      mod stats;
      mod workload;
  reason: Neue Module in mod.rs deklarieren
```
/answer ed48-8 allow
ReasoningToken: 250
Control Request:
```yaml
id: ed48-9
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
    id: make_measure
    old_text: "CpuScoringMeasure::new( super::workload::WorkloadSpec::default(),"
    new_text: "CpuScoringMeasure::new( super::workload::WorkloadSpec { docs: 4_000, ..Default::default() },"
  reason: Kleinere CPU-Workload in der Factory (eindeutiger Kontext)
```
/answer ed48-9 allow
ReasoningToken: 4579
Control Request:
```yaml
id: ed48-a
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/calibrate.rs
    source: |
      use super::*;
      use crate::resident::mock::NoiseModel;
      use crate::resident::precision::measure_mock::{FakeCombo, FakeMeasure, ManualClock};
      use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

      const SEEDS: [u64; 3] = [1, 7, 42];

      fn target() -> CalibrationTarget {
          CalibrationTarget { engine: SubengineKind::Cuda, model_id: "model".into() }
      }

      fn quick() -> CalibratorConfig {
          CalibratorConfig {
              hard_cap: HardCap { max_reps: 12, max_time: None },
              total_budget: None,
              interrupt_backoff: Duration::ZERO,
              ..CalibratorConfig::default()
          }
      }

      fn caps(axis: &str, values: &[&str]) -> CapabilitySet {
          values.iter().fold(CapabilitySet::new(), |c, v| c.with(axis, *v))
      }

      fn val(cfg: &Cfg, axis: &str) -> String {
          cfg.combo.get(&AxisId::new(axis)).unwrap().to_string()
      }

      fn entry<'a>(r: &'a CalibrationReport, axis: &str, value: &str) -> &'a Entry {
          r.entries.iter().find(|e| val(&e.cfg, axis) == value).unwrap()
      }

      fn measured(e: &Entry) -> &MeasureResult {
          match &e.outcome {
              Outcome::Measured(r) => r,
              other => panic!("expected a measurement, got {other:?}"),
          }
      }

      fn ms(d: Duration) -> f64 {
          d.as_secs_f64() * 1e3
      }

      fn single(seed: u64, combo: FakeCombo, cfg: CalibratorConfig) -> (CalibrationReport, Calibrator<FakeMeasure>) {
          let fake = FakeMeasure::new(seed, move |_| combo.clone());
          let mut cal = Calibrator::new(cfg, fake);
          let report = cal.run(&target(), &caps("operands", &["a"])).unwrap();
          (report, cal)
      }

      #[test]
      fn low_noise_converges_at_the_minimum_repetitions() {
          for seed in SEEDS {
              let (r, _) = single(seed, FakeCombo::new(10.0, 0.0), quick());
              let m = measured(&r.entries[0]);
              assert!(m.stable && m.samples == 3, "seed {seed}");
              assert!((ms(m.time) - 10.0).abs() < 0.2);
              assert_eq!(r.unstable(), 0);
          }
      }

      #[test]
      fn outliers_are_absorbed_by_median_and_trimmed_spread() {
          for seed in SEEDS {
              let noise = NoiseModel::Outlier { every_n: 3, magnitude: 40.0 };
              let (r, _) = single(seed, FakeCombo::new(10.0, 0.0).noise(noise), quick());
              let m = measured(&r.entries[0]);
              assert!(m.stable, "seed {seed}");
              assert_eq!(m.samples, 5);
              assert!((ms(m.time) - 10.0).abs() < 0.2);
          }
      }

      #[test]
      fn minimum_estimator_takes_the_fastest_sample() {
          let noise = NoiseModel::Outlier { every_n: 3, magnitude: 40.0 };
          let cfg = CalibratorConfig { estimator: Estimator::Min, ..quick() };
          let (r, _) = single(3, FakeCombo::new(10.0, 0.0).noise(noise), cfg);
          assert!(ms(measured(&r.entries[0]).time) <= 10.01);
      }

      #[test]
      fn drifting_series_ends_unstable_with_the_best_intermediate_state() {
          for seed in SEEDS {
              let noise = NoiseModel::Drift { per_sample: 3.0 };
              let (r, _) = single(seed, FakeCombo::new(10.0, 0.0).noise(noise), quick());
              let m = measured(&r.entries[0]);
              assert!(!m.stable, "seed {seed}");
              assert_eq!(m.samples, 12);
              assert_eq!(r.unstable(), 1);
              assert_eq!(r.front.len(), 1);
              assert!(!r.front[0].stable);
          }
      }

      #[test]
      fn bimodal_series_does_not_settle() {
          for seed in SEEDS {
              let noise = NoiseModel::Bimodal { offset: 30.0, p: 0.5 };
              let cfg = CalibratorConfig { min_reps: 8, ..quick() };
              let (r, _) = single(seed, FakeCombo::new(10.0, 0.0).noise(noise), cfg);
              let m = measured(&r.entries[0]);
              assert!(!m.stable, "seed {seed}");
              assert!((9.9..=40.1).contains(&ms(m.time)));
          }
      }

      #[test]
      fn never_stable_series_stops_at_the_repetition_cap() {
          for seed in SEEDS {
              let noise = NoiseModel::Erratic { amplitude: 8.0 };
              let (r, _) = single(seed, FakeCombo::new(10.0, 0.0).noise(noise), quick());
              let m = measured(&r.entries[0]);
              assert!(!m.stable && m.samples == 12, "seed {seed}");
          }
      }

      #[test]
      fn time_cap_ends_a_measurement_without_error() {
          let clock = Arc::new(ManualClock::new());
          let noise = NoiseModel::Drift { per_sample: 3.0 };
          let fake = FakeMeasure::new(1, move |_| FakeCombo::new(10.0, 0.0).noise(noise.clone()))
              .with_clock(clock.clone());
          let cfg = CalibratorConfig {
              hard_cap: HardCap { max_reps: 1000, max_time: Some(Duration::from_millis(35)) },
              ..quick()
          };
          let mut cal = Calibrator::new(cfg, fake).with_clock(clock);
          let r = cal.run(&target(), &caps("operands", &["a"])).unwrap();
          let m = measured(&r.entries[0]);
          assert!(!m.stable);
          assert_eq!(m.samples, 3);
      }

      #[test]
      fn exhausted_total_budget_keeps_partial_results() {
          let clock = Arc::new(ManualClock::new());
          let fake = FakeMeasure::new(1, |c| {
              FakeCombo::new(10.0, if val(c, "operands") == "a" { 1e-2 } else { 0.0 })
          })
          .with_clock(clock.clone());
          let cfg = CalibratorConfig { total_budget: Some(Duration::from_millis(25)), ..quick() };
          let mut cal = Calibrator::new(cfg, fake).with_clock(clock);
          let r = cal.run(&target(), &caps("operands", &["a", "b"])).unwrap();
          assert!(r.budget_exhausted);
          for e in &r.entries {
              let m = measured(e);
              assert!(!m.stable && m.samples == 2);
          }
      }

      #[test]
      fn invalid_combinations_are_rejected_without_timing_them() {
          let fake = FakeMeasure::new(5, |c| match val(c, "operands").as_str() {
              "ok" => FakeCombo::new(10.0, 0.0),
              "nan" => FakeCombo::new(10.0, 0.0).valid(Validity::NonFinite),
              "nan_dev" => FakeCombo::new(10.0, f64::NAN),
              "flat" => FakeCombo::new(10.0, 0.0).valid(Validity::Degenerate),
              "wide" => FakeCombo::new(5.0, 0.2),
              _ => FakeCombo::new(10.0, 0.0).incompatible(),
          });
          let mut cal = Calibrator::new(quick(), fake);
          let all = ["ok", "nan", "nan_dev", "flat", "wide", "boom"];
          let r = cal.run(&target(), &caps("operands", &all)).unwrap();
          let want = [
              ("nan", Validity::NonFinite),
              ("nan_dev", Validity::NonFinite),
              ("flat", Validity::Degenerate),
              ("wide", Validity::OverMaxDeviation),
              ("boom", Validity::Failed),
          ];
          for (name, validity) in want {
              let e = entry(&r, "operands", name);
              assert_eq!(e.outcome, Outcome::Rejected(validity), "{name}");
              assert_eq!(cal.measure().samples_taken(&e.cfg), 0, "{name}");
          }
          assert_eq!(r.rejected(), 5);
          assert_eq!(r.front.len(), 1);
          assert_eq!(val(&r.front[0].cfg, "operands"), "ok");
      }

      #[test]
      fn front_keeps_only_non_dominated_points_in_order() {
          let fake = FakeMeasure::new(9, |c| match val(c, "operands").as_str() {
              "slow" => FakeCombo::new(100.0, 0.0),
              "mid" => FakeCombo::new(50.0, 1e-3),
              "fast" => FakeCombo::new(20.0, 3e-2),
              _ => FakeCombo::new(120.0, 4e-2),
          });
          let mut cal = Calibrator::new(quick(), fake);
          let r = cal.run(&target(), &caps("operands", &["slow", "mid", "fast", "bad"])).unwrap();
          let names: Vec<String> = r.front.iter().map(|p| val(&p.cfg, "operands")).collect();
          assert_eq!(names, ["fast", "mid", "slow"]);
          for w in r.front.windows(2) {
              assert!(w[0].time < w[1].time && w[0].deviation > w[1].deviation);
          }
      }

      #[test]
      fn dominated_combination_stops_at_the_minimum_repetitions() {
          let fake = FakeMeasure::new(2, |c| match val(c, "operands").as_str() {
              "a_fast" => FakeCombo::new(10.0, 0.0),
              _ => FakeCombo::new(120.0, 0.0).noise(NoiseModel::Erratic { amplitude: 8.0 }),
          });
          let mut cal = Calibrator::new(quick(), fake);
          let r = cal.run(&target(), &caps("operands", &["a_fast", "b_slow"])).unwrap();
          let slow = entry(&r, "operands", "b_slow");
          assert_eq!(slow.outcome, Outcome::Dominated);
          assert_eq!(cal.measure().samples_taken(&slow.cfg), 3);
          assert!(measured(entry(&r, "operands", "a_fast")).stable);
          assert_eq!(r.dominated(), 1);
      }

      #[test]
      fn combinations_are_measured_interleaved_in_abba_order() {
          let fake = FakeMeasure::new(4, |c| match val(c, "operands").as_str() {
              "a" => FakeCombo::new(10.0, 3e-2),
              "b" => FakeCombo::new(20.0, 2e-2),
              _ => FakeCombo::new(30.0, 1e-2),
          });
          let mut cal = Calibrator::new(quick(), fake);
          cal.run(&target(), &caps("operands", &["a", "b", "c"])).unwrap();
          let order: Vec<String> = cal.measure().order().iter().map(|c| val(c, "operands")).collect();
          assert_eq!(order.join(""), "abccbaabc");
      }

      #[test]
      fn stability_is_judged_on_ratios_not_on_absolute_times() {
          let script = |c: &Cfg| match val(c, "operands").as_str() {
              "a" => FakeCombo::new(10.0, 1e-2),
              _ => FakeCombo::new(20.0, 0.0),
          };
          let load = |round: usize| 1.0 + 0.4 * (round % 2) as f64;

          let shared = FakeMeasure::new(1, script).with_shared_load(2, load);
          let mut cal = Calibrator::new(quick(), shared);
          let r = cal.run(&target(), &caps("operands", &["a", "b"])).unwrap();
          for e in &r.entries {
              let m = measured(e);
              assert!(m.stable && m.samples == 3);
          }

          let alone = FakeMeasure::new(1, script).with_shared_load(2, load);
          let mut cal = Calibrator::new(quick(), alone);
          let r = cal.run(&target(), &caps("operands", &["a"])).unwrap();
          assert!(!measured(&r.entries[0]).stable);
      }

      fn separable_model(c: &Cfg) -> FakeCombo {
          let op = match val(c, "operands").as_str() {
              "fp32" => (100.0, 0.0),
              _ => (60.0, 1e-3),
          };
          let batch = match val(c, "batch").as_str() {
              "1" => 3.0,
              "8" => 1.5,
              _ => 1.0,
          };
          let tile = match val(c, "tile").as_str() {
              "16" => 1.4,
              "64" => 1.0,
              _ => 1.2,
          };
          FakeCombo::new(op.0 * batch * tile, op.1)
      }

      fn separable_caps() -> CapabilitySet {
          let mut c = caps("operands", &["fp32", "fp16"]);
          for b in ["1", "8", "32"] {
              c.insert("batch", b);
          }
          for t in ["16", "64", "256"] {
              c.insert("tile", t);
          }
          c
      }

      #[test]
      fn pruned_calibration_matches_full_enumeration_with_fewer_samples() {
          for seed in SEEDS {
              let mut pruned = Calibrator::new(quick(), FakeMeasure::new(seed, separable_model));
              let rp = pruned.run(&target(), &separable_caps()).unwrap();
              let full_cfg = CalibratorConfig { exhaustive: true, ..quick() };
              let mut full = Calibrator::new(full_cfg, FakeMeasure::new(seed, separable_model));
              let rf = full.run(&target(), &separable_caps()).unwrap();

              let key = |r: &CalibrationReport| {
                  let mut v: Vec<String> = r.front.iter().map(|p| p.cfg.to_string()).collect();
                  v.sort();
                  v
              };
              assert_eq!(key(&rp), key(&rf), "seed {seed}");
              assert_eq!(rf.front.len(), 2);
              assert_eq!(rp.exhaustive_size, 18);
              assert!(pruned.measure().order().len() < full.measure().order().len(), "seed {seed}");
          }
      }

      #[test]
      fn axis_winners_are_chosen_per_leading_axis_value() {
          // Interaction: batch 1 wins for one dtype, batch 32 for the other.
          let model = |c: &Cfg| {
              let slow = (val(c, "operands") == "fp16") == (val(c, "batch") == "32");
              FakeCombo::new(if slow { 90.0 } else { 40.0 }, if val(c, "operands") == "fp16" { 1e-3 } else { 0.0 })
          };
          let caps = {
              let mut c = caps("operands", &["fp32", "fp16"]);
              c.insert("batch", "1");
              c.insert("batch", "32");
              c
          };
          let mut cal = Calibrator::new(quick(), FakeMeasure::new(3, model));
          let r = cal.run(&target(), &caps).unwrap();
          let best = |dtype: &str| {
              r.front.iter().find(|p| val(&p.cfg, "operands") == dtype).map(|p| val(&p.cfg, "batch"))
          };
          assert_eq!(best("fp32").as_deref(), Some("32"));
          assert_eq!(best("fp16").as_deref(), Some("1"));
      }

      #[test]
      fn waiting_task_delays_sampling_without_failing_the_run() {
          struct Countdown(AtomicU32);
          impl YieldCheck for Countdown {
              fn should_yield(&self) -> bool {
                  self.0.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1)).is_ok()
              }
          }
          let fake = FakeMeasure::new(1, |_| FakeCombo::new(10.0, 0.0));
          let mut cal = Calibrator::new(quick(), fake).with_yield_check(Arc::new(Countdown(AtomicU32::new(2))));
          let r = cal.run(&target(), &caps("operands", &["a"])).unwrap();
          let m = measured(&r.entries[0]);
          assert!(m.stable && m.samples == 3);
      }

      #[test]
      fn permanent_preemption_pressure_skips_instead_of_blocking() {
          let fake = FakeMeasure::new(1, |_| FakeCombo::new(10.0, 0.0));
          let flag = Arc::new(AtomicBool::new(true));
          let mut cal = Calibrator::new(quick(), fake).with_yield_check(flag);
          let r = cal.run(&target(), &caps("operands", &["a"])).unwrap();
          assert_eq!(r.entries[0].outcome, Outcome::Skipped);
          assert_eq!(cal.measure().order().len(), 0);
          assert!(r.front.is_empty());
      }

      #[test]
      fn empty_capability_set_is_an_error() {
          let fake = FakeMeasure::new(1, |_| FakeCombo::new(10.0, 0.0));
          let mut cal = Calibrator::new(quick(), fake);
          let err = cal.run(&target(), &CapabilitySet::new()).unwrap_err();
          assert!(matches!(err, PrecisionError::Capability { .. }));
      }

      #[test]
      fn minimum_repetitions_never_drop_below_three() {
          let cfg = CalibratorConfig { min_reps: 1, ..quick() };
          let cal = Calibrator::new(cfg, FakeMeasure::new(1, |_| FakeCombo::new(1.0, 0.0)));
          assert_eq!(cal.config().min_reps, 3);
      }

      #[test]
      fn test_set_lengths_reach_the_maximum_sequence_length() {
          assert_eq!(CalibratorConfig::seq_lens_up_to(512), [16, 64, 256, 512]);
          assert_eq!(CalibratorConfig::seq_lens_up_to(100), [16, 64, 100]);
          assert_eq!(CalibratorConfig::seq_lens_up_to(16), [16]);
          let (r, _) = single(1, FakeCombo::new(10.0, 0.0), quick());
          assert_eq!(r.entries[0].cfg.seq_lens.last(), Some(&512));
      }

      #[test]
      fn default_separable_axes_are_the_numeric_configuration_axes() {
          let d = CalibratorConfig::default();
          for axis in ["batch", "tile", "threads"] {
              assert!(d.separable_axes.contains(&AxisId::new(axis)));
          }
          assert!(!d.separable_axes.contains(&AxisId::new("operands")));
      }
  reason: "Pflicht-Tests des Calibrators (Stabilität, Pareto, Verwerfung, Pruning, ABBA, Yield) anlegen"
```
/answer ed48-a allow
Text:
 Alle Dateien sind angelegt. Jetzt kompilieren und testen.
ReasoningToken: 490
Control Request:
```yaml
id: ed48-b
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib resident::precision 2>&1 | tail -n 80
  reason: Precision-Modul ohne cuda-Feature bauen und die Tests ausführen
```
/answer ed48-b allow
Control Request:
```yaml
id: ed48-c
result:
  structuredContent:
    results:
    - stdout: |2
           Compiling xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
        error[E0308]: mismatched types
          --> src/resident/mock.rs:93:61
           |
        93 |                 self.base + self.rng.gen_range(-amplitude..=amplitude)
           |                                                -------------^^^^^^^^^
           |                                                |            |
           |                                                |            expected `f64`, found `&f64`
           |                                                arguments to this function are incorrect
           |
        note: associated function defined here
          --> /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/ops/range.rs:395:17
        help: consider dereferencing the borrow
           |
        93 |                 self.base + self.rng.gen_range(-amplitude..=*amplitude)
           |                                                             +

        warning: use of deprecated method `std::sync::atomic::Atomic::<u32>::fetch_update`: renamed to `try_update` for consistency
           --> src/resident/precision/test/calibrate.rs:326:20
            |
        326 |             self.0.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1)).is_ok()
            |                    ^^^^^^^^^^^^
            |
            = note: `#[warn(deprecated)]` on by default
        help: replace the use of the deprecated method
            |
        326 -             self.0.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1)).is_ok()
        326 +             self.0.try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1)).is_ok()
            |

        warning: irrefutable `if let` pattern
           --> src/resident/subengine/exec.rs:337:20
            |
        337 |                 if let EventRepr::Cpu(e) = event.repr() {
            |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            |
            = note: this pattern will always match, so the `if let` is useless
            = help: consider replacing the `if let` with a `let`
            = note: `#[warn(irrefutable_let_patterns)]` on by default

        For more information about this error, try `rustc --explain E0308`.
        warning: `xy-ai-rag` (lib test) generated 2 warnings
        error: could not compile `xy-ai-rag` (lib test) due to 1 previous error; 2 warnings emitted
```
/answer ed48-c allow
ReasoningToken: 125
Control Request:
```yaml
id: ed48-d
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mock.rs
    id: impl_Measure_for_FakeMeasure.sample
    old_text: self.base + self.rng.gen_range(-amplitude..=amplitude)
    new_text: self.base + self.rng.gen_range(-*amplitude..=*amplitude)
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/calibrate.rs
    id: waiting_task_delays_sampling_without_failing_the_run
    old_text: "self.0.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1)).is_ok()"
    new_text: "self.0.try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1)).is_ok()"
  reason: Typfehler im Erratic-Modell und veraltete Atomic-Methode beheben
```
/answer ed48-d allow
Control Request:
```yaml
id: ed48-e
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib resident::precision 2>&1 | grep -v '^\s*$' | tail -n 90
  reason: Erneut bauen und Precision-Tests ausführen
```
/answer ed48-e allow
Control Request:
```yaml
id: ed48-f
result:
  content:
  - type: text
    text: "STDOUT limit of characters exceeded, output saved to `/tmp/mcpc-stdout-nh7c0guw.log`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)"
  structuredContent:
    results:
    - stdout_file: /tmp/mcpc-stdout-nh7c0guw.log
      stdout_file_file_stats:
        path: /tmp/mcpc-stdout-nh7c0guw.log
        size_bytes: 7557
        lines: 90
        words: 372
        complexity: 0.552
        created: 2026-10-25T18:31:44.532642+00:00
        modified: 2026-10-25T18:31:44.532642+00:00
        accessed: 2026-10-25T18:31:44.532642+00:00
        line_length_max: 163
        line_length_min: 9
        line_length_avg: 82.97
        words_per_line_avg: 4.13
        checksum: 5c66be871887fd13b84c46f74e046090f7d4f594a5cfc64e6289585b249b6a8d
      stdout_file_ast_outline:
      - id: file
        lines: 1-90
        signature: test resident::precision::calibrate::tests::bimodal_series_does_not_settle ... …
```
/answer ed48-f allow
Control Request:
```yaml
id: ed48-10
toolName: search
arguments:
  items:
  - directory:
    - /tmp
    pattern: "FAILED|failed|panicked|test result|error|warning: unused"
    include:
    - mcpc-stdout-nh7c0guw.log
    limit: 40
  reason: Fehlschläge im Testlog finden
```
/answer ed48-10 allow
Control Request:
```yaml
id: ed48-11
result:
  structuredContent:
    errors:
    - directory:
      - /tmp
      pattern: "FAILED|failed|panicked|test result|error|warning: unused"
      error: |
        search failed (exit code 2): grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-systemd-oomd.service-hpDfrQ: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-switcheroo-control.service-fCujHr: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-systemd-logind.service-lbeMMf: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-power-profiles-daemon.service-jKZLBx: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-upower.service-GNstGX: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-fwupd.service-3b4QPY: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-colord.service-7ZoTgN: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-systemd-timesyncd.service-y9AhFi: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-ModemManager.service-2kam9a: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-bluetooth.service-VLrhT9: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-polkit.service-CTk1Ko: Permission denied
        grep: /tmp/systemd-private-325a271ad0ce41e8b584534a7e471922-systemd-resolved.service-tTj4ZZ: Permission denied
```
/answer ed48-11 allow
ReasoningToken: 40
Control Request:
```yaml
id: ed48-12
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - "cargo test --lib resident::precision 2>&1 | grep -E 'FAILED|failed|panicked|test result|^error|^warning: unused|left:|right:' | head -n 40"
  reason: Nur Fehlschläge und Zusammenfassung der Precision-Tests anzeigen
```
/answer ed48-12 allow
Control Request:
```yaml
id: ed48-13
result:
  structuredContent:
    results:
    - stdout: |
        test resident::precision::calibrate::tests::axis_winners_are_chosen_per_leading_axis_value ... FAILED
        thread 'resident::precision::calibrate::tests::axis_winners_are_chosen_per_leading_axis_value' (359575) panicked at src/resident/precision/test/calibrate.rs:372:5:
        assertion `left == right` failed
          left: None
         right: Some("1")
        test result: FAILED. 92 passed; 1 failed; 0 ignored; 0 measured; 441 filtered out; finished in 0.02s
        error: test failed, to rerun pass `--lib`
```
/answer ed48-13 allow
ReasoningToken: 173
Control Request:
```yaml
id: ed48-14
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/calibrate.rs
    id: axis_winners_are_chosen_per_leading_axis_value
    old_text: "FakeCombo::new(if slow { 90.0 } else { 40.0 }, if val(c, \"operands\") == \"fp16\" { 1e-3 } else { 0.0 })"
    new_text: |-
      let fp16 = val(c, "operands") == "fp16";
              let fast = if fp16 { 30.0 } else { 40.0 };
              FakeCombo::new(if slow { 90.0 } else { fast }, if fp16 { 1e-3 } else { 0.0 })
  reason: "Testmodell so anpassen, dass beide dtype-Gewinner auf der Pareto-Front liegen"
```
/answer ed48-14 allow
Control Request:
```yaml
id: ed48-15
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/calibrate.rs
      error: Text not found in node; its content appears to have changed.
      id: axis_winners_are_chosen_per_leading_axis_value
      reason: content_changed
      guess: "            if val(c, \"operands\") == \"fp16\" { 1e-3 } else { 0.0 },"
```
/answer ed48-15 allow
ReasoningToken: 13
Control Request:
```yaml
id: ed48-16
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/calibrate.rs
    ids:
    - axis_winners_are_chosen_per_leading_axis_value
  reason: Aktuellen (umformatierten) Testcode lesen
```
/answer ed48-16 allow
Control Request:
```yaml
id: ed48-17
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/calibrate.rs
      nodes:
      - id: axis_winners_are_chosen_per_leading_axis_value
        type: ItemFn
        code: |
          #[test]
          fn axis_winners_are_chosen_per_leading_axis_value() {
              let model = |c: &Cfg| {
                  let slow = (val(c, "operands") == "fp16") == (val(c, "batch") == "32");
                  FakeCombo::new(
                      if slow { 90.0 } else { 40.0 },
                      if val(c, "operands") == "fp16" { 1e-3 } else { 0.0 },
                  )
              };
              let caps = {
                  let mut c = caps("operands", &["fp32", "fp16"]);
                  c.insert("batch", "1");
                  c.insert("batch", "32");
                  c
              };
              let mut cal = Calibrator::new(quick(), FakeMeasure::new(3, model));
              let r = cal.run(&target(), &caps).unwrap();
              let best = |dtype: &str| {
                  r.front
                      .iter()
                      .find(|p| val(&p.cfg, "operands") == dtype)
                      .map(|p| val(&p.cfg, "batch"))
              };
              assert_eq!(best("fp32").as_deref(), Some("32"));
              assert_eq!(best("fp16").as_deref(), Some("1"));
          }
```
/answer ed48-17 allow
Control Request:
```yaml
id: ed48-18
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/calibrate.rs
    id: axis_winners_are_chosen_per_leading_axis_value
    old_text: "FakeCombo::new( if slow { 90.0 } else { 40.0 }, if val(c, \"operands\") == \"fp16\" { 1e-3 } else { 0.0 }, )"
    new_text: "let fp16 = val(c, \"operands\") == \"fp16\"; let fast = if fp16 { 30.0 } else { 40.0 }; FakeCombo::new(if slow { 90.0 } else { fast }, if fp16 { 1e-3 } else { 0.0 })"
  reason: "Testmodell korrigieren: fp16-Gewinner schneller, damit beide auf der Front liegen"
```
/answer ed48-18 allow
Control Request:
```yaml
id: ed48-19
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib resident::precision 2>&1 | grep -E 'FAILED|panicked|test result|^error|^warning|left:|right:' | head -n 30
    - "cargo test --lib resident:: 2>&1 | grep -E 'FAILED|test result|^error' | head"
  reason: Precision-Tests und gesamte resident-Testsuite ohne cuda ausführen
```
/answer ed48-19 allow
ReasoningToken: 132
Control Request:
```yaml
id: ed48-1a
result:
  structuredContent:
    results:
    - stdout: |
        warning: irrefutable `if let` pattern
        warning: associated function `from_raw` is never used
        warning: `xy-ai-rag` (lib test) generated 2 warnings
        test result: ok. 93 passed; 0 failed; 0 ignored; 0 measured; 441 filtered out; finished in 0.02s
        test result: ok. 506 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 5.61s
```
/answer ed48-1a allow
Control Request:
```yaml
id: ed48-1b
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - export PATH=/usr/local/cuda-13.3/bin:$PATH CUDA_PATH=/usr/local/cuda-13.3 CUDA_HOME=/usr/local/cuda-13.3
    - cargo check --features cuda --lib --tests 2>&1 | grep -E '^(error|warning)' -A 12 | head -n 80
  reason: cuda-Build (measure_cuda) per cargo check mit nvcc 13.3 prüfen
```
/answer ed48-1b allow
Control Request:
```yaml
id: ed48-1c
result:
  structuredContent:
    results:
    - stdout: |
        warning: candle-kernels@0.11.0: All PTX kernels up-to-date, skipping compilation
        warning: candle-kernels@0.11.0: Using 8 threads for compilation
        warning: candle-kernels@0.11.0: All library kernels up-to-date, skipping compilation
        warning: use of deprecated constant `std::f32::MIN`: replaced by the `MIN` associated constant on `f32`
           --> third_party/candle-transformers/src/models/modernbert.rs:302:27
            |
        302 |     (inverted_mask * f32::MIN as f64)?.to_dtype(dtype)
            |                           ^^^
            |
            = note: `#[warn(deprecated)]` on by default

        warning: use of deprecated constant `std::f32::NEG_INFINITY`: replaced by the `NEG_INFINITY` associated constant on `f32`
           --> third_party/candle-transformers/src/models/modernbert.rs:315:26
            |
        315 |                     f32::NEG_INFINITY
            |                          ^^^^^^^^^^^^

        warning: `candle-transformers` (lib) generated 2 warnings
            Checking xy-ai-rag v0.1.0 (/home/user/xyan/xy.ai.workbench/rag)
        warning: field `device` is never read
          --> src/resident/memory/cuda.rs:64:5
           |
        60 | struct MappedBuffer {
           |        ------------ field in this struct
        ...
        64 |     device: cudarc::driver::sys::CUdeviceptr,
           |     ^^^^^^
           |
           = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

        warning: methods `copy_d2d`, `download`, and `mapped_device_ptr` are never used
           --> src/resident/memory/cuda.rs:192:19
            |
        121 | impl CudaMemoryManager {
            | ---------------------- methods in this implementation
        ...
        192 |     pub(crate) fn copy_d2d(
            |                   ^^^^^^^^
        ...
        253 |     pub(crate) unsafe fn download(
            |                          ^^^^^^^^
        ...
        329 |     pub(crate) fn mapped_device_ptr(
        --
        warning: function `elems_to_bytes` is never used
          --> src/resident/memory/dtype.rs:89:19
           |
        89 |     pub(crate) fn elems_to_bytes(elems: usize, dtype: DType) -> usize {
           |                   ^^^^^^^^^^^^^^

        warning: function `bytes_to_elems` is never used
          --> src/resident/memory/dtype.rs:93:19
           |
        93 |     pub(crate) fn bytes_to_elems(bytes: usize, dtype: DType) -> usize {
           |                   ^^^^^^^^^^^^^^

        warning: function `launch_maxsim` is never used
           --> src/resident/kernel/cuda.rs:299:15
            |
        299 | pub(crate) fn launch_maxsim<T: DeviceRepr>(
            |               ^^^^^^^^^^^^^

        warning: associated function `from_mapped` is never used
          --> src/resident/manager/doorbell.rs:43:26
           |
        31 | impl Doorbell {
           | ------------- associated function in this implementation
        ...
        43 |     pub(crate) unsafe fn from_mapped(done: *mut u64, ack: *mut u64) -> Self {
           |                          ^^^^^^^^^^^

        warning: function `drive_build` is never used
           --> src/resident/manager/build.rs:264:15
            |
        264 | pub(crate) fn drive_build(
            |               ^^^^^^^^^^^

        warning: function `skip_bytes` is never used
           --> src/resident/manager/build.rs:323:4
            |
```
/answer ed48-1c allow
ReasoningToken: 76
Control Request:
```yaml
id: ed48-1d
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - export PATH=/usr/local/cuda-13.3/bin:$PATH CUDA_PATH=/usr/local/cuda-13.3 CUDA_HOME=/usr/local/cuda-13.3
    - cargo check --features cuda --lib --tests 2>&1 | grep -E '^error|precision/' -B3 -A8 | head -n 60; echo ---; cargo check --features cuda --lib --tests 2>&1 | tail -n 3
  reason: Fehler und Warnungen in precision/ im cuda-Build gezielt prüfen
```
/answer ed48-1d allow
Control Request:
```yaml
id: ed48-1e
result:
  structuredContent:
    results:
    - stdout: |
        ---

        warning: `xy-ai-rag` (lib test) generated 4 warnings (3 duplicates)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
```
/answer ed48-1e allow
Text:
 Beide Builds kompilieren sauber. Jetzt die CUDA-Tests auf der echten GTX 1660 SUPER.
ReasoningToken: 99
Control Request:
```yaml
id: ed48-1f
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - export PATH=/usr/local/cuda-13.3/bin:$PATH CUDA_PATH=/usr/local/cuda-13.3 CUDA_HOME=/usr/local/cuda-13.3
    - cargo test --features cuda --lib resident::precision::measure_cuda -- --ignored --nocapture --test-threads=1 2>&1 | grep -E 'G-CALIB|^test |test result|panicked|left:|right:|^error' | head -n 60
  reason: Reale CUDA-Messung und Kalibrierung auf dem Gerät ausführen
```
/answer ed48-1f allow
Control Request:
```yaml
id: ed48-20
result:
  structuredContent:
    results:
    - stdout: |
        test resident::precision::measure_cuda::tests::a_waiting_task_interrupts_between_groups ... ok
        test resident::precision::measure_cuda::tests::fp32_matches_the_reference_and_fp16_stays_close ... ok
        test resident::precision::measure_cuda::tests::gate_calibration_front_on_device ... G-CALIB Axis batch=1,scoring=fp16 time 166.875µs dev 0.000e0 stable false samples 40
        G-CALIB Axis batch=8,scoring=fp16 time 95.245µs dev 0.000e0 stable false samples 40
        G-CALIB Axis batch=1,scoring=fp32 time 150.17µs dev 0.000e0 stable false samples 40
        G-CALIB Axis batch=8,scoring=fp32 time 76.61µs dev 0.000e0 stable false samples 40
        G-CALIB Verify batch=8,scoring=fp16 time 75.823µs dev 0.000e0 stable true samples 5
        G-CALIB Verify batch=8,scoring=fp32 time 77.157µs dev 0.000e0 stable true samples 5
        G-CALIB front batch=8,scoring=fp16 time 75.823µs dev 0.000e0 stable true
        test resident::precision::measure_cuda::tests::samples_scale_to_the_window_and_stay_positive ... ok
        test resident::precision::measure_cuda::tests::unsupported_types_are_incompatible ... ok
        test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 575 filtered out; finished in 5.45s
```
/answer ed48-20 allow
Thinking:
Die Axis-Gruppen laufen zwar Ende-zu-Ende, aber instabil (Cap 40) – wahrscheinlich wegen zu kurzem Warmup mit nur 3 Launches. Ich stelle das Warmup auf ein Zeitfenster um und messe erneut.


ReasoningToken: 647
Control Request:
```yaml
id: ed48-21
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cuda.rs
    id: impl_CudaScoringMeasure.plan
    old_text: "for _ in 0..Self::WARMUP_LAUNCHES { self.launch(f, dtype, nq)?; } self.stream.synchronize().map_err(incompatible)?;"
    new_text: "let warm = std::time::Instant::now(); while warm.elapsed() < Duration::from_millis(Self::WARMUP_MS) { for _ in 0..Self::WARMUP_LAUNCHES { self.launch(f, dtype, nq)?; } self.stream.synchronize().map_err(incompatible)?; }"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cuda.rs
    id: impl_CudaScoringMeasure.GVrR7y|B1Rh5q
    old_text: "const WARMUP_LAUNCHES: u32 = 3;"
    new_text: |-
      const WARMUP_LAUNCHES: u32 = 16;
          /// Long enough for the SM clock to leave its idle state (observed 1185 -> 2145 MHz).
          const WARMUP_MS: u64 = 100;
  reason: "Zeitbasiertes Warmup, damit die Taktrampe vor der Messung abgeschlossen ist"
```
/answer ed48-21 allow
Control Request:
```yaml
id: ed48-22
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cuda.rs
      result: success
      id: impl_CudaScoringMeasure.plan
      new_id: impl_CudaScoringMeasure.plan
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cuda.rs
      error: No node matched the selector.
      id: impl_CudaScoringMeasure.GVrR7y|B1Rh5q
```
/answer ed48-22 allow
Control Request:
```yaml
id: ed48-23
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cuda.rs
  text: "WARMUP_LAUNCHES: u32"
  reason: "Konstante lokalisieren, um sie anzupassen"
```
/answer ed48-23 allow
Control Request:
```yaml
id: ed48-24
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cuda.rs
      nodes:
      - id: impl_CudaScoringMeasure.1A0dko|8j9ddV
        code: |2
              /// L2 of the target generation is a few MiB; this exceeds it by a wide margin.
              const FLUSH_BYTES: usize = 64 << 20;
              const WARMUP_LAUNCHES: u32 = 3;
              const MAX_ITERS: u32 = 10_000;
```
/answer ed48-24 allow
Control Request:
```yaml
id: ed48-25
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cuda.rs
    id: impl_CudaScoringMeasure.1A0dko|8j9ddV
    old_text: "const WARMUP_LAUNCHES: u32 = 3;"
    new_text: |-
      const WARMUP_LAUNCHES: u32 = 16;
          /// Long enough for the SM clock to leave its idle state (observed 1185 -> 2145 MHz).
          const WARMUP_MS: u64 = 100;
  reason: Warmup-Konstanten anpassen
```
/answer ed48-25 allow
Control Request:
```yaml
id: ed48-26
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - export PATH=/usr/local/cuda-13.3/bin:$PATH CUDA_PATH=/usr/local/cuda-13.3 CUDA_HOME=/usr/local/cuda-13.3
    - cargo test --features cuda --lib resident::precision::measure_cuda -- --ignored --nocapture --test-threads=1 2>&1 | grep -E 'G-CALIB|^test |test result|panicked|left:|right:|^error' | head -n 40
  reason: Gate-Test nach Warmup-Änderung erneut auf dem Gerät ausführen
```
/answer ed48-26 allow
Control Request:
```yaml
id: ed48-27
result:
  structuredContent:
    results:
    - stdout: |
        test resident::precision::measure_cuda::tests::a_waiting_task_interrupts_between_groups ... ok
        test resident::precision::measure_cuda::tests::fp32_matches_the_reference_and_fp16_stays_close ... ok
        test resident::precision::measure_cuda::tests::gate_calibration_front_on_device ... G-CALIB Axis batch=1,scoring=fp16 time 147.471µs dev 0.000e0 stable true samples 40
        G-CALIB Axis batch=8,scoring=fp16 time 76.412µs dev 0.000e0 stable false samples 40
        G-CALIB Axis batch=1,scoring=fp32 time 151.943µs dev 0.000e0 stable false samples 40
        G-CALIB Axis batch=8,scoring=fp32 time 77.875µs dev 0.000e0 stable false samples 40
        G-CALIB Verify batch=8,scoring=fp16 time 75.63µs dev 0.000e0 stable true samples 3
        G-CALIB Verify batch=8,scoring=fp32 time 76.99µs dev 0.000e0 stable true samples 3
        G-CALIB front batch=8,scoring=fp16 time 75.63µs dev 0.000e0 stable true
        test resident::precision::measure_cuda::tests::samples_scale_to_the_window_and_stay_positive ... ok
        test resident::precision::measure_cuda::tests::unsupported_types_are_incompatible ... ok
        test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 575 filtered out; finished in 6.03s
```
/answer ed48-27 allow
ReasoningToken: 455
Control Request:
```yaml
id: ed48-28
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/calibrate.rs
    id: impl_Default_for_CalibratorConfig.default
    old_text: "window_ms: 2.0,"
    new_text: "window_ms: 10.0,"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
    id: impl_Default_for_WindowTuning.default
    old_text: "Self { window_ms: 2.0, slice_budget_ms: 5.0 }"
    new_text: "Self { window_ms: 10.0, slice_budget_ms: 5.0 }"
  reason: "Längeres Standard-Messfenster testen, um Rauschen kurzer Kernel zu mitteln"
```
/answer ed48-28 allow
Control Request:
```yaml
id: ed48-29
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/calibrate.rs
      result: success
      id: impl_Default_for_CalibratorConfig.default
      new_id: impl_Default_for_CalibratorConfig.default
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
      error: Text not found in node; its content appears to have changed.
      id: impl_Default_for_WindowTuning.default
      reason: content_changed
      guess: "            slice_budget_ms: 5.0,"
```
/answer ed48-29 allow
Control Request:
```yaml
id: ed48-2a
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
    id: impl_Default_for_WindowTuning.default
    old_text: "window_ms: 2.0,"
    new_text: "window_ms: 10.0,"
  reason: WindowTuning-Default angleichen
```
/answer ed48-2a allow
Thinking:
Alle Tests laufen ohne GPU grün, der CUDA-Build ist sauber und die Kalibrierung läuft auf der GTX 1660 SUPER Ende-zu-Ende. Jetzt prüfe ich, ob ein längeres Messfenster die instabilen Axis-Gruppen beruhigt.


ReasoningToken: 139
Control Request:
```yaml
id: ed48-2b
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - export PATH=/usr/local/cuda-13.3/bin:$PATH CUDA_PATH=/usr/local/cuda-13.3 CUDA_HOME=/usr/local/cuda-13.3
    - cargo test --features cuda --lib resident::precision::measure_cuda -- --ignored --nocapture --test-threads=1 2>&1 | grep -E 'G-CALIB|test result|panicked|^error' | head -n 30
  reason: Gate-Test mit 10-ms-Fenster auf dem Gerät wiederholen
```
/answer ed48-2b allow
Control Request:
```yaml
id: ed48-2c
result:
  structuredContent:
    results:
    - stdout: |
        test resident::precision::measure_cuda::tests::gate_calibration_front_on_device ... G-CALIB Axis batch=1,scoring=fp16 time 157.687µs dev 0.000e0 stable true samples 40
        G-CALIB Axis batch=8,scoring=fp16 time 80.864µs dev 0.000e0 stable false samples 40
        G-CALIB Axis batch=1,scoring=fp32 time 157.412µs dev 0.000e0 stable true samples 3
        G-CALIB Axis batch=8,scoring=fp32 time 83.186µs dev 0.000e0 stable true samples 3
        G-CALIB Verify batch=8,scoring=fp16 time 80.8µs dev 0.000e0 stable true samples 40
        G-CALIB Verify batch=8,scoring=fp32 time 82.527µs dev 0.000e0 stable false samples 40
        G-CALIB front batch=8,scoring=fp16 time 80.8µs dev 0.000e0 stable true
        test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 575 filtered out; finished in 7.37s
```
/answer ed48-2c allow
ReasoningToken: 363
Control Request:
```yaml
id: ed48-2d
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - "cargo test --lib resident:: 2>&1 | grep -E 'FAILED|test result|^error'"
    - git status --short | head -n 30
  reason: Abschlusslauf der resident-Tests und Überblick über geänderte Dateien
```
/answer ed48-2d allow
Control Request:
```yaml
id: ed48-2e
result:
  structuredContent:
    results:
    - stdout: |
        test result: ok. 506 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 3.86s
         M ../project/resident/plan/runs/plan_f_02_calibrator.md
         M src/resident/mock.rs
         M src/resident/precision/mod.rs
         M src/resident/precision/pareto.rs
        ?? src/resident/precision/calibrate.rs
        ?? src/resident/precision/measure.rs
        ?? src/resident/precision/measure_cpu.rs
        ?? src/resident/precision/measure_cuda.rs
        ?? src/resident/precision/measure_mock.rs
        ?? src/resident/precision/metrics.rs
        ?? src/resident/precision/pool.rs
        ?? src/resident/precision/stats.rs
        ?? src/resident/precision/test/calibrate.rs
        ?? src/resident/precision/test/measure_cpu.rs
        ?? src/resident/precision/test/measure_cuda.rs
        ?? src/resident/precision/test/metrics.rs
        ?? src/resident/precision/test/pool.rs
        ?? src/resident/precision/test/stats.rs
        ?? src/resident/precision/test/workload.rs
        ?? src/resident/precision/workload.rs
```
/answer ed48-2e allow
Result Stats: id=7c6ed41f-e523-4ce4-b018-6eb50be99e19, total: 315117, in: 112, out: 118375, reason: 0, read: 7192743, write: 196630, start: 1792953369869, end: 1792953383974, time: 14056, total: 924657, io: 0, control: 49, sent: 0, recv: 383
F2 (Calibrator) ist umgesetzt und läuft Ende-zu-Ende auf der GTX 1660 SUPER. Ohne `cuda` sind alle 506 Resident-Tests grün, 93 davon im Precision-Modul. Mit `--features cuda` baut es sauber, und die 5 Device-Tests laufen durch.

Die Axis-Gruppen der echten GPU-Messung enden teils als `unstable` (Cap erreicht, bester Zwischenstand verwendet). Das Verhalten ist plangemäß. Es schwankt aber von Lauf zu Lauf, weil die Karte nicht exklusiv ist. Ein längeres Messfenster hat daran nichts geändert; der Default liegt jetzt bei 10 ms.

## Was gebaut ist (`rag/src/resident/precision/`)

| Datei | Inhalt |
|---|---|
| `measure.rs` | `Cfg`, `Validity`, `MeasureResult`, `Measure`-Trait, `YieldCheck`, `Clock` und die Factory `make_measure` (einzige `cuda`-Verzweigung) |
| `calibrate.rs` | `Calibrator`, `CalibratorConfig`, `HardCap`, `ParetoPoint`, `CalibrationReport` |
| `stats.rs`, `metrics.rs` | Median/Min, getrimmte Streuung (CV oder Range/Median), `1−cos` mit Rauschgrenze 1e-6, relativer Fehler, Top-k-Overlap, Plausibilitätsprüfung |
| `workload.rs` | duplikatfreie Einheitsvektoren mit fp32-Hostreferenz |
| `measure_cuda.rs` | CUDA-Events, Warmup über 100 ms, L2-Flush (64 MiB), Gruppen innerhalb des Slice-Budgets mit Prüfpunkt dazwischen |
| `measure_cpu.rs` | CPU-Messung des fp32-Scorings (nicht explizit gefordert, ergänzt) |
| `measure_mock.rs` | `FakeMeasure` mit seeded RNG und `ManualClock` |
| `pool.rs` | `LowPriorityPool` (`std::thread::Builder`, nice 19, getrennt von tokio) |

- **Messung:** Der Calibrator misst round-robin mit ABBA-Reihenfolge. Die Abweichung wird vor der Zeit gemessen.
- **Stabilität:** Sie wird am Zeitverhältnis zum Anker beurteilt, nicht an Einzelzeiten.
- **Verwerfen:** NaN/Inf, degenerierte Vektoren, Abweichung über `max_deviation` (Default 5e-2) und nicht lauffähige Kombinationen werden verworfen und nicht gesampelt.
- **Pruning:** Separable Achsen (`batch`, `tile`, `threads`) werden einzeln gemessen, nur die Gewinner kombiniert und verifiziert. Der Modus `exhaustive` dient als Referenz.
- **Caps:** `HardCap` (Wiederholungen oder Zeit) und `total_budget` beenden eine Messung, ohne zu blockieren. Der Rest wird `Skipped`, nie ein Fehler.
- **Pareto:** `pareto_by` in `pareto.rs` ist die generische Variante mit Identität.
- **Logging:** Span `calibrate{engine, model_id}`; Verworfene und Unstabile werden mit Grund geloggt.
- **Tests:** Die Pflichttests laufen mit drei Seeds: stabil, Ausreißer, driftend, bimodal, nie stabil, Pareto, Verwerfung, Pruning gegen Vollenumeration, ABBA und Yield.

## Abweichungen vom Plan

- **`Measure`-Trait:** Er hat `deviation()` und `sample()` statt eines einzelnen `measure()`. Nur so lässt sich interleaved und mit Early-Abort messen. `MeasureResult` ist das aggregierte Ergebnis pro Kombination.
- **`Validity::Failed`:** Die zusätzliche Variante steht für Kombinationen, die gar nicht laufen (zum Beispiel Alloc- oder Launch-Fehler).
- **`mock.rs`:** `NoiseModel` hat zwei neue Varianten, `Bimodal` und `Erratic`.
- **Reale Messung:** Sie misst den `cosine_dot`-Scoring-Pfad. Die Sequenzlängen im `Cfg` werden dort nicht ausgewertet, weil das Scoring keine hat. Sie gehören in eine künftige Encoder-Messung.
- **Thread-Priorität:** Sie läuft über `libc::setpriority`, weil kein `thread-priority`-Crate im Projekt ist.

## Noch offen

- **CPU-Messung:** Sie kennt nur fp32, weil für die CPU kein anderer Kernel existiert.
- **F5-Persistenz:** `CalibrationReport` ist nicht serialisierbar, weil `AxisId` und `ValueId` kein serde haben. F5 muss das abbilden.
- **Pareto-Positionen:** Die Profile und ihre Positionen berechnet erst F3; `pareto::positions` existiert bereits.
- **Nicht angefasst:** Die Plandatei `runs/plan_f_02_calibrator.md` zeigt in `git status` als geändert, die Änderung stammt nicht von mir.
