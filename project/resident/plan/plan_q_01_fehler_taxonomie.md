Setze Q1 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Scope-Charta erstellt unter `/home/user/xyan/xy.ai.workbench/rag/docs/architecture/scope_charta.md`
- Projektdokumente immer in englisch ausgeben

# Umsetzungsplan Q1 · Fehler-Taxonomie (`thiserror`)

**Querschnitt Q — verbindliche Konvention.** Nach A4 initial zu fixieren, je Gruppe fortzuschreiben. Speist alle Gruppen (A–H).

## 1. Zweck

Einheitliche, typisierte Fehlerbehandlung über alle Pakete. Konsolidiert die im Metaplan/den Quellen verstreuten Fehlerarten zu einer geschichteten Taxonomie mit stabiler öffentlicher API.

## 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q1)

- **`thiserror` 2.x** **[prüfen Version]**. Muster: `#[derive(Error, Debug)]`, `#[error("…{field}")]` (Display), `#[from]` (auto-`From` + Source), `#[source]` (Ursachenkette ohne `From`), `#[error(transparent)]` (Durchreichen). Varianten mit **strukturierten Feldern**, keine Strings. `#[non_exhaustive]` auf öffentlichen Enums → spätere Varianten ohne Breaking Change.
- **Geschichtet:** ein Enum **pro Komponente/Modul** (klein, präzise); ein Top-Level-Enum aggregiert per `#[from]`. Kein zentrales Mega-Enum (koppelt alle Module).
- **Library vs. App:** Engine (öffentliche API) → `thiserror`, typisiert, matchbar. Anwendung/Binary/Tests → `anyhow` (`.context()`). **Die Engine gibt nie `anyhow::Error` in der öffentlichen API zurück.**
- **Intern vs. extern:** öffentlicher Fehler als opaker Struct mit privatem `ErrorKind`-Enum (wie `std::io::Error`) + `kind()`-Accessor auf ein `#[non_exhaustive]` öffentliches Kind. Interne Varianten (z. B. `StaleMapping`) in `pub(crate)`-Enum, an der API-Grenze per `From`/`map_err` abgebildet oder intern per Retry behandelt.

## 3. Verbindliche Konvention (Umsetzung)

**Pro-Komponenten-Enums** (`#[derive(Error, Debug)]`, `#[non_exhaustive]`), zugeordnet aus den Quellen:

| Komponente / Modul | Enum | Varianten (aus Quellen) | Quelle |
|---|---|---|---|
| Index/Manager | `IndexError` | `UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `TransferFailed` | `resident_index.md` (Fehler) |
| Index/Manager (intern) | `ManagerError` *(pub(crate))* | `StaleMapping` | `resident_index.md` („nur intern") |
| Präzision/Kalibrierung | `PrecisionError` | `Incompatible`, `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision`(NaN) | `resident_precision.md` |
| Planner | `PlannerError` | `InvalidConfig`, `MissingCapability`, `UnknownModel` | `resident_planner.md` |
| Config/Fassade | `ConfigError`, `CapabilityError` | ungültige Config, fehlende/unbekannte Capability | Metaplan A4/B6, `resident.md` |

**Top-Level-Aggregat** `ResidentError` (`#[non_exhaustive]`), das die Komponenten-Enums per `#[from]` aggregiert:
```rust
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ResidentError {
    #[error(transparent)] Index(#[from] IndexError),
    #[error(transparent)] Precision(#[from] PrecisionError),
    #[error(transparent)] Planner(#[from] PlannerError),
    #[error(transparent)] Config(#[from] ConfigError),
    #[error(transparent)] Capability(#[from] CapabilityError),
    // Transfer/IO/… nach Bedarf
}
```

**Regeln:**
- Öffentliche API (Fassade, `Index`-Trait, Planner, Calibrator) gibt `Result<_, ResidentError>` bzw. das jeweilige Komponenten-Enum zurück — **nie** `anyhow`.
- `StaleMapping` bleibt `pub(crate)` und wird intern per Retry (Kommunikationsslot) behandelt oder an der Grenze auf eine öffentliche Variante gemappt; es erscheint **nicht** nach außen.
- Task-Zustände (`Queued|Taken|Done|Cancelled`) sind **keine** Fehler; `Cancelled` ist lokal abgeleitet (`resident.md` Queue-Logik). Fehler eines Batchs werden als `Err` in die betroffenen Task-Results geschrieben (`resident_processor.md`).
- GPU-Initfehler → Fallback auf CPU-Engine, GPU als nicht verfügbar markiert (wiederholbar), als Fehlerzustand `Failed` der Ressource, nicht als API-Fehler des Nutzers (`resident_subengine.md` §6).
- Fehlervarianten mit **strukturierten Feldern** (z. B. `UnsupportedScope { method: AccessMethod }`, `ArenaExhausted { requested, available }`), keine Strings.
- Optional (falls API-Stabilität über interne Refactorings gefordert): die **Fassade** exponiert einen opaken `Error` + `kind() -> &ErrorKind` (`#[non_exhaustive]`), während intern die Komponenten-Enums genutzt werden.

## 4. Umsetzungsschritte

1. `error`-Modul je Komponente anlegen; Komponenten-Enums mit `#[non_exhaustive]` + strukturierten Feldern.
2. `ResidentError` als Aggregat mit `#[from]`/`transparent`.
3. `pub(crate)`-interne Enums (`ManagerError`) + Mapping an der API-Grenze.
4. Doku: Skala/Semantik dokumentieren (z. B. `InvalidPrecision` = NaN; `precision`-Klemmen ist kein Fehler, `resident_precision.md`).
5. `anyhow` nur in Binärs/Tests als dev-/bin-dependency, nicht in der Lib-API.

## 5. Tests

- `#[from]`-Konvertierungen: Komponenten-Enum → `ResidentError`.
- `UnsupportedScope` entsteht bei nicht deklariertem Verfahren/fehlendem Resolver (Zusammenspiel mit E2); kein stilles `All`.
- `InvalidPrecision` bei NaN; Klemmen `[0,1]` erzeugt **keinen** Fehler (F4).
- Interne `StaleMapping` erscheint nicht in der öffentlichen Signatur (Kompilier-/Sichtbarkeitstest + API-Grenze-Mapping).
- `#[non_exhaustive]`: Hinzufügen einer Variante bricht kein `match` in Tests mit `_`-Arm.

## 6. Offene Verifikationen ([prüfen])

- `thiserror`-Version (2.x) in der Projekt-`Cargo.toml`.

## 7. Definition of Done

- Komponenten-Enums + `ResidentError` existieren, alle gelisteten Varianten zugeordnet, `#[non_exhaustive]`, strukturierte Felder, keine `anyhow`-Leaks in der Lib-API, interne Varianten gekapselt; Tests aus §5 grün.

SystemInit:  id=b3439145-df31-4eac-8ec1-200f7cb0013d, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5
Control Request:
```yaml
id: f91b-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag
  reason: Check existing project structure before implementing Q1 error taxonomy
```
/answer f91b-1 allow
Control Request:
```yaml
id: f91b-2
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag
      error: |-
        Path is a directory, not a file; listing its contents instead.
        Result reduced to top-level directories the limit of 50 is exceeded (809). Narrow down the result using the 'pattern' regular expression parameter.
        .: (5 files below)
        ./.bin: (10413 files below)
        ./.cargo: (1 file below)
        ./.settings: (1 file below)
        ./.xyrag: (31 files below)
        ./benches: (1 file below)
        ./docs: (4 files below)
        ./src: (40 files below)
        ./tests: (1 file below)
```
/answer f91b-2 allow
Control Request:
```yaml
id: f91b-3
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src
  reason: List existing src module structure to understand where to place error module per crate
```
/answer f91b-3 allow
Control Request:
```yaml
id: f91b-4
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src
      error: |-
        Path is a directory, not a file; listing its contents instead.
        .:
        lib.rs
        ./bin:
        xyrag.rs
        ./core:
        engine.rs
        executor.rs
        layer.rs
        mod.rs
        persistence.rs
        query.rs
        registry.rs
        result.rs
        topology.rs
        ./layers:
        dir_cache.rs
        glob_layer.rs
        grep_layer.rs
        merge_layer.rs
        mod.rs
        ./layers/trigram:
        index.rs
        layer.rs
        mod.rs
        normalize.rs
        signature.rs
        trie.rs
        vocab.rs
        ./resident:
        error.rs
        mock.rs
        mod.rs
        telemetry.rs
        ./resident/facade:
        mod.rs
        ./resident/index:
        mod.rs
        ./resident/kernel:
        cuda.rs
        mod.rs
        ./resident/manager:
        mod.rs
        ./resident/memory:
        mod.rs
        ./resident/op:
        mod.rs
        ./resident/planner:
        mod.rs
        ./resident/precision:
        mod.rs
        ./resident/processor:
        mod.rs
        ./resident/queue:
        mod.rs
        ./resident/subengine:
        device.rs
        mod.rs
```
/answer f91b-4 allow
Control Request:
```yaml
id: f91b-5
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag
  node_type: dummy
  text: name =
  reason: Find Cargo.toml files to check workspace structure
```
/answer f91b-5 allow
Control Request:
```yaml
id: f91b-6
result:
  content:
  - type: text
    text: Not a regular file.
  isError: true
```
/answer f91b-6 allow
Control Request:
```yaml
id: f91b-7
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
  reason: Inspect current error handling and related modules to implement Q1 taxonomy consistently
```
/answer f91b-7 allow
Control Request:
```yaml
id: f91b-8
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
      - id: MemoryError
        signature: "/// Device/memory layer errors (arenas, host<->device transfers, format compati…"
        docstring: "Device/memory layer errors (arenas, host<->device transfers, format compatibili…"
      - id: impl_IsRetryable_for_MemoryError
        signature: "impl IsRetryable for MemoryError {"
        children:
        - id: impl_IsRetryable_for_MemoryError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: CalibrationError
        signature: "/// Precision/calibration layer errors. #[derive(Error, Debug)] #[non_exhaustiv…"
        docstring: Precision/calibration layer errors.
      - id: impl_IsRetryable_for_CalibrationError
        signature: "impl IsRetryable for CalibrationError {"
        children:
        - id: impl_IsRetryable_for_CalibrationError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: PlannerError
        signature: /// Planner/task-expansion layer errors. Concrete variants are added by the own…
        docstring: Planner/task-expansion layer errors. Concrete variants are added by the owning …
      - id: impl_IsRetryable_for_PlannerError
        signature: "impl IsRetryable for PlannerError {"
        children:
        - id: impl_IsRetryable_for_PlannerError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: CapabilityError
        signature: /// Capability/profile-matching layer errors. Concrete variants are added by th…
        docstring: Capability/profile-matching layer errors. Concrete variants are added by the ow…
      - id: impl_IsRetryable_for_CapabilityError
        signature: "impl IsRetryable for CapabilityError {"
        children:
        - id: impl_IsRetryable_for_CapabilityError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
      - id: ResidentError
        signature: "/// Top-level resident engine error, wrapping every layer error via `#[from]`. …"
        docstring: "Top-level resident engine error, wrapping every layer error via `#[from]`."
      - id: impl_IsRetryable_for_ResidentError
        signature: "impl IsRetryable for ResidentError {"
        children:
        - id: impl_IsRetryable_for_ResidentError.is_retryable
          signature: "fn is_retryable(&self) -> bool {"
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Resident engine: embedding/similarity substrate, kept warm."
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index trait, Scope, AccessMethod, Stamp."
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! StreamedIndexManager, ScopeResolver, update queue, residency."
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! CapabilitySet/Profile.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Stateless task expansion.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Public resident facade API.
```
/answer f91b-8 allow
ReasoningToken: 30
Control Request:
```yaml
id: f91b-9
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - IndexError
    - MemoryError
    - CalibrationError
    - PlannerError
    - CapabilityError
    - ResidentError
  reason: Read full existing error enums to assess gaps vs Q1 spec
```
/answer f91b-9 allow
Control Request:
```yaml
id: f91b-a
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
              #[error("scope '{scope}' is not supported")]
              UnsupportedScope { scope: String },
              #[error("index for scope '{scope}' is not available")]
              IndexUnavailable { scope: String },
              /// Internal: a cached mapping no longer matches the current index generation.
              #[error("stale mapping for scope '{scope}'")]
              StaleMapping { scope: String },
          }
      - id: MemoryError
        type: ItemEnum
        code: |
          /// Device/memory layer errors (arenas, host<->device transfers, format compatibility).
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum MemoryError {
              #[error("arena exhausted: requested {requested} bytes, available {available} bytes")]
              ArenaExhausted { requested: usize, available: usize },
              #[error("transfer failed: {reason}")]
              TransferFailed { reason: String },
              #[error("incompatible layout/version: {reason}")]
              Incompatible { reason: String },
          }
      - id: CalibrationError
        type: ItemEnum
        code: |
          /// Precision/calibration layer errors.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum CalibrationError {
              #[error("calibration unstable: {reason}")]
              CalibrationUnstable { reason: String },
              #[error("no active calibration profile")]
              NoActiveProfile,
              #[error("invalid precision requested: {requested}")]
              InvalidPrecision { requested: String },
          }
      - id: PlannerError
        type: ItemEnum
        code: |
          /// Planner/task-expansion layer errors. Concrete variants are added by the owning group's plan.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum PlannerError {
              #[error("planner error: {reason}")]
              Other { reason: String },
          }
      - id: CapabilityError
        type: ItemEnum
        code: |
          /// Capability/profile-matching layer errors. Concrete variants are added by the owning group's plan.
          #[derive(Error, Debug)]
          #[non_exhaustive]
          pub enum CapabilityError {
              #[error("capability error: {reason}")]
              Other { reason: String },
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
              Memory(#[from] MemoryError),
              #[error(transparent)]
              Calibration(#[from] CalibrationError),
              #[error(transparent)]
              Planner(#[from] PlannerError),
              #[error(transparent)]
              Capability(#[from] CapabilityError),
          }
```
/answer f91b-a allow
Control Request:
```yaml
id: f91b-b
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/project/resident
    pattern: Error|StaleMapping|ArenaExhausted|TransferFailed|UnsupportedScope|InvalidConfig|MissingCapability|UnknownModel|InvalidPrecision|Incompatible|NoActiveProfile|CalibrationUnstable|IndexUnavailable
    include:
    - '*.md'
    limit: 50
  reason: Check source docs for exact error variant definitions to align implementation with plan
```
/answer f91b-b allow
Control Request:
```yaml
id: f91b-c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/project/resident
      matches:
      - path: resident.md
        matches:
        - lineno: 130
          match: "result: OnceCell<Result<Output, Error>>,"
      - path: resident_index.md
        matches:
        - lineno: 57
          match: "- Kompatibel heißt: Der Index deklariert das Verfahren **und** der Manager hat einen Resolver dafür. Sonst Fehler `UnsupportedScope`, kein stilles Ausweichen auf `All`."
        - lineno: 176
          match: "- Typisierte Fehler (`thiserror`), u. a. `UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `StaleMapping` (nur intern), `TransferFailed`."
        - lineno: 205
          match: '- `UnsupportedScope` bei nicht deklariertem Verfahren oder fehlendem Resolver.'
      - path: resident_planplan.md
        matches:
        - lineno: 190
          match: "- Zweck: Scope-Auflösung durch Manager, pluggable `ScopeResolver` pro `AccessMethod`, `UnsupportedScope` (kein stilles Ausweichen), Zentroid-Beispiel, kompakte Entry-Mengen (Ranges/Bitmap)."
        - lineno: 321
          match: "- **Q1 · Fehler-Taxonomie** (`thiserror`): konsolidiert u. a. `UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `StaleMapping`(intern), `TransferFailed`, `Incompatible`, `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision`, Planner-/Config-/Capability-Fehler."
      - path: context/context_c.md
        matches:
        - lineno: 106
          match: "- cudarc: `CudaEvent::try_is_complete() -> Result<bool, DriverError>` (`Ok(false)` nur bei `NOT_READY`); `is_complete()` gibt bei Fehlern `false` zurück → **`try_is_complete` verwenden**."
        - lineno: 156
          match: "- Sender-Drop ohne Senden macht `recv` zum Fehler → diesen als `Cancelled` abbilden (Beispiel `atomr-accel-cuda`: „callback dropped without firing\" → typed Error)."
        - lineno: 205
          match: "- `thiserror` 2.0.21: erscheint nicht in der öffentlichen API (Wechsel zu Handimplementierung ist kein Breaking Change); `#[from]` impliziert `#[source]`; `#[error(transparent)]` leitet Display/source weiter; opaker öffentlicher Fehler (`pub struct PublicError(#[from] ErrorRepr)`) hält die Repräsentation änderbar."
        - lineno: 206
          match: "- Empfehlung [W]: ein Fehler-Enum je Modul (`PlannerError`, `ConfigError`, `CapabilityError`, `OrchestrationError`), oben zusammengeführt mit `#[from]`; `#[non_exhaustive]` auf öffentlichen Enums. Über `oneshot` `Result<T, TaskError>` senden; wenn derselbe Fehler an mehrere Warter geht: `Arc<TaskError>` oder `Clone`. `Cancelled` als eigene Variante (nicht als String)."
      - path: context/context_h_q.md
        matches:
        - lineno: 174
          match: "- Muster: `#[derive(Error, Debug)]`, `#[error(\"…{field}\")]` für Display, `#[from]` für automatische `From`-Impl plus Source, `#[source]` für Ursachenkette ohne `From`, `#[error(transparent)]` zum Durchreichen."
        - lineno: 184
          match: '- Die Engine selbst gibt kein `anyhow::Error` in der öffentlichen API zurück.'
        - lineno: 187
          match: "- Muster: öffentlicher Fehler als opaker Struct mit privatem `ErrorKind`-Enum (wie `std::io::Error`), plus Accessor `kind()` auf ein `#[non_exhaustive]` öffentliches Kind."
        - lineno: 188
          match: '- Interne Varianten wie `StaleMapping` leben in einem `pub(crate)`-Enum und werden an der API-Grenze per `From`/`map_err` auf öffentliche Varianten abgebildet (oder intern per Retry behandelt).'
      - path: context/context_a.md
        matches:
        - lineno: 148
          match: "- Ein Fehler-Enum pro Schicht (`AdapterError`, `OrchestratorError`, …), obere Schichten wrappen untere mit `#[from]`/`#[source]`; `#[non_exhaustive]` für öffentliche Enums."
        - lineno: 150
          match: "- Worker-Pool: Ergebnis als `Result<T, E>` per Channel zurück, nie `panic`; Panics mit `JoinError` abfangen. `anyhow` nur in `main`/Binary."
      - path: context/context_b.md
        matches:
        - lineno: 60
          match: '| `tokio::sync::OnceCell<T>` | async | `set` → `Err(SetError)`; `get_or_init` async | wartet async statt Thread-Blockade; **kein** „warte bis gesetzt“-API |'
        - lineno: 62
          match: "- **Empfehlung für `result`:** `std::sync::OnceLock<Result<Output, Error>>` + `Notify`/Zustand für async-Wartende. `set` genau einmal durch den Claim-Inhaber (durch Automat garantiert); `Err` beim zweiten Set als Invariantenverletzung loggen (`debug_assert!`)."
        - lineno: 102
          match: "- `std::sync::Mutex`: Poisoning (nach Panic `Err`); seit Rust 1.62 futex-basiert, in Praxis schnell. Poison behandeln mit `lock().unwrap_or_else(PoisonError::into_inner)`."
        - lineno: 141
          match: "impl TryFrom<DType> for candle_core::DType { type Error = UnsupportedDType; /* … */ }"
        - lineno: 142
          match: "impl TryFrom<candle_core::DType> for DType { type Error = UnsupportedDType; /* … */ }"
      - path: resident_precision.md
        matches:
        - lineno: 108
          match: "- Typisierte Fehler (`thiserror`): `Incompatible`, `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision` (NaN)."
      - path: archive/context_research_hq.md
        matches:
        - lineno: 94
          match: "25. Best Practices für **Fehler-Taxonomie-Design mit `thiserror`** (Error-Enums, `#[from]`, `#[error(...)]`, Source-Chaining, `#[source]`)."
        - lineno: 97
          match: 28. Muster für **intern-sichtbare vs. nach außen exponierte Fehler** (z. B. `StaleMapping` nur intern) — wie API-stabil kapseln?
        - lineno: 123
          match: "- Konkrete Konsolidierung der projekteigenen Fehler-Liste (`UnsupportedScope`, `ArenaExhausted`, `TransferFailed`, `Incompatible`, `NoActiveProfile` usw.) zu einer Taxonomie (Q1 — nutzt aber Frage 25–28)."
      - path: archive/create_plan_a.md
        matches:
        - lineno: 481
          match: "die A4-Fehler-Taxonomie ein (`Incompatible`, `TransferFailed`)."
        - lineno: 505
          match: "- Fehler-/Logging-Hinweise der Fachdokumente (z. B. `resident_index.md` → `UnsupportedScope`,"
        - lineno: 506
          match: "`IndexUnavailable`, `ArenaExhausted`, `StaleMapping` (intern), `TransferFailed`;"
        - lineno: 526
          match: "1. **Ein Fehler-Enum pro Schicht** (z. B. `AdapterError`, `OrchestratorError`, `ManagerError`,"
        - lineno: 527
          match: "`CalibrationError`); obere Schichten wrappen untere mit `#[from]`/`#[source]`."
        - lineno: 532
          match: "`UnsupportedScope`, `IndexUnavailable`, `ArenaExhausted`, `StaleMapping` (intern),"
        - lineno: 533
          match: "`TransferFailed`, `Incompatible`, `CalibrationUnstable`, `NoActiveProfile`, `InvalidPrecision`,"
        - lineno: 537
          match: '`JoinError` abfangen. `anyhow` nur in Binary/`main`.'
        - lineno: 592
          match: "- Gegencheck mit A3: Lade-/Kompatibilitätsfehler (`Incompatible`, `TransferFailed`) sind abgedeckt."
      - path: archive/create_plan_c.md
        matches:
        - lineno: 116
          match: "- **Q1 Fehler (`thiserror` 2.0.21):** ein Fehler-Enum je Modul (`ConfigError`, `PlannerError`, `OrchestrationError`, `CapabilityError`), oben via `#[from]` zusammengeführt, öffentliche Enums `#[non_exhaustive]`, `Cancelled` als eigene Variante. Fan-out an mehrere Warter: `Arc<TaskError>` oder `Clone`. [B]"
        - lineno: 187
          match: "- Begründung: Fehlkonfiguration scheitert zur Compile-Zeit (fehlende Pflichtfelder) bzw. in `build()` (`ConfigError`)."
        - lineno: 216
          match: "- `submit(input, params) -> TaskHandle`. `TaskHandle` implementiert `IntoFuture` (`.await` → `Result<Output, TaskError>`) und bietet zusätzlich `wait_timeout(Duration)` und `try_get()`."
        - lineno: 218
          match: "- Intern: Task mit eingebettetem Ergebnis-Kanal; Reihenfolge immer **Ergebnis zuerst in den Zustand, dann Signal**. Sender-Drop ohne Senden → `TaskError::Cancelled` (nicht als String)."
        - lineno: 244
          match: "pub fn init(self) -> Result<Resident, ConfigError>;   // verbrauchend → bereit"
        - lineno: 256
          match: "pub fn shutdown(self) -> Result<(), OrchestrationError>; // Worker joinen, Leases freigeben"
        - lineno: 263
          match: "-> Result<Output, TaskError>;  // intern rt.block_on(handle)"
        - lineno: 274
          match: "impl std::future::IntoFuture for TaskHandle { type Output = Result<Output, TaskError>; /* ... */ }"
        - lineno: 276
          match: "pub fn wait_timeout(self, d: std::time::Duration) -> Result<Output, TaskError>;"
        - lineno: 277
          match: "pub fn try_get(&self) -> Option<Result<Output, TaskError>>;"
        - lineno: 282
          match: "- `precision: f32` wird nur **durchgereicht** (geklemmt/validiert in Gruppe F). C1 lehnt `NaN` ab (`ConfigError`/`TaskError::InvalidPrecision`)."
        - lineno: 290
          match: "2. **Fehler-Taxonomie-Fassade** (Q1): `ConfigError`, öffentlicher opaker `ResidentError`/`TaskError` mit `#[from]`, `#[non_exhaustive]`, `Cancelled`-Variante."
        - lineno: 304
          match: "- `ConfigError` (fehlende Pflichtfelder, unbekanntes Modell, ungültige `precision`), `TaskError` (`Cancelled`, `Timeout`, `InvalidPrecision`, nach außen gereichte Orchestrierungs-/Planner-Fehler)."
        - lineno: 305
          match: "- Öffentlicher Fehler opak (`pub struct ResidentError(#[from] ErrorRepr)`), Repr änderbar ohne Breaking Change; `#[non_exhaustive]`."
        - lineno: 306
          match: "- Gleicher Fehler an mehrere Warter (watch/broadcast): `Arc<TaskError>` oder `Clone`."
        - lineno: 315
          match: "- Builder-Tests: Pflichtfeld fehlt ⇒ `ConfigError`; `precision = NaN` ⇒ Fehler."
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer f91b-c allow
Control Request:
```yaml
id: f91b-d
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/project/resident
    pattern: ConfigError|CapabilityError|InvalidConfig|MissingCapability|UnknownModel
    include:
    - resident_planner.md
    - resident.md
    - plan_q_01_fehler_taxonomie.md
    limit: 50
  reason: Find exact ConfigError/CapabilityError/PlannerError variant definitions in source docs
```
/answer f91b-d allow
Control Request:
```yaml
id: f91b-e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/project/resident
      matches:
      - path: plan/plan_q_01_fehler_taxonomie.md
        matches:
        - lineno: 34
          match: "| Planner | `PlannerError` | `InvalidConfig`, `MissingCapability`, `UnknownModel` | `resident_planner.md` |"
        - lineno: 35
          match: "| Config/Fassade | `ConfigError`, `CapabilityError` | ungültige Config, fehlende/unbekannte Capability | Metaplan A4/B6, `resident.md` |"
        - lineno: 45
          match: "#[error(transparent)] Config(#[from] ConfigError),"
        - lineno: 46
          match: "#[error(transparent)] Capability(#[from] CapabilityError),"
        - lineno: 281
          match: "- id: CapabilityError"
        - lineno: 284
          match: "- id: impl_IsRetryable_for_CapabilityError"
        - lineno: 285
          match: "signature: \"impl IsRetryable for CapabilityError {\""
        - lineno: 287
          match: "- id: impl_IsRetryable_for_CapabilityError.is_retryable"
        - lineno: 364
          match: '- CapabilityError'
        - lineno: 430
          match: "- id: CapabilityError"
        - lineno: 436
          match: "pub enum CapabilityError {"
        - lineno: 456
          match: "Capability(#[from] CapabilityError),"
        - lineno: 468
          match: "pattern: Error|StaleMapping|ArenaExhausted|TransferFailed|UnsupportedScope|InvalidConfig|MissingCapability|UnknownModel|InvalidPrecision|Incompatible|NoActiveProfile|CalibrationUnstable|IndexUnavailable"
        - lineno: 511
          match: "match: \"- Empfehlung [W]: ein Fehler-Enum je Modul (`PlannerError`, `ConfigError`, `CapabilityError`, `OrchestrationError`), oben zusammengeführt mit `#[from]`; `#[non_exhaustive]` auf öffentlichen Enums. Über `oneshot` `Result<T, TaskError>` senden; wenn derselbe Fehler an mehrere Warter geht: `Arc<TaskError>` oder `Clone`. `Cancelled` als eigene Variante (nicht als String).\""
        - lineno: 575
          match: "match: \"- **Q1 Fehler (`thiserror` 2.0.21):** ein Fehler-Enum je Modul (`ConfigError`, `PlannerError`, `OrchestrationError`, `CapabilityError`), oben via `#[from]` zusammengeführt, öffentliche Enums `#[non_exhaustive]`, `Cancelled` als eigene Variante. Fan-out an mehrere Warter: `Arc<TaskError>` oder `Clone`. [B]\""
        - lineno: 577
          match: "match: \"- Begründung: Fehlkonfiguration scheitert zur Compile-Zeit (fehlende Pflichtfelder) bzw. in `build()` (`ConfigError`).\""
        - lineno: 583
          match: "match: \"pub fn init(self) -> Result<Resident, ConfigError>;   // verbrauchend → bereit\""
        - lineno: 595
          match: "match: \"- `precision: f32` wird nur **durchgereicht** (geklemmt/validiert in Gruppe F). C1 lehnt `NaN` ab (`ConfigError`/`TaskError::InvalidPrecision`).\""
        - lineno: 597
          match: "match: \"2. **Fehler-Taxonomie-Fassade** (Q1): `ConfigError`, öffentlicher opaker `ResidentError`/`TaskError` mit `#[from]`, `#[non_exhaustive]`, `Cancelled`-Variante.\""
        - lineno: 599
          match: "match: \"- `ConfigError` (fehlende Pflichtfelder, unbekanntes Modell, ungültige `precision`), `TaskError` (`Cancelled`, `Timeout`, `InvalidPrecision`, nach außen gereichte Orchestrierungs-/Planner-Fehler).\""
        - lineno: 605
          match: "match: \"- Builder-Tests: Pflichtfeld fehlt ⇒ `ConfigError`; `precision = NaN` ⇒ Fehler.\""
        - lineno: 617
          match: "pattern: ConfigError|CapabilityError|InvalidConfig|MissingCapability|UnknownModel"
        - lineno: 623
          match: "reason: Find exact ConfigError/CapabilityError/PlannerError variant definitions in source docs"
```
/answer f91b-e allow
Control Request:
```yaml
id: f91b-f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q_01_fehler_taxonomie.md
  lineno: 280
  reason: Inspect content around lines 270-470 which appear to contain an outline/appendix in the plan file
```
/answer f91b-f allow
Control Request:
```yaml
id: f91b-10
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q_01_fehler_taxonomie.md
      nodes:
      - id: 938d0f.2f64fe.fred8l|Ur4Em9
        code: |
          ```yaml
          id: f91b-8
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
                - id: MemoryError
                  signature: "/// Device/memory layer errors (arenas, host<->device transfers, format compati…"
                  docstring: "Device/memory layer errors (arenas, host<->device transfers, format compatibili…"
                - id: impl_IsRetryable_for_MemoryError
                  signature: "impl IsRetryable for MemoryError {"
                  children:
                  - id: impl_IsRetryable_for_MemoryError.is_retryable
                    signature: "fn is_retryable(&self) -> bool {"
                - id: CalibrationError
                  signature: "/// Precision/calibration layer errors. #[derive(Error, Debug)] #[non_exhaustiv…"
                  docstring: Precision/calibration layer errors.
                - id: impl_IsRetryable_for_CalibrationError
                  signature: "impl IsRetryable for CalibrationError {"
                  children:
                  - id: impl_IsRetryable_for_CalibrationError.is_retryable
                    signature: "fn is_retryable(&self) -> bool {"
                - id: PlannerError
                  signature: /// Planner/task-expansion layer errors. Concrete variants are added by the own…
                  docstring: Planner/task-expansion layer errors. Concrete variants are added by the owning …
                - id: impl_IsRetryable_for_PlannerError
                  signature: "impl IsRetryable for PlannerError {"
                  children:
                  - id: impl_IsRetryable_for_PlannerError.is_retryable
                    signature: "fn is_retryable(&self) -> bool {"
                - id: CapabilityError
                  signature: /// Capability/profile-matching layer errors. Concrete variants are added by th…
                  docstring: Capability/profile-matching layer errors. Concrete variants are added by the ow…
                - id: impl_IsRetryable_for_CapabilityError
                  signature: "impl IsRetryable for CapabilityError {"
                  children:
                  - id: impl_IsRetryable_for_CapabilityError.is_retryable
                    signature: "fn is_retryable(&self) -> bool {"
                - id: ResidentError
                  signature: "/// Top-level resident engine error, wrapping every layer error via `#[from]`. …"
                  docstring: "Top-level resident engine error, wrapping every layer error via `#[from]`."
                - id: impl_IsRetryable_for_ResidentError
                  signature: "impl IsRetryable for ResidentError {"
                  children:
                  - id: impl_IsRetryable_for_ResidentError.is_retryable
                    signature: "fn is_retryable(&self) -> bool {"
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: "//! Resident engine: embedding/similarity substrate, kept warm."
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
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: "//! Index trait, Scope, AccessMethod, Stamp."
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/manager/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: "//! StreamedIndexManager, ScopeResolver, update queue, residency."
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/precision/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: //! CapabilitySet/Profile.
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: //! Stateless task expansion.
              - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
                nodes:
                - id: ModuleDoc
                  signature: //! Public resident facade API.
          ```
```
/answer f91b-10 allow
Control Request:
```yaml
id: f91b-11
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q_01_fehler_taxonomie.md
  reason: Understand full structure of the plan file including appended transcript to see if prior implementation attempt exists
```
/answer f91b-11 allow
Control Request:
```yaml
id: f91b-12
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_q_01_fehler_taxonomie.md
      nodes:
      - id: VXZHbD|DHSkZu
        signature: Setze Q1 um.
      - id: L2TwUV|erk8b9
        signature: "[include](/home/user/xyan/xy.…"
      - id: 4phta9|9Q99gE
        signature: '- minimiere die Kommentare un…'
      - id: 938d0f
        signature: Umsetzungsplan Q1 · Fehler-Taxonomie (`thiserror`)
        children:
        - id: 938d0f.lgSU2w|nxZCr3
          signature: '**Querschnitt Q — verbindlich…'
        - id: 938d0f.f323b1
          signature: 1. Zweck
          children:
          - id: 938d0f.f323b1.xjtdze|347SyR
            signature: "Einheitliche, typisierte Fehl…"
        - id: 938d0f.5f8fb1
          signature: 2. Rechercheergebnis (destilliert aus `context_h_q.md` Q1)
        - id: 938d0f.bed199
          signature: 3. Verbindliche Konvention (Umsetzung)
          children:
          - id: 938d0f.bed199.aBbID5|SMxwde
            signature: '**Pro-Komponenten-Enums** (`#…'
          - id: 938d0f.bed199.reNQtl|QbfN5p
            signature: '**Top-Level-Aggregat** `Resid…'
          - id: 938d0f.bed199.WOEFaP|uP2HMX
            signature: |-
              ```rust
              #[derive(Error, Debug…
          - id: 938d0f.bed199.FnqWa0|BFy9zL
            signature: '**Regeln:**'
        - id: 938d0f.15740b
          signature: 4. Umsetzungsschritte
        - id: 938d0f.cfae00
          signature: 5. Tests
        - id: 938d0f.a8a23e
          signature: "6. Offene Verifikationen ([prüfen])"
        - id: 938d0f.2f64fe
          signature: 7. Definition of Done
          children:
          - id: 938d0f.2f64fe.ABI0rM|ctr4Lc
            signature: "SystemInit:  id=b3439145-df31…"
          - id: 938d0f.2f64fe.zGaask|b75n7Q
            signature: |-
              ```yaml
              id: f91b-1
              toolName: …
          - id: 938d0f.2f64fe.vFEAAM|JHqeig
            signature: |-
              /answer f91b-1 allow
              Control …
          - id: 938d0f.2f64fe.Bxhrqt|g9Zsvt
            signature: |-
              ```yaml
              id: f91b-2
              result:
                …
          - id: 938d0f.2f64fe.0MJE3F|Qoh0Tx
            signature: |-
              /answer f91b-2 allow
              Control …
          - id: 938d0f.2f64fe.chVUJV|Z1yZYD
            signature: |-
              ```yaml
              id: f91b-3
              toolName: …
          - id: 938d0f.2f64fe.mWIOMC|QQd63h
            signature: |-
              /answer f91b-3 allow
              Control …
          - id: 938d0f.2f64fe.Vs9PyR|mIaIHP
            signature: |-
              ```yaml
              id: f91b-4
              result:
                …
          - id: 938d0f.2f64fe.vOMzCl|w6xHxy
            signature: |-
              /answer f91b-4 allow
              Control …
          - id: 938d0f.2f64fe.qGPE9d|dGZlay
            signature: |-
              ```yaml
              id: f91b-5
              toolName: …
          - id: 938d0f.2f64fe.zCKOHE|A8GAbz
            signature: |-
              /answer f91b-5 allow
              Control …
          - id: 938d0f.2f64fe.fmfdXg|laYis8
            signature: |-
              ```yaml
              id: f91b-6
              result:
                …
          - id: 938d0f.2f64fe.KNCR73|57SgVi
            signature: |-
              /answer f91b-6 allow
              Control …
          - id: 938d0f.2f64fe.k6AWz9|WRg0en
            signature: |-
              ```yaml
              id: f91b-7
              toolName: …
          - id: 938d0f.2f64fe.glJDZP|L25WkU
            signature: |-
              /answer f91b-7 allow
              Control …
          - id: 938d0f.2f64fe.fred8l|Ur4Em9
            signature: |-
              ```yaml
              id: f91b-8
              result:
                …
          - id: 938d0f.2f64fe.UnpcJn|H1IcIj
            signature: |-
              /answer f91b-8 allow
              Reasonin…
          - id: 938d0f.2f64fe.JIS6ql|itaoEC
            signature: |-
              ```yaml
              id: f91b-9
              toolName: …
          - id: 938d0f.2f64fe.pITp5R|jLpSec
            signature: |-
              /answer f91b-9 allow
              Control …
          - id: 938d0f.2f64fe.Iac4FP|b2E338
            signature: |-
              ```yaml
              id: f91b-a
              result:
                …
          - id: 938d0f.2f64fe.TqCk5C|S7dVkr
            signature: |-
              /answer f91b-a allow
              Control …
          - id: 938d0f.2f64fe.1K58l3|0jaiJT
            signature: |-
              ```yaml
              id: f91b-b
              toolName: …
          - id: 938d0f.2f64fe.1p6Xgw|ZlkAjW
            signature: |-
              /answer f91b-b allow
              Control …
          - id: 938d0f.2f64fe.99kMFw|mveqgA
            signature: |-
              ```yaml
              id: f91b-c
              result:
                …
          - id: 938d0f.2f64fe.HNtS8s|jkBVgR
            signature: |-
              /answer f91b-c allow
              Control …
          - id: 938d0f.2f64fe.ETCXit|OrJ8IZ
            signature: |-
              ```yaml
              id: f91b-d
              toolName: …
          - id: 938d0f.2f64fe.AKoWOQ|mMtWOP
            signature: |-
              /answer f91b-d allow
              Control …
          - id: 938d0f.2f64fe.cv5NZH|p43WAt
            signature: |-
              ```yaml
              id: f91b-e
              result:
                …
          - id: 938d0f.2f64fe.4Hzx9g|2F8gPR
            signature: |-
              /answer f91b-e allow
              Control …
          - id: 938d0f.2f64fe.vTre2b|yfa1Cp
            signature: |-
              ```yaml
              id: f91b-f
              toolName: …
          - id: 938d0f.2f64fe.vsO4jo|oEeJXX
            signature: |-
              /answer f91b-f allow
              Control …
          - id: 938d0f.2f64fe.HbQ2sP|Q8wdh0
            signature: |-
              ```yaml
              id: f91b-10
              result:
               …
          - id: 938d0f.2f64fe.yMvXST|l4i09I
            signature: |-
              ```
              /answer f91b-10 allow
              Con…
          - id: 938d0f.2f64fe.FEoiNc|iebxK5
            signature: |-
              /answer f91b-11 allow
              [xy.ai.…
```
/answer f91b-12 allow
Control Request:
```yaml
id: f91b-13
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: CalibrationError|MemoryError|IndexError|PlannerError|CapabilityError|StaleMapping|ResidentError|ConfigError|ManagerError
    include:
    - '*.rs'
    limit: 50
  reason: Find all usages of error enums across src to understand impact of renaming/restructuring
```
/answer f91b-13 allow
Control Request:
```yaml
id: f91b-14
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/error.rs
        matches:
        - lineno: 17
          match: "pub enum IndexError {"
        - lineno: 24
          match: "StaleMapping { scope: String },"
        - lineno: 27
          match: "impl IsRetryable for IndexError {"
        - lineno: 29
          match: "matches!(self, IndexError::IndexUnavailable { .. })"
        - lineno: 36
          match: "pub enum MemoryError {"
        - lineno: 45
          match: "impl IsRetryable for MemoryError {"
        - lineno: 47
          match: "matches!(self, MemoryError::TransferFailed { .. })"
        - lineno: 54
          match: "pub enum CalibrationError {"
        - lineno: 63
          match: "impl IsRetryable for CalibrationError {"
        - lineno: 65
          match: "matches!(self, CalibrationError::CalibrationUnstable { .. })"
        - lineno: 72
          match: "pub enum PlannerError {"
        - lineno: 77
          match: "impl IsRetryable for PlannerError {"
        - lineno: 86
          match: "pub enum CapabilityError {"
        - lineno: 91
          match: "impl IsRetryable for CapabilityError {"
        - lineno: 100
          match: "pub enum ResidentError {"
        - lineno: 102
          match: "Index(#[from] IndexError),"
        - lineno: 104
          match: "Memory(#[from] MemoryError),"
        - lineno: 106
          match: "Calibration(#[from] CalibrationError),"
        - lineno: 108
          match: "Planner(#[from] PlannerError),"
        - lineno: 110
          match: "Capability(#[from] CapabilityError),"
        - lineno: 113
          match: "impl IsRetryable for ResidentError {"
        - lineno: 116
          match: "ResidentError::Index(e) => e.is_retryable(),"
        - lineno: 117
          match: "ResidentError::Memory(e) => e.is_retryable(),"
        - lineno: 118
          match: "ResidentError::Calibration(e) => e.is_retryable(),"
        - lineno: 119
          match: "ResidentError::Planner(e) => e.is_retryable(),"
        - lineno: 120
          match: "ResidentError::Capability(e) => e.is_retryable(),"
```
/answer f91b-14 allow
Control Request:
```yaml
id: f91b-15
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - ModuleDoc
    - IsRetryable
  reason: Read module doc and IsRetryable trait to preserve style while rewriting
```
/answer f91b-15 allow
Control Request:
```yaml
id: f91b-16
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Error taxonomy.
          //!
          //! One enum per layer; upper layers wrap lower ones via `#[from]`/`#[source]`. All variants
          //! are `Send + Sync + 'static` and carry context as typed fields, never pre-formatted strings.
          //! Public enums are `#[non_exhaustive]` so new variants don't break downstream matches.
      - id: IsRetryable
        type: ItemTrait
        signature: "/// Classifies an error as retryable (transient) vs. fatal, for backoff decisio…"
        docstring: "Classifies an error as retryable (transient) vs. fatal, for backoff decisions."
        children:
        - id: IsRetryable.is_retryable
          type: TraitItemFn
          code: |2
                fn is_retryable(&self) -> bool;
```
/answer f91b-16 allow
Control Request:
```yaml
id: f91b-17
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    pattern: AccessMethod|enum Scope|pub struct Scope
    include:
    - '*.rs'
    limit: 30
  reason: Check AccessMethod/Scope type definitions for structured error fields
```
/answer f91b-17 allow
Control Request:
```yaml
id: f91b-18
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/index/mod.rs
        matches:
        - lineno: 1
          match: "//! Index trait, Scope, AccessMethod, Stamp."
```
/answer f91b-18 allow
Control Request:
```yaml
id: f91b-19
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/project/resident
    pattern: ConfigError|Pflichtfeld|unbekanntes Modell|precision.*NaN|init\(
    include:
    - resident.md
    - resident_planner.md
    - resident_precision.md
    limit: 30
  reason: Verify exact ConfigError field semantics for facade builder in source doc
```
/answer f91b-19 allow
Control Request:
```yaml
id: f91b-1a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/project/resident
      matches:
      - path: resident_planner.md
        matches:
        - lineno: 46
          match: "- Strukturierte Fehler mit `thiserror` (ungültige Config, fehlende Capability, unbekanntes Modell)."
```
/answer f91b-1a allow
ReasoningToken: 53
Control Request:
```yaml
id: f91b-1b
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: IndexError
    source: |-
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
      }
  reason: "Align IndexError with Q1 spec: UnsupportedScope, IndexUnavailable, ArenaExhausted, TransferFailed"
```
/answer f91b-1b allow
Control Request:
```yaml
id: f91b-1c
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: impl_IsRetryable_for_IndexError
    source: |-
      impl IsRetryable for IndexError {
          fn is_retryable(&self) -> bool {
              matches!(self, IndexError::IndexUnavailable { .. } | IndexError::ArenaExhausted { .. })
          }
      }
  reason: Update retryability for new IndexError variant set
```
/answer f91b-1c allow
Control Request:
```yaml
id: f91b-1d
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: MemoryError
    source: |-
      /// Internal index-manager errors; mapped to a public variant or retried at the API boundary,
      /// never exposed directly.
      #[derive(Error, Debug)]
      #[non_exhaustive]
      pub(crate) enum ManagerError {
          #[error("stale mapping for scope '{scope}'")]
          StaleMapping { scope: String },
      }
  reason: Replace MemoryError with internal pub(crate) ManagerError carrying StaleMapping per Q1 spec
```
/answer f91b-1d allow
Control Request:
```yaml
id: f91b-1e
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: impl_IsRetryable_for_MemoryError
    source: |-
      impl IsRetryable for ManagerError {
          fn is_retryable(&self) -> bool {
              matches!(self, ManagerError::StaleMapping { .. })
          }
      }
  reason: Add retryability for internal ManagerError
```
/answer f91b-1e allow
Control Request:
```yaml
id: f91b-1f
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: CalibrationError
    source: |-
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
  reason: Rename CalibrationError to PrecisionError and add Incompatible per Q1 spec
```
/answer f91b-1f allow
Control Request:
```yaml
id: f91b-20
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: impl_IsRetryable_for_CalibrationError
    source: |-
      impl IsRetryable for PrecisionError {
          fn is_retryable(&self) -> bool {
              matches!(self, PrecisionError::CalibrationUnstable { .. })
          }
      }
  reason: Update retryability impl for renamed PrecisionError
```
/answer f91b-20 allow
Control Request:
```yaml
id: f91b-21
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: PlannerError
    source: |-
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
  reason: Replace placeholder PlannerError with concrete variants per Q1 spec
```
/answer f91b-21 allow
Control Request:
```yaml
id: f91b-22
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: impl_IsRetryable_for_PlannerError
    source: |-
      impl IsRetryable for PlannerError {
          fn is_retryable(&self) -> bool {
              false
          }
      }
  reason: Planner errors are fatal/non-retryable per spec (config/model/capability mismatches)
```
/answer f91b-22 allow
Control Request:
```yaml
id: f91b-23
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: CapabilityError
    source: |-
      /// Capability/profile-matching layer errors.
      #[derive(Error, Debug)]
      #[non_exhaustive]
      pub enum CapabilityError {
          #[error("missing capability: {capability}")]
          Missing { capability: String },
          #[error("unknown capability: {capability}")]
          Unknown { capability: String },
      }

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
  reason: Give CapabilityError concrete variants and add ConfigError per Q1 spec
```
/answer f91b-23 allow
Control Request:
```yaml
id: f91b-24
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: impl_IsRetryable_for_CapabilityError
    source: |-
      impl IsRetryable for CapabilityError {
          fn is_retryable(&self) -> bool {
              false
          }
      }

      impl IsRetryable for ConfigError {
          fn is_retryable(&self) -> bool {
              false
          }
      }
  reason: Add retryability impls for CapabilityError and new ConfigError
```
/answer f91b-24 allow
Control Request:
```yaml
id: f91b-25
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: ResidentError
    source: |-
      /// Top-level resident engine error, wrapping every layer error via `#[from]`.
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
  reason: "Update ResidentError aggregate: drop Memory, rename Calibration to Precision, add Config\"}]"
```
/answer f91b-25 allow
Control Request:
```yaml
id: f91b-26
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - ResidentError
    - impl_IsRetryable_for_ResidentError
  reason: Verify ResidentError enum still has required derive/non_exhaustive attributes after edit
```
/answer f91b-26 allow
Control Request:
```yaml
id: f91b-27
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: ResidentError
        type: ItemEnum
        code: |
          /// Top-level resident engine error, wrapping every layer error via `#[from]`.
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
                        ResidentError::Memory(e) => e.is_retryable(),
                        ResidentError::Calibration(e) => e.is_retryable(),
                        ResidentError::Planner(e) => e.is_retryable(),
                        ResidentError::Capability(e) => e.is_retryable(),
                    }
                }
```
/answer f91b-27 allow
Control Request:
```yaml
id: f91b-28
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: ResidentError
    old_text: |-
      /// Top-level resident engine error, wrapping every layer error via `#[from]`.
      pub enum ResidentError {
    new_text: |-
      /// Top-level resident engine error, wrapping every layer error via `#[from]`.
      #[derive(Error, Debug)]
      #[non_exhaustive]
      pub enum ResidentError {
  reason: Restore derive and non_exhaustive attributes dropped during replacement
```
/answer f91b-28 allow
Control Request:
```yaml
id: f91b-29
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    id: impl_IsRetryable_for_ResidentError
    source: |-
      impl IsRetryable for ResidentError {
          fn is_retryable(&self) -> bool {
              match self {
                  ResidentError::Index(e) => e.is_retryable(),
                  ResidentError::Precision(e) => e.is_retryable(),
                  ResidentError::Planner(e) => e.is_retryable(),
                  ResidentError::Config(e) => e.is_retryable(),
                  ResidentError::Capability(e) => e.is_retryable(),
              }
          }
      }
  reason: Update match arms for renamed/added ResidentError variants
```
/answer f91b-29 allow
Control Request:
```yaml
id: f91b-2a
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  reason: Verify syntax correctness after restructuring the error taxonomy
```
/answer f91b-2a allow
Control Request:
```yaml
id: f91b-2b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - ModuleDoc
  reason: Read full file top to confirm nothing orphaned; will view entire file via outline next
```
/answer f91b-2b allow
Control Request:
```yaml
id: f91b-2c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
      - id: ModuleDoc
        type: ModuleAttrs
        code: |
          //! Error taxonomy.
          //!
          //! One enum per layer; upper layers wrap lower ones via `#[from]`/`#[source]`. All variants
          //! are `Send + Sync + 'static` and carry context as typed fields, never pre-formatted strings.
          //! Public enums are `#[non_exhaustive]` so new variants don't break downstream matches.
```
/answer f91b-2c allow
ReasoningToken: 172
Control Request:
```yaml
id: f91b-2d
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  reason: Confirm final structure of error.rs matches Q1 spec
```
/answer f91b-2d allow
Control Request:
```yaml
id: f91b-2e
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
```
/answer f91b-2e allow
[xy.ai.req:ClaudeCode:0dfae7e8-aff2-4c04-85dd-6e31554af553]