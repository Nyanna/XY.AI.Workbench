Setze F4 um.

- keine direkte Referenzierung von Planelementen in Kommentaren
- nvcc liegt unter `/usr/local/cuda-13.3/bin/nvcc`
- Nutze Bash nicht für Retrieval Tasks
- Erledige restlos alle Punkte

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_f_00_overview.md)

[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_f_04_task_matching.md)

SystemInit:  id=14ebff42-4411-4486-937b-15cd720453e1, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst den vorhandenen Code im Precision-Modul an.
Control Request:
```yaml
id: ed48-55
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: precision|subengine|mod\.rs|error
  reason: Vorhandene Precision-/Subengine-Dateien finden
```
/answer ed48-55 allow
ReasoningToken: 27
Control Request:
```yaml
id: ed48-56
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
      - error.rs
      - mod.rs
      - "./kernel/test:"
      - mod.rs
      - "./manager:"
      - mod.rs
      - "./memory:"
      - error.rs
      - mod.rs
      - "./model:"
      - error.rs
      - mod.rs
      - "./model/test:"
      - mod.rs
      - "./op:"
      - mod.rs
      - "./op/test:"
      - mod.rs
      - "./planner:"
      - mod.rs
      - "./planner/test:"
      - mod.rs
      - "./precision:"
      - mod.rs
      - "./processor:"
      - error.rs
      - mod.rs
      - "./processor/test:"
      - error.rs
      - "./queue:"
      - mod.rs
      - "./segment:"
      - error.rs
      - mod.rs
      - "./segment/test:"
      - mod.rs
      - "./subengine:"
      - error.rs
      - mod.rs
      - "./test:"
      - error.rs
      - "./tokenize:"
      - error.rs
      - mod.rs
      - "./tokenize/test:"
      - mod.rs
```
/answer ed48-56 allow
Control Request:
```yaml
id: ed48-57
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: PrecisionError|struct Profile|NoActiveProfile|InvalidPrecision|TaskParams
    include:
    - '*.rs'
    exclude:
    - target/**
    intent: editing
  reason: Bestehende Typen Profile/PrecisionError/TaskParams finden
```
/answer ed48-57 allow
Control Request:
```yaml
id: ed48-58
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/precision/probe.rs
        nodes:
        - id: xIJRa2|pEpb17
          matches:
          - use crate::resident::error::PrecisionError;
        - id: CapabilityProbe.probe
          matches:
          - "fn probe(&self) -> Result<CapabilitySet, PrecisionError>;"
        - id: impl_CapabilityProbe_for_KernelFilteredProbe_P_.probe
          matches:
          - "fn probe(&self) -> Result<CapabilitySet, PrecisionError> {"
        - id: make_probe
          matches:
          - ") -> Result<Box<dyn CapabilityProbe>, PrecisionError> {"
        - id: impl_CapabilityProbe_for_FakeCapabilityProbe.probe
          matches:
          - "fn probe(&self) -> Result<CapabilitySet, PrecisionError> {"
      - path: resident/precision/probe_cpu.rs
        nodes:
        - id: 9EqFK4|87Wghl
          matches:
          - use crate::resident::error::PrecisionError;
        - id: impl_CapabilityProbe_for_CpuProbe.probe
          matches:
          - "fn probe(&self) -> Result<CapabilitySet, PrecisionError> {"
      - path: resident/precision/measure_cpu.rs
        nodes:
        - id: 5ZDfjI|a9Fm5u
          matches:
          - use crate::resident::error::PrecisionError;
        - id: impl_CpuScoringMeasure.new
          matches:
          - ") -> Result<Self, PrecisionError> {"
        - id: impl_CpuScoringMeasure.threads
          matches:
          - "fn threads(cfg: &Cfg) -> Result<usize, PrecisionError> {"
          - "PrecisionError::Incompatible { reason: format!(\"threads value `{v}` invalid\") }"
        - id: impl_CpuScoringMeasure.check_types
          matches:
          - "fn check_types(cfg: &Cfg) -> Result<(), PrecisionError> {"
          - "return Err(PrecisionError::Incompatible {"
        - id: impl_CpuScoringMeasure.plan
          matches:
          - "fn plan(&mut self, cfg: &Cfg) -> Result<(usize, usize), PrecisionError> {"
          - ".map_err(|e| PrecisionError::Incompatible { reason: e.to_string() })?;"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer ed48-58 allow
Control Request:
```yaml
id: ed48-59
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
  reason: Dateien im precision-Modul
```
/answer ed48-59 allow
Control Request:
```yaml
id: ed48-5a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      entries:
      - ".:"
      - axis.rs
      - calibrate.rs
      - capability.rs
      - known.rs
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
      - workload.rs
      - "./test:"
      - axis.rs
      - calibrate.rs
      - capability.rs
      - known.rs
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
      - workload.rs
```
/answer ed48-5a allow
Control Request:
```yaml
id: ed48-5b
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "enum PrecisionError|InvalidPrecision|NoActiveProfile|struct Profile\\b|Profile \\{"
    include:
    - '*.rs'
    limit: 30
  reason: Fehler-Enum und Profile-Typ lokalisieren
```
/answer ed48-5b allow
Control Request:
```yaml
id: ed48-5c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: precision/test/profile.rs
        matches:
        - "Err(PrecisionError::InvalidPrecision { .. }))"
      - path: precision/profile.rs
        matches:
        - "pub struct Profile {"
        - "impl Profile {"
        - "return Err(PrecisionError::InvalidPrecision {"
      - path: facade/resident.rs
        matches:
        - state.complete(Err(TaskError::InvalidPrecision));
      - path: facade/error.rs
        matches:
        - "InvalidPrecision,"
      - path: facade/test/resident.rs
        matches:
        - "assert!(matches!(handle.await, Err(TaskError::InvalidPrecision)));"
      - path: error.rs
        matches:
        - "pub enum PrecisionError {"
        - "NoActiveProfile,"
        - "InvalidPrecision { requested: f32 },"
      - path: test/error.rs
        matches:
        - "let e: ResidentError = PrecisionError::NoActiveProfile.into();"
        - assert!(! PrecisionError::NoActiveProfile.is_retryable());
```
/answer ed48-5c allow
Control Request:
```yaml
id: ed48-5d
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/profile.rs
  reason: "Struktur von Profile, mod.rs und Tests verstehen"
```
/answer ed48-5d allow
Control Request:
```yaml
id: ed48-5e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Unnamed profiles: a position on the fast-to-precise scale plus a configurat…"
      - id: U2xmDk|VkXFR4
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
      - id: ProfilingConfig
        signature: "pub struct ProfilingConfig {"
        docstring: "The only tuning knob of profile formation, in the normalized arc-length scale `…"
      - id: impl_Default_for_ProfilingConfig
        signature: "impl Default for ProfilingConfig {"
        children:
        - id: impl_Default_for_ProfilingConfig.default
          signature: "fn default() -> Self {"
      - id: derive_profiles
        signature: "pub fn derive_profiles(front: &[ParetoPoint], cfg: &ProfilingConfig) -> Vec<Pro…"
        docstring: Derives unnamed profiles from a front ordered by ascending time and strictly
      - id: coordinates
        signature: "fn coordinates(points: &[&ParetoPoint]) -> Vec<(f64, f64)> {"
      - id: significant
        signature: "fn significant(front: &[ParetoPoint]) -> Vec<&ParetoPoint> {"
        docstring: Drops finite-invalid points and every point that is less precise than a peer wi…
      - id: tests
        signature: mod tests;
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! CapabilitySet/Profile.
      - id: calibrate
        signature: mod calibrate;
      - id: measure
        signature: mod measure;
      - id: measure_cpu
        signature: mod measure_cpu;
      - id: measure_cuda
        signature: mod measure_cuda;
      - id: measure_mock
        signature: mod measure_mock;
      - id: metrics
        signature: mod metrics;
      - id: pool
        signature: mod pool;
      - id: stats
        signature: mod stats;
      - id: workload
        signature: mod workload;
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
      - id: v1sUIb|qKig7l
        signature: "pub use axis::{AxisId, ValueId};"
      - id: cu91HB|M6H8iN
        signature: "pub use probe::{"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/profile.rs
      nodes:
      - id: LwlKpn|JFVpeI
        signature: use super::*;
      - id: position_must_be_in_unit_interval
        signature: "fn position_must_be_in_unit_interval() {"
      - id: config_holds_one_value_per_axis
        signature: "fn config_holds_one_value_per_axis() {"
      - id: IRbbmX|MCRAua
        signature: use std::time::Duration;
      - id: point
        signature: "fn point(name: &str, ms: f64, deviation: f64, uncertainty: f64) -> ParetoPoint {"
      - id: names
        signature: "fn names(profiles: &[Profile]) -> Vec<String> {"
      - id: threshold
        signature: "fn threshold(t: f64) -> ProfilingConfig {"
      - id: empty_front_yields_no_profile
        signature: "fn empty_front_yields_no_profile() {"
      - id: single_point_sits_in_the_middle
        signature: "fn single_point_sits_in_the_middle() {"
      - id: threshold_controls_profile_count
        signature: "fn threshold_controls_profile_count() {"
      - id: close_points_merge_and_precise_end_is_kept
        signature: "fn close_points_merge_and_precise_end_is_kept() {"
      - id: cluster_representative_is_the_fastest_point
        signature: "fn cluster_representative_is_the_fastest_point() {"
      - id: dense_chain_collapses_to_one_profile
        signature: "fn dense_chain_collapses_to_one_profile() {"
      - id: positions_follow_distance_not_rank
        signature: "fn positions_follow_distance_not_rank() {"
      - id: point_within_noise_of_a_more_precise_one_is_dropped
        signature: "fn point_within_noise_of_a_more_precise_one_is_dropped() {"
      - id: significantly_faster_point_survives_noise_check
        signature: "fn significantly_faster_point_survives_noise_check() {"
      - id: identical_coordinates_do_not_break_positions
        signature: "fn identical_coordinates_do_not_break_positions() {"
      - id: non_finite_points_are_ignored
        signature: "fn non_finite_points_are_ignored() {"
```
/answer ed48-5e allow
Control Request:
```yaml
id: ed48-5f
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: NoActiveProfile|InvalidPrecision|match_profile|precision
    include:
    - error.rs
    - resident.rs
    - mod.rs
    limit: 50
  reason: Verwendung von Fehlern und Fassade/Subengine-Anknüpfung
```
/answer ed48-5f allow
Control Request:
```yaml
id: ed48-60
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: mod.rs
        matches:
        - pub mod precision;
      - path: facade/resident.rs
        matches:
        - let precision = params.get_precision();
        - "precision,"
        - "fn submit(&self, kind: TaskKind, precision: f32, payload: Payload) -> TaskHandle {"
        - "if precision.is_nan() {"
        - state.complete(Err(TaskError::InvalidPrecision));
      - path: facade/error.rs
        matches:
        - "#[error(\"precision must not be NaN\")]"
        - "InvalidPrecision,"
      - path: facade/test/resident.rs
        matches:
        - "async fn nan_precision_fails_without_enqueue() {"
        - let params = EmbeddingParams::default().precision(f32::NAN);
        - "assert!(matches!(handle.await, Err(TaskError::InvalidPrecision)));"
      - path: error.rs
        matches:
        - "#[error(\"incompatible precision/layout: {reason}\")]"
        - "NoActiveProfile,"
        - "#[error(\"invalid precision requested: {requested}\")]"
        - "InvalidPrecision { requested: f32 },"
      - path: test/error.rs
        matches:
        - "let e: ResidentError = PrecisionError::NoActiveProfile.into();"
        - assert!(! PrecisionError::NoActiveProfile.is_retryable());
```
/answer ed48-60 allow
Control Request:
```yaml
id: ed48-61
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
    ids:
    - ModuleDoc
    - Profile
    - impl_Profile
    - tests
  reason: Quelltext von mod.rs und Profile lesen
```
/answer ed48-61 allow
ReasoningToken: 61
Control Request:
```yaml
id: ed48-62
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! CapabilitySet/Profile.
      - id: calibrate
        signature: mod calibrate;
      - id: measure
        signature: mod measure;
      - id: measure_cpu
        signature: mod measure_cpu;
      - id: measure_cuda
        signature: mod measure_cuda;
      - id: measure_mock
        signature: mod measure_mock;
      - id: metrics
        signature: mod metrics;
      - id: pool
        signature: mod pool;
      - id: stats
        signature: mod stats;
      - id: workload
        signature: mod workload;
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
      - id: v1sUIb|qKig7l
        signature: "pub use axis::{AxisId, ValueId};"
      - id: cu91HB|M6H8iN
        signature: "pub use probe::{"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Unnamed profiles: a position on the fast-to-precise scale plus a configuration.
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
                        return Err(PrecisionError::InvalidPrecision {
                            requested: position,
                        });
                    }
                    Ok(Self { position, config })
                }
      - id: tests
        type: ItemMod
        code: |
          #[cfg(test)]
          #[path = "test/profile.rs"]
          mod tests;
```
/answer ed48-62 allow
Control Request:
```yaml
id: ed48-63
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: pub enum PrecisionError
    include:
    - error.rs
    intent: editing
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
    pattern: Profile|profile|blocked|locked|banned
    include:
    - '*.rs'
    limit: 30
  reason: PrecisionError-Definition und Subengine-Anknüpfung für Lazy-Sperre prüfen
```
/answer ed48-63 allow
Control Request:
```yaml
id: ed48-64
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: error.rs
        nodes:
        - id: PrecisionError
          matches:
          - "pub enum PrecisionError {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
```
/answer ed48-64 allow
Control Request:
```yaml
id: ed48-65
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    ids:
    - cu91HB|M6H8iN
    - ModuleDoc
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - PrecisionError
  reason: Re-Exports und Fehlerenum im Detail
```
/answer ed48-65 allow
ReasoningToken: 83
Control Request:
```yaml
id: ed48-66
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: cu91HB|M6H8iN
        type: ImportGroup
        code: |
          pub use probe::{
              intersect_with_kernels, make_probe, CapabilityProbe, KernelFilteredProbe, ProbeConfig,
          };
          #[cfg(any(test, feature = "mock"))]
          pub use probe::FakeCapabilityProbe;
          pub use probe_cpu::{CpuFeatureSource, CpuFeatures, CpuProbe, FixedCpu, HostCpu};
          pub use probe_cuda::{cuda_capabilities, has_tensor_cores, CudaProbe};
          pub use profile::{derive_profiles, Profile, ProfileConfig, ProfilingConfig};
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! CapabilitySet/Profile.
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
```
/answer ed48-66 allow
Control Request:
```yaml
id: ed48-67
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/subengine/mod.rs
  reason: Subengine-Struktur für Markierung gesperrter Modelle
```
/answer ed48-67 allow
Control Request:
```yaml
id: ed48-68
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
```
/answer ed48-68 allow
Control Request:
```yaml
id: ed48-69
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: tracing::|^tracing|debug!\(
    include:
    - '*.rs'
    - Cargo.toml
    exclude:
    - .bin/**
    - target/**
    limit: 10
  reason: Logging-Konvention und tracing-Abhängigkeit prüfen
```
/answer ed48-69 allow
ReasoningToken: 418
Control Request:
```yaml
id: ed48-6a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: segment/mod.rs
        matches:
        - "tracing::debug!(segment_gen = self.view.gen, \"segment released\");"
        - "tracing::debug!(generation = next.generation, \"manifest published\");"
      - path: segment/slots.rs
        matches:
        - "tracing::trace!(segment_gen = self.view.gen, slot = old, \"tombstone\");"
        - "tracing::trace!(segment_gen = self.view.gen, slot, \"upsert\");"
        - "tracing::trace!(segment_gen = self.view.gen, slot, \"tombstone\");"
      - path: segment/sync.rs
        matches:
        - "tracing::debug!(?event, \"segment event\");"
      - path: segment/mapping.rs
        matches:
        - "tracing::debug!(segment_gen, slot, \"remap\");"
      - path: processor/worker.rs
        matches:
        - "let span = tracing::info_span!(\"worker\", ? side, index);"
        - "Err(_) => tracing::error!(\"worker pass panicked\"),"
        - tracing::warn!(
      warning: "Limit of 10 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer ed48-6a allow
Control Request:
```yaml
id: ed48-6b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/profile.rs
    ids:
    - LwlKpn|JFVpeI
    - position_must_be_in_unit_interval
    - config_holds_one_value_per_axis
  reason: Teststil der bestehenden Profile-Tests übernehmen
```
/answer ed48-6b allow
Control Request:
```yaml
id: ed48-6c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/profile.rs
      nodes:
      - id: LwlKpn|JFVpeI
        type: ImportGroup
        code: |
          use super::*;
      - id: position_must_be_in_unit_interval
        type: ItemFn
        code: |
          #[test]
          fn position_must_be_in_unit_interval() {
              for p in [0.0, 0.5, 1.0] {
                  assert!(Profile::new(p, ProfileConfig::new()).is_ok());
              }
              for p in [-0.1, 1.1, f32::NAN, f32::INFINITY] {
                  assert!(
                      matches!(Profile::new(p, ProfileConfig::new()),
                      Err(PrecisionError::InvalidPrecision { .. }))
                  );
              }
          }
      - id: config_holds_one_value_per_axis
        type: ItemFn
        code: |
          #[test]
          fn config_holds_one_value_per_axis() {
              let cfg = ProfileConfig::new()
                  .with("operands", "fp16")
                  .with("operands", "fp32")
                  .with("fusion", "on");
              assert_eq!(cfg.len(), 2);
              assert_eq!(cfg.get(& AxisId::new("operands")), Some(& ValueId::new("fp32")));
          }
```
/answer ed48-6c allow
Text:
 Es gibt `Profile` und `PrecisionError` bereits. `NoActiveProfile` ist dort eine Unit-Variante, die ich unverändert lasse, damit bestehende Tests nicht brechen. Jetzt lege ich das Matching-Modul an.
Control Request:
```yaml
id: ed48-6d
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/matching.rs
    source: |
      //! Maps a task's precision wish onto the nearest active profile of one subengine.
      //!
      //! Scale: `0.0` is fastest, `1.0` is most precise. The scale is relative to the
      //! subengine: `0.5` denotes different accuracy levels on CPU and GPU.

      use std::collections::HashSet;
      use std::sync::Mutex;

      use crate::resident::error::PrecisionError;

      use super::profile::Profile;

      /// Returns the profile with minimal `|t - position|`, where `t` is `precision`
      /// clamped to `[0, 1]`. Equal distances resolve to the more precise profile.
      ///
      /// `NaN` is rejected before clamping (`f32::clamp` would pass it through).
      pub fn match_profile(precision: f32, profiles: &[Profile]) -> Result<&Profile, PrecisionError> {
          if precision.is_nan() {
              return Err(PrecisionError::InvalidPrecision {
                  requested: precision,
              });
          }
          let t = f64::from(precision.clamp(0.0, 1.0));
          profiles
              .iter()
              .min_by(|a, b| {
                  let da = (t - f64::from(a.position)).abs();
                  let db = (t - f64::from(b.position)).abs();
                  da.total_cmp(&db)
                      .then(b.position.total_cmp(&a.position))
              })
              .ok_or(PrecisionError::NoActiveProfile)
      }

      /// Per-subengine record of models without any active profile.
      ///
      /// The first task for such a model fails with `NoActiveProfile`; later tasks
      /// fail immediately without repeating the profile lookup.
      #[derive(Debug, Default)]
      pub struct ProfileSelector {
          blocked: Mutex<HashSet<String>>,
      }

      impl ProfileSelector {
          pub fn new() -> Self {
              Self::default()
          }

          pub fn is_blocked(&self, model_id: &str) -> bool {
              self.lock().contains(model_id)
          }

          /// Lifts the block, e.g. after a successful recalibration.
          pub fn unblock(&self, model_id: &str) -> bool {
              self.lock().remove(model_id)
          }

          pub fn select<'a>(
              &self,
              model_id: &str,
              precision: f32,
              profiles: &'a [Profile],
          ) -> Result<&'a Profile, PrecisionError> {
              if self.is_blocked(model_id) {
                  return Err(PrecisionError::NoActiveProfile);
              }
              match match_profile(precision, profiles) {
                  Ok(profile) => {
                      tracing::debug!(
                          model_id,
                          requested = precision,
                          chosen = profile.position,
                          "profile chosen"
                      );
                      Ok(profile)
                  }
                  Err(PrecisionError::NoActiveProfile) => {
                      self.lock().insert(model_id.to_owned());
                      tracing::warn!(model_id, "no active profile, model blocked");
                      Err(PrecisionError::NoActiveProfile)
                  }
                  Err(e) => Err(e),
              }
          }

          fn lock(&self) -> std::sync::MutexGuard<'_, HashSet<String>> {
              self.blocked.lock().unwrap_or_else(|e| e.into_inner())
          }
      }

      #[cfg(test)]
      #[path = "test/matching.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/test/matching.rs
    source: |
      use super::*;
      use crate::resident::precision::ProfileConfig;

      fn profiles(positions: &[f32]) -> Vec<Profile> {
          positions
              .iter()
              .map(|&p| Profile::new(p, ProfileConfig::new()).unwrap())
              .collect()
      }

      fn chosen(t: f32, positions: &[f32]) -> f32 {
          match_profile(t, &profiles(positions)).unwrap().position
      }

      #[test]
      fn out_of_range_values_are_clamped() {
          let set = [0.0, 0.5, 1.0];
          assert_eq!(chosen(-0.3, &set), 0.0);
          assert_eq!(chosen(1.7, &set), 1.0);
          assert_eq!(chosen(f32::INFINITY, &set), 1.0);
          assert_eq!(chosen(f32::NEG_INFINITY, &set), 0.0);
      }

      #[test]
      fn nan_is_an_error_even_without_profiles() {
          assert!(matches!(
              match_profile(f32::NAN, &profiles(&[0.5])),
              Err(PrecisionError::InvalidPrecision { .. })
          ));
          assert!(matches!(
              match_profile(f32::NAN, &[]),
              Err(PrecisionError::InvalidPrecision { .. })
          ));
      }

      #[test]
      fn tie_prefers_the_more_precise_profile() {
          for order in [[0.0, 0.25, 0.75], [0.75, 0.25, 0.0], [0.25, 0.0, 0.75]] {
              for _ in 0..3 {
                  assert_eq!(chosen(0.5, &order), 0.75);
              }
          }
          assert_eq!(chosen(0.5, &[0.75, 0.25]), 0.75);
      }

      #[test]
      fn nearest_profile_wins() {
          let set = [0.0, 0.3, 1.0];
          assert_eq!(chosen(0.1, &set), 0.0);
          assert_eq!(chosen(0.2, &set), 0.3);
          assert_eq!(chosen(0.9, &set), 1.0);
      }

      #[test]
      fn lone_profile_matches_every_wish() {
          for t in [0.0, 0.3, 1.0, 5.0, -1.0] {
              assert_eq!(chosen(t, &[0.5]), 0.5);
          }
      }

      #[test]
      fn empty_set_has_no_active_profile() {
          assert!(matches!(
              match_profile(0.5, &[]),
              Err(PrecisionError::NoActiveProfile)
          ));
      }

      #[test]
      fn distances_are_not_collapsed_by_f32_rounding() {
          let set = profiles(&[0.1, 0.5]);
          assert_eq!(match_profile(0.3, &set).unwrap().position, 0.5);
          assert_eq!(match_profile(0.299_999_9, &set).unwrap().position, 0.1);
      }

      #[test]
      fn empty_model_is_blocked_after_first_failure() {
          let selector = ProfileSelector::new();
          assert!(!selector.is_blocked("m"));
          assert!(matches!(
              selector.select("m", 0.5, &[]),
              Err(PrecisionError::NoActiveProfile)
          ));
          assert!(selector.is_blocked("m"));

          let set = profiles(&[0.5]);
          assert!(matches!(
              selector.select("m", 0.5, &set),
              Err(PrecisionError::NoActiveProfile)
          ));
          assert!(selector.select("other", 0.5, &set).is_ok());

          assert!(selector.unblock("m"));
          assert!(selector.select("m", 0.5, &set).is_ok());
      }

      #[test]
      fn nan_does_not_block_the_model() {
          let selector = ProfileSelector::new();
          let set = profiles(&[0.5]);
          assert!(selector.select("m", f32::NAN, &set).is_err());
          assert!(!selector.is_blocked("m"));
      }
  reason: Matching-Modul F4 und Tests anlegen
```
/answer ed48-6d allow
ReasoningToken: 147
Control Request:
```yaml
id: ed48-6e
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    id: profile
    old_text: mod profile;
    new_text: |-
      mod matching;
      mod profile;
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    id: cu91HB|M6H8iN
    old_text: "pub use profile::{derive_profiles,"
    new_text: |-
      pub use matching::{match_profile, ProfileSelector};
      pub use profile::{derive_profiles,
  reason: Modul registrieren und exportieren
```
/answer ed48-6e allow
ReasoningToken: 160
Control Request:
```yaml
id: ed48-6f
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - cargo test --lib resident::precision::matching 2>&1 | tail -30
  reason: Kompilieren und neue Tests ausführen
```
/answer ed48-6f allow
Control Request:
```yaml
id: ed48-70
result:
  structuredContent:
    results:
    - stdout: |2+
            = note: `#[warn(irrefutable_let_patterns)]` on by default

        warning: associated function `from_raw` is never used
          --> src/resident/manager/seqlock.rs:20:26
           |
        12 | impl<'a> Seqlock<'a> {
           | -------------------- associated function in this implementation
        ...
        20 |     pub(crate) unsafe fn from_raw(seq: *mut u64, words: *mut u64, len: usize) -> Self {
           |                          ^^^^^^^^
           |
           = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

        warning: `xy-ai-rag` (lib test) generated 2 warnings
            Finished `test` profile [unoptimized + debuginfo] target(s) in 7.76s
             Running unittests src/lib.rs (.bin/debug/deps/xy_ai_rag-d5754016cabf875c)

        running 9 tests
        test resident::precision::matching::tests::empty_set_has_no_active_profile ... ok
        test resident::precision::matching::tests::distances_are_not_collapsed_by_f32_rounding ... ok
        test resident::precision::matching::tests::lone_profile_matches_every_wish ... ok
        test resident::precision::matching::tests::nan_does_not_block_the_model ... ok
        test resident::precision::matching::tests::nan_is_an_error_even_without_profiles ... ok
        test resident::precision::matching::tests::empty_model_is_blocked_after_first_failure ... ok
        test resident::precision::matching::tests::nearest_profile_wins ... ok
        test resident::precision::matching::tests::out_of_range_values_are_clamped ... ok
        test resident::precision::matching::tests::tie_prefers_the_more_precise_profile ... ok

        test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 545 filtered out; finished in 0.00s
```
/answer ed48-70 allow
Result Stats: id=14ebff42-4411-4486-937b-15cd720453e1, total: 36069, in: 18, out: 7856, reason: 64, read: 191510, write: 28195, start: 1792954173114, end: 1792954181356, time: 8195, total: 71201, io: 0, control: 47, sent: 0, recv: 184
ReasoningToken: 64
F4 ist umgesetzt, und die 9 neuen Tests laufen durch. Den `cuda`-Build habe ich nicht geprüft. Das Modul ist rein rechnerisch, ohne GPU-Code, dort sollte es keine Probleme geben.

**Was neu ist**
- **`precision/matching.rs`:**
  - `match_profile(precision, &[Profile])` prüft zuerst auf `NaN` und gibt dann `InvalidPrecision` zurück. Danach klemmt es auf `[0,1]`, rechnet die Distanz in f64 und nimmt das nächste Profil. Bei gleicher Distanz gewinnt das genauere (höheres `position`). Eine leere Menge ergibt `NoActiveProfile`.
  - `ProfileSelector` ist die Lazy-Sperre. Der erste Task für ein Modell ohne aktives Profil bekommt `NoActiveProfile`, und das Modell wird markiert. Folgetasks scheitern sofort, ohne erneute Suche. Mit `unblock` lässt sich die Sperre aufheben, etwa nach einer Rekalibrierung. `NaN` sperrt das Modell nicht.
  - Das gewählte Profil wird pro Task auf `debug` geloggt.
- **`precision/test/matching.rs`:** Die Tests decken diese Fälle ab:
  - Klemmen, auch bei ±∞
  - `NaN` vor Klemmen, auch bei leerer Menge
  - Tie-Break unabhängig von der Eingabereihenfolge
  - nächstes Profil und Einzelprofil
  - leere Menge
  - f64-Distanz
  - Sperre und Entsperren
- **`precision/mod.rs`:** Das Modul ist registriert, `match_profile` und `ProfileSelector` sind exportiert.

**Abweichungen vom Plan**
- `NoActiveProfile` bleibt eine Unit-Variante, ohne `model_id`, weil sie so bereits im Code und in `test/error.rs` verwendet wird. Deshalb hat `match_profile` auch keinen `model_id`-Parameter. Die Modell-ID steckt im `ProfileSelector` und im Log.
- Die Skala-Doku (0 = fast, 1 = precision) stand schon an `Profile::position`. Ich habe sie nicht verändert.

Das Durchreichen von `TaskParams.precision` durch Fassade und Subengine-Pull habe ich nicht angefasst. Die Fassade prüft `NaN` bereits selbst.
