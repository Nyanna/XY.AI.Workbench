Setze B6 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben

# Umsetzungsplan B6 · CapabilitySet & Profile-Typen

| | |
|---|---|
| **Paket** | B6 (Gruppe B) |
| **Abhängt von** | A4 |
| **Blockiert** | C3 (`requires()`-Caps), F1 (Capability-Erhebung), F2/F3 (Calibrator/Profilbildung), F4 (Matching), F5 (Store) |
| **Ergebnis** | Präzisions-Datentypen: `CapabilitySet`, `Profile{position:f32, config}`, **keine** festen Profilnamen/-Enums |
| **Quellen** | `../resident_precision.md` (Capability-Listen, Profilbildung, Matching), `../context/context_b.md` §B6 (24–26) |

---

## 1. Zweck & Abgrenzung

B6 liefert die **Datentypen** für Hardware-Fähigkeiten und abgeleitete Profile. Leitprinzip (`resident_precision.md`): **keine vordefinierten Profilnamen, kein fest codiertes Profil-Enum.** Achsen und Werte sind **pro Hardware erweiterbar, ohne Änderung an Fassade oder Task-Schnittstelle**. Profile sind **unbenannte Indizes** mit Position `p ∈ [0,1]`.

**Nicht Teil von B6:** Erhebung (F1), Messung/Pareto/Pruning/Stabilität (F2), Positions-Berechnung aus der Pareto-Front (F3), Task-Matching `|t−p|` (F4), Persistenz/Fingerprint (F5). B6 definiert nur die **Typen + algebraischen Operationen** (`satisfies`), auf die F aufsetzt. Die Pareto-/Bogenlängen-Logik wird als **Helfer** bereitgestellt (von F3 genutzt), aber der Kalibrierungsablauf gehört zu F.

---

## 2. Zu implementierende Typen

### 2.1 Achsen & Werte (erweiterbar, string-basiert, interniert möglich)

```rust
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)] pub struct AxisId(Arc<str>);  // oder interniert als u32
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)] pub struct ValueId(Arc<str>);
```
- Neue Achsen/Werte = neue Strings → **keine** Änderung an Fassade/Task. Typsicherheit optional per Newtype-Konstanten über `LazyLock` oder Registry mit Validierung.
- `enum_dispatch` o. Ä. ist **ungeeignet** (löst nur geschlossene Varianten) — bewusst nicht verwenden.

### 2.2 CapabilitySet (Achsen → Wertemengen)

```rust
#[derive(Clone, Default)]
pub struct CapabilitySet(BTreeMap<AxisId, BTreeSet<ValueId>>);

impl CapabilitySet {
    pub fn satisfies(&self, req: &CapabilitySet) -> bool {       // req ⊆ self, achsenweise
        req.0.iter().all(|(a, vs)| self.0.get(a).map_or(false, |have| vs.is_subset(have)))
    }
}
```
- `BTreeMap`/`BTreeSet` ⇒ **stabile Ordnung** → reproduzierbare Serialisierung/Hashes/Profile (wichtig für F5-Fingerprint).
- `satisfies` ist die Grundlage für C3/`requires()` (B3): `Requirements.caps` wird gegen das Engine-`CapabilitySet` gematcht.

### 2.3 Profile (unbenannter Index + Position)

```rust
pub struct ProfileConfig(BTreeMap<AxisId, ValueId>);  // genau EIN Wert pro gewählter Achse
pub struct Profile {
    pub position: f32,       // p ∈ [0,1]: 0 = fast, 1 = precision; Einzelprofil = 0.5 (Konvention)
    pub config:   ProfileConfig,
}
```
- **Keine Namen**, nur Index + `position`. Reihenfolge/Herabstufung ergibt sich aus der Messung (F3): langsamer-und-ungenauer (dominierte/Emulations-Pfade) werden herabgestuft oder nicht aktiviert.
- Eine Engine **ohne aktives Profil** für ein Modell ist für dieses Modell gesperrt (Status/Logik in F, B6 stellt nur den Typ).

### 2.4 Pareto-/Positions-Helfer (von F3 genutzt)

```rust
// Front: minimiere (Zeit t, Abweichung d). Nach t aufsteigend, nur strikt fallendes d behalten. O(n log n).
fn pareto(mut pts: Vec<(f64 /*t*/, f64 /*d*/)>) -> Vec<(f64, f64)> {
    pts.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    let mut out: Vec<(f64, f64)> = Vec::new();
    for p in pts { if out.last().map_or(true, |l| p.1 < l.1) { out.push(p); } }
    out
}
// Position: Achsen min-max normieren (Zeit ggf. logarithmisch), kumulierte Bogenlänge s_i; p = s/S_total.
// Entartung: 1 Punkt → p = 0.5 (Konvention der Spec); konstante Achse → Division durch 0 abfangen.
```
- Eigenimplementierung (~50 Zeilen, `f64::total_cmp`); keine ungeprüfte Crate. **Spread-Schwellwert** (F3) ist die **einzige** Konfiguration, die die Profilanzahl bestimmt — in der **normierten Skala** definiert (modell-/hardwareunabhängig).

---

## 3. Well-known Achsen/Werte für Zielhardware (GTX 1660, TU116, sm75)

Als `LazyLock`-Konstanten bereitstellen (F1 erhebt die realen Werte; B6 liefert die Bezeichner, damit Typen/Tests stabil sind):

| Achse (GPU) | Werte | Hinweis (context_b §B6.24) |
|---|---|---|
| `operands` | `fp32`, `fp16` | **keine** Tensor Cores; FP16 ~2× FP32-Rate auf CUDA-Cores |
| `accumulator` | `fp32`, `fp16` | |
| `scoring` | `fp32`, `fp16`, `int8-dp4a` | `dp4a` ab sm61 vorhanden; **keine** int8-Tensor-Cores |
| `fusion` | `an`, `aus` | |
| `batch`, `tile` | numerisch | unabhängig messen, nur Gewinner kombinieren |

**`bf16` NICHT anbieten** (bf16-Rechnen erst ab sm80; auf TU116 nicht nativ → f16 wählen). fp8/fp4 nicht nutzbar.

| Achse (CPU) | Werte |
|---|---|
| `operands` | `fp32`, `fp16→fp32` |
| `scoring` | `fp32`, `int8` |
| `threads`, `tile` | numerisch |

> `// TODO(A3): candle Quantkernel/dp4a-Nutzung und bf16-Verhalten auf sm75 praktisch testen (context_b offener Punkt 4).`

---

## 4. Fehler & Logging

- Q1: `Incompatible` (Kombination über Maximalabweichung/degeneriert — aktiviert in F2), `NoActiveProfile` (F3/F4), `InvalidPrecision` (NaN — F4 beim Matching). B6 **definiert** die Varianten/Signaturen, die F benutzt; die Laufzeit-Logik liegt in F.
- Q2: `tracing` — abgeleitete Profile (Anzahl, Positionen) auf Debug (gefüllt in F3).

## 5. Teststrategie (Q3, ohne GPU)

- **satisfies:** `req ⊆ self` achsenweise true/false; fehlende Achse ⇒ false; leeres `req` ⇒ true.
- **Stabile Ordnung:** gleiche Mengen ⇒ identische Serialisierung/Hash (BTreeMap-Determinismus).
- **pareto():** dominierte Punkte verworfen; sortierte Front strikt fallendes `d`.
- **Position:** Bogenlänge korrekt; Einzelprofil ⇒ `p = 0.5`; konstante Achse ⇒ keine Division durch 0; schnellstes `p=0`, genauestes `p=1`.
- **Erweiterbarkeit:** neue Achse/neuer Wert ohne Signaturänderung aufnehmbar (Typ-Test).

*(Vollständige Matching-/Stabilitäts-/Persistenz-Tests sind F2–F5.)*

## 6. Offene Punkte (→ A3 / F)

1. `AxisId`/`ValueId` als `Arc<str>` vs. interniert (`u32`-Registry) — Performance/Ergonomie.
2. Zeit-Normierung (logarithmisch?) und Spread-Schwellwert-Default (gehört inhaltlich zu F3, hier nur Typ/Helfer).
3. candle dp4a/bf16 auf sm75 (A3-Prototyp).

## 7. Definition of Done

- `AxisId`, `ValueId`, `CapabilitySet` (+`satisfies`), `ProfileConfig`, `Profile{position, config}` implementiert.
- Pareto-/Positions-Helfer vorhanden (für F3), reine Logik, keine ungeprüfte Crate.
- Well-known sm75-Achsen/Werte als `LazyLock`-Konstanten (ohne `bf16`-Rechenpfad).
- Keine festen Profilnamen/-Enums; stabile (BTreeMap-)Serialisierung; baut ohne `cuda`/`candle`.

# Umsetzung

SystemInit:  id=d5903832-a956-43fc-a385-bedf7064c619, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 66
Control Request:
```yaml
id: 78c8-40
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
  - path: /home/user/xyan/xy.ai.workbench/rag
    recursive: false
  reason: Inspect existing resident module layout and crate root to place B6 types
```
/answer 78c8-40 allow
Control Request:
```yaml
id: 78c8-41
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
/answer 78c8-41 allow
Control Request:
```yaml
id: 78c8-42
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
    ids: []
  reason: Read conventions
```
/answer 78c8-42 allow
Control Request:
```yaml
id: 78c8-43
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/docs/architecture/conventions.md
      nodes:
      - id: 40b374
        signature: "Cross-Cutting Conventions (A4): Errors, Logging, Testing, Feature Flags"
        children:
        - id: 40b374.1ab48d
          signature: Q1 — Error Taxonomy (`thiserror`)
        - id: 40b374.90377b
          signature: Q2 — Logging (`tracing`)
        - id: 40b374.efc803
          signature: Q3 — Test Strategy & Mocks
        - id: 40b374.ace7f8
          signature: Q4 — Feature Flags
        - id: 40b374.73ab5c
          signature: Basis Scaffolding
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 78c8-43 allow
Control Request:
```yaml
id: 78c8-44
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: \.rs$
  reason: List resident module files
```
/answer 78c8-44 allow
Control Request:
```yaml
id: 78c8-45
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
      - memory.rs
      - mod.rs
      - stamp.rs
      - "./kernel:"
      - cuda.rs
      - mock.rs
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
      - lease.rs
      - mod.rs
      - requirements.rs
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
/answer 78c8-45 allow
Control Request:
```yaml
id: 78c8-46
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: mod |Error|thiserror
    include:
    - lib.rs
    - mod.rs
    - error.rs
    limit: 50
  reason: See module declarations and error conventions
```
/answer 78c8-46 allow
Control Request:
```yaml
id: 78c8-47
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
          match: pub mod telemetry;
        - lineno: 6
          match: pub mod mock;
        - lineno: 7
          match: pub mod queue;
        - lineno: 8
          match: pub mod memory;
        - lineno: 9
          match: pub mod op;
        - lineno: 10
          match: pub mod index;
        - lineno: 11
          match: pub mod kernel;
        - lineno: 12
          match: pub mod precision;
        - lineno: 13
          match: pub mod facade;
        - lineno: 14
          match: pub mod processor;
        - lineno: 15
          match: pub mod planner;
        - lineno: 16
          match: pub mod subengine;
        - lineno: 17
          match: pub mod manager;
      - path: resident/index/mod.rs
        matches:
        - lineno: 9
          match: pub mod memory;
        - lineno: 10
          match: pub mod stamp;
      - path: resident/queue/mod.rs
        matches:
        - lineno: 12
          match: mod node;
        - lineno: 13
          match: mod queue;
        - lineno: 14
          match: mod state;
        - lineno: 15
          match: pub mod task;
      - path: resident/kernel/mod.rs
        matches:
        - lineno: 7
          match: mod cuda;
        - lineno: 12
          match: pub mod mock;
        - lineno: 16
          match: use crate::resident::error::IndexError;
        - lineno: 79
          match: "fn begin(&self, batch: &BatchDesc) -> Result<BatchId, IndexError>;"
        - lineno: 80
          match: "fn run(&self, batch: BatchId, input: &[RegionSpan]) -> Result<KernelOutput, IndexError>;"
        - lineno: 83
          match: "fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, IndexError>;"
        - lineno: 103
          match: "mod tests {"
        - lineno: 184
          match: "assert!(matches!(k.run(b, &spans(1)), Err(IndexError::KernelFailed { .. })));"
        - lineno: 187
          match: "assert!(matches!(k.end(b), Err(IndexError::UnknownBatch { .. })));"
      - path: resident/error.rs
        matches:
        - lineno: 1
          match: //! Error taxonomy.
        - lineno: 6
          match: use thiserror::Error;
        - lineno: 12
          match: "#[derive(Error, Debug)]"
        - lineno: 14
          match: "pub enum IndexError {"
        - lineno: 28
          match: "impl IsRetryable for IndexError {"
        - lineno: 31
          match: "self, IndexError::IndexUnavailable { .. } | IndexError::ArenaExhausted { .. }"
        - lineno: 37
          match: "#[derive(Error, Debug)]"
        - lineno: 39
          match: "pub(crate) enum ManagerError {"
        - lineno: 45
          match: "impl IsRetryable for ManagerError {"
        - lineno: 48
          match: "self, ManagerError::StaleMapping { .. } | ManagerError::StaleRegion { .. }"
        - lineno: 53
          match: "#[derive(Error, Debug)]"
        - lineno: 55
          match: "pub enum PrecisionError {"
        - lineno: 65
          match: "impl IsRetryable for PrecisionError {"
        - lineno: 67
          match: "matches!(self, PrecisionError::CalibrationUnstable { .. })"
        - lineno: 71
          match: "#[derive(Error, Debug)]"
        - lineno: 73
          match: "pub enum PlannerError {"
        - lineno: 81
          match: "impl IsRetryable for PlannerError {"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 78c8-47 allow
Control Request:
```yaml
id: 78c8-48
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
    ids: []
  reason: "Read existing precision module, errors, requirements to integrate"
```
/answer 78c8-48 allow
Control Request:
```yaml
id: 78c8-49
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! CapabilitySet/Profile.
      - id: mUxD4y|OSUPt5
        signature: use std::collections::BTreeSet;
      - id: CapabilitySet
        signature: /// Named capabilities; an empty set is satisfied by any provider.
        docstring: Named capabilities; an empty set is satisfied by any provider.
      - id: impl_CapabilitySet
        signature: "impl CapabilitySet {"
        children:
        - id: impl_CapabilitySet.new
          signature: "pub fn new() -> Self {"
        - id: impl_CapabilitySet.with
          signature: "pub fn with(mut self, cap: impl Into<String>) -> Self {"
        - id: impl_CapabilitySet.contains
          signature: "pub fn contains(&self, cap: &str) -> bool {"
        - id: impl_CapabilitySet.satisfies
          signature: "/// `self` (required) is a subset of `available`. pub fn satisfies(&self, avail…"
          docstring: '`self` (required) is a subset of `available`.'
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
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
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
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
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 78c8-49 allow
Control Request:
```yaml
id: 78c8-4a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    ids:
    - CapabilitySet
    - impl_CapabilitySet
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - PrecisionError
    - CapabilityError
  reason: Read stub and error variants
```
/answer 78c8-4a allow
ReasoningToken: 41
Control Request:
```yaml
id: 78c8-4b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: CapabilitySet
        type: ItemStruct
        code: |
          /// Named capabilities; an empty set is satisfied by any provider.
          #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
          pub struct CapabilitySet(BTreeSet<String>);
      - id: impl_CapabilitySet
        type: ItemImpl
        signature: "impl CapabilitySet {"
        children:
        - id: impl_CapabilitySet.new
          type: ImplItemFn
          code: |2
                pub fn new() -> Self {
                    Self::default()
                }
        - id: impl_CapabilitySet.with
          type: ImplItemFn
          code: |2
                pub fn with(mut self, cap: impl Into<String>) -> Self {
                    self.0.insert(cap.into());
                    self
                }
        - id: impl_CapabilitySet.contains
          type: ImplItemFn
          code: |2
                pub fn contains(&self, cap: &str) -> bool {
                    self.0.contains(cap)
                }
        - id: impl_CapabilitySet.satisfies
          type: ImplItemFn
          code: |2
                /// `self` (required) is a subset of `available`.
                pub fn satisfies(&self, available: &CapabilitySet) -> bool {
                    self.0.is_subset(&available.0)
                }
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
```
/answer 78c8-4b allow
Control Request:
```yaml
id: 78c8-4c
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    - /home/user/xyan/xy.ai.workbench/rag/tests
    - /home/user/xyan/xy.ai.workbench/rag/benches
    pattern: CapabilitySet|\.with\("|caps
    include:
    - '*.rs'
    limit: 50
  reason: Find usages of the existing stub API to keep compatibility
```
/answer 78c8-4c allow
Control Request:
```yaml
id: 78c8-4d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      - /home/user/xyan/xy.ai.workbench/rag/tests
      - /home/user/xyan/xy.ai.workbench/rag/benches
      matches:
      - path: resident/precision/mod.rs
        matches:
        - lineno: 1
          match: //! CapabilitySet/Profile.
        - lineno: 5
          match: pub struct CapabilitySet(BTreeSet<String>);
        - lineno: 6
          match: "impl CapabilitySet {"
        - lineno: 18
          match: "pub fn satisfies(&self, available: &CapabilitySet) -> bool {"
      - path: resident/op/requirements.rs
        matches:
        - lineno: 4
          match: use crate::resident::precision::CapabilitySet;
        - lineno: 60
          match: "pub caps: CapabilitySet,"
        - lineno: 71
          match: "caps: CapabilitySet::new(),"
        - lineno: 86
          match: "pub fn with_caps(mut self, caps: CapabilitySet) -> Self {"
        - lineno: 87
          match: self.caps = caps;
        - lineno: 97
          match: "pub fn satisfied_by(&self, dev: DeviceKind, available: &CapabilitySet) -> bool {"
        - lineno: 98
          match: self.engine.accepts(dev) && self.caps.satisfies(available)
        - lineno: 107
          match: "fn engine_and_caps_matching() {"
        - lineno: 109
          match: .with_caps(CapabilitySet::new().with("f16"));
        - lineno: 110
          match: let have = CapabilitySet::new().with("f16").with("graph");
        - lineno: 113
          match: "assert!(!req.satisfied_by(DeviceKind::Gpu, &CapabilitySet::new()));"
        - lineno: 117
          match: "fn either_accepts_both_and_empty_caps_pass() {"
        - lineno: 119
          match: let none = CapabilitySet::new();
        - lineno: 128
          match: "assert!(req.satisfied_by(DeviceKind::Cpu, &CapabilitySet::new()));"
```
/answer 78c8-4d allow
ReasoningToken: 326
Control Request:
```yaml
id: 78c8-4e
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "edition|rust-version|serde|^\\["
    include:
    - Cargo.toml
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: precision::|allow\(|cfg\(feature
    include:
    - mod.rs
    limit: 30
  reason: Check edition/MSRV for LazyLock and serde availability
```
/answer 78c8-4e allow
Control Request:
```yaml
id: 78c8-4f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: .xyrag/layers/trigram/mirror/Cargo.toml
        matches:
        - lineno: 4
          match: edition 2021
        - lineno: 20
          match: serde version 1 features derive
        - lineno: 21
          match: serde json 1
        - lineno: 22
          match: serde yaml 0 9
      - path: Cargo.toml
        matches:
        - lineno: 1
          match: "[package]"
        - lineno: 4
          match: edition = "2021"
        - lineno: 7
          match: "[lib]"
        - lineno: 11
          match: "[[bin]]"
        - lineno: 15
          match: "[dependencies]"
        - lineno: 20
          match: "serde = { version = \"1\", features = [\"derive\"] }"
        - lineno: 21
          match: serde_json = "1"
        - lineno: 22
          match: serde_yaml = "0.9"
        - lineno: 51
          match: "[features]"
        - lineno: 58
          match: "[dev-dependencies]"
        - lineno: 67
          match: "[[bench]]"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: kernel/mod.rs
        matches:
        - lineno: 6
          match: "#[cfg(feature = \"cuda\")]"
        - lineno: 8
          match: "#[cfg(feature = \"cuda\")]"
      - path: subengine/mod.rs
        matches:
        - lineno: 11
          match: "#[cfg(feature = \"cuda\")]"
        - lineno: 13
          match: "#[cfg(feature = \"cuda\")]"
      - path: op/mod.rs
        matches:
        - lineno: 250
          match: "#[allow(unreachable_patterns)]"
```
/answer 78c8-4f allow
Claude Code CLI error
ReasoningToken: 2131
Control Request:
```yaml
id: 78c8-50
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/axis.rs
    overwrite: false
    source: |
      //! Extensible axis and value identifiers.

      use std::fmt;
      use std::sync::Arc;

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

      string_id!(
          /// Capability axis such as `operands` or `scoring`; any string is valid.
          AxisId
      );
      string_id!(
          /// A value on an axis such as `fp16`; any string is valid.
          ValueId
      );

      impl ValueId {
          pub fn numeric(n: u32) -> Self {
              Self::new(n.to_string())
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          #[test]
          fn ids_compare_by_content() {
              assert_eq!(AxisId::new("operands"), AxisId::from("operands"));
              assert!(AxisId::new("a") < AxisId::new("b"));
              assert_eq!(ValueId::numeric(64).as_str(), "64");
          }

          #[test]
          fn display_is_plain() {
              assert_eq!(ValueId::new("fp16").to_string(), "fp16");
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/capability.rs
    overwrite: false
    source: |
      //! Capability sets: axes mapped to the values a provider offers or a task requires.

      use std::collections::{BTreeMap, BTreeSet};

      use super::axis::{AxisId, ValueId};

      /// Ordered containers keep iteration, hashing and serialization reproducible.
      /// Axes never map to an empty value set.
      #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
      pub struct CapabilitySet(BTreeMap<AxisId, BTreeSet<ValueId>>);

      impl CapabilitySet {
          pub fn new() -> Self {
              Self::default()
          }

          pub fn with(mut self, axis: impl Into<AxisId>, value: impl Into<ValueId>) -> Self {
              self.insert(axis, value);
              self
          }

          pub fn insert(&mut self, axis: impl Into<AxisId>, value: impl Into<ValueId>) -> bool {
              self.0.entry(axis.into()).or_default().insert(value.into())
          }

          pub fn contains(&self, axis: &AxisId, value: &ValueId) -> bool {
              self.0.get(axis).is_some_and(|vs| vs.contains(value))
          }

          pub fn values(&self, axis: &AxisId) -> Option<&BTreeSet<ValueId>> {
              self.0.get(axis)
          }

          pub fn axes(&self) -> impl Iterator<Item = &AxisId> {
              self.0.keys()
          }

          pub fn iter(&self) -> impl Iterator<Item = (&AxisId, &BTreeSet<ValueId>)> {
              self.0.iter()
          }

          pub fn is_empty(&self) -> bool {
              self.0.is_empty()
          }

          /// Every required value on every required axis is present in `self`.
          pub fn satisfies(&self, req: &CapabilitySet) -> bool {
              req.0
                  .iter()
                  .all(|(axis, wanted)| self.0.get(axis).is_some_and(|have| wanted.is_subset(have)))
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;
          use std::collections::hash_map::DefaultHasher;
          use std::hash::{Hash, Hasher};

          fn hash_of(c: &CapabilitySet) -> u64 {
              let mut h = DefaultHasher::new();
              c.hash(&mut h);
              h.finish()
          }

          #[test]
          fn satisfies_is_axiswise_subset() {
              let have = CapabilitySet::new()
                  .with("operands", "fp32")
                  .with("operands", "fp16")
                  .with("fusion", "on");
              assert!(have.satisfies(&CapabilitySet::new().with("operands", "fp16")));
              assert!(have.satisfies(
                  &CapabilitySet::new()
                      .with("operands", "fp16")
                      .with("operands", "fp32")
                      .with("fusion", "on")
              ));
              assert!(!have.satisfies(&CapabilitySet::new().with("operands", "bf16")));
              assert!(!have.satisfies(
                  &CapabilitySet::new()
                      .with("operands", "fp16")
                      .with("operands", "bf16")
              ));
          }

          #[test]
          fn missing_axis_fails_and_empty_requirement_passes() {
              let have = CapabilitySet::new().with("operands", "fp16");
              assert!(!have.satisfies(&CapabilitySet::new().with("scoring", "fp16")));
              assert!(have.satisfies(&CapabilitySet::new()));
              assert!(CapabilitySet::new().satisfies(&CapabilitySet::new()));
              assert!(!CapabilitySet::new().satisfies(&have));
          }

          #[test]
          fn insertion_order_does_not_affect_identity() {
              let a = CapabilitySet::new()
                  .with("scoring", "fp16")
                  .with("operands", "fp32")
                  .with("operands", "fp16");
              let b = CapabilitySet::new()
                  .with("operands", "fp16")
                  .with("operands", "fp32")
                  .with("scoring", "fp16");
              assert_eq!(a, b);
              assert_eq!(hash_of(&a), hash_of(&b));
              assert_eq!(format!("{a:?}"), format!("{b:?}"));
              assert_eq!(a.axes().map(AxisId::as_str).collect::<Vec<_>>(), ["operands", "scoring"]);
          }

          #[test]
          fn unknown_axes_and_values_are_accepted() {
              let c = CapabilitySet::new().with("future-axis", "future-value");
              assert!(c.contains(&AxisId::new("future-axis"), &ValueId::new("future-value")));
              assert!(c.satisfies(&c.clone()));
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
    overwrite: false
    source: |
      //! Unnamed profiles: a position on the fast-to-precise scale plus a configuration.

      use std::collections::BTreeMap;

      use super::axis::{AxisId, ValueId};
      use crate::resident::error::PrecisionError;

      /// Exactly one value per chosen axis.
      #[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
      pub struct ProfileConfig(BTreeMap<AxisId, ValueId>);

      impl ProfileConfig {
          pub fn new() -> Self {
              Self::default()
          }

          pub fn with(mut self, axis: impl Into<AxisId>, value: impl Into<ValueId>) -> Self {
              self.0.insert(axis.into(), value.into());
              self
          }

          pub fn get(&self, axis: &AxisId) -> Option<&ValueId> {
              self.0.get(axis)
          }

          pub fn iter(&self) -> impl Iterator<Item = (&AxisId, &ValueId)> {
              self.0.iter()
          }

          pub fn len(&self) -> usize {
              self.0.len()
          }

          pub fn is_empty(&self) -> bool {
              self.0.is_empty()
          }
      }

      #[derive(Clone, Debug, PartialEq)]
      pub struct Profile {
          /// 0 is fastest, 1 is most precise.
          pub position: f32,
          pub config: ProfileConfig,
      }

      impl Profile {
          /// Position of a lone profile.
          pub const SINGLE_POSITION: f32 = 0.5;

          pub fn new(position: f32, config: ProfileConfig) -> Result<Self, PrecisionError> {
              if !(0.0..=1.0).contains(&position) {
                  return Err(PrecisionError::InvalidPrecision { requested: position });
              }
              Ok(Self { position, config })
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          #[test]
          fn position_must_be_in_unit_interval() {
              for p in [0.0, 0.5, 1.0] {
                  assert!(Profile::new(p, ProfileConfig::new()).is_ok());
              }
              for p in [-0.1, 1.1, f32::NAN, f32::INFINITY] {
                  assert!(matches!(
                      Profile::new(p, ProfileConfig::new()),
                      Err(PrecisionError::InvalidPrecision { .. })
                  ));
              }
          }

          #[test]
          fn config_holds_one_value_per_axis() {
              let cfg = ProfileConfig::new()
                  .with("operands", "fp16")
                  .with("operands", "fp32")
                  .with("fusion", "on");
              assert_eq!(cfg.len(), 2);
              assert_eq!(cfg.get(&AxisId::new("operands")), Some(&ValueId::new("fp32")));
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/pareto.rs
    overwrite: false
    source: |
      //! Pareto front and arc-length positions over (time, deviation) measurements.

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

      fn normalize(v: &[f64]) -> Vec<f64> {
          let min = v.iter().copied().fold(f64::INFINITY, f64::min);
          let max = v.iter().copied().fold(f64::NEG_INFINITY, f64::max);
          let range = max - min;
          if range > 0.0 {
              v.iter().map(|x| (x - min) / range).collect()
          } else {
              vec![0.0; v.len()]
          }
      }

      #[cfg(test)]
      mod tests {
          use super::*;

          #[test]
          fn dominated_points_are_dropped() {
              let front = pareto(vec![(3.0, 0.1), (1.0, 0.9), (2.0, 0.95), (2.0, 0.5), (4.0, 0.1)]);
              assert_eq!(front, vec![(1.0, 0.9), (2.0, 0.5), (3.0, 0.1)]);
              assert!(front.windows(2).all(|w| w[0].0 < w[1].0 && w[0].1 > w[1].1));
          }

          #[test]
          fn non_finite_points_are_ignored() {
              let front = pareto(vec![(f64::NAN, 0.1), (1.0, f64::INFINITY), (2.0, 0.3)]);
              assert_eq!(front, vec![(2.0, 0.3)]);
              assert!(pareto(Vec::new()).is_empty());
          }

          #[test]
          fn positions_follow_arc_length() {
              let p = positions(&[(0.0, 1.0), (1.0, 0.0), (2.0, -1.0)], false);
              assert_eq!(p.len(), 3);
              assert_eq!(p[0], 0.0);
              assert!((p[1] - 0.5).abs() < 1e-12);
              assert_eq!(p[2], 1.0);

              let skewed = positions(&[(0.0, 1.0), (1.0, 0.9), (10.0, 0.0)], false);
              assert!(skewed[1] < 0.5);
          }

          #[test]
          fn single_point_sits_in_the_middle() {
              assert_eq!(positions(&[(1.0, 1.0)], false), vec![0.5]);
              assert!(positions(&[], false).is_empty());
          }

          #[test]
          fn constant_axis_does_not_divide_by_zero() {
              let p = positions(&[(1.0, 0.5), (2.0, 0.4), (3.0, 0.3)], false);
              assert!(p.iter().all(|x| x.is_finite()));
              assert_eq!(p[0], 0.0);
              assert_eq!(*p.last().unwrap(), 1.0);

              let flat = positions(&[(1.0, 1.0), (1.0, 1.0)], false);
              assert_eq!(flat, vec![0.0, 1.0]);
          }

          #[test]
          fn log_time_compresses_large_ratios() {
              let front = [(1.0, 1.0), (10.0, 0.5), (1000.0, 0.0)];
              let lin = positions(&front, false);
              let log = positions(&front, true);
              assert!(log[1] > lin[1]);
              let non_positive = positions(&[(0.0, 1.0), (1.0, 0.0)], true);
              assert_eq!(non_positive, positions(&[(0.0, 1.0), (1.0, 0.0)], false));
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
    overwrite: false
    source: |
      //! Well-known axes and values for the target hardware (GTX 1660, sm_75).
      //! bf16 is deliberately absent: it has no native path before sm_80.

      use std::sync::LazyLock;

      use super::axis::{AxisId, ValueId};
      use super::capability::CapabilitySet;

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
          INT8_DP4A = "int8-dp4a",
          ON = "on",
          OFF = "off",
      );

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

      #[cfg(test)]
      mod tests {
          use super::*;

          #[test]
          fn gpu_offers_no_bf16() {
              let gpu = gpu_sm75();
              assert!(gpu.contains(&OPERANDS, &FP16));
              assert!(gpu.contains(&SCORING, &INT8_DP4A));
              assert!(!gpu.contains(&OPERANDS, &ValueId::new("bf16")));
              assert!(!gpu.contains(&ACCUMULATOR, &ValueId::new("bf16")));
          }

          #[test]
          fn gpu_satisfies_cpu_independent_requirements() {
              let req = CapabilitySet::new().with(&*OPERANDS, &*FP16).with(&*FUSION, &*ON);
              assert!(gpu_sm75().satisfies(&req));
              assert!(!cpu().satisfies(&req));
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
    overwrite: true
    source: |
      //! CapabilitySet/Profile.

      mod axis;
      mod capability;
      pub mod known;
      mod pareto;
      mod profile;

      pub use axis::{AxisId, ValueId};
      pub use capability::CapabilitySet;
      pub use pareto::{pareto, positions};
      pub use profile::{Profile, ProfileConfig};
  reason: "Implement B6 types, helpers and well-known constants as submodules of precision"
```
/answer 78c8-50 allow
Control Request:
```yaml
id: 78c8-51
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
    ids:
    - impl_Requirements.satisfied_by
    - tests.engine_and_caps_matching
    - tests.either_accepts_both_and_empty_caps_pass
  reason: Adapt callers to new satisfies direction and API
```
/answer 78c8-51 allow
Control Request:
```yaml
id: 78c8-52
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
      nodes:
      - id: impl_Requirements.satisfied_by
        type: ImplItemFn
        code: |2
              /// Affinity is deliberately ignored.
              pub fn satisfied_by(&self, dev: DeviceKind, available: &CapabilitySet) -> bool {
                  self.engine.accepts(dev) && self.caps.satisfies(available)
              }
      - id: tests.engine_and_caps_matching
        type: ItemFn
        code: |2
              #[test]
              fn engine_and_caps_matching() {
                  let req = Requirements::new(ResourceKind::Encoder, EngineSelector::Gpu)
                      .with_caps(CapabilitySet::new().with("f16"));
                  let have = CapabilitySet::new().with("f16").with("graph");
                  assert!(req.satisfied_by(DeviceKind::Gpu, &have));
                  assert!(!req.satisfied_by(DeviceKind::Cpu, &have));
                  assert!(!req.satisfied_by(DeviceKind::Gpu, &CapabilitySet::new()));
              }
      - id: tests.either_accepts_both_and_empty_caps_pass
        type: ItemFn
        code: |2
              #[test]
              fn either_accepts_both_and_empty_caps_pass() {
                  let req = Requirements::new(ResourceKind::Tokenizer, EngineSelector::Either);
                  let none = CapabilitySet::new();
                  assert!(req.satisfied_by(DeviceKind::Cpu, &none));
                  assert!(req.satisfied_by(DeviceKind::Gpu, &none));
              }
```
/answer 78c8-52 allow
ReasoningToken: 243
Control Request:
```yaml
id: 78c8-53
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
    id: impl_Requirements.satisfied_by
    old_text: self.caps.satisfies(available)
    new_text: available.satisfies(&self.caps)
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
    id: tests.engine_and_caps_matching
    old_text: .with_caps(CapabilitySet::new().with("f16"));
    new_text: ".with_caps(CapabilitySet::new().with(\"operands\", \"fp16\"));"
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
    id: tests.engine_and_caps_matching
    old_text: CapabilitySet::new().with("f16").with("graph");
    new_text: |-
      CapabilitySet::new()
                  .with("operands", "fp16")
                  .with("fusion", "on");
  reason: Adapt Requirements to the axis-based CapabilitySet and satisfies direction
```
/answer 78c8-53 allow
Control Request:
```yaml
id: 78c8-54
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/axis.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/capability.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/profile.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/pareto.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/known.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
  reason: Syntax check of changed files
```
/answer 78c8-54 allow
Result Stats: id=d5903832-a956-43fc-a385-bedf7064c619, total: 50820, in: 18, out: 14505, reason: 0, read: 178138, write: 36297, start: 1792312896817, end: 1792312906277, time: 9388, total: 43241, io: 0, control: 72, sent: 0, recv: 242
B6 ist umgesetzt. Ich konnte aber nur die Syntax prüfen, nicht kompilieren. `cargo build` und `cargo test` habe ich nicht ausgeführt, und ich habe keinen Zugriff auf eine Shell.

Der neue Code liegt in `rag/src/resident/precision/`:

- **`axis.rs`:** `AxisId` und `ValueId` sind Strings auf `Arc<str>`. Neue Achsen und Werte brauchen keine Typänderung. `ValueId::numeric(n)` erzeugt numerische Werte.
- **`capability.rs`:** `CapabilitySet` bildet Achsen auf Wertemengen ab (`BTreeMap` und `BTreeSet`), die Ordnung ist also stabil. `satisfies` prüft achsenweise, ob `req` in `self` enthalten ist. Eine fehlende Achse ergibt `false`, ein leeres `req` ergibt `true`.
- **`profile.rs`:** `ProfileConfig` hält genau einen Wert pro Achse. `Profile { position, config }` hat keine Namen und kein Enum. `Profile::new` lehnt Positionen außerhalb von [0,1] sowie NaN mit `PrecisionError::InvalidPrecision` ab. `SINGLE_POSITION` ist 0.5.
- **`pareto.rs`:** `pareto()` entfernt nicht endliche Punkte und liefert eine Front mit strikt fallender Abweichung. `positions()` berechnet die Position über die Bogenlänge auf min-max-normierten Achsen, optional mit logarithmischer Zeit. Ein einzelner Punkt ergibt 0.5, und eine konstante Achse führt nicht zu einer Division durch 0.
- **`known.rs`:** Achsen und Werte für sm75 als `LazyLock`-Konstanten. Es gibt `gpu_sm75()` und `cpu()` als Obergrenze der Fähigkeiten, aber kein `bf16`. Die numerischen Achsen `batch`, `tile` und `threads` sind als Bezeichner da, tauchen aber nicht in den Fähigkeitsmengen auf.
- **Tests:** Jede Datei hat Unit-Tests zu den Punkten aus Abschnitt 5 des Plans.

**Änderung an bestehendem Code:**
- **API:** Das vorhandene `CapabilitySet` war eine flache Stringmenge. Ich habe es ersetzt. `with` nimmt jetzt `(axis, value)`.
- **Richtung von `satisfies`:** Sie folgt dem Plan als `available.satisfies(&req)`.
- **Anpassung:** Ich habe `op/requirements.rs` entsprechend angepasst, einschließlich der beiden Tests, die `"f16"` und `"graph"` benutzten.

**Abweichungen vom Plan:**
- **Werte:** Die Werte der `fusion`-Achse heißen `on` und `off` statt `an` und `aus`.
- **Fehlervarianten:** Die benötigten Varianten (`Incompatible`, `NoActiveProfile`, `InvalidPrecision`) gab es in `PrecisionError` schon, ich habe dort nichts geändert.
- **`is_none_or`:** `pareto()` nutzt `Option::is_none_or`, das erst ab Rust 1.82 verfügbar ist.

Die offenen Punkte aus Abschnitt 6 bleiben offen: `Arc<str>` oder Interning, Zeitnormierung und Schwellwert in F3 sowie dp4a/bf16 auf sm75 in A3.
