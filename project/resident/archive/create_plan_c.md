Erstelle Umsetzungspläne für Agenten im Verzeichnis `/home/user/xyan/xy.ai.workbench/project/resident/plan` für Gruppe C aus `resident_planplan.md`.

* `resident_index.md` ist nur relevant für die Implementierung eines Vektorindexes und keine Indexdatei.

Projektpläne:
```json
{
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident",
      "entries" : [ ".:", "resident.md", "resident_first.md", "resident_index.md", "resident_planner.md", "resident_precision.md", "resident_processor.md", "resident_subengine.md", "resident_planplan.md" ]
    } ]
  }
}
Kontext Dateien für Gruppen:
{
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/project/resident/context",
      "entries" : [ ".:", "context_h_q.md", "context_a.md", "context_b.md", "context_c.md", "context_d.md", "context_e.md", "context_f.md", "context_g.md" ]
    } ]
  }
}
```
SystemInit:  id=a2410e84-bdfb-4959-9e99-b657158a114d, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 Ich beginne damit, die relevanten Dateien zu lesen, um die Aufgabe und Gruppe C zu verstehen.
Control Request:
```yaml
id: 0f9c-10
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_c.md
  reason: "Lesen des Planplans und des Kontexts für Gruppe C, um zu verstehen, welche Umsetzungspläne erstellt werden müssen."
```
/answer 0f9c-10 allow
ReasoningToken: 284
Text:
 Ich habe den Planplan und den Kontext für Gruppe C. Jetzt lese ich die Primärquellen für Gruppe C (Orchestrierung), um die Umsetzungspläne fundiert zu erstellen.
Control Request:
```yaml
id: 0f9c-11
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md
  reason: "Lesen der Primärquellen für Gruppe C (Fassade, Processor, Planner), um die Umsetzungspläne C1–C3 inhaltlich korrekt zu verankern."
```
/answer 0f9c-11 allow
ReasoningToken: 256
Text:
 Ich habe alle relevanten Quellen gelesen. Gruppe C umfasst drei Pakete (C1 Fassade, C2 Processor, C3 Planner). Der Kontext `context_c.md` liefert die recherchierten Entscheidungsgrundlagen (mit Belegstufen [B]/[D]/[W]). Ich erstelle nun pro Paket einen ausführbaren Umsetzungsplan plus eine Gruppenübersicht im Verzeichnis `plan`.
Control Request:
```yaml
id: 0f9c-12
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_C_overview.md
    mode: replace
    content: |
      # Umsetzungspläne Gruppe C — Orchestrierung (Übersicht)

      Stand: 10.10.2026 · Zielsprache Rust · GPU über `candle` + `cudarc` · Projekt `/home/user/xyan/xy.ai.workbench/rag`

      Diese Übersicht bündelt die Stufe‑2‑Umsetzungspläne der Gruppe C aus `resident_planplan.md`.
      Grundlage ist die Recherche in `context/context_c.md` (Belegstufen **[B]** belegt · **[D]** Drittquelle · **[W]** Standardwissen/unverifiziert).
      Primärquellen: `resident.md` (Fassade, Kontrollfluss, Queue-Logik), `resident_processor.md`, `resident_planner.md`.

      ## Pakete und Dateien

      | Paket | Rolle | Datei | Hängt ab von |
      |---|---|---|---|
      | **C1** | Fassade „Resident" — Einstiegspunkt, Config/Init, Task-Einreihung, Index-Verwaltung, Ergebnisabholung | `plan_C1_fassade.md` | B1 (Queue/Task), B3 (Op/Value), B4 (Index-Trait) |
      | **C2** | Processor — Queues, Worker-Parkplatz, Worker-Loop, GPU-Completion, Leases, Backpressure, Eviction | `plan_C2_processor.md` | B1, B3, C3 |
      | **C3** | Planner — zustandslose Task→Operation-Expansion, Device-Wahl, `materialize` | `plan_C3_planner.md` | B3 (Op/Value), B6 (CapabilitySet/Profile) |

      ## Umsetzungsreihenfolge (laut Metaplan)

      ```
      C1  →  C3  →  C2
      ```

      Begründung: Die Fassade (C1) definiert die öffentliche Task-API und die Datentypen, die der Planner konsumiert.
      Der Planner (C3) ist eine reine Funktion ohne Laufzeitabhängigkeiten und lässt sich isoliert bauen/testen.
      Der Processor (C2) orchestriert beide und braucht Planner-Ausgabe + Fassaden-Queue, kommt daher zuletzt.

      ## Komponentenbeziehung (Soll-Architektur)

      ```
                  ┌──────────────────────────── Resident (C1, Fassade) ───────────────────────────┐
                  │  Config → init() baut Processor + spawnt Worker-std::threads                   │
                  │  submit(task) → push in Task-Queue (B1)      add_index(Arc<dyn Index>) (B4)     │
                  │  TaskHandle (IntoFuture / wait_timeout / Drop=Cancel)                           │
                  └───────────────┬───────────────────────────────────────────────┬────────────────┘
                                  │ Task-Queue (geteilt)                           │ Residency-Tabelle
                                  ▼                                                 ▼
                  ┌──────────── Processor (C2) ───────────┐              Index-ID → Zustand/Leases
                  │ Task-Queue + 3 Op-Queues (CPU/GPU/sh) │
                  │ Worker-Parkplatz, Notify, Starved-Hints│  ── expand(task, &ExpandContext) ──▶  Planner (C3)
                  │ Worker-Loop (GPU-Vorrang), Leases      │  ◀── Vec<Operation> (requires/materialize) ──
                  │ GPU-Completion (Callback + cuEventQuery)│
                  └────────────────────────────────────────┘
      ```

      - Fassade und Processor teilen sich **eine** Task-Queue (Datentyp aus B1). Die Fassade reiht ein, Subengine-Worker ziehen per `claim`.
      - Der Processor ruft den Planner synchron und zustandslos für die Expansion (C2 → C3).
      - Subengines (Gruppe D) sind hier **nicht** Teil der Pläne; C2 definiert nur die Schnittstellen, über die Subengines Operationen claimen und Ergebnisse zurückmelden.

      ## Querschnitt (gilt für C1–C3)

      - **Q1 Fehler (`thiserror` 2.0.21):** ein Fehler-Enum je Modul (`ConfigError`, `PlannerError`, `OrchestrationError`, `CapabilityError`), oben via `#[from]` zusammengeführt, öffentliche Enums `#[non_exhaustive]`, `Cancelled` als eigene Variante. Fan-out an mehrere Warter: `Arc<TaskError>` oder `Clone`. [B]
      - **Q2 Logging (`tracing` 0.1.44):** ein Span pro Task-ID (bei `submit` erzeugt, im Task-Zustand gehalten), ein Span pro Worker-Thread-Lebenszeit (nicht pro Zyklus); Hot-Path (`claim`/`launch`/`complete`) nur Events auf `trace`/`debug`. `Instrument` statt `Span::enter()` über `.await`. Bibliothek ruft **nie** `set_global_default`. [B/W]
      - **Q3 Tests/Mocks:** No-GPU-Pfad immer lauffähig; `proptest`/`proptest-state-machine` für Invarianten und Lebenszyklen; Kernel-/Mess-/Completion-Mocks. [B/W]
      - **Q4 Feature-Flag `cuda`:** reale CUDA-Pfade hinter `#[cfg(feature = "cuda")]`, orchestrierungsseitige Logik ist ohne GPU übersetzbar und testbar. Trait `Completion` immer kompiliert, `ManualCompletion`-Mock immer verfügbar. [D/W]

      ## Vorbedingungen aus anderen Gruppen (Eingangsverträge)

      Diese Pläne konsumieren Typen aus Gruppe B. Sind diese noch nicht gebaut, definiert der jeweilige C-Plan einen **minimalen Platzhalter-Trait/-Typ** mit identischer Signatur, der später gegen die B-Implementierung getauscht wird:
      - **B1** `Queue`, `Node`, `Task`, `TaskState` (lock-free Liste, `claim`=CAS `Queued→Taken`).
      - **B3** `Operation`, `requires()→Requirement`, `materialize:bool`, `Value::Host|Device(handle)`.
      - **B4** `Index`-Trait (objekt-sicher: `stamp/read/apply/changed/capabilities/...`).
      - **B6** `CapabilitySet`, `Profile` (nur von C3 benötigt, Matching selbst ist Gruppe F).

      ## Offene Risiken, die in Gruppe C nur abgesichert (nicht gelöst) werden

      1. Keine belastbare Quelle zu „launch-bound GPU-Worker"-Dimensionierung (Q13) → Worker-Zahl als Parameter, per Messung bestimmen.
      2. `launch_host_function`-Signatur und candle-Stream-Erzeugung nur in der **gepinnten** Version verifizierbar (Q12) → als erster Schritt in C2 gegenprüfen.
      3. Starved-Hint-Verfall und Shared-Queue ohne Head-of-Line-Blocking (Q17/Q21) sind Eigenentwürfe [W] → mit Lasttests absichern.
      4. Ringbuffer-/Graph-Capture-Modi bleiben Ausblick (Gruppe H); C2 sichert nur Erweiterungspunkte.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_C1_fassade.md
    mode: replace
    content: |
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
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_C2_processor.md
    mode: replace
    content: |
      # Umsetzungsplan C2 — Processor (Orchestrierung)

      Stand: 10.10.2026 · Modul: `processor` · Zielsprache Rust · GPU über `candle`/`cudarc`
      Quellen: `resident_processor.md` (maßgeblich) · `resident.md` (Queue-Logik, Notify-Topologie, Kontrollfluss) · Recherche `context/context_c.md` §C2 (Punkte 10–21), Querschnitt 30–32.

      ---

      ## 1. Ziel & Einordnung

      Der Processor ist die Orchestrierungsschicht zwischen Fassade (C1) und Subengines (Gruppe D). Er besitzt:
      - die **Task-Queue** und **drei Operations-Queues** (CPU, GPU, shared — shared angelegt, leer, ohne Logik),
      - den **Worker-Parkplatz** (persistente Worker-Threads),
      - **Notify**-Topologie, **Starved-Hints** und **Backpressure**,
      - die **GPU-Completion**-Mechanik (Callback + `cuEventQuery`),
      - Datenübergabe per **Lease/Handle**, **Eviction** und Task-Abschluss/Fehler.

      Er ruft den **Planner (C3)** für die Expansion und bedient sich zustandslos dessen Ausgabe. Subengines claimen Operationen selbst (Pull-Modell, `resident.md`).

      ---

      ## 2. Scope

      **In:** Queue-Verdrahtung (Task + 3 Op-Queues), Worker-Parkplatz & Worker-Loop mit GPU-Vorrang, GPU-Completion ohne Doppelverarbeitung, Lease-/Handle-Lebenszyklus, Starved-Hints & Mindestpuffer, Eviction-Trigger, Task-Abschluss/Fehlerpfad, Completion-Trait hinter `cuda`-Flag.

      **Out:** Queue-Datentyp selbst (B1), Operation-/Value-Typen (B3), Expansion (C3), Subengine-Interna/Batching-Mechanik (D — der Processor stellt nur die Claim-/Rückmelde-Schnittstelle bereit), Kernel/Arena (D/E).

      ---

      ## 3. Abhängigkeiten (Eingangsverträge)

      | Vertrag | Herkunft | Nutzung |
      |---|---|---|
      | `Queue`/`Node`/`claim` (CAS `Queued→Taken`) | B1 | Task-Queue **und** Op-Queues nutzen denselben lock-freien Listentyp. |
      | `Operation`, `requires()→Requirement`, `materialize`, `Value::Host\|Device(handle)` | B3 | Op-Queue-Inhalt, Datenübergabe-Entscheidung. |
      | `Planner::expand(task, &ExpandContext) -> Vec<Operation>` | C3 | Expansion in Worker-Loop-Schritt 4. |
      | Subengine-Claim-/Run-/Complete-Schnittstelle | D (Vertrag hier mitdefiniert) | Worker rufen Subengine, Subengine meldet Completion/Result zurück. |
      | `cudarc`/`candle` (gepinnte Version) | Stack | Stream/Event/Host-Callback für GPU-Completion. |

      ---

      ## 4. Entscheidungen zu den offenen Punkten

      Mappt die „Zu klären"-Punkte des Metaplans (Callback + `cuEventQuery` ohne Doppelverarbeitung; Worker-Zahl; Starved-Hint-Verfall; Mindestpuffer) auf verankerte Entscheidungen; adressiert die `Prüfen`-Checkliste aus `resident_processor.md`.

      ### 4.1 Worker-Parkplatz & Worker-Zahl [B/Lücke, context §13, §C1.5]
      - Worker sind **eigene benannte `std::thread`s** (tokio bietet kein Mindest-Idle). Sie parken im Processor und wachen per `Notify` bei jeder Zustandsänderung auf.
      - Gesamtzahl ≈ CPU-Kerne (Obergrenze gegen Kontextwechsel, `resident_processor.md`).
      - Richtwert [W]: **GPU-Worker = Streams/Devices (1–2 pro Device)**, **CPU-Worker = Kerne − (GPU-Worker + Reserve)**. Kein Literaturbeleg für launch-bound GPU-Worker → **Worker-Zahl ist Parameter** (`WorkerConfig`) und wird per Messung (Worker-Auslastung, Launch-Latenz, GPU-Idle-Anteil) bestimmt. → deckt `Prüfen`-Punkt „Worker-Zahl vs. launch-bound".

      ### 4.2 Worker-Loop mit GPU-Vorrang [B/W, `resident_processor.md`, context §16/§21]
      Nach **jeder** Operation neu prüfen, feste Priorität:
      1. **GPU-Ergebnisse verarbeiten** (fertige Lane übernehmen, Index zurückspiegeln).
      2. **GPU-Ressourcen füttern** (Index in VRAM synchronisieren, Launch nur bei **freier Lane**).
      3. **CPU-Ressourcen füttern/claimen/ausführen**, Result zurückgeben.
      4. **Neue Tasks expandieren** (Planner) — nur bei Hint oder leerer Op-Queue.
      - GPU-Vorrang ist selbstlimitierend: Schritt 2 greift nur bei freier Lane, Schritt 4 nur bei Hint/leerer Queue.

      ### 4.3 GPU-Completion ohne Doppelverarbeitung [B/W, context §10, §11]
      - **Callback** (`cuLaunchHostFunc` am Stream-Ende) tut **nur**: Completion-Eintrag (lock-free) ablegen + `Notify` auslösen. **Keine** CUDA-API, keine Synchronisation, kein Blockieren (Callback blockiert sonst nachfolgende Stream-Arbeit).
      - **Poll-Fallback** `cuEventQuery`/`CudaEvent::try_is_complete()` in Worker-Schritt 1 deckt verpasste Callbacks ab (Callback wird bei CUDA-Fehler **nicht** aufgerufen). **`try_is_complete()` statt `is_complete()`** (letzteres liefert bei Fehlern `false`).
      - **Doppelverarbeitung verhindern:** pro Lane ein `AtomicU8`-Zustand; Callback **und** Poller versuchen `compare_exchange(Launched → Completing)`; nur der Gewinner verarbeitet. → deckt `Prüfen`-Punkt „keine doppelte Verarbeitung einer Lane".
      - Event mit `CU_EVENT_DISABLE_TIMING` (schnellstes `cuEventQuery`).

      ### 4.4 Zugang über `candle`/`cudarc` [B/D, context §12]
      - **Genau die aus `candle` re-exportierte `cudarc`-Version** verwenden (`candle::cuda_backend::cudarc`), nicht separat dazuziehen (Version 0.19.10 beim Rechercheabgleich).
      - `CudaDevice::cuda_stream() -> Arc<CudaStream>`; Host-Funktion über `cudarc::driver::result::stream::launch_host_function(stream.cu_stream(), trampoline, arg)` (kein sicherer Wrapper). Trampoline gibt z. B. `Box<Completion-Eintrag>` frei.
      - **Erster Umsetzungsschritt:** `launch_host_function`-Signatur und candle-Stream-Erzeugung (`context.new_stream()` vs. `default_stream()`) in der **gepinnten** Version gegenprüfen (context Q12).
      - Host-Callback auf dem candle-Stream blockiert dort Folge-Kernel → minimal halten; Stream-Wechsel per Event/Wait.

      ### 4.5 Datenübergabe & Lease/Handle [D/W, context §19, §20, `resident_processor.md`]
      - Operation-Output ist `Value::Host(data)` oder `Value::Device(handle)`; `materialize` (vom Planner gesetzt) entscheidet.
      - Handle = `Region` mit `gen`, gehört dem Task-Kontext per **RAII-Lease** (`Drop` = Use-Zähler −1). Zugriff nur über `Lease::get() -> &Region`, nie rohe Pointer weitergeben.
      - Offenes Handle = Lease auf Arena+Ressource (Use-Zähler > 0) → Ressource wird nicht evicted. `gen`-Prüfung beim Claim nur als **Debug-Assertion** (Reload erhöht `gen`, alte Handles scheitern mit `Stale`).
      - Lease endet mit der letzten konsumierenden Operation oder im Cancel-Pfad; Task-Timeout bricht blockierte Tasks ab und gibt Leases frei (Leases im Task-Zustand per RAII, damit auch bei Drop/Panic freigegeben).

      ### 4.6 Backpressure & Starved-Hints [B/Lücke, context §17, §18, `resident_processor.md`]
      - Subengine-Iteration ohne Pick bei freier Ressource erzeugt `Starved(resource_kind, params)`.
      - Hints liegen als **verfallendes Set pro Engine-Typ** (keine Queue). Implementierung [W]: `AtomicU32`-Bitmaske pro Ressourcenart mit Epoche/Zeitstempel; Bit wird beim Pick dieser Art **oder** bei Belegung gelöscht. Keine Modellierung als Queue. → deckt `Prüfen`-Punkt „Starved-Hint-Verfall unter wechselnder Last".
      - `expand()` läuft nur bei Hint oder leerer Op-Queue; bevorzugt Tasks, deren **erste** Operation die bedürftige Ressource braucht.
      - **Mindestpuffer** an Operationen als Parameter (`WorkerConfig::min_op_buffer`): Richtwert `(Refill-Latenz des Planners / Zeit pro Op) + 1`, adaptiv als **untere** Wasserlinie; **keine** feste obere Füllschwelle.

      ### 4.7 Shared-Queue-Semantik [B-Analogie/W, context §21, `resident_processor.md`]
      - Subengine prüft **erst die eigene Queue, dann shared** (shared ist angelegt, leer, ohne Logik).
      - Beim Pop aus shared nur nehmen, was zur eigenen Capability passt (`requires()`), sonst liegen lassen; **kein Head-of-Line-Blocking** (bis zu K Einträge überspringen statt vom Kopf blockieren). Analogie: tokio-Scheduler prüft lokale Queue bevorzugt, globale alle N Ticks.

      ### 4.8 Notify-Topologie [B/W, context §16, `resident.md`]
      | Quelle | Wirkung |
      |---|---|
      | Queue-Append | Relay auf das Notify jeder Subengine-Schleife |
      | Result / Init fertig / Evict-Timer | eigenes Notify |
      | Iteration mit Fortschritt | eigenes Notify |
      - **Muster A (Standard):** pro Subengine-Schleife ein `Notify`, Relay ruft nach jedem Append `notify_one()` (Permit-Speicherung verhindert Lost-Wakeups bei einzelnem Konsumenten).
      - **Muster B (Option):** `watch<u64>`-Epochenzähler als Level-Signal, Schleife `changed().await`, danach Queue leer lesen.
      - Reihenfolge immer: `notified()` → `pin!` → `enable()` → Zustand prüfen → `.await`.

      ### 4.9 Worker-Priorität (GPU > CPU, Linux) [B/W, context §14, `resident.md` Klarstellung]
      - Scheduling ist **pro Thread**. Empfehlung: **CPU-Engine-Worker herunterstufen** (höherer Nice-Wert via `libc::setpriority(PRIO_PROCESS, gettid, nice)`) — braucht keine Privilegien. Heraufstufen der GPU-Worker bräuchte `CAP_SYS_NICE`/`RLIMIT_NICE`.
      - Echtzeit-Policies (`SCHED_FIFO/RR`) **vermeiden**. Fehler (`EPERM`) nur loggen. Alles hinter `cfg(target_os = "linux")`. Setzen im Worker-Start (eigener Thread setzt sich selbst).

      ### 4.10 Blocking in Candle/rayon [B/Lücke, context §15, `resident_processor.md`]
      - Blockiert ein Worker in Candle/rayon, ist das das **natürliche Rückdrucksignal** — aber nur, weil Worker **dedizierte Non-Runtime-Threads** sind. Auf tokio-Runtime-Threads wäre Blockieren Starvation → daher Worker nicht auf Runtime-Threads legen.
      - CPU-Compute über rayon + `oneshot` zurück; expliziter Rückdruck über beschränkte `mpsc`/`Semaphore`, nicht über den Blocking-Pool.
      - Candle-CPU-Begrenzung (rayon) bleibt Parameter, keine Sonderlogik.

      ### 4.11 Lazy Init, Batching, Eviction [B, `resident_processor.md`]
      - Lazy Init nur durch die Iteration, nie durch Einreihen; `get_or_try_init`, parallele Inits per `Semaphore`. Ressource `Loading` ⇒ Operation bleibt in Queue, Iteration überspringt.
      - Zustände: `Uninit | Loading | Ready | Backoff`; ressourcenintern `Unloaded → Loading → Resident → Leased(n) → Idle → Evicting` + `Failed`.
      - Batching ist Aufgabe der Subengine (ab Seed kompatible Ops derselben Ressource claimen) — Processor stellt nur Queue + Claim bereit.
      - Eviction bei `in_flight == 0`, offenen Leases 0 und abgelaufener TTL (VRAM deutlich länger als RAM).
      - Init-/Launch-Fehler: `Err` in die Task-Results des Batchs, Ressource → `Backoff`; GPU-Initfehler: GPU als nicht verfügbar markieren, wiederholbar.

      ### 4.12 Abschluss & Fehler [B/W, context §27, §30, `resident_processor.md`]
      - Task führt `AtomicUsize pending` offener Operationen. Folgeops **erst zählen und einreihen** (`fetch_add(n, Relaxed)` + Enqueue), **danach** den Vorgänger abschließen (`fetch_sub(1, AcqRel) == 1` ⇒ `Done`), damit der Zähler nie vorzeitig 0 erreicht.
      - Erster Fehler: `compare_exchange` auf `AtomicU8`-Status (`Running → Failed/Cancelled`); weitere Ops dieses Tasks beim Claim überspringen; Handles freigeben; Ergebnis über Oneshot + Notify/Event.

      ---

      ## 5. Datentypen (Signatur-Skizze)

      ```rust
      pub struct Processor {
          tasks: Queue,                 // B1
          ops_cpu: Queue, ops_gpu: Queue, ops_shared: Queue,
          workers: Vec<std::thread::JoinHandle<()>>,
          notify_cpu: tokio::sync::Notify, notify_gpu: tokio::sync::Notify,
          starved_cpu: StarvedSet, starved_gpu: StarvedSet, // AtomicU32 + Epoche
          planner: std::sync::Arc<Planner>,                  // C3
          cfg: WorkerConfig,
      }

      pub struct WorkerConfig { /* gpu_workers, cpu_workers, min_op_buffer, candle_rayon_threads */ }

      // GPU-Completion (Q4: Trait immer kompiliert)
      pub trait Completion: Send + Sync {
          fn arm(&self, lane: LaneId);                 // Callback registrieren / Event aufnehmen
          fn poll(&self, lane: LaneId) -> LaneState;   // cuEventQuery-Fallback
      }
      #[cfg(feature = "cuda")] pub struct HostFnCompletion { /* launch_host_function */ }
      #[cfg(feature = "cuda")] pub struct PolledCompletion { /* cuEventQuery */ }
      pub struct ManualCompletion { /* Mock, im Test auslösbar — immer verfügbar */ }

      // Lease (RAII)
      pub struct Lease { /* gen, region, use_counter */ }
      impl Drop for Lease { /* use_counter -= 1 */ }
      impl Lease { pub fn get(&self) -> &Region; }

      // Starved-Set
      pub struct StarvedSet(std::sync::atomic::AtomicU32 /* + Epoche */);
      impl StarvedSet { pub fn mark(&self, kind: ResourceKind); pub fn clear(&self, kind: ResourceKind); pub fn snapshot(&self) -> StarvedSnapshot; }
      ```

      ---

      ## 6. Umsetzungsschritte

      1. **cudarc/candle-Verifikation (zuerst):** gepinnte Version prüfen — `launch_host_function`-Signatur, Stream-Erzeugung, `try_is_complete`, `record_event(CU_EVENT_DISABLE_TIMING)`. Ergebnis als Risikonotiz festhalten (context Q12).
      2. **Queue-Verdrahtung:** Task-Queue + 3 Op-Queues aus B1 instanzieren; shared leer.
      3. **Worker-Parkplatz:** benannte `std::thread`s starten, Park/Notify; `WorkerConfig`-Dimensionierung (4.1); Nice-Herabstufung der CPU-Worker (4.9) hinter `cfg(linux)`.
      4. **Worker-Loop** (4.2) mit den vier Prioritätsstufen; Shared-Queue-Semantik (4.7); Notify-Topologie (4.8).
      5. **Completion-Trait** (4.3/4.4): `ManualCompletion` zuerst (No-GPU, testbar), dann `HostFnCompletion`/`PolledCompletion` hinter `cuda`; Lane-`AtomicU8` + `compare_exchange(Launched→Completing)`.
      6. **Lease/Handle** (4.5): RAII-Lease, Use-Zähler, `gen`-Debug-Assertion, Freigabe im Cancel/Timeout.
      7. **Starved-Hints + Backpressure** (4.6): `StarvedSet`, `expand()`-Trigger, `min_op_buffer`.
      8. **Lazy-Init/Eviction/Batching-Hooks** (4.11): Zustandsautomat `Uninit|Loading|Ready|Backoff`, Eviction-Trigger.
      9. **Abschluss/Fehler** (4.12): `pending`-Zähler-Protokoll, Fehler-CAS, Result-Zustellung.
      10. **Erweiterungspunkte sichern (Gruppe H):** `requires()`-Affinität, Ringbuffer-/Graph-Capture-Modus nur als nicht verbaute Hooks — keine Detailimplementierung.
      11. **Tests** (§9).

      **Deliverables:** Processor baut und läuft im No-GPU-Pfad (`ManualCompletion`); mit `--features cuda` nutzt er Host-Callback + `cuEventQuery`.

      ---

      ## 7. Fehlerbehandlung (Q1) [B, context §30]
      `OrchestrationError` mit Varianten u. a. `TransferFailed`, `StaleMapping` (intern), `Backoff`, `IndexUnavailable`, plus `Cancelled`. Zusammenführung oben via `#[from]`, `#[non_exhaustive]`. „callback dropped without firing" ⇒ typisierter `Cancelled`, nicht String.

      ## 8. Logging (Q2) [B/W, context §31]
      Span pro Task-ID und pro Ressource; **ein** Span pro Worker-Thread-Lebenszeit (nicht pro Zyklus). Hot-Path `claim`/`launch`/`complete` nur Events (`trace`/`debug`) mit `task_id`/`lane`. Init-/Launch-Fehler mit `tracing` + Task-ID.

      ## 9. Teststrategie & Mocks (Q3) [B/W, context §29, §32, `resident_processor.md` Prüfen]
      - **Completion-Mock** (`ManualCompletion`): verpassten Callback simulieren → Poll-Fallback muss Lane übernehmen; **gleichzeitiges** Callback+Poll → genau **eine** Verarbeitung (CAS-Test).
      - **Starved-Set:** unter wechselnder Last testen, dass Bits verfallen (Pick/Belegung) und `expand()` korrekt getriggert wird.
      - **Worker-Loop:** `proptest-state-machine` für Prioritätsreihenfolge und „kein Head-of-Line-Blocking" in der Shared-Queue.
      - **Abschluss:** Property „`pending` erreicht nie vorzeitig 0"; „erster Fehler ⇒ alle übrigen Ops scheitern beim Claim, Leases frei".
      - **Lasttest (manuell/bench):** Worker-Zahl-Dimensionierung (GPU-Idle-Anteil, Launch-Latenz) — context Q13.

      ## 10. Feature-Flags (Q4) [D/W, context §32]
      `Completion`-Trait immer kompiliert; reale CUDA-Pfade (`HostFnCompletion`/`PolledCompletion`, `launch_host_function`, `cuEventQuery`) hinter `#[cfg(feature = "cuda")]`; `ManualCompletion` immer verfügbar. candle nutzt ohne CUDA `dummy_cuda_backend`.

      ## 11. Offene Risiken / zu verifizieren
      - `launch_host_function`-Signatur & candle-Stream-API nur in gepinnter Version sicher (context Q12) — Schritt 1.
      - Launch-bound GPU-Worker-Zahl ohne Literatur (context Q13) — per Messung.
      - Starved-Set und Shared-Queue-Überspringen sind Eigenentwürfe [W] — mit Lasttests absichern.
      - Forum-Aussage „ein Callback-Thread pro Device" ist **ungeprüft** — nicht als Fakt übernehmen.
      - Reihenfolge-Invarianz bei Index-Rückspiegelung berührt Gruppe E (Kommunikationsslot) — Schnittstelle sauber halten.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_C3_planner.md
    mode: replace
    content: |
      # Umsetzungsplan C3 — Planner (reine Taskexpansion)

      Stand: 10.10.2026 · Modul: `planner` · Zielsprache Rust
      Quellen: `resident_planner.md` (maßgeblich) · `resident.md` (Tasktypen, Kontrollfluss) · Recherche `context/context_c.md` §C3 (Punkte 22–29), Querschnitt 30–32.

      ---

      ## 1. Ziel & Einordnung

      Der Planner übersetzt einen **Task** (Daten + Parameter) in eine **geordnete Folge von Operationen**. Er ist eine **reine Funktion**: gleiche Eingabe (Task + Config-Snapshot + Capabilities + optionaler Hint) ⇒ gleiche Operationen. **Keine** Queues, Worker, Notify, kein veränderlicher Zustand.

      Der Planner arbeitet nur mit **Config-Regeln** und den **Capabilities der Subengines**. Er setzt pro Operation `requires()` (Ressourcenart, Modell, Index, erlaubte Engine), `materialize` und Ein-/Ausgabe als `Value::Host|Device`.

      ---

      ## 2. Scope

      **In:** Expander-Trait + explizite Registry (pro Modell/Tasktyp), Config-Format + Validierung + Hot-Reload-Snapshot, Expansionsregeln für Embedding/Similarity/Inferenz, `materialize`-Regel, aufgeschobene Device-Wahl, Op-Zähler-Vertrag (Definition; Zählung selbst führt C2 aus), Starved-Hint als reine Eingabe.

      **Out:** Orchestrierung/Queues/Worker (C2), Capability-Erhebung und Präzisions-Matching (Gruppe F — der Planner liest `CapabilitySet`/`precision`, berechnet aber keine Profile), Kernel/Index-Interna (D/E), gleitende Indizierung (Ausblick H2 — nur Parameter-Hook).

      ---

      ## 3. Abhängigkeiten (Eingangsverträge)

      | Vertrag | Herkunft | Nutzung |
      |---|---|---|
      | `Operation`, `requires()→Requirement`, `materialize`, `Value::Host\|Device` | B3 | Ausgabe-Typ des Planners. |
      | `CapabilitySet` (welche Engine welche Ressourcenart anbietet) | B6 | Device-Wahl-Grundlage; `precision`-Feld nur durchgereicht. |
      | Parameterobjekte `EmbeddingParams`/`SimilarityParams`/`InferenceParams`, `TextRole`, `PairInput` | C1 | Eingabe-Task. |
      | Pipeline-Config pro Modell (Präfixe, Pooling, Normalisierung, `max_seq_len`, dtype) | C3 (hier definiert) | Expansionsregeln. |

      ---

      ## 4. Entscheidungen zu den offenen Punkten

      Mappt die „Zu klären"-Punkte des Metaplans (Konfigurationsformat der Expansionsregeln als Registry pro Modell/Tasktyp; Gewichtung des Starved-Hints) auf verankerte Entscheidungen; adressiert `resident_planner.md` „Offen".

      ### 4.1 Konfigurationsformat der Expansionsregeln [W, context §22]
      - **TOML** via `serde` + **Validierung nach dem Deserialisieren** (Registry-Check: unbekannte Ops/Ressourcenarten/Modelle ⇒ `PlannerError`/`ConfigError`). RON nur, falls Enums-mit-Daten häufig werden.
      - Regeln, die **Logik** brauchen, bleiben **code-basiert** (Expander-Impl), nicht in Config. Config liefert nur Daten (Präfixe, Pooling, `max_seq_len`, dtype, Scorer-Auswahl).
      - **Hot-Reload:** Datei-Watcher (`notify`-Crate) + atomarer Austausch (`ArcSwap` bzw. `RwLock<Arc<…>>`). Der Planner bleibt rein, weil er **pro Aufruf einen Snapshot** (`Arc<RuleSet>`) erhält.

      ### 4.2 Registry-Pattern [B/W, context §23]
      - **Explizite Registry** als Standard: `HashMap<(ModelId, TaskKind), Box<dyn Expander>>` bzw. Enum-Dispatch → deterministisch, leicht mockbar.
      - `inventory`/`linkme` (Life-before-main) **nur**, falls Fremdcrates sich selbst registrieren sollen — vorerst nicht.
      - Expander als **`fn`-Pointer/Trait-Objekte**, **nicht** `Fn`-Closures mit Zustand (Reinheit erhalten).

      ### 4.3 Impliziter Op-Graph [W, context §24, `resident_planner.md`]
      - Ausgabe ist eine **geordnete `Vec<Operation>`** pro Task (Reihenfolge = Abhängigkeit). Operation trägt nur `requires()`.
      - Verzweigung nur als **Folgeops beim Abschluss** erzeugen (keine automatische Parallelität zwischen unabhängigen Ops — bewusst, zugunsten Determinismus/Testbarkeit).

      ### 4.4 `materialize`-Regel [W, context §25, `resident_planner.md`]
      - `materialize = false` (Device-Handle bleibt im VRAM), wenn ein **GPU-Nachfolger** folgt.
      - `materialize = true` (Host-Kopie), wenn der **Nachfolger auf CPU** läuft, der **Task endet**, oder das Ergebnis in den **Index gespiegelt** werden muss.
      - Entscheidung trifft der Planner anhand von Op-Metadaten; die Operation **führt** sie aus (nicht der Worker). D2H-Kopien sind stream-geordnet (Host-Daten erst nach Completion lesen — relevant für C2).

      ### 4.5 Aufgeschobene Device-Wahl [W, context §26, `resident_planner.md`]
      - Op deklariert nur `ResourceKind` + optionale Affinität: `requires() -> Requirement { kind: ResourceKind, model, index, engine, affinity: Option<HandleId> }`, `#[non_exhaustive]`.
      - Die **Subengine bindet beim Claim** ein konkretes Device. Vorerst keine Handle-Affinität: zwei Engines, jede Ressourcenart höchstens einmal pro Device, `requires()` bestimmt die Engine eindeutig.
      - **Erweiterungspunkt:** `affinity`-Feld, falls dieselbe Art später auf beiden Devices existiert (Gruppe H / C2-Hook).

      ### 4.6 Op-Zähler & Abschluss-Vertrag [W, context §27, `resident_planner.md`]
      - Planner **definiert** den Vertrag: Folgeoperationen werden **am Ende der Queue eingereiht, vor** dem Dekrement des Vorgängers. Die eigentliche atomare Zählung (`AtomicUsize pending`, `fetch_add` vor Enqueue, `fetch_sub==1 ⇒ Done`) führt C2 aus.
      - Erster Fehler ⇒ `Cancelled` + `Err`, übrige Ops scheitern beim Claim.

      ### 4.7 Starved-Hint ohne Reinheitsverletzung [W, context §28, `resident_planner.md` Offen]
      - Hint ist **Eingabe** der Expansion: `expand(task, &ExpandContext { starved, caps, rules })` — **kein** versteckter Zustand. Gleiche Eingabe + gleicher Hint ⇒ gleiche Ops.
      - Der Hint darf **nur zwischen äquivalenten Varianten** wählen (Reihenfolge/Präferenz, z. B. CPU- vs. GPU-Encoder, wenn beide erlaubt), **nie** den Vertrag (Op-Menge/Ergebnis) ändern. → löst „Gewichtung des Starved-Hints" aus dem Metaplan.

      ---

      ## 5. Datentypen (Signatur-Skizze)

      ```rust
      pub trait Expander: Send + Sync {
          // rein: kein &mut self, kein innerer Zustand
          fn expand(&self, task: &Task, ctx: &ExpandContext) -> Result<Vec<Operation>, PlannerError>;
      }

      pub struct Planner { rules: arc_swap::ArcSwap<RuleSet> }     // Snapshot pro Aufruf
      impl Planner {
          pub fn new(rules: RuleSet) -> Self;
          pub fn expand(&self, task: &Task, starved: &StarvedSnapshot, caps: &CapabilitySet)
              -> Result<Vec<Operation>, PlannerError>;             // reine Funktion über Snapshot
          pub fn reload(&self, rules: RuleSet);                    // Hot-Reload: ArcSwap::store
      }

      pub struct RuleSet {
          expanders: std::collections::HashMap<(ModelId, TaskKind), Box<dyn Expander>>,
          pipelines: std::collections::HashMap<ModelId, PipelineConfig>, // aus TOML, validiert
      }

      pub struct ExpandContext<'a> {
          pub starved: &'a StarvedSnapshot,  // Präferenz, kein Vertrag
          pub caps: &'a CapabilitySet,       // B6
          pub pipelines: &'a std::collections::HashMap<ModelId, PipelineConfig>,
      }

      #[non_exhaustive]
      pub struct PipelineConfig { /* prefixes(TextRole→String), pooling, normalize, max_seq_len, dtype, scorer */ }

      pub enum TaskKind { Embedding, Similarity, Inference }
      ```

      **Expansionsregeln (aus `resident_planner.md`, verbindlich):**
      - Embedding: `Tokenize(CPU) → Embed(CPU|GPU) → Pool + L2 → [optional WriteIndex]`.
      - Query-Similarity: `Tokenize → Embed → Similarity(Scope) → TopK`.
      - Similarity mit CrossEncoder: Paare `(Query, Text aus Payload)` → `Infer(CrossEncoder) → Scores`, Scope aus früherem Ergebnis.
      - Gleitende Indizierung (Ausblick H2): nur Parameter-Hook, keine Kernel-Logik.

      ---

      ## 6. Umsetzungsschritte

      1. **Modul `planner/`** anlegen; Platzhalter für B3-`Operation`/`Requirement`/`Value` und B6-`CapabilitySet`, falls noch nicht vorhanden.
      2. **`Expander`-Trait + explizite Registry** (4.2): Enum-Dispatch/`HashMap`, `fn`-Pointer/Trait-Objekte.
      3. **Config** (4.1): `PipelineConfig` + `RuleSet`-Deserialisierung (TOML/serde) + Post-Deser-Validierung (Registry-Check); `ArcSwap`-Snapshot; optionaler `notify`-Watcher für Hot-Reload.
      4. **Expander-Implementierungen** für Embedding/Similarity/Inferenz gemäß §5.
      5. **`requires()`/Device-Wahl** (4.5): `Requirement` mit `ResourceKind` + `affinity: Option<HandleId>` (`#[non_exhaustive]`), Engine aus Capabilities eindeutig.
      6. **`materialize`-Regel** (4.4) in jeden Expander einbauen.
      7. **Starved-Hint** (4.7): `ExpandContext.starved` als Präferenz zwischen äquivalenten Varianten.
      8. **Abschluss-Vertrag dokumentieren** (4.6) für C2 (Enqueue vor Dekrement).
      9. **Tests** (§9).

      **Deliverables:** `planner`-Modul ohne Laufzeitabhängigkeiten, vollständig ohne GPU testbar; Golden-Expansionen pro Modell/Tasktyp.

      ---

      ## 7. Fehlerbehandlung (Q1) [B, context §30]
      `PlannerError` (ungültige Config, fehlende Capability, unbekanntes Modell, unbekannte Op/Ressourcenart). `#[non_exhaustive]`, nach oben via `#[from]` zusammengeführt. Keine Panics bei Fehl-Config — Fehler beim Deserialisieren/Validieren.

      ## 8. Logging (Q2) [B/W, context §31]
      `tracing`-Span pro Task-ID (vom aufrufenden Worker gesetzt). Expansionsentscheidungen nur als `debug`/`trace`-Events (gewähltes Device, `materialize`, Regel-Treffer). Planner ruft **nie** `set_global_default`.

      ## 9. Teststrategie & Mocks (Q3) [B/W, context §29]
      - **`proptest`** für Invarianten:
        - „Op-Menge ist unabhängig vom Starved-Hint" (nur Reihenfolge/Präferenz ändert sich).
        - „Reine Funktion": gleiche Eingabe ⇒ identische `Vec<Operation>`.
        - „Zähler endet bei 0" (Vertrag, gegen Referenz-Zustandsautomat via `proptest-state-machine`).
      - **Golden-Tests** (`insta`) für Expansionen pro Modell/Tasktyp.
      - **Mock-Capabilities** als `CapabilitySet`-Impl (z. B. nur CPU-Encoder → Embed muss CPU wählen; CPU+GPU → Hint entscheidet zwischen äquivalenten Varianten).
      - **Config-Tests:** unbekannte Op/Ressourcenart ⇒ Validierungsfehler; Hot-Reload tauscht Snapshot atomar.

      ## 10. Feature-Flags (Q4) [D/W, context §32]
      `planner` ist vollständig ohne `cuda` übersetz- und testbar (keine CUDA-Berührung). Device-Wahl basiert nur auf `CapabilitySet`-Daten.

      ## 11. Offene Risiken / zu verifizieren
      - Exakte Form von B3-`Operation`/`Requirement` und B6-`CapabilitySet` kann Feldnamen/Signaturen verschieben → nach B3/B6 angleichen.
      - Config-Schema (`PipelineConfig`) muss mit der konkreten Modell-Integration (Gruppe G1, e5-small: Präfixe `query:`/`passage:`, Mean-Pooling+L2) kompatibel bleiben → Felder dort gegenprüfen.
      - Starved-Hint-Gewichtung bleibt „Präferenz, kein Vertrag" (`resident_planner.md` Offen) — Invariante durch Property-Test absichern.
  reason: "Erstellen der Umsetzungspläne für Gruppe C (Übersicht + C1 Fassade, C2 Processor, C3 Planner) im Plan-Verzeichnis, fundiert auf resident_planplan.md, context_c.md und den Primärquellen."
```
/answer 0f9c-12 allow
Result Stats: id=a2410e84-bdfb-4959-9e99-b657158a114d, total: 153584, in: 123, out: 32859, reason: 5960, read: 34339, write: 120602
ReasoningToken: 5939
ReasoningToken: 21
Die Umsetzungspläne für Gruppe C (Orchestrierung) sind erstellt. Ich habe zuerst `resident_planplan.md`, den Gruppenkontext `context_c.md` sowie die Primärquellen `resident.md`, `resident_processor.md` und `resident_planner.md` gelesen und daraus vier Dateien im Verzeichnis `.../resident/plan` erzeugt:

## Erstellte Dateien

| Datei | Inhalt |
|---|---|
| `plan_C_overview.md` | Gruppenübersicht: Paket-Tabelle, Umsetzungsreihenfolge **C1 → C3 → C2**, Soll-Architektur (Fassade/Processor/Planner), Querschnitt Q1–Q4, Eingangsverträge aus Gruppe B, gruppenübergreifende Risiken |
| `plan_C1_fassade.md` | Fassade „Resident": Config-vor-Init (verbrauchender Builder), erweiterbare Parameterobjekte (`#[non_exhaustive]`), `TextRole`-Feld + eigener `PairInput`, Lazy-Init via `tokio::sync::OnceCell`, Worker als `std::thread`, `TaskHandle` (`IntoFuture`/`wait_timeout`/`Drop=Cancel`), Index-Residency-Tabelle, Daemon+CLI-Wrapper |
| `plan_C2_processor.md` | Processor: Queue-Verdrahtung, Worker-Loop mit GPU-Vorrang, GPU-Completion ohne Doppelverarbeitung (Callback + `cuEventQuery` + Lane-CAS), RAII-Leases, Starved-Hints als verfallendes Set, Shared-Queue ohne Head-of-Line-Blocking, Eviction, Abschluss/Fehler, `Completion`-Trait hinter `cuda` |
| `plan_C3_planner.md` | Planner: reine Expansion `Task→Vec<Operation>`, explizite Registry pro Modell/Tasktyp, TOML-Config + Validierung + Hot-Reload-Snapshot (`ArcSwap`), `materialize`-Regel, aufgeschobene Device-Wahl, Starved-Hint als reine Eingabe |

## Vorgehensprinzipien
- Jede Entscheidung ist auf einen Kontextpunkt mit **Belegstufe** ([B]/[D]/[W]) und – wo vorhanden – die Quelle zurückgeführt; die „Zu klären"-Punkte und die `Prüfen`-Checkliste aus `resident_processor.md` sind explizit aufgelöst.
- Jeder Plan enthält Scope (In/Out), Eingangsverträge (B1/B3/B4/B6), Signatur-Skizzen, sequenzierte Umsetzungsschritte mit Deliverables, Q1–Q4 und eine Risikoliste.
- Die **Fassaden-API** wurde bewusst noch nicht „eingefroren": Der Metaplan markiert die Scope-Charta (A1′) als Blocker für `SimilarityParams.scope` – das ist als Risiko vermerkt.

Hinweis beachtet: `resident_index.md` habe ich nicht als Inhaltsverzeichnis, sondern als Vektorindex-Spezifikation behandelt – für Gruppe C spielt es nur als Quelle des `Index`-Vertrags (B4) eine Rolle, der in C1 konsumiert wird.

Zwei zentrale, gruppenübergreifende Risiken sollten vor dem Start von Stufe 2 geklärt werden: die **`launch_host_function`-Signatur/candle-Stream-API in der gepinnten Version** (erster Schritt in C2) und die **Scope-Charta A1′** (blockiert den API-Freeze in C1). Soll ich diese beiden Punkte als Vorab-Abklärungspaket separat ausarbeiten?
