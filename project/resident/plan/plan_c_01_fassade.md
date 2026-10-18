Setze C1 um.

[include](/home/user/xyan/xy.ai.workbench/project/resident/index.md)
[include](/home/user/xyan/xy.ai.workbench/project/resident/plan/plan_d_00_overview.md)

- minimiere die Kommentare und keine direkte Referenzierung von Planelementen in Kommentaren
- Projektdokumente immer in englisch ausgeben

# Umsetzungsplan C1 — Fassade „Resident"

Stand: 10.10.2026 · Modul: `facade` (bzw. Crate-Root `resident`) · Zielsprache Rust
Quellen: `resident.md` (Einstieg/Fassade, Kontrollfluss, Klarstellung, Queue-Logik) · Recherche `context/context_c.md` §C1 (Punkte 1–9), Querschnitt 30–32.

---

## 1. Ziel & Einordnung

Die `Resident`-Struktur ist der einzige öffentliche Einstiegspunkt der Engine. Sie
- nimmt eine Konfiguration **vor** der Initialisierung entgegen,
- initialisiert sich einmalig (startet den Processor und dessen Worker-Threads),
- reiht die drei Tasktypen **Embedding / Similarity / Inferenz** ein,
- verwaltet übergebene Indizes (Residency-Tabelle, kein Persistenz-Besitz),
- stellt Ergebnisse per Event/Notify zur Abholung bereit,
- ist in Daemon/Server **und** CLI nutzbar (async-Kern + dünner blocking-Wrapper).

Die Fassade ist **lazy**: `init()` verbraucht keine Modell-/GPU-Ressourcen; diese entstehen erst beim ersten Task (Lazy-Init durch die Processor-Iteration, nicht durch das Einreihen).

---

## 2. Scope

**In:** öffentliche API-Typen, Config/Builder, Lifecycle, Task-Submission-API + `TaskHandle`, Parameterobjekte, Index-Übergabe/Residency-Tabelle, Ergebnis-/Cancel-Mechanik, blocking-Wrapper, öffentliche Fehler-Taxonomie-Fassade.

**Out (andere Pakete):** die eigentliche Orchestrierung/Worker-Loop (C2), die Expansion (C3), Subengine-Interna (Gruppe D), Index-Implementierung/-Persistenz (außerhalb der Engine, `resident.md` „Klarstellung"), Präzisions-Matching (Gruppe F, nur Feld `precision` wird durchgereicht).

---

## 3. Abhängigkeiten (Eingangsverträge)

| Vertrag | Herkunft | Nutzung in C1 |
|---|---|---|
| `Queue`, `Task`, `TaskState` | B1 | Fassade erzeugt `Task` und reiht ihn in die vom Processor gehaltene Task-Queue ein; `TaskState.result: OnceCell`, `ready: Notify`. |
| `Operation`, `Value` | B3 | nur Typ-Referenz; die Fassade erzeugt **keine** Operationen (das macht der Planner). |
| `Index`-Trait (objekt-sicher) | B4 | `add_index(Arc<dyn Index + Send + Sync>)`. |
| `Processor` | C2 | `init()` konstruiert und besitzt den Processor; `submit` delegiert an dessen Task-Queue. |

Solange B1/B3/B4 nicht vorliegen: minimale Platzhalter mit identischer Signatur definieren (siehe `plan_C_overview.md`).

---

## 4. Entscheidungen zu den offenen Punkten

Mappt die „Zu klären"-Punkte des Metaplans (Parameterobjekt-Design; Lazy-Init-Fassade) auf verankerte Entscheidungen.

### 4.1 Config-vor-Init-Lebenszyklus — **verbrauchender Builder** [B/W, context §C1.1]
- `ResidentBuilder` (via `typed-builder` oder handgeschrieben) → konsumierendes `build()` bzw. `init()` liefert ein **bereites** `Resident`.
- **Kein** Typestate (`Resident<Konfiguriert>`/`<Bereit>`), weil es keinen dauerhaft nutzbaren Zwischenzustand gibt und Typparameter `dyn`/Speicherung erschweren. Quelle: cliffle.com/blog/rust-typestate.
- Begründung: Fehlkonfiguration scheitert zur Compile-Zeit (fehlende Pflichtfelder) bzw. in `build()` (`ConfigError`).

### 4.2 Erweiterbare Parameterobjekte — **`#[non_exhaustive]` + privat + Builder** [B, context §C1.2]
- Alle öffentlichen Parameter-Structs sind `#[non_exhaustive]`, haben **private** Felder (API-Guideline C-STRUCT-PRIVATE) und `Default`, Konstruktion ausschließlich über Konstruktor/Setter (konsumierend).
- Merke: `#[non_exhaustive]`-Structs sind außerhalb des Crates **nicht** per Struct-Ausdruck konstruierbar (auch nicht mit `..Default::default()`) → ohne Builder unbenutzbar. Daher ist der Builder Pflicht.
- Enums, die erweiterbar sein sollen, sind ebenfalls `#[non_exhaustive]` (neue Varianten = kein Breaking Change). Keine versionierten Enums (`ParamsV1/V2`) — nur bei Serialisierungsverträgen sinnvoll.
- Sealed Traits (C-SEALED), falls Traits öffentlich werden.

### 4.3 Query/Passage-Modell — **Rolle als Feld, Cross-Encoder als eigener Eingabetyp** [W, context §C1.3]
- `#[non_exhaustive] pub enum TextRole { Query, Passage }` ist ein **Feld** der Embedding-Parameter, keine Parametrisierung pro Variante.
- Cross-Encoder-Paare NICHT als weitere `TextRole`-Variante, sondern eigener Eingabetyp `PairInput { query, chunk }` mit eigenem Parameterobjekt → Rolle und Zusatzsettings bleiben orthogonal.

### 4.4 Lazy-Fassade ohne Ressourcenverbrauch — **`tokio::sync::OnceCell` pro Ressource** [B, context §C1.4]
- `init()` legt nur **leere** `OnceCell`s an (pro Modell, pro Executor/Subengine). Keine Modell-/GPU-Last bei `init()`.
- Initialisierung später via `OnceCell::get_or_try_init`: gleichzeitige Aufrufer warten auf **einen** Init; bei Fehler/Abbruch/Panic wird der Versuch verworfen, ein wartender Aufrufer versucht erneut. Parallele Inits mit `Semaphore` begrenzen.
- **Verbot:** rekursive Initialisierung (Deadlock). `std::sync::OnceLock::get_or_try_init` ist in std 1.99.0 noch nightly → **nicht** verwenden; async-Pfad über `tokio::sync::OnceCell`.
- Die eigentliche Ausführung des Lazy-Init triggert die Processor-Iteration (nicht das `submit`).

### 4.5 Executor / Mindest-Idle-Threads — **eigene benannte `std::thread`s** [B, context §C1.5]
- tokio hat **keinen** „Mindest-Idle-Threads"-Schalter; der Blocking-Pool hat keinen Backpressure.
- Persistente Worker-Loops laufen als eigene benannte `std::thread`s (Mindestzahl = Zahl gestarteter Loops; sie warten per `Notify`/`park`). Diese Threads gehören dem Processor (C2); die Fassade löst nur deren Start in `init()` aus (`resident.md` Kontrollfluss: „Der interne Executor startet Anzahl min Idle-Threads").
- Für optionale async-Aufgaben (z. B. periodische Index-Persistenz-Trigger) eine `multi_thread`-Runtime, **nicht** `current_thread` (sonst frieren Hintergrund-Tasks außerhalb von `block_on` ein).

### 4.6 Ergebnisabholung — **oneshot + Notify, Level-Signal per watch** [B, context §C1.6]
- Einzelergebnis pro Task: `oneshot` (ein Wert, ein Sender/Empfänger) **oder** das `TaskState.result: OnceCell` + `ready: Notify` aus B1. C1 verwendet die B1-Mechanik und bietet darüber das `TaskHandle` an.
- Mehrere Warter / Level-Signal „bereit": `watch`. Fan-out echter Events: `broadcast`.
- **Lost-Wakeup-Reihenfolge strikt einhalten:** `notified()` erzeugen → `tokio::pin!` → `enable()` → Zustand prüfen → erst dann `.await`.

### 4.7 Task-Queueing-API — **`submit()` → `TaskHandle`** [D/W, context §C1.9, §20]
- `submit(input, params) -> TaskHandle`. `TaskHandle` implementiert `IntoFuture` (`.await` → `Result<Output, TaskError>`) und bietet zusätzlich `wait_timeout(Duration)` und `try_get()`.
- **`Drop` eines nicht abgeholten `TaskHandle` = Cancel** (CAS `Queued → Cancelled` auf `TaskState`; war der Task schon `Taken`, kein Effekt — `resident.md` Queue-Logik).
- Intern: Task mit eingebettetem Ergebnis-Kanal; Reihenfolge immer **Ergebnis zuerst in den Zustand, dann Signal**. Sender-Drop ohne Senden → `TaskError::Cancelled` (nicht als String).

### 4.8 Index-Übergabe/-Besitz — **`Arc<dyn Index>`, Residency-Tabelle, keine Persistenz** [W, context §C1.8]
- `add_index(Arc<dyn Index + Send + Sync>) -> IndexId`. `Index::stamps` returns `Box<dyn Iterator<Item = (EntryId, Stamp)> + '_>` so the trait stays object-safe. Die Engine hält eine **Residency-Tabelle** `IndexId → (Zustand, Leases)` und **kein** Persistenz-Handle.
- Der Index kapselt Mutex/Persistenz selbst (`resident.md` Klarstellung: „Ein Index Objekt … besitzt den Mutex selbst"). Die Engine ruft nur Trait-Methoden.
- `Index`-Trait objekt-sicher halten (keine Generics in Methoden). **Nie** einen `std::sync::Mutex`-Guard über `.await` halten.
- Indizes werden hinzugefügt, aber erst durch einen Task geladen (`resident.md` Kontrollfluss).

### 4.9 Eine API für Daemon und CLI [B, context §C1.7]
- Kern ist async. Für CLI/synchron: dünner `blocking`-Wrapper mit **eigener** `Runtime` (nicht die des Daemons teilen; `block_on` innerhalb einer laufenden Runtime panikt).
- Daemon-Pfad: `multi_thread`-Runtime oder Runtime in eigenem Thread + `mpsc` (Actor-Stil); Hintergrund-Loops dürfen nicht an `block_on`-Lebenszeit hängen.

---

## 5. Öffentliche API (Signatur-Skizze)

> Signaturen sind verbindlich bzgl. `#[non_exhaustive]`, Objekt-Sicherheit und Konsum-Semantik; Feldnamen sind Vorschläge.

```rust
// ---- Konfiguration ----
#[derive(Default)]
pub struct ResidentBuilder { /* privat */ }
impl ResidentBuilder {
    pub fn model(self, spec: ModelSpec) -> Self;          // fixer Modellsatz vor Init
    pub fn worker_hint(self, cfg: WorkerConfig) -> Self;  // CPU/GPU-Worker-Zahlen (C2)
    pub fn min_op_buffer(self, n: usize) -> Self;         // Mindestpuffer (C2, §18)
    pub fn init(self) -> Result<Resident, ConfigError>;   // verbrauchend → bereit
}

// ---- Fassade ----
pub struct Resident { /* Processor, OnceCells, Residency-Tabelle */ }
impl Resident {
    pub fn add_index(&self, index: std::sync::Arc<dyn Index + Send + Sync>) -> IndexId;

    pub fn submit_embedding(&self, input: TextInput, params: EmbeddingParams) -> TaskHandle;
    pub fn submit_similarity(&self, input: VectorInput, params: SimilarityParams) -> TaskHandle;
    pub fn submit_inference(&self, input: PairInput,  params: InferenceParams)  -> TaskHandle;

    pub fn shutdown(self) -> Result<(), OrchestrationError>; // Worker joinen, Leases freigeben
}

// ---- Blocking-Wrapper (CLI) ----
pub struct ResidentBlocking { /* eigene Runtime + Resident */ }
impl ResidentBlocking {
    pub fn embedding(&self, input: TextInput, params: EmbeddingParams)
        -> Result<Output, TaskError>;  // intern rt.block_on(handle)
}

// ---- Parameterobjekte (alle #[non_exhaustive], private Felder, Builder) ----
#[non_exhaustive] pub enum TextRole { Query, Passage }
pub struct EmbeddingParams { /* role: TextRole, write_index: Option<IndexId>, precision: f32, ... */ }
pub struct SimilarityParams { /* index: IndexId, top_k: usize, scope: Scope, precision: f32, ... */ }
pub struct InferenceParams { /* model, precision: f32, ... */ }

// ---- Ergebnis-Handle ----
pub struct TaskHandle { /* Arc<TaskState>, task_id */ }
impl std::future::IntoFuture for TaskHandle { type Output = Result<Output, TaskError>; /* ... */ }
impl TaskHandle {
    // async on purpose: a blocking wait inside a running runtime panics; sync callers use ResidentBlocking
    pub async fn wait_timeout(self, d: std::time::Duration) -> Result<Output, TaskError>;
    pub fn try_get(&self) -> Option<Result<Output, TaskError>>;
}
impl Drop for TaskHandle { /* Cancel: CAS Queued→Cancelled */ }
```

- `precision: f32` wird nur **durchgereicht** (geklemmt/validiert in Gruppe F). C1 lehnt `NaN` ab (`ConfigError`/`TaskError::InvalidPrecision`).
- `write_index: Option<IndexId>` deckt „Embedding schreibt direkt in den Index" ab (`resident.md` Beispiel).

---

## 6. Umsetzungsschritte

1. **Modul-Grundgerüst** `facade/` anlegen (abhängig von A2-Layout). Platzhalter-Traits für B1/B3/B4 einziehen, falls noch nicht vorhanden.
2. **Fehler-Taxonomie-Fassade** (Q1): `ConfigError`, öffentlicher opaker `ResidentError`/`TaskError` mit `#[from]`, `#[non_exhaustive]`, `Cancelled`-Variante.
3. **Parameterobjekte** (4.2/4.3): `TextRole`, `EmbeddingParams`, `SimilarityParams`, `InferenceParams`, `PairInput` mit Buildern; `precision`-Validierung.
4. **Builder + Lifecycle** (4.1/4.5): `ResidentBuilder::init()` konstruiert Processor (C2) und löst Worker-Thread-Start aus; `OnceCell`-Zellen leer anlegen (4.4).
5. **Index-Verwaltung** (4.8): Residency-Tabelle `IndexId → (Zustand, Leases)`; `add_index` registriert `Arc<dyn Index>`.
6. **Submission-API + TaskHandle** (4.6/4.7): `submit_*` baut `Task` (B1) und reiht in die Task-Queue des Processors ein; `TaskHandle` mit `IntoFuture`/`wait_timeout`/`try_get`/`Drop=Cancel`.
7. **Blocking-Wrapper** (4.9): `ResidentBlocking` mit eigener `multi_thread`-Runtime.
8. **Logging** (Q2): `#[tracing::instrument]`/manueller Span pro `submit` mit `task_id` (Feld via `field::Empty` + `record`).
9. **Tests** (siehe §9).

**Deliverables:** kompilierendes `facade`-Modul ohne `cuda`-Feature, dokumentierte öffentliche API, Tests grün im No-GPU-Pfad.

---

## 7. Fehlerbehandlung (Q1) [B, context §30]
- `ConfigError` (fehlende Pflichtfelder, unbekanntes Modell, ungültige `precision`), `TaskError` (`Cancelled`, `Timeout`, `InvalidPrecision`, nach außen gereichte Orchestrierungs-/Planner-Fehler).
- Öffentlicher Fehler opak (`pub struct ResidentError(#[from] ErrorRepr)`), Repr änderbar ohne Breaking Change; `#[non_exhaustive]`.
- Gleicher Fehler an mehrere Warter (watch/broadcast): `Arc<TaskError>` oder `Clone`.

## 8. Logging (Q2) [B/W, context §31]
- Span pro Task-ID bei `submit`, Felder `task_id`, `task_type` (via `field::Empty`+`record`). **Kein** `Span::enter()`-Guard über `.await` → `Instrument`.
- Fassade ruft **nie** `set_global_default` (Bibliotheks-Konvention).

## 9. Teststrategie & Mocks (Q3) [B/W, context §29/§32]
- **No-GPU:** Processor-/Subengine-Stub, der Tasks sofort mit Dummy-Output abschließt → testet Submission, Handle-Semantik, Cancel-on-Drop, Timeout.
- `proptest`: Invarianten „jeder `submit` erzeugt genau ein Ergebnis oder `Cancelled`", „`Drop` vor Abschluss ⇒ `Cancelled`".
- Builder-Tests: Pflichtfeld fehlt ⇒ `ConfigError`; `precision = NaN` ⇒ Fehler.
- Index: `add_index` registriert, lädt aber nichts (Residency-Zustand = nicht geladen) bis ein Task läuft.

## 10. Feature-Flags (Q4) [D/W, context §32]
- `facade` ist vollständig ohne `cuda` übersetz- und testbar. CUDA betrifft nur Subengines (D) und die Completion-Strategie (C2).

## 11. Offene Risiken / zu verifizieren
- `once_cell`-Crate (stabiles `get_or_try_init`) nicht geprüft [W] → beim async-Init bei `tokio::sync::OnceCell` bleiben.
- Genaue Form von B1-`Task`/`TaskState` kann die `TaskHandle`-Implementierung beeinflussen → nach B1-Fertigstellung angleichen.
- Grenze Engine ↔ Suchschicht (A1′ „Scope-Charta") blockiert laut Metaplan das endgültige API-Design der `SimilarityParams.scope` → vor Freeze der öffentlichen API bestätigen.

SystemInit:  id=228d7642-db5c-48ab-ad16-d8d4c4b4e668, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 I'll start by inspecting the existing facade and related modules.
Control Request:
```yaml
id: 4eac-91
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
  reason: "Understand existing facade, processor, planner stubs before implementing C1"
```
/answer 4eac-91 allow
ReasoningToken: 35
Control Request:
```yaml
id: 4eac-92
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Public resident facade API.
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Orchestration: task queue + op queues."
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/planner/mod.rs
      nodes:
      - id: ModuleDoc
        signature: //! Stateless task expansion.
```
/answer 4eac-92 allow
Control Request:
```yaml
id: 4eac-93
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: pub (struct|trait|enum) (Queue|Task|TaskState|Index|Value|Operation|Processor|IndexId)\b
    include:
    - '*.rs'
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^(tokio|thiserror|tracing|typed-builder|once_cell|\\[features\\]|cuda)"
    include:
    - Cargo.toml
    exclude:
    - target/**
    - .bin/**
  reason: Locate B1/B3/B4 contract types and available dependencies
```
/answer 4eac-93 allow
Control Request:
```yaml
id: 4eac-94
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: index/mod.rs
        matches:
        - lineno: 158
          match: "pub trait Index {"
      - path: queue/queue.rs
        matches:
        - lineno: 12
          match: "pub struct Queue<P, D, O, E> {"
      - path: queue/state.rs
        matches:
        - lineno: 34
          match: "pub struct TaskState<O, E> {"
      - path: op/mod.rs
        matches:
        - lineno: 87
          match: "pub enum Value {"
      - path: op/requirements.rs
        matches:
        - lineno: 7
          match: pub struct IndexId(pub String);
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
        - lineno: 49
          match: "cudarc = { version = \"=0.19.9\", optional = true }"
        - lineno: 52
          match: "[features]"
        - lineno: 55
          match: "cuda = [\"candle\", \"dep:candle-transformers\", \"dep:cudarc\", \"candle-core/cuda\", \"candle-transformers/cuda\"]"
        - lineno: 66
          match: "tokio = { version = \"1\", features = [\"test-util\"] }"
```
/answer 4eac-94 allow
Control Request:
```yaml
id: 4eac-95
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/state.rs
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/queue.rs
    ids: []
  reason: Read B1 contracts
```
/answer 4eac-95 allow
Control Request:
```yaml
id: 4eac-96
result:
  structuredContent:
    results:
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
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
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
      errors:
      - "ids [] resolved to no node; redirected to ast_outline, returning the full outline instead."
```
/answer 4eac-96 allow
Control Request:
```yaml
id: 4eac-97
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/op/requirements.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/mod.rs
  reason: See module structure and exports
```
/answer 4eac-97 allow
Control Request:
```yaml
id: 4eac-98
result:
  structuredContent:
    results:
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
      nodes:
      - id: ModuleDoc
        signature: "//! Index trait, Scope, AccessMethod, Stamp."
      - id: memory
        signature: pub mod memory;
      - id: stamp
        signature: pub mod stamp;
      - id: kAsHBt|OqD8dC
        signature: use std::ops::Range;
      - id: EntryId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]"
      - id: impl_EntryId
        signature: "impl EntryId {"
        children:
        - id: impl_EntryId.new
          signature: "pub const fn new(raw: u64) -> Self {"
        - id: impl_EntryId.get
          signature: "pub const fn get(self) -> u64 {"
      - id: NodeId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      - id: CellId
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]"
      - id: Stamp
        signature: /// Opaque change marker. Compare with `!=` only; ordering is deliberately abse…
        docstring: Opaque change marker. Compare with `!=` only; ordering is deliberately absent.
      - id: impl_Stamp
        signature: "impl Stamp {"
        children:
        - id: impl_Stamp.from_raw
          signature: "pub(crate) const fn from_raw(raw: u64) -> Self {"
        - id: impl_Stamp.raw
          signature: "/// Raw value, e.g. as `watch` watermark or for persistence. pub const fn raw(s…"
          docstring: "Raw value, e.g. as `watch` watermark or for persistence."
      - id: AccessMethod
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] #[non_exhaustive] pub enum A…"
      - id: Scope
        signature: "#[derive(Clone, Debug, PartialEq)] pub enum Scope {"
      - id: EntrySet
        signature: "/// Compact set of entry ids stored as sorted, merged, half-open ranges. #[deri…"
        docstring: "Compact set of entry ids stored as sorted, merged, half-open ranges."
      - id: impl_EntrySet
        signature: "impl EntrySet {"
        children:
        - id: impl_EntrySet.new
          signature: "pub fn new() -> Self {"
        - id: impl_EntrySet.from_ranges
          signature: "pub fn from_ranges(ranges: impl IntoIterator<Item = Range<u64>>) -> Self {"
        - id: impl_EntrySet.from_ids
          signature: "pub fn from_ids(ids: impl IntoIterator<Item = EntryId>) -> Self {"
        - id: impl_EntrySet.ranges
          signature: "pub fn ranges(&self) -> &[Range<u64>] {"
        - id: impl_EntrySet.is_empty
          signature: "pub fn is_empty(&self) -> bool {"
        - id: impl_EntrySet.len
          signature: "pub fn len(&self) -> u64 {"
        - id: impl_EntrySet.contains
          signature: "pub fn contains(&self, id: EntryId) -> bool {"
        - id: impl_EntrySet.iter
          signature: "pub fn iter(&self) -> impl Iterator<Item = EntryId> + '_ {"
      - id: EntryData
        signature: /// Variable-length vector region of one entry; never a copy of backing storage…
        docstring: Variable-length vector region of one entry; never a copy of backing storage.
      - id: impl_EntryData____
        signature: "impl EntryData<'_> {"
        children:
        - id: impl_EntryData____.as_bytes
          signature: "pub fn as_bytes(&self) -> &[u8] {"
      - id: SummaryRef
        signature: "/// Summary structure such as a centroid table, exposed as vector data."
        docstring: "Summary structure such as a centroid table, exposed as vector data."
      - id: HashAlgo
        signature: "#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[non_exhaustive] pub enum HashAlg…"
      - id: IndexHeader
        signature: /// Binding between index and model; persistence format is owned by the index. …
        docstring: Binding between index and model; persistence format is owned by the index.
      - id: Index
        signature: "pub trait Index {"
        children:
        - id: Index.stamp
          signature: /// `None` = entry removed.
          docstring: '`None` = entry removed.'
        - id: Index.stamps
          signature: /// Must not touch vector data.
          docstring: Must not touch vector data.
        - id: Index.read
          signature: /// Missing entries yield empty data. May block on I/O.
          docstring: Missing entries yield empty data. May block on I/O.
        - id: Index.apply
          signature: /// Serializes writers (last writer wins in dequeue order). Returns the stamp
          docstring: Serializes writers (last writer wins in dequeue order). Returns the stamp
        - id: Index.changed
          signature: /// Level signal carrying the latest stamp watermark. Consume with
          docstring: Level signal carrying the latest stamp watermark. Consume with
        - id: Index.capabilities
          signature: "fn capabilities(&self) -> &[AccessMethod];"
        - id: Index.children
          signature: "fn children(&self, node: NodeId) -> EntrySet;"
        - id: Index.members
          signature: "fn members(&self, cell: CellId) -> EntrySet;"
        - id: Index.summary
          signature: "fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>;"
        - id: Index.prefetch
          signature: /// Hint only; may be a no-op.
          docstring: Hint only; may be a no-op.
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
```
/answer 4eac-98 allow
Control Request:
```yaml
id: 4eac-99
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/state.rs
    ids:
    - TaskState
    - impl_TaskState_O_E_.new
    - impl_TaskState_O_E_.claim
    - impl_TaskState_O_E_.complete
    - impl_TaskState_O_E_.cancel
    - impl_TaskState_O_E_.result
    - impl_TaskState_O_E_.wait
    - ClaimState
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
    ids:
    - Index
    - Scope
  reason: Need exact TaskState API and Index trait for handle and add_index
```
/answer 4eac-99 allow
ReasoningToken: 62
Control Request:
```yaml
id: 4eac-9a
result:
  structuredContent:
    results:
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/index/mod.rs
      nodes:
      - id: Index
        type: ItemTrait
        signature: "pub trait Index {"
        children:
        - id: Index.stamp
          type: TraitItemFn
          code: |2
                /// `None` = entry removed.
                fn stamp(&self, entry: EntryId) -> Option<Stamp>;
        - id: Index.stamps
          type: TraitItemFn
          code: |2
                /// Must not touch vector data.
                fn stamps(&self) -> impl Iterator<Item = (EntryId, Stamp)>;
        - id: Index.read
          type: TraitItemFn
          code: |2
                /// Missing entries yield empty data. May block on I/O.
                fn read(&self, entry: EntryId) -> EntryData<'_>;
        - id: Index.apply
          type: TraitItemFn
          code: |2
                /// Serializes writers (last writer wins in dequeue order). Returns the stamp
                /// of this very write; callers register it as known to suppress their echo.
                fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp;
        - id: Index.changed
          type: TraitItemFn
          code: |2
                /// Level signal carrying the latest stamp watermark. Consume with
                /// `borrow_and_update()`; coalesced signals are harmless.
                fn changed(&self) -> watch::Receiver<u64>;
        - id: Index.capabilities
          type: TraitItemFn
          code: |2
                fn capabilities(&self) -> &[AccessMethod];
        - id: Index.children
          type: TraitItemFn
          code: |2
                fn children(&self, node: NodeId) -> EntrySet;
        - id: Index.members
          type: TraitItemFn
          code: |2
                fn members(&self, cell: CellId) -> EntrySet;
        - id: Index.summary
          type: TraitItemFn
          code: |2
                fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>;
        - id: Index.prefetch
          type: TraitItemFn
          code: |2
                /// Hint only; may be a no-op.
                fn prefetch(&self, set: &EntrySet);
      - id: Scope
        type: ItemEnum
        code: |
          #[derive(Clone, Debug, PartialEq)]
          pub enum Scope {
              All,
              Range(Range<u64>),
              Set(EntrySet),
              Subtree(NodeId),
              Prefix(Vec<u8>),
              Centroid { k: u32, radius: Option<f32> },
          }
```
/answer 4eac-9a allow
Control Request:
```yaml
id: 4eac-9b
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: ^\s*(pub|pub\(crate\)) (mod|use|struct|enum|trait|fn|type)
    include:
    - error.rs
    - task.rs
    - telemetry.rs
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: ModelSpec|struct WorkerConfig|enum Output
    include:
    - '*.rs'
    exclude:
    - .bin/**
    - target/**
    - .xyrag/**
  reason: "Check error module, Task definition, and whether ModelSpec/WorkerConfig exist"
```
/answer 4eac-9b allow
Control Request:
```yaml
id: 4eac-9c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: error.rs
        matches:
        - lineno: 8
          match: "pub trait IsRetryable {"
        - lineno: 14
          match: "pub enum IndexError {"
        - lineno: 39
          match: "pub(crate) enum ManagerError {"
        - lineno: 51
          match: "pub enum PrecisionError {"
        - lineno: 69
          match: "pub enum PlannerError {"
        - lineno: 85
          match: "pub enum CapabilityError {"
        - lineno: 94
          match: "pub enum ConfigError {"
        - lineno: 115
          match: "pub enum ResidentError {"
      - path: telemetry.rs
        matches:
        - lineno: 36
          match: "pub fn init_tracing() {"
        - lineno: 44
          match: "pub fn init_tracing_non_blocking<W>(writer: W, json: bool) -> WorkerGuard"
        - lineno: 66
          match: "pub fn task_span(task_id: &str) -> Span {"
        - lineno: 71
          match: "pub fn resource_span(task_id: &str, resource_kind: &str) -> Span {"
        - lineno: 76
          match: "pub fn manager_cycle_span(manager_cycle: u64) -> Span {"
        - lineno: 81
          match: "pub fn for_blocking(span: &Span) -> Span {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches: []
```
/answer 4eac-9c allow
Control Request:
```yaml
id: 4eac-9d
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/telemetry.rs
  reason: "Read error taxonomy, Task definition and span helpers"
```
/answer 4eac-9d allow
ReasoningToken: 126
Control Request:
```yaml
id: 4eac-9e
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
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
      nodes:
      - id: ModuleDoc
        signature: //! Task-queue instantiation notes.
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
/answer 4eac-9e allow
Control Request:
```yaml
id: 4eac-9f
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: dyn Index|object.safe|dyn-compat|DynIndex
    include:
    - '*.rs'
  reason: Check how Index is meant to be held as trait object
```
/answer 4eac-9f allow
Control Request:
```yaml
id: 4eac-a0
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches: []
```
/answer 4eac-a0 allow
Control Request:
```yaml
id: 4eac-a1
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
    ids:
    - ConfigError
    - ResidentError
    - PlannerError
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/task.rs
    ids:
    - ModuleDoc
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/telemetry.rs
    ids:
    - task_span
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/queue/queue.rs
    ids:
    - Queue
    - impl_Queue_P_D_O_E_.new
    - impl_Queue_P_D_O_E_.push
  reason: Need exact variants and queue push signature to build submit
```
/answer 4eac-a1 allow
Control Request:
```yaml
id: 4eac-a2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/error.rs
      nodes:
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
```
/answer 4eac-a2 allow
ReasoningToken: 2782
Control Request:
```yaml
id: 4eac-a3
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident
    pattern: "pub (\\(crate\\) )?(fn|struct|type|use)|^impl|pub .*:"
    include:
    - node.rs
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    pattern: pub struct (ModelId|IndexId)
    include:
    - requirements.rs
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag
    pattern: "^(proptest|arc-swap|loom|\\[dev-dep|\\[dependencies|mock)"
    include:
    - Cargo.toml
    exclude:
    - .xyrag/**
    - .bin/**
  reason: "Need Node accessors, ModelId shape, and dev-dependencies for tests"
```
/answer 4eac-a3 allow
Control Request:
```yaml
id: 4eac-a4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident
      matches:
      - path: queue/node.rs
        matches:
        - lineno: 20
          match: "pub struct Node<P, D, O, E> {"
        - lineno: 25
          match: "impl<P, D, O, E> Node<P, D, O, E> {"
        - lineno: 33
          match: "pub fn new(params: P, state: Arc<TaskState<O, E>>, data: D) -> Arc<Self> {"
        - lineno: 45
          match: "pub fn params(&self) -> Option<&P> {"
        - lineno: 52
          match: "pub fn task_state(&self) -> Option<&Arc<TaskState<O, E>>> {"
        - lineno: 61
          match: "pub fn is_live(&self) -> bool {"
        - lineno: 72
          match: "pub fn claim(&self) -> Option<D> {"
        - lineno: 103
          match: "impl<P, D, O, E> Drop for Node<P, D, O, E> {"
        - lineno: 119
          match: "pub struct Iter<P, D, O, E> {"
        - lineno: 123
          match: "impl<P, D, O, E> Iter<P, D, O, E> {"
        - lineno: 129
          match: "impl<P, D, O, E> Iterator for Iter<P, D, O, E> {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      matches:
      - path: requirements.rs
        matches:
        - lineno: 5
          match: pub struct ModelId(pub String);
        - lineno: 7
          match: pub struct IndexId(pub String);
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag
      matches:
      - path: third_party/candle-kernels/Cargo.toml
        matches:
        - lineno: 37
          match: "[dependencies]"
      - path: .xyrag/layers/trigram/mirror/Cargo.toml
        matches:
        - lineno: 34
          match: proptest 1
      - path: Cargo.toml
        matches:
        - lineno: 15
          match: "[dependencies]"
        - lineno: 39
          match: arc-swap = "1.9.2"
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
```
/answer 4eac-a4 allow
ReasoningToken: 3491
Control Request:
```yaml
id: 4eac-a5
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue
    pattern: type Item|pub fn iter(_live)?\(
    include:
    - node.rs
    - queue.rs
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    pattern: mod |pub use
    include:
    - mod.rs
  reason: Confirm iterator item type and op module paths
```
/answer 4eac-a5 allow
Control Request:
```yaml
id: 4eac-a6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue
      matches:
      - path: queue.rs
        matches:
        - lineno: 62
          match: "pub fn iter(&self) -> Iter<P, D, O, E> {"
        - lineno: 66
          match: "pub fn iter_live(&self) -> impl Iterator<Item = Arc<Node<P, D, O, E>>> {"
      - path: node.rs
        matches:
        - lineno: 130
          match: "type Item = Arc<Node<P, D, O, E>>;"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      matches:
      - path: mod.rs
        matches:
        - lineno: 3
          match: mod lease;
        - lineno: 4
          match: mod requirements;
        - lineno: 9
          match: "pub use lease::{DeviceHandle, Lease, PinCount};"
        - lineno: 10
          match: "pub use requirements::{"
        - lineno: 179
          match: "mod tests {"
```
/answer 4eac-a6 allow
Control Request:
```yaml
id: 4eac-a7
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
    pattern: ModelId|IndexId
    include:
    - mod.rs
    limit: 5
  - directory:
    - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue
    pattern: pub use|pub mod
    include:
    - mod.rs
  reason: Confirm re-exports of ModelId/IndexId and Queue/TaskState from queue module
```
/answer 4eac-a7 allow
Control Request:
```yaml
id: 4eac-a8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/op
      matches:
      - path: mod.rs
        matches:
        - lineno: 11
          match: "Affinity, DeviceId, EngineSelector, IndexId, ModelId, Requirements, ResourceKind,"
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/queue
      matches:
      - path: mod.rs
        matches:
        - lineno: 17
          match: pub mod task;
        - lineno: 19
          match: "pub use node::{Iter, Node};"
        - lineno: 20
          match: pub use queue::Queue;
        - lineno: 21
          match: "pub use state::{ClaimState, TaskState};"
```
/answer 4eac-a8 allow
ReasoningToken: 973
Control Request:
```yaml
id: 4eac-a9
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
    overwrite: true
    source: |
      //! Public resident facade API.

      mod blocking;
      mod error;
      mod handle;
      mod params;
      mod resident;
      mod task;

      pub use blocking::ResidentBlocking;
      pub use error::TaskError;
      pub use handle::TaskHandle;
      pub use params::{
          EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, TextRole,
          VectorInput,
      };
      pub use resident::{
          IndexInfo, Residency, Resident, ResidentBuilder, ResidentConfig, WorkerConfig,
      };
      pub use task::{Payload, TaskKind, TaskMeta, TaskQueue};
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/error.rs
    source: |
      use std::sync::Arc;

      use thiserror::Error;

      use crate::resident::error::ResidentError;

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

      impl From<ResidentError> for TaskError {
          fn from(err: ResidentError) -> Self {
              Self::Engine(Arc::new(err))
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
    source: |
      use crate::resident::index::{EntryId, Scope};
      use crate::resident::op::{IndexId, ModelId};

      const DEFAULT_PRECISION: f32 = 1.0;
      const DEFAULT_TOP_K: usize = 10;

      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
      #[non_exhaustive]
      pub enum TextRole {
          Query,
          #[default]
          Passage,
      }

      #[derive(Clone, Debug, Default)]
      #[non_exhaustive]
      pub struct TextInput {
          texts: Vec<String>,
      }

      impl TextInput {
          pub fn new(texts: impl IntoIterator<Item = impl Into<String>>) -> Self {
              Self {
                  texts: texts.into_iter().map(Into::into).collect(),
              }
          }

          pub fn single(text: impl Into<String>) -> Self {
              Self {
                  texts: vec![text.into()],
              }
          }

          pub fn texts(&self) -> &[String] {
              &self.texts
          }
      }

      #[derive(Clone, Debug, Default)]
      #[non_exhaustive]
      pub struct VectorInput {
          vectors: Vec<Vec<f32>>,
      }

      impl VectorInput {
          pub fn new(vectors: Vec<Vec<f32>>) -> Self {
              Self { vectors }
          }

          pub fn vectors(&self) -> &[Vec<f32>] {
              &self.vectors
          }
      }

      /// Cross-encoder input; kept apart from [`TextRole`] so role and pair scoring stay orthogonal.
      #[derive(Clone, Debug)]
      #[non_exhaustive]
      pub struct PairInput {
          query: String,
          chunk: String,
      }

      impl PairInput {
          pub fn new(query: impl Into<String>, chunk: impl Into<String>) -> Self {
              Self {
                  query: query.into(),
                  chunk: chunk.into(),
              }
          }

          pub fn query(&self) -> &str {
              &self.query
          }

          pub fn chunk(&self) -> &str {
              &self.chunk
          }
      }

      #[derive(Clone, Debug)]
      #[non_exhaustive]
      pub struct EmbeddingParams {
          role: TextRole,
          write_index: Option<IndexId>,
          precision: f32,
      }

      impl EmbeddingParams {
          pub fn new(role: TextRole) -> Self {
              Self {
                  role,
                  write_index: None,
                  precision: DEFAULT_PRECISION,
              }
          }

          pub fn write_index(mut self, index: IndexId) -> Self {
              self.write_index = Some(index);
              self
          }

          pub fn precision(mut self, precision: f32) -> Self {
              self.precision = precision;
              self
          }

          pub fn role(&self) -> TextRole {
              self.role
          }

          pub fn target_index(&self) -> Option<&IndexId> {
              self.write_index.as_ref()
          }

          pub fn get_precision(&self) -> f32 {
              self.precision
          }
      }

      impl Default for EmbeddingParams {
          fn default() -> Self {
              Self::new(TextRole::default())
          }
      }

      #[derive(Clone, Debug)]
      #[non_exhaustive]
      pub struct SimilarityParams {
          index: IndexId,
          top_k: usize,
          scope: Scope,
          precision: f32,
      }

      impl SimilarityParams {
          pub fn new(index: IndexId) -> Self {
              Self {
                  index,
                  top_k: DEFAULT_TOP_K,
                  scope: Scope::All,
                  precision: DEFAULT_PRECISION,
              }
          }

          pub fn top_k(mut self, top_k: usize) -> Self {
              self.top_k = top_k;
              self
          }

          pub fn scope(mut self, scope: Scope) -> Self {
              self.scope = scope;
              self
          }

          pub fn precision(mut self, precision: f32) -> Self {
              self.precision = precision;
              self
          }

          pub fn index(&self) -> &IndexId {
              &self.index
          }

          pub fn get_top_k(&self) -> usize {
              self.top_k
          }

          pub fn get_scope(&self) -> &Scope {
              &self.scope
          }

          pub fn get_precision(&self) -> f32 {
              self.precision
          }
      }

      #[derive(Clone, Debug)]
      #[non_exhaustive]
      pub struct InferenceParams {
          model: Option<ModelId>,
          precision: f32,
      }

      impl InferenceParams {
          pub fn new() -> Self {
              Self {
                  model: None,
                  precision: DEFAULT_PRECISION,
              }
          }

          pub fn model(mut self, model: ModelId) -> Self {
              self.model = Some(model);
              self
          }

          pub fn precision(mut self, precision: f32) -> Self {
              self.precision = precision;
              self
          }

          pub fn get_model(&self) -> Option<&ModelId> {
              self.model.as_ref()
          }

          pub fn get_precision(&self) -> f32 {
              self.precision
          }
      }

      impl Default for InferenceParams {
          fn default() -> Self {
              Self::new()
          }
      }

      #[derive(Clone, Debug, PartialEq)]
      #[non_exhaustive]
      pub enum Output {
          Embeddings(Vec<Vec<f32>>),
          Hits(Vec<(EntryId, f32)>),
          Scores(Vec<f32>),
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
    source: |
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
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/handle.rs
    source: |
      use std::future::{Future, IntoFuture};
      use std::pin::Pin;
      use std::sync::Arc;
      use std::time::Duration;

      use super::error::TaskError;
      use super::params::Output;
      use crate::resident::queue::{ClaimState, TaskState};

      pub(crate) type State = TaskState<Output, TaskError>;

      /// Dropping an unfinished handle cancels the task if no worker has claimed it yet.
      pub struct TaskHandle {
          state: Arc<State>,
          id: u64,
      }

      impl TaskHandle {
          pub(crate) fn new(state: Arc<State>, id: u64) -> Self {
              Self { state, id }
          }

          pub fn id(&self) -> u64 {
              self.id
          }

          pub fn try_get(&self) -> Option<Result<Output, TaskError>> {
              match self.state.claim_state() {
                  ClaimState::Done => Some(
                      self.state
                          .result()
                          .cloned()
                          .unwrap_or(Err(TaskError::Cancelled)),
                  ),
                  ClaimState::Cancelled => Some(Err(TaskError::Cancelled)),
                  ClaimState::Queued | ClaimState::Taken => None,
              }
          }

          /// On timeout the handle is dropped, which cancels the task if still queued.
          pub async fn wait_timeout(self, limit: Duration) -> Result<Output, TaskError> {
              tokio::time::timeout(limit, self.resolve())
                  .await
                  .unwrap_or(Err(TaskError::Timeout))
          }

          async fn resolve(self) -> Result<Output, TaskError> {
              self.state.wait().await;
              self.try_get().unwrap_or(Err(TaskError::Cancelled))
          }
      }

      impl IntoFuture for TaskHandle {
          type Output = Result<Output, TaskError>;
          type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

          fn into_future(self) -> Self::IntoFuture {
              Box::pin(self.resolve())
          }
      }

      impl Drop for TaskHandle {
          fn drop(&mut self) {
              self.state.cancel();
          }
      }

      impl std::fmt::Debug for TaskHandle {
          fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
              f.debug_struct("TaskHandle")
                  .field("id", &self.id)
                  .field("state", &self.state.claim_state())
                  .finish()
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/blocking.rs
    source: |
      use std::future::IntoFuture;
      use std::io;

      use tokio::runtime::{Builder, Runtime};

      use super::error::TaskError;
      use super::handle::TaskHandle;
      use super::params::{
          EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, VectorInput,
      };
      use super::resident::Resident;

      /// Owns a private runtime; must not be used from inside another tokio runtime.
      pub struct ResidentBlocking {
          rt: Runtime,
          inner: Resident,
      }

      impl ResidentBlocking {
          pub fn new(inner: Resident) -> io::Result<Self> {
              let rt = Builder::new_multi_thread()
                  .worker_threads(1)
                  .thread_name("resident-blocking")
                  .enable_all()
                  .build()?;
              Ok(Self { rt, inner })
          }

          pub fn resident(&self) -> &Resident {
              &self.inner
          }

          pub fn embedding(
              &self,
              input: TextInput,
              params: EmbeddingParams,
          ) -> Result<Output, TaskError> {
              self.wait(self.inner.submit_embedding(input, params))
          }

          pub fn similarity(
              &self,
              input: VectorInput,
              params: SimilarityParams,
          ) -> Result<Output, TaskError> {
              self.wait(self.inner.submit_similarity(input, params))
          }

          pub fn inference(
              &self,
              input: PairInput,
              params: InferenceParams,
          ) -> Result<Output, TaskError> {
              self.wait(self.inner.submit_inference(input, params))
          }

          fn wait(&self, handle: TaskHandle) -> Result<Output, TaskError> {
              self.rt.block_on(handle.into_future())
          }
      }
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
    source: |
      use std::any::Any;
      use std::collections::HashMap;
      use std::sync::atomic::{AtomicU64, Ordering};
      use std::sync::{Arc, Mutex, PoisonError};

      use super::error::TaskError;
      use super::handle::{State, TaskHandle};
      use super::params::{
          EmbeddingParams, InferenceParams, PairInput, SimilarityParams, TextInput, VectorInput,
      };
      use super::task::{Payload, TaskKind, TaskMeta, TaskQueue};
      use crate::resident::error::{ConfigError, ResidentError};
      use crate::resident::index::Index;
      use crate::resident::op::{IndexId, ModelId};
      use crate::resident::queue::{Queue, TaskState};
      use crate::resident::telemetry::task_span;

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      #[non_exhaustive]
      pub struct WorkerConfig {
          pub cpu: usize,
          pub gpu: usize,
      }

      impl WorkerConfig {
          pub fn new(cpu: usize, gpu: usize) -> Self {
              Self { cpu, gpu }
          }
      }

      impl Default for WorkerConfig {
          fn default() -> Self {
              Self::new(1, 0)
          }
      }

      #[derive(Clone, Debug)]
      #[non_exhaustive]
      pub struct ResidentConfig {
          model: ModelId,
          workers: WorkerConfig,
          min_op_buffer: usize,
      }

      impl ResidentConfig {
          pub fn model(&self) -> &ModelId {
              &self.model
          }

          pub fn workers(&self) -> WorkerConfig {
              self.workers
          }

          pub fn min_op_buffer(&self) -> usize {
              self.min_op_buffer
          }
      }

      #[derive(Debug, Default)]
      pub struct ResidentBuilder {
          model: Option<ModelId>,
          workers: WorkerConfig,
          min_op_buffer: Option<usize>,
      }

      impl ResidentBuilder {
          pub fn new() -> Self {
              Self::default()
          }

          pub fn model(mut self, model: ModelId) -> Self {
              self.model = Some(model);
              self
          }

          pub fn worker_hint(mut self, workers: WorkerConfig) -> Self {
              self.workers = workers;
              self
          }

          pub fn min_op_buffer(mut self, n: usize) -> Self {
              self.min_op_buffer = Some(n);
              self
          }

          /// Acquires no model or device resources; those are created by the first task.
          pub fn init(self) -> Result<Resident, ConfigError> {
              let model = self.model.ok_or_else(|| ConfigError::MissingField {
                  field: "model".into(),
              })?;
              let min_op_buffer = self.min_op_buffer.unwrap_or(1);
              if min_op_buffer == 0 {
                  return Err(ConfigError::InvalidValue {
                      field: "min_op_buffer".into(),
                      reason: "must be at least 1".into(),
                  });
              }
              Ok(Resident {
                  config: ResidentConfig {
                      model,
                      workers: self.workers,
                      min_op_buffer,
                  },
                  queue: Arc::new(Queue::new(Vec::new())),
                  next_task: AtomicU64::new(0),
                  next_index: AtomicU64::new(0),
                  indexes: Mutex::new(HashMap::new()),
              })
          }
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      #[non_exhaustive]
      pub enum Residency {
          NotLoaded,
          Loaded,
      }

      #[derive(Clone, Copy, Debug, PartialEq, Eq)]
      #[non_exhaustive]
      pub struct IndexInfo {
          pub residency: Residency,
          pub leases: usize,
      }

      struct IndexEntry {
          info: IndexInfo,
          _index: Arc<dyn Any + Send + Sync>,
      }

      pub struct Resident {
          config: ResidentConfig,
          queue: Arc<TaskQueue>,
          next_task: AtomicU64,
          next_index: AtomicU64,
          indexes: Mutex<HashMap<IndexId, IndexEntry>>,
      }

      impl Resident {
          pub fn builder() -> ResidentBuilder {
              ResidentBuilder::new()
          }

          pub fn config(&self) -> &ResidentConfig {
              &self.config
          }

          pub fn queue(&self) -> &Arc<TaskQueue> {
              &self.queue
          }

          /// Registers the index without loading it; loading is triggered by a task.
          /// `Index` has return-position `impl Trait` and is not dyn-compatible, so the
          /// table keeps the concrete index type-erased.
          pub fn add_index<I>(&self, index: Arc<I>) -> IndexId
          where
              I: Index + Send + Sync + 'static,
          {
              let id = IndexId(format!("idx-{}", self.next_index.fetch_add(1, Ordering::Relaxed)));
              self.lock_indexes().insert(
                  id.clone(),
                  IndexEntry {
                      info: IndexInfo {
                          residency: Residency::NotLoaded,
                          leases: 0,
                      },
                      _index: index,
                  },
              );
              id
          }

          pub fn index_info(&self, id: &IndexId) -> Option<IndexInfo> {
              self.lock_indexes().get(id).map(|entry| entry.info)
          }

          pub fn submit_embedding(&self, input: TextInput, params: EmbeddingParams) -> TaskHandle {
              let precision = params.get_precision();
              self.submit(
                  TaskKind::Embedding,
                  precision,
                  Payload::Embedding { input, params },
              )
          }

          pub fn submit_similarity(&self, input: VectorInput, params: SimilarityParams) -> TaskHandle {
              let precision = params.get_precision();
              self.submit(
                  TaskKind::Similarity,
                  precision,
                  Payload::Similarity { input, params },
              )
          }

          pub fn submit_inference(&self, input: PairInput, params: InferenceParams) -> TaskHandle {
              let precision = params.get_precision();
              self.submit(
                  TaskKind::Inference,
                  precision,
                  Payload::Inference { input, params },
              )
          }

          /// Cancels every task that no worker has claimed yet.
          pub fn shutdown(self) -> Result<(), ResidentError> {
              drop(self);
              Ok(())
          }

          fn submit(&self, kind: TaskKind, precision: f32, payload: Payload) -> TaskHandle {
              let id = self.next_task.fetch_add(1, Ordering::Relaxed);
              let state: Arc<State> = Arc::new(TaskState::new());
              let handle = TaskHandle::new(state.clone(), id);
              task_span(&id.to_string()).in_scope(|| {
                  if precision.is_nan() {
                      state.complete(Err(TaskError::InvalidPrecision));
                  } else {
                      self.queue.push(TaskMeta { id, kind }, state, payload);
                  }
                  tracing::debug!(?kind, "task submitted");
              });
              handle
          }

          fn lock_indexes(&self) -> std::sync::MutexGuard<'_, HashMap<IndexId, IndexEntry>> {
              self.indexes.lock().unwrap_or_else(PoisonError::into_inner)
          }
      }

      impl Drop for Resident {
          fn drop(&mut self) {
              for node in self.queue.iter_live() {
                  if let Some(state) = node.task_state() {
                      state.cancel();
                  }
              }
          }
      }

      #[cfg(test)]
      mod tests {
          use std::time::Duration;

          use super::*;
          use crate::resident::facade::Output;

          fn resident() -> Resident {
              Resident::builder()
                  .model(ModelId("e5-small".into()))
                  .init()
                  .unwrap()
          }

          fn serve_one(r: &Resident, out: Output) -> bool {
              for node in r.queue().iter_live() {
                  if node.claim().is_some() {
                      node.task_state().unwrap().complete(Ok(out));
                      return true;
                  }
              }
              false
          }

          #[test]
          fn builder_requires_model() {
              let err = Resident::builder().init().err().unwrap();
              assert!(matches!(err, ConfigError::MissingField { .. }));
          }

          #[test]
          fn builder_rejects_zero_op_buffer() {
              let err = Resident::builder()
                  .model(ModelId("m".into()))
                  .min_op_buffer(0)
                  .init()
                  .err()
                  .unwrap();
              assert!(matches!(err, ConfigError::InvalidValue { .. }));
          }

          #[tokio::test]
          async fn submitted_task_resolves_with_output() {
              let r = resident();
              let handle = r.submit_embedding(TextInput::single("hi"), EmbeddingParams::default());
              assert!(handle.try_get().is_none());
              assert!(serve_one(&r, Output::Embeddings(vec![vec![1.0]])));
              assert_eq!(handle.await.unwrap(), Output::Embeddings(vec![vec![1.0]]));
          }

          #[tokio::test]
          async fn nan_precision_fails_without_enqueue() {
              let r = resident();
              let params = EmbeddingParams::default().precision(f32::NAN);
              let handle = r.submit_embedding(TextInput::single("hi"), params);
              assert!(matches!(handle.await, Err(TaskError::InvalidPrecision)));
              assert!(!serve_one(&r, Output::Scores(vec![])));
          }

          #[test]
          fn dropping_handle_cancels_queued_task() {
              let r = resident();
              let handle = r.submit_inference(PairInput::new("q", "c"), InferenceParams::default());
              drop(handle);
              assert!(!serve_one(&r, Output::Scores(vec![])));
          }

          #[test]
          fn dropping_handle_after_claim_keeps_result() {
              let r = resident();
              let handle = r.submit_inference(PairInput::new("q", "c"), InferenceParams::default());
              assert!(serve_one(&r, Output::Scores(vec![0.5])));
              drop(handle);
          }

          #[tokio::test(start_paused = true)]
          async fn wait_timeout_reports_timeout_and_cancels() {
              let r = resident();
              let handle = r.submit_similarity(
                  VectorInput::new(vec![vec![0.0]]),
                  SimilarityParams::new(IndexId("x".into())),
              );
              let res = handle.wait_timeout(Duration::from_millis(10)).await;
              assert!(matches!(res, Err(TaskError::Timeout)));
              assert!(!serve_one(&r, Output::Scores(vec![])));
          }

          #[test]
          fn shutdown_cancels_pending_tasks() {
              let r = resident();
              let handle = r.submit_embedding(TextInput::single("a"), EmbeddingParams::default());
              r.shutdown().unwrap();
              assert!(matches!(handle.try_get(), Some(Err(TaskError::Cancelled))));
          }
      }
  reason: "Implement the C1 facade module: builder, Resident, params, TaskHandle, blocking wrapper, tests"
```
/answer 4eac-a9 allow
ReasoningToken: 172
Control Request:
```yaml
id: 4eac-aa
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/mod.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/resident.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/handle.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/params.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/blocking.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/error.rs
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/facade/task.rs
  reason: Check syntax of the new facade files
```
/answer 4eac-aa allow
Result Stats: id=228d7642-db5c-48ab-ad16-d8d4c4b4e668, total: 81548, in: 22, out: 21602, reason: 0, read: 376201, write: 59924, start: 1792319181679, end: 1792319187664, time: 5935, total: 137177, io: 0, control: 50, sent: 0, recv: 156
C1 ist im Modul `rag/src/resident/facade/` umgesetzt. Ich konnte nur die Syntax prüfen (`ast_validate`). `cargo check` und `cargo test` habe ich nicht ausgeführt, die Tests sind also ungetestet.

Der Plan sah an drei Stellen etwas anderes vor:
- **`add_index`:** Das Plan-Signatur `Arc<dyn Index + Send + Sync>` geht nicht. Das `Index`-Trait hat `impl Iterator` im Rückgabetyp und ist deshalb nicht objekt-sicher. `add_index<I: Index + Send + Sync + 'static>(Arc<I>)` legt den Index typ-gelöscht in der Residency-Tabelle ab. Für die spätere Nutzung durch den Processor braucht es entweder einen objekt-sicheren Adapter oder eine Anpassung am Trait (B4).
- **`wait_timeout`:** Die Methode ist `async`, nicht synchron wie im Plan. Ein synchrones Warten würde in einer laufenden Runtime panikieren.
- **Task-Queue:** Der Processor (C2) existiert noch nicht. `Resident` hält deshalb vorläufig selbst eine `Queue` und gibt sie über `queue()` frei. Einen eigenen `Processor` oder Worker-Threads startet `init()` nicht.

Die Lazy-`OnceCell`s habe ich nicht angelegt, weil es noch keine Ressourcentypen gibt, die sie halten könnten. Die Typen `WorkerConfig` und `ResidentConfig` liegen vorläufig ebenfalls in der Fassade, bis C2 sie übernimmt.

Die Tests liegen in `resident.rs`. Sie decken ab:
- Builder-Validierung
- Task-Ergebnis über `.await`
- NaN-Precision
- Cancel beim Drop
- Timeout
- Shutdown

Für `add_index` und den `ResidentBlocking`-Wrapper gibt es keine Tests.
