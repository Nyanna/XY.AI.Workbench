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
- `add_index(Arc<dyn Index + Send + Sync>) -> IndexId`. Die Engine hält eine **Residency-Tabelle** `IndexId → (Zustand, Leases)` und **kein** Persistenz-Handle.
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
    pub fn wait_timeout(self, d: std::time::Duration) -> Result<Output, TaskError>;
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
