# Aufgabe: Design-Dokument mit Implementierung abgleichen

Gleiche das Design-Dokument `/home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md` mit der Implementierung in `/home/user/xyan/xy.ai.workbench/rag/src/resident/precision` ab. Liefere zwei Ergebnisse:
1. Die README der Implementierung, überarbeitet.
2. Eine Ausgabe mit den Abweichungen.

## Vorgehen
- Lies zuerst Design, bestehende README und den gesamten Code. Tests liest du nur, wenn du ein Verhalten nicht aus dem Code erkennst.
- Halte für jeden Punkt des Designs fest, ob er umgesetzt ist, abweicht oder nicht abgebildet ist. Schreibe erst danach.

## README
- Sie beschreibt den **Ist-Zustand**. Das Design erklärt und begründet die Umsetzung, soweit der Code es widerspiegelt.
- Design und Ist-Zustand sind **integriert** dargestellt, in der bestehenden Struktur. Du hängst keine eigenen Abschnitte an, auch nicht für Lücken, Abweichungen oder Begründungen.
- Sie enthält keine Bewertung, keine Lösungsvorschläge und keine Perspektivpunkte.
- Nebensächliches und alles, was nicht ausdrücklich im Design steht, lässt du weg. Inhalt, der in ein anderes Paket gehört, lässt du ebenfalls weg.
- Sprache und Stil richten sich nach der bestehenden README.

## Abweichungsdatei
- Sie ist auf Englisch und für agentische Verarbeitung gedacht: kurze, eindeutige Punkte.
- Sie enthält nur Abweichungen und nicht abgebildete Aspekte, die konkret umzusetzen sind. Perspektivische Punkte und Ausblicke gehören nicht hinein.
- Lösungsvorschläge gibt es nicht, weil du den Kontext nicht gut genug kennst, um sie zu beurteilen.

## Arbeitsweise
- Das Design wird nicht verändert und ist nicht Gegenstand der Arbeit.
- Ablehnungen von Änderungen haben immer einen inhaltlichen Grund. Rate dann nicht weiter, sondern frage gezielt nach.

SystemInit:  id=2e14fb9c-3faf-4561-9736-41a29d527cc4, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: b503-1b
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md
  reason: Design-Dokument lesen
```
/answer b503-1b allow
Control Request:
```yaml
id: b503-1c
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
  reason: Dateien der Implementierung auflisten
```
/answer b503-1c allow
Control Request:
```yaml
id: b503-1d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision
      entries:
      - ".:"
      - README.md
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
      - "./test:"
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
/answer b503-1d allow
Control Request:
```yaml
id: b503-1e
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/README.md
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/axis.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/calibrate.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/capability.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/matching.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cpu.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_cuda.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/measure_mock.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/metrics.rs
  reason: README und erste Hälfte des Codes lesen
```
/answer b503-1e allow
Control Request:
```yaml
id: b503-1f
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/pareto.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/pool.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe_cpu.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/probe_cuda.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/stats.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/store.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/workload.rs
  reason: Zweite Hälfte des Codes lesen
```
/answer b503-1f allow
ReasoningToken: 137
Control Request:
```yaml
id: b503-20
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: enum PrecisionError|InvalidPrecision|NoActiveProfile|CalibrationUnstable|struct TaskParams|DENSITY|ProfileSelector|spawn_blocking
    include:
    - '*.rs'
    exclude:
    - test/**
    limit: 40
  reason: "Prüfen, wo Fehler, Selector und Density außerhalb des Pakets genutzt werden"
```
/answer b503-20 allow
Control Request:
```yaml
id: b503-21
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/precision/mod.rs
        matches:
        - "pub use matching::{match_profile, ProfileSelector};"
      - path: resident/precision/known.rs
        matches:
        - "FUSION = \"fusion\", BATCH = \"batch\", TILE = \"tile\", THREADS = \"threads\", DENSITY ="
      - path: resident/precision/test/store.rs
        matches:
        - "assert!(matches!(stored.to_profile(), Err(PrecisionError::InvalidPrecision { .. })));"
        - "Err(PrecisionError::CalibrationUnstable { reason: \"never stable\".into() })"
      - path: resident/precision/test/profile.rs
        matches:
        - "Err(PrecisionError::InvalidPrecision { .. }))"
      - path: resident/precision/test/matching.rs
        matches:
        - "Err(PrecisionError::InvalidPrecision { .. })"
        - Err(PrecisionError::NoActiveProfile)
        - let selector = ProfileSelector::new();
      - path: resident/precision/profile.rs
        matches:
        - "return Err(PrecisionError::InvalidPrecision {"
      - path: resident/precision/matching.rs
        matches:
        - "return Err(PrecisionError::InvalidPrecision {"
        - .ok_or(PrecisionError::NoActiveProfile)
        - /// The first task for such a model fails with `NoActiveProfile`; later tasks
        - "pub struct ProfileSelector {"
        - "impl ProfileSelector {"
        - return Err(PrecisionError::NoActiveProfile);
        - "Err(PrecisionError::NoActiveProfile) => {"
        - Err(PrecisionError::NoActiveProfile)
      - path: resident/precision/calibrate.rs
        matches:
        - "% cfg, error = % PrecisionError::CalibrationUnstable { reason :"
      - path: resident/facade/resident.rs
        matches:
        - state.complete(Err(TaskError::InvalidPrecision));
      - path: resident/facade/error.rs
        matches:
        - "InvalidPrecision,"
      - path: resident/facade/test/resident.rs
        matches:
        - "assert!(matches!(handle.await, Err(TaskError::InvalidPrecision)));"
      - path: resident/error.rs
        matches:
        - "pub enum PrecisionError {"
        - "CalibrationUnstable { reason: String },"
        - "NoActiveProfile,"
        - "InvalidPrecision { requested: f32 },"
        - "matches!(self, PrecisionError::CalibrationUnstable { .. })"
      - path: resident/test/error.rs
        matches:
        - "let e: ResidentError = PrecisionError::NoActiveProfile.into();"
        - "assert!(PrecisionError::CalibrationUnstable { reason : \"r\".into() } .is_retryable());"
        - assert!(! PrecisionError::NoActiveProfile.is_retryable());
      - path: resident/manager/blocking.rs
        matches:
        - self.handle.spawn_blocking(move || span.in_scope(read)).await
      - path: resident/manager/residency.rs
        matches:
        - use crate::resident::precision::known::DENSITY;
        - "match config.get(&DENSITY).and_then(|v| v.as_str().parse::<u32>().ok()) {"
        - let bytes = tokio::task::spawn_blocking(move || execute(
      - path: resident/manager/test/residency.rs
        matches:
        - "let cfg = ProfileConfig::new().with(&*DENSITY, ValueId::numeric(300));"
        - "let garbage = ProfileConfig::new().with(&*DENSITY, ValueId::new(\"high\"));"
      - path: resident/telemetry.rs
        matches:
        - "//! into `spawn_blocking`/plain threads automatically: capture `Span::current()`, then"
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer b503-21 allow
ReasoningToken: 2560
Control Request:
```yaml
id: b503-22
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: precision::|resolve\(|recalibrate_in_background|derive_profiles|most precise|conservative
    include:
    - '*.rs'
    exclude:
    - '*/test/*'
    - precision/*
    limit: 40
  reason: "Prüfen, ob Missing-Default und Pipeline außerhalb des Pakets existieren"
```
/answer b503-22 allow
Control Request:
```yaml
id: b503-23
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/segment/test/mapping.rs
        matches:
        - "assert_eq!(m.resolve(& hit).unwrap(), eref(5));"
        - "matches!(m.resolve(& stale), Err(SegmentError::StaleGeneration { gen : 99 }))"
        - "matches!(m.resolve(& empty), Err(SegmentError::MappingMissing { slot : 1, .. }))"
      - path: resident/segment/mapping.rs
        matches:
        - "pub fn resolve(&self, hit: &SegmentHit) -> Result<EntryRef, SegmentError> {"
        - "let Ok(entry) = source.resolve(&hit) else {"
      - path: resident/precision/mod.rs
        matches:
        - "pub use profile::{derive_profiles, Profile, ProfileConfig, ProfilingConfig};"
        - "recalibrate_in_background, resolve, state, CalibrationRecord, CalibrationStore,"
      - path: resident/precision/store.rs
        matches:
        - "/// Present for `Calibrated` and `Stale`; `Missing` means: use the most precise valid combination."
        - "pub fn resolve(store: &dyn CalibrationStore, current: &Fingerprint) -> Resolution {"
        - "tracing::warn!(model = %current.model_id, \"no calibration, using most precise combination\");"
        - pub fn recalibrate_in_background<F>(
      - path: resident/precision/test/probe.rs
        matches:
        - "use crate::resident::precision::known::{self, FP16, FP32, FUSION, INT8_DP4A, ON, OPERANDS, SCORING};"
      - path: resident/precision/test/probe_cpu.rs
        matches:
        - use crate::resident::precision::AxisId;
      - path: resident/precision/test/measure_cpu.rs
        matches:
        - "use crate::resident::precision::known::{BATCH, FP16};"
        - "use crate::resident::precision::measure::{NeverYield, Validity};"
        - use crate::resident::precision::profile::ProfileConfig;
      - path: resident/precision/test/measure_cuda.rs
        matches:
        - "use crate::resident::precision::calibrate::{CalibrationTarget, Calibrator, CalibratorConfig, Outcome};"
        - use crate::resident::precision::known::BATCH;
        - "use crate::resident::precision::measure::{NeverYield, Validity};"
        - use crate::resident::precision::profile::ProfileConfig;
        - use crate::resident::precision::CapabilitySet;
        - "fn cfg(dtype: &crate::resident::precision::ValueId, batch: &str) -> Cfg {"
        - let int8 = crate::resident::precision::known::INT8_DP4A.clone();
      - path: resident/precision/test/store.rs
        matches:
        - "let resolution = resolve(&store, &fingerprint(13040));"
        - "let resolution = resolve(&store, &fingerprint(1));"
        - "assert_eq!(resolve(&store, &fingerprint(1)).state, StoreState::Missing);"
        - "assert_eq!(resolve(&store, &fingerprint(13030)).state, StoreState::Calibrated);"
        - "let handle = recalibrate_in_background(&pool, store.clone(), || Ok(record(2)));"
        - "let handle = recalibrate_in_background(&pool, store.clone(), || {"
      - path: resident/precision/test/probe_cuda.rs
        matches:
        - "use crate::resident::precision::known::{FUSION, ON};"
        - "use crate::resident::precision::{AxisId, ValueId};"
        - use crate::resident::precision::make_probe;
      - path: resident/precision/test/profile.rs
        matches:
        - use crate::resident::precision::measure::Cfg;
        - "assert!(derive_profiles(& [], & ProfilingConfig::default()).is_empty());"
        - let p = derive_profiles(
        - "assert_eq!(derive_profiles(& front, & threshold(0.01)).len(), 4);"
        - "assert_eq!(derive_profiles(& front, & threshold(0.35)).len(), 2);"
        - "assert_eq!(derive_profiles(& front, & threshold(10.0)).len(), 1);"
        - "let p = derive_profiles(&front, &ProfilingConfig::default());"
        - "assert_eq!(names(& derive_profiles(& front, & threshold(0.05))).len(), 3);"
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer b503-23 allow
ReasoningToken: 892
Control Request:
```yaml
id: b503-24
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/README.md
  reason: Knoten-IDs der README für gezielte Bearbeitung
```
/answer b503-24 allow
Control Request:
```yaml
id: b503-25
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/README.md
      nodes:
      - id: 138002
        signature: precision
        children:
        - id: 138002.1IrqLK|qugQXC
          signature: "Capability sets, calibration,…"
        - id: 138002.c05baf
          signature: Concept
        - id: 138002.6ce6c5
          signature: Files
          children:
          - id: 138002.6ce6c5.oQbj5n|L6iDHV
            signature: "Tests: `test/` per file; CUDA…"
```
/answer b503-25 allow
ReasoningToken: 16
Control Request:
```yaml
id: b503-26
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/README.md
    id: 138002.c05baf
    source: |
      ## Concept

      - **Capabilities, not profile names.** A capability set maps open axis identifiers (operand type, accumulator, scoring type, fusion, batch, tile, threads, …) to the values a provider offers or a task requires. There is no profile enum; axes and values are extensible per hardware without touching facade or task interface. Each subengine decides its compute paths itself; there is no rounding-point or dtype contract between CPU and GPU, and results are accepted variables, not guarantees. `known.rs` lists the axes for the target hardware (GTX 1660, sm_75; bf16 deliberately absent). Numeric axes (`batch`, `tile`, `threads`) carry measured values and are not listed statically; the same holds for `density`, the index manager's gather-versus-segment load threshold (permille of a segment's bytes that must be requested for the whole segment to be loaded). A profile configuration containing `density` overrides the manager default.
      - **Probe, then calibrate.** Probing is optimistic and measures nothing; its result is intersected with the kernels that exist. Unusable combinations are removed by calibration.
      - **Calibrator.** Enumerates combinations (with pruning), measures them interleaved against the fp32 reference, rejects invalid ones (NaN/Inf, degenerate output, deviation above the configurable maximum; such a combination counts as incompatible and is merely not activated) and keeps the Pareto front over (time, deviation).
        - *Measurement:* every combination is a mini-benchmark with at least 3 repetitions. Rounds run round-robin over all active combinations, odd rounds in reverse order. Only relations count: stability is judged on the time ratio to the first combination, not on single times. Estimates are robust (median or minimum), spread is a coefficient of variation or relative range with the slowest fifth ignored.
        - *Termination:* measurement ends on stability, not after a fixed time. A hard cap (repetitions, time per combination, budget of the whole run) ends it otherwise; then the best intermediate state is kept, logged and marked `unstable`. Calibration never blocks or aborts.
        - *Load:* calibration runs on dedicated low-priority threads, not on tokio workers. As a GPU kernel cannot be preempted, a timing window is split into launch groups within a slice budget; between groups a pending production task is polled and interrupts the sample.
        - *Pruning:* axes with independent effect (`batch`, `tile`, `threads`) are measured one at a time per remaining combination, only their winners are combined and verified. A combination slower than a peer by more than a margin without being more precise is stopped early.
      - **Profiles are unnamed positions.** Their number follows from the spread of the front alone: the spread threshold in the normalised arc-length scale is the only knob; neighbours closer than it merge. Points that are not faster than a more precise peer beyond the measurement uncertainty are not activated. Each profile has a position in [0, 1] derived from arc length on the normalised front (time logarithmic): 0 fastest, 1 most precise, the rest proportional to the cumulative distance. A single profile sits at 0.5. A model without any active profile is blocked for the subengine.
      - **Matching.** The task's wish is only a wish. It is clamped to [0, 1] (NaN is an error) and mapped to the profile with minimal distance in the engine's own set; a tie goes to the more precise one. The scale is relative per subengine, and a wish is always satisfiable.
      - **Store.** Results are kept per model × subengine as a JSON file with a fingerprint of the system (engine, GPU name, compute capability and CUDA driver version, CPU features, model ID, engine version). Equal fingerprint = calibrated, different = stale (old result stays usable while a background run replaces it), no file or unreadable file = missing (warning, no profiles). Besides the profiles the record holds the `unstable` flag, the combinations rejected with reason, and the pruned or skipped ones.
      - **Measurement is an interface** with CPU, CUDA and scripted implementations; a shared synthetic workload and fp32 reference serve the real ones.
  reason: Concept-Abschnitt der README mit Design-Inhalt integriert überarbeiten
```
/answer b503-26 allow
Result Stats: id=2e14fb9c-3faf-4561-9736-41a29d527cc4, total: 80799, in: 14, out: 8806, reason: 133, read: 285577, write: 71979, start: 1793188014829, end: 1793188031233, time: 16313, total: 79906, io: 0, control: 91, sent: 0, recv: 337
ReasoningToken: 133
Ich habe die README überarbeitet. Die Abweichungen stehen unten, es gibt dafür keine Datei. Ich habe nichts ausgeführt, auch keine Tests.

## 1. README
`rag/src/resident/precision/README.md`: Nur den Abschnitt „Concept" habe ich überarbeitet und die Design-Inhalte dort in die bestehenden Punkte eingearbeitet. Die Struktur ist unverändert, es gibt keine neuen Abschnitte und keine Bewertung. Neu aufgenommen sind:
- **Capabilities:** keine Profil-Enums, kein Rundungs- oder dtype-Vertrag zwischen CPU und GPU, Ergebnisse sind keine Garantien.
- **Numerische Achsen:** `density` und sein Überschreiben des Manager-Defaults.
- **Calibrator:** Messmethodik, Abbruch, Last und Pruning.
- **Profile:** Spread-Schwelle, Position über Bogenlänge, Sperre ohne aktives Profil.
- **Store:** Inhalt des Fingerprints und des Records.

Die Dateitabelle habe ich nicht angefasst.

## 2. Deviations (English)

```
# Deviations: resident_precision.md vs. rag/src/resident/precision

## Deviations
D1 Measurement ignores sequence length. Cfg.seq_lens (CalibratorConfig.seq_lens, up to max 512) is carried but not read by CpuScoringMeasure or CudaScoringMeasure. Design: test set with length variance up to configured max sequence length (processability: memory, overflow).
D2 Measurement is not model-dependent. Workload is synthetic scoring with fixed dim 384 (WorkloadSpec); CalibrationTarget.model_id is only a label. Design: calibration per model x subengine, behaviour depends on dimension and sequence length.
D3 Several axes have no effect on measurement. `tile`, `fusion`, `accumulator` are not read by CPU/CUDA measures; `threads` is not read by CUDA measure. Probes offer them; calibration measures identical behaviour for all their values.
D4 Adaptive stability decision incomplete. Code only repeats rounds until ratio series is stable or cap is hit. Missing: (a) more samples per repetition when samples are internally noisy (window_ms fixed); (b) repeating the whole run when repetitions drift but samples are stable.
D5 Fingerprint lacks CPU model. Fields: engine, gpu_name, gpu_cc, cuda_driver_version, cpu_features (flags only), model_id, engine_version. Design requires CPU model.
D6 `unstable` is persisted only as one record-level bool (CalibrationRecord.unstable). Per-combination/per-profile unstable marking is not stored; no basis to select which results to remeasure.
D7 CPU measurement uses rayon pools (thread name `cpu-measure-*`) that are not lowered in priority. Only LowPriorityPool threads get nice 19. Design: low thread priority for calibration.

## Not mapped
N1 Missing state: no conservative default is built. resolve() returns record=None with a warning; nothing derives "most precise valid combination" from capabilities/kernels.
N2 No explicit `calibrate` command exists in the package or elsewhere in rag/src.
N3 No orchestration from probe -> Calibrator.run -> derive_profiles -> CalibrationRecord::from_report -> store. Nothing outside the package calls resolve, recalibrate_in_background or derive_profiles.
N4 Lazy model lock is only reachable via ProfileSelector with an empty profile list (NoActiveProfile). No path marks a subengine for a model after an Incompatible/CalibrationUnstable error at first task; PrecisionError::Incompatible/CalibrationUnstable are not returned to waiting tasks (CalibrationUnstable is only logged).
N5 Pull mechanism (claim predicates: priority, model/index affinity, model availability; no profile predicate; no calibration throughput for load distribution) is not in this package; no counterpart found.
```

Die Punkte N3 bis N5 hängen davon ab, wie das Paket außerhalb eingebunden wird. Ich habe nur per Suche in `rag/src` geprüft, ob es Aufrufer gibt, und keine gefunden. Ob die Anbindung bewusst noch fehlt oder in einem anderen Paket liegt, kann ich nicht beurteilen. Das Design habe ich nicht verändert.
