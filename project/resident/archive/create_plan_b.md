Erstelle Umsetzungspläne für Agenten im Verzeichnis `/home/user/xyan/xy.ai.workbench/project/resident/plan` für Gruppe B aus `resident_planplan.md`.

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
SystemInit:  id=657c2698-a929-41ee-beaa-769df39f0712, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-opus-4-8
Text:
 I'll start by reading the key files to understand what Group B requires and how the plans should be structured.
Control Request:
```yaml
id: 0f9c-c
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planplan.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/context/context_b.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident.md
  reason: "Reading the plan-of-plans to identify Group B's scope and deliverables, the Group B context file, and the main resident project overview to understand the overall architecture before writing implementation plans."
```
/answer 0f9c-c allow
ReasoningToken: 313
Text:
 Now let me read the source documents referenced by Group B's packages to extract the exact specifications needed for the implementation plans.
Control Request:
```yaml
id: 0f9c-d
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_subengine.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_index.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_precision.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_planner.md
  reason: "Reading all source documents referenced by Group B packages (B1 queue/processor, B2 subengine §4.2 memory, B3 planner/op model, B4 index contract, B5 kernel abstraction, B6 precision capabilities) to extract exact contract/type specifications for the implementation plans."
```
/answer 0f9c-d allow
ReasoningToken: 1064
Text:
 Before writing, let me check whether the `plan` directory already contains plans (e.g., from Group A) so I can match the existing format and naming convention.
Control Request:
```yaml
id: 0f9c-e
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/resident_plan_a1.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/resident_a1.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/A1.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/plan_a1.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/README.md
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/resident_plan_b1.md
  reason: Probing the plan directory for any existing Group A implementation plans or README to match naming convention and document format before creating Group B plans.
```
/answer 0f9c-e deny matching is not required
ReasoningToken: 1575
Text:
 The probe was denied — understood, I won't try to match an existing format. I now have everything I need from the planplan, the Group B research context, and all referenced source docs. I'll create one implementation plan per package (B1–B6) plus a short group overview, written as Stufe‑2 plans an agent can execute directly.
Control Request:
```yaml
id: 0f9c-f
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/resident_plan_b.md
    mode: replace
    content: |
      # Umsetzungspläne Gruppe B — Kern-Datentypen & Verträge

      **Stufe 2 (Ausführung).** Diese Datei ist die Gruppenübersicht zu den Paketplänen `resident_plan_b1.md` … `resident_plan_b6.md`. Sie bündelt Reihenfolge, Abhängigkeiten, Modulzuordnung, gemeinsame Konventionen (Querschnitt Q) und Typ-Eigentümerschaft. Grundlagen: `../resident_planplan.md` (Metaplan Gruppe B) und `../context/context_b.md` (Recherche B1–B6, Stand 10.10.2026).

      **Zielprojekt:** Rust-Workspace `/home/user/xyan/xy.ai.workbench/rag`. **Zielhardware:** GTX 1660 (TU116, sm75, 6 GB).

      ---

      ## 1. Zweck der Gruppe

      Gruppe B legt die **Kern-Datentypen und Verträge** fest, bevor Orchestrierung (Gruppe C), Subengines (D), Index-Management (E) und Präzision (F) darauf aufsetzen. Alle Pakete liefern **Spezifikation + Rust-Typen/Traits + Tests**, aber **keine** GPU-/CUDA-Laufzeitlogik (die liegt in D/E hinter Feature-Flag `cuda`).

      Oberste Invariante für alle B-Pakete (aus `resident_subengine.md` §2):
      > **Kein `Tensor` und kein Raw-Pointer in öffentlichen Signaturen** — nur `Region` + `Layout` (B2) bzw. `Value`/`Lease` (B3). **Kein `unsafe` außerhalb von `arc-swap`.** Candle-Typen nur in `pub(crate)`-Adaptern hinter `feature = "candle"`.

      ---

      ## 2. Reihenfolge & Abhängigkeiten

      Empfohlene Sequenz (aus Metaplan): **B1 → B2 → B3 → B4 → B5 → B6**.

      | Paket | Titel | Abhängt von | Blockiert |
      |---|---|---|---|
      | B1 | Queue & Task-Primitive | A4 | C1, C2, D1, E9 |
      | B2 | Region / Layout / Speicher-Modell | A4 | B3, B5, D2, D3 |
      | B3 | Operation-/Op-Modell | B1, B2 | C1, C2, C3 |
      | B4 | Index-Trait & -Vertrag | A4 | C1, E1, E2, E5, E6, G2 |
      | B5 | Kernel-Abstraktion | B2 | D6, E4, E5, E8 |
      | B6 | CapabilitySet & Profile-Typen | A4 | C3, F1–F5 |

      B4 ist **eigenständig vom Kern** (Index-Trait) und kann parallel zu B1–B3 bearbeitet werden. B1/B2/B4/B6 brauchen nur A4 (Querschnittskonventionen).

      ---

      ## 3. Modulzuordnung (vorläufig — endgültig entscheidet A2)

      Alle B-Typen bilden die kern­nahe Vertragsschicht. Vorschlag (A2 besitzt die finale Crate-/Modulgrenze):

      ```
      rag/
      └─ crates/resident-core/           # reine Verträge, kein CUDA, kein candle-Zwang
         ├─ queue.rs        (B1)  Node, ClaimState, TaskState, Queue
         ├─ mem.rs          (B2)  Region, Layout, DType, MemoryManager-Trait
         ├─ op.rs           (B3)  Op-Trait, OpParams, Requirements, Value, Lease
         ├─ index.rs        (B4)  Index-Trait, Scope, AccessMethod, Stamp, EntrySet
         ├─ kernel.rs       (B5)  Kernel-Trait, SegmentDesc, SegmentLayout, KernelHit, Mock
         ├─ caps.rs         (B6)  AxisId, ValueId, CapabilitySet, Profile
         └─ error.rs        (Q1)  gemeinsame Fehler-Taxonomie (von A4/Q1 geliefert)
      ```

      Candle-/safetensors-Adapter liegen in einem separaten `pub(crate)`-Modul (`adapter_candle.rs`) hinter `feature = "candle"`; CUDA-Pfade hinter `feature = "cuda"`.

      ---

      ## 4. Typ-Eigentümerschaft (wer definiert was)

      | Typ | Definiert in | Benutzt von |
      |---|---|---|
      | `Node`, `ClaimState`, `TaskState`, `Queue`, `Notify`-Relay | B1 | C1/C2 (Task-Queue + 3 Op-Queues), D1, E9 |
      | `Region`, `Layout`, `DType`, `MemoryManager` (Trait), `DeviceKind` | B2 | B3 (`Value::Device`), B5 (Spans), D2/D3/E3 |
      | `Op` (Trait), `OpKind`, `OpParams`, `Requirements`, `ResourceKind`, `Value`, `Lease`, `Affinity` | B3 | C3 (Planner-Ausgabe), C2, D |
      | `Index` (Trait), `EntryId`, `Stamp`, `Scope`, `AccessMethod`, `EntrySet`, `EntryData`, `SummaryRef` | B4 | E1/E2/E5/E6, G2 |
      | `Kernel` (Trait), `SegmentDesc`, `SegmentLayout`, `KernelHit`, `WrittenRange`, `MockKernel` | B5 | D6, E4/E5/E8 |
      | `AxisId`, `ValueId`, `CapabilitySet`, `Profile` | B6 | C3 (`requires()`-Caps), F1–F5 |

      **Namenskollision vermeiden:** Es gibt zwei „Layout“-Begriffe. B2 `Layout{shape,strides,dtype}` (Tensor-Layout). B5 Segment-Layout heißt deshalb **`SegmentLayout`** (`Fixed{stride}|Variable{offsets}`).

      ---

      ## 5. Gemeinsame Konventionen (Querschnitt Q, aus A4)

      - **Q1 Fehler (`thiserror`):** gemeinsame Taxonomie in `error.rs`. Für B relevant: `ArenaExhausted` (B2), `IndexUnavailable`/`StaleMapping`(intern) (B4), `TransferFailed` (B5), `Incompatible`/`InvalidPrecision` (B6), Config-/Capability-Fehler (B3). Jedes Paket trägt seine Varianten bei; kein stilles Ausweichen.
      - **Q2 Logging (`tracing`):** Span pro Task-ID (B1/B3), pro Manager-Zyklus (B4), pro Batch/Kernel (B5). Invariantenverletzungen auf `debug_assert!` + `warn!`.
      - **Q3 Tests & Mocks:** No-GPU-Pfad verpflichtend. Mocks: `MockKernel` (B5), kein GPU in CI. Testmatrizen je Paket (insb. B1 Queue-Semantik, B4 Stamp/Racy, B6 Pareto/Matching-Vorarbeit).
      - **Q4 Feature-Flags:** reale CUDA-/candle-Pfade hinter `cuda`/`candle`; die reine Logik (Zustandsautomat, Free-List, Stamp-Vergleich, CapabilitySet) ist ohne GPU testbar.

      ---

      ## 6. Definition of Done (Gruppe)

      1. `resident-core` kompiliert **ohne** `cuda`/`candle`-Feature (reine Logik).
      2. Alle öffentlichen Signaturen frei von `Tensor`/Raw-Pointer; kein `unsafe` außer via `arc-swap`.
      3. Jedes Paket B1–B6 erfüllt seine eigene DoD (siehe Paketdatei) inkl. Tests.
      4. Offene Punkte sind als `// TODO(A3): …` markiert und im jeweiligen Plan-Abschnitt „Offene Punkte“ gelistet.
      5. Die drei Namens-/Konsistenz-Hinweise des Metaplans sind beachtet (Layout-vs-SegmentLayout, Stamp nur `!=`, DType `#[non_exhaustive]`).
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/resident_plan_b1.md
    mode: replace
    content: |
      # Umsetzungsplan B1 · Queue & Task-Primitive

      | | |
      |---|---|
      | **Paket** | B1 (Gruppe B — Kern-Datentypen & Verträge) |
      | **Abhängt von** | A4 (Querschnittskonventionen Q1–Q4) |
      | **Blockiert** | C1 (Fassade), C2 (Processor: Task-Queue + 3 Op-Queues), D1, E9 |
      | **Ergebnis** | Datentyp- & Zustandsautomaten-Spezifikation + Rust-Implementierung der lock-freien Queue und Task-Primitive |
      | **Quellen** | `../resident.md` (Queue Logik, Task, Notify-Topologie), `../resident_processor.md` (Queues, Worker), `../context/context_b.md` §B1 (1–7) |

      ---

      ## 1. Zweck & Abgrenzung

      Eine **einzige** einfach verkettete, lock-freie Liste: nur **Append** (CAS auf `tail.next`) und **Unlink**. Reihenfolge = Alter. Keine Version, kein Snapshot, kein Lock. Priorität, Ressource, Modell, Modus sind **Metadata in `params`** — die Queue kennt sie nicht.

      Die Struktur wird mehrfach instanziiert: eine **Task-Queue** und drei **Op-Queues** (CPU/GPU/shared — Letztere angelegt, leer, ohne Logik; siehe `resident_processor.md`). B1 liefert den **generischen** Queue-/Node-Baustein plus die konkrete **Task-Instanziierung**. Die Operation-Instanziierung folgt in B3/C2.

      **Nicht Teil von B1:** Worker-Loop, Batching, Eviction, Expansion (C2/C3). B1 liefert nur Datentypen, Zustandsautomat, Notify-Relay-Hook.

      **Invarianten:** Kein `unsafe` außerhalb von `arc-swap`. Iterierer blockieren nie. Drop iterativ.

      ---

      ## 2. Crates (Pins kommen aus A3)

      - `arc-swap` (≥ 1.6, wegen Miri-/Stacked-Borrows-Fixes) — `ArcSwapOption<Node>` als Link.
      - `tokio` (`sync`-Feature) — `Notify`, später `watch`.
      - `parking_lot` — `Mutex` für `payload` (kein Poisoning, lock-freie Umgebung). Alternativ `std::sync::Mutex` mit Poison-Recovery (A3-Entscheidung).
      - `std::sync::OnceLock` (stable ≥ 1.70) für das Set-once-Ergebnis.

      ---

      ## 3. Zu implementierende Typen

      ### 3.1 Zustandsautomat (`ClaimState`)

      ```rust
      const QUEUED: u8 = 0; const TAKEN: u8 = 1; const DONE: u8 = 2; const CANCELLED: u8 = 3;

      pub struct TaskState {
          state:  AtomicU8,                          // QUEUED | TAKEN | DONE | CANCELLED
          result: OnceLock<Result<Output, Error>>,   // genau einmal gesetzt durch Claim-Inhaber
          ready:  Notify,                            // weckt wartende Aufrufer
      }
      ```

      Übergänge (alle **ein** Schritt → `compare_exchange` *strong*, `AcqRel`/`Acquire`):
      - `claim`: `QUEUED → TAKEN`. Einzige Stelle, an der Cancel relevant ist (scheitert bei `CANCELLED`/`TAKEN`/`DONE`).
      - Fertigstellung: **erst** `result.set(out)`, **dann** `state.store(DONE, Release)`. Alleiniger Eigentümer (Taken) → `store` genügt. Danach `ready.notify_waiters()`.
      - `cancel`: `QUEUED → CANCELLED`, danach `ready.notify_waiters()`. War schon `TAKEN`: kein Effekt. `TAKEN → CANCELLED` ist **nicht** erlaubt (laufende Arbeit); falls je nötig, separates Flag `cancel_requested`.
      - Leser/Waiter: `state.load(Acquire)` — Release auf `DONE` + Acquire garantiert Sichtbarkeit von `result`. **Kein `SeqCst`.**
      - Zweites `result.set` → `Err(value)`: als Invariantenverletzung `debug_assert!` + `tracing::warn!`.

      ### 3.2 Node & Link

      ```rust
      pub struct Node<P, D> {
          pub params: P,                        // unveränderlich, zum Filtern VOR dem Claim
          pub state:  Arc<TaskState>,           // Handle des Aufrufers
          payload:    parking_lot::Mutex<Option<D>>, // beim Claim per take() entnommen
          next:       ArcSwapOption<Node<P, D>>,     // Append/Unlink per CAS
      }
      ```

      - **Append** (`append(tail, new)`): `tail.next.compare_and_swap(&None, Some(new.clone()))`; Swap fand statt, wenn der Rückgabe-`Guard` pointer-gleich `None` ist (Vergleich ist **Pointer-Identität**). Konflikt → Tail vorlaufen und erneut.
      - **Payload-Übergabe:** `let data = node.payload.lock().take();` **nur nach gewonnenem Claim-CAS**. Lock kurz halten, Daten außerhalb des Locks verarbeiten, **kein `await` unter dem Lock**.
      - Rückverweise (falls je nötig) nur `Weak`; `next` ist der einzige starke Zeiger → keine Zyklen.

      ### 3.3 Queue mit Sentinel-Tail

      ```rust
      pub struct Queue<P, D> {
          head:   Arc<Node<P, D>>,   // Dummy-Sentinel; echte Knoten hängen dahinter
          relays: Vec<Arc<Notify>>,  // Notify jeder Subengine (Append-Relay)
      }
      ```

      - `push(params, state, data)`: Knoten anlegen, am Tail anhängen, danach **jedes** `relays`-Notify `notify_one()`-en (Append-Relay der Notify-Topologie).
      - `iter()`: traversiert per `next.load()` (**Guard**, kein `load_full()`) — vermeidet Atomic-RMW-Contention auf geteilter Cacheline. Hält pro Schritt ein `Arc<Node>` → Pointer-ABA ausgeschlossen, Use-after-free unmöglich (kein Epoch/Hazard-Pointer nötig).
      - `claim(node) -> bool`: delegiert an `node.state`-CAS `QUEUED → TAKEN`.

      ### 3.4 Unlink — **Entscheidung A3 vorbereiten, Default festlegen**

      Reines CAS-Unlink einer einfach verketteten Liste ist ohne Markierung **nicht korrekt** (Harris-Problem: paralleles Aushängen von B und Anhängen/Aushängen hinter B verliert Änderungen). Default dieses Plans:

      1. **Logisches Löschen** über `state` (`CANCELLED`/`TAKEN`/`DONE`); physisches Aushängen nur **lazy** durch Traversierer.
      2. **Niemals den Sentinel-/Tail-Knoten aushängen.**
      3. Alternativ (A3): Unlink unter kurzem `Mutex` (Queue-Mutation selten, Durchlauf lock-frei per `load`).

      > `// TODO(A3): logisches Löschen + Dummy-Tail (Empfehlung) vs. Mutex-geschützte Mutation endgültig festlegen (context_b §B1.1, offener Punkt 3).`

      ### 3.5 Iterativer Drop (Pflicht)

      ```rust
      impl<P, D> Drop for Node<P, D> {
          fn drop(&mut self) {
              let mut cur = self.next.swap(None);
              while let Some(arc) = cur {
                  match Arc::try_unwrap(arc) {
                      Ok(mut node) => cur = node.next.swap(None),
                      Err(_shared) => break, // anderer Besitzer übernimmt den Rest
                  }
              }
          }
      }
      ```
      Ohne dies: Rekursionstiefe = Kettenlänge → Stack-Overflow bei Zehntausenden Knoten.

      ---

      ## 4. Aufrufer- & Engine-Protokoll (Notify)

      **Aufrufer** (wartet auf Ergebnis) — Reihenfolge gegen verlorene Wakeups:
      ```rust
      let notified = state.ready.notified();
      tokio::pin!(notified);
      notified.as_mut().enable();                 // registrieren VOR dem Prüfen
      match state.state.load(Acquire) {
          DONE | CANCELLED => { /* fertig */ }
          _ => notified.await,                    // sonst warten
      }
      ```
      Timeout = eigene Deadline des Aufrufers; `Cancelled` lokal abgeleitet. `notify_waiters()` (bei Fertig/Cancel) hat **kein** Permit → `Notified` muss vor der Prüfung existieren.

      **Notify-Topologie** (aus `resident.md`), B1 liefert die Hooks:
      | Quelle | Wirkung |
      |---|---|
      | Queue-Append | Relay auf das Notify **jeder** Subengine (`relays`) |
      | Result / Init fertig / Evict-Timer | eigenes Notify (von C2 ausgelöst) |
      | Iteration mit Fortschritt | eigenes Notify |

      ---

      ## 5. Wiederverwendung für Op-Queues (Hinweis an C2/B3)

      `Queue<P, D>` ist über `params: P` und `payload: D` generisch. Instanzen:
      - **Task-Queue:** `P = TaskParams`, `D = TaskData`, `state = Arc<TaskState>` mit `Result<Output, Error>`.
      - **Op-Queues (CPU/GPU/shared):** `P = OpParams` (B3), `D = Value`-Eingabe; der Abschluss läuft über den Task-Op-Zähler (C2/B3), nicht über `TaskState::result`.
      Die **shared**-Queue wird angelegt, bleibt leer und ohne Logik; Subengine prüft erst die eigene, dann shared.

      ---

      ## 6. Fehler & Logging

      - Fehler via gemeinsame Taxonomie (Q1). B1 selbst wirft kaum typisierte Fehler; `result` transportiert `Result<Output, Error>` des Tasks.
      - Doppel-Set von `result`, unerwarteter CAS-Zustand → `debug_assert!` + `tracing::warn!`.
      - `tracing`-Span pro Task-ID (Q2); Events: `push`, `claim` (erfolgreich/abgelehnt mit aktuellem Zustand), `complete`, `cancel`.

      ---

      ## 7. Teststrategie (Q3, ohne GPU)

      - **Zustandsautomat:** jeder erlaubte/verbotene Übergang; `claim` nach `cancel` scheitert; `cancel` nach `claim` wirkungslos; `Taken → Cancelled` abgelehnt.
      - **Sichtbarkeit:** `complete` setzt `result` vor `DONE`; Leser sieht nach `load(Acquire)==DONE` immer das Ergebnis (loom-Modell oder gezielte Thread-Tests).
      - **Nebenläufigkeit:** `loom`-Test (Feature-gated) für Append + Claim + Cancel; mehrere Claimer, genau einer gewinnt.
      - **Notify:** kein verlorenes Wakeup (Completion zwischen `enable()` und `await`); `notify_waiters` ohne Wartende + anschließende Zustandsprüfung.
      - **Payload:** `take()` genau einmal; nach Claim ist `payload` `None`.
      - **Drop:** Kette mit ≥ 100 000 Knoten droppt ohne Stack-Overflow (iterativ).
      - **ABA/Retention:** ausgehängter Knoten mit lebendem Durchläufer bleibt gültig; dieselbe ID neu eingereiht erhält neuen Knoten/Zustand (Zustand über Automat, nicht Pointer).

      ---

      ## 8. Offene Punkte (→ A3 / Spezifikation)

      1. Unlink-Strategie: logisches Löschen + Dummy-Tail (Default) vs. Mutex-Mutation.
      2. `parking_lot::Mutex` vs. `std::sync::Mutex` (+ Poison-Recovery) für `payload`.
      3. Exakte Pins: `arc-swap`, `tokio`, `parking_lot` (A3).

      ## 9. Definition of Done

      - `Queue<P,D>`, `Node`, `TaskState`/`ClaimState` implementiert, kein `unsafe` außer `arc-swap`.
      - Append/Claim/Complete/Cancel/Unlink + iterativer Drop + Notify-Relay vorhanden.
      - Testmatrix §7 grün, inkl. `loom`-Test (Feature-gated) und Drop-Stresstest.
      - Task- und Op-Instanziierung dokumentiert (für C2/B3).
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/resident_plan_b2.md
    mode: replace
    content: |
      # Umsetzungsplan B2 · Region / Layout / Speicher-Modell

      | | |
      |---|---|
      | **Paket** | B2 (Gruppe B) |
      | **Abhängt von** | A4 |
      | **Blockiert** | B3 (`Value::Device`), B5 (Spans), D2/D3 (Device/Arena), E3 (ArenaAllocator) |
      | **Ergebnis** | Speicher-Abstraktionsvertrag: `Region`, `Layout`, eigenes `DType`, `MemoryManager`-Trait, Arena-Konzept (ohne GPU-Details) |
      | **Quellen** | `../resident_subengine.md` §4.2, `../context/context_b.md` §B2 (8–11) |

      ---

      ## 1. Zweck & Abgrenzung

      Einheitliches, **pointer- und tensorfreies** Speichermodell. Öffentliche Signaturen kennen nur `Region` (Handle) und `Layout` (Form/Strides/dtype). Die Auflösung Handle → Device-Pointer liegt **ausschließlich im Backend** (D2/D3/E3), nie in B2.

      **Nicht Teil von B2:** konkrete GPU-Arenen, `cuMemGetInfo`-Budget, Pinned Staging, Stream-geordnete Wiederverwendung (alles D3/E3). B2 definiert die **Schnittstelle** und das **Free-List-Konzept** generisch und ohne GPU.

      **Invariante (No-Tensor/No-Pointer):** Der Allokator kennt nur `(offset, len, align)`; `Region` ist ein Handle, kein Zeiger.

      ---

      ## 2. Zu implementierende Typen

      ### 2.1 DeviceKind & dtype-Policy

      ```rust
      #[non_exhaustive]
      pub enum DeviceKind { Cpu, Gpu }

      pub fn compute_dtype(dev: DeviceKind) -> DType {
          match dev { DeviceKind::Gpu => DType::F16, DeviceKind::Cpu => DType::F32 }
      }
      ```
      Policy: **GPU f16 / CPU f32** (passt zu 6 GB VRAM; f16 halbiert Bedarf). Index-dtype ist pro Index fest; gemischte CPU/GPU-Indexierung nur mit definierter Toleranz (F/Index-Kalibrierung).

      ### 2.2 Eigenes DType-Enum (kein candle-Leak)

      ```rust
      #[non_exhaustive]
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
      pub enum DType { F32, F16, Bf16, U8, U32, I64 /* erweiterbar */ }
      impl DType { pub const fn size_bytes(self) -> usize { /* … */ } }
      ```
      - `#[non_exhaustive]`, weil candle (0.10.2/0.11.0) **14** Varianten inkl. Dummy-Typen führt (`F8E4M3`, `F6E2M3`, …). Öffentliche Signaturen nennen **nur** das eigene Enum.
      - Adapter **nur** in `pub(crate)`-Modul hinter `feature = "candle"`:
        ```rust
        impl TryFrom<DType> for candle_core::DType { type Error = UnsupportedDType; /* … */ }
        impl TryFrom<candle_core::DType> for DType { type Error = UnsupportedDType; /* Wildcard-Arm Pflicht */ }
        ```
        `TryFrom` (nicht `From`), da candle neue/Dummy-Varianten hat. Analog `TryFrom<safetensors::Dtype>` (Namen weichen ab: F16/BF16/F32/U8/I64).
      - **sm75-Hinweis (aus B6-Recherche):** `Bf16` existiert als Typ, ist aber auf TU116 **nicht nativ**; dtype-Wahl für GPU-Rechnen = `F16`.

      ### 2.3 Region (Handle) & generationale Entwertung

      ```rust
      pub struct Region {
          pub arena:  ArenaId,   // welche Arena
          pub offset: u64,       // Arena-Offset in BYTES
          pub len:    u64,       // Länge in BYTES
          pub gen:    u32,       // entwertet Handle nach Evict/Reload
      }
      ```
      - `gen` wird beim Freigeben/Reload des Slots erhöht; ein veraltetes Handle erkennt der Manager per `slot.gen == handle.gen` (O(1)-Vergleich). Stale-Zugriff ist intern `StaleMapping` (Q1), nach außen nie ein roher Fehlzugriff.
      - Handle-Verwaltung optional über `slotmap`/`thunderdome` (8 B Key, `gen` als `u32`); bei Überlauf Slot stilllegen. Bei Offset-Bezug reicht das **eigene** `Region`-Struct (keine Crate-Bindung).

      ### 2.4 Layout (Tensor-Layout)

      ```rust
      pub struct Layout { pub shape: Vec<usize>, pub strides: Vec<usize>, pub dtype: DType }
      ```
      - **Entscheidung:** `strides` und (impliziter) Offset werden in **Bytes** geführt (Region ist offset-/byte-basiert). Umrechnung an der candle-Grenze: candle `Layout` nutzt **Elemente** → `bytes = elems * dtype.size_bytes()` im Adapter.
        > `// TODO(A3): Bytes- vs. Elemente-Konvention final bestätigen (context_b §B2.10). candle narrow/transpose/permute/broadcast ändern nur Layout.`
      - Views (narrow/transpose/permute/broadcast) sind reine Layout-Operationen, keine Kopien.

      ### 2.5 MemoryManager-Trait + Arena-Konzept

      ```rust
      pub trait MemoryManager {
          fn alloc(&self, dev: DeviceKind, bytes: u64, align: u64) -> Result<Region, Error>; // Error::ArenaExhausted
          fn free(&self, region: Region);
          // resolve(): NUR intern/pub(crate), gibt Backend-spezifische Adresse — nie in öffentlicher API
      }
      ```
      - **Arenen nach Lebensdauer** (Benennung/Belegung in D3/E3): Weights (langlebig), Index-Segment (gestreamt/evictable), Workspace/Scratch pro Lane (kurzlebig), Staging (Pinned, nur GPU), ParamBlock pro Lane (klein, fest).
      - **Free-List-Konzept (B2, backend-agnostisch):** Offsets als `u64`, Handles statt Zeiger, Free-List als `BTreeMap<u64 /*offset*/, u64 /*len*/>` mit Coalescing (Nachbarsuche). Varianten First-/Best-Fit, TLSF (O(1)), Buddy. Konkrete GPU-Arena (VRAM-Budget via `cuMemGetInfo`, CoW-Doppelbedarf, Copy-Stream) = D3/E3.
      - Rust-Bausteine (A3-geprüft): `offset-allocator`, `gpu-allocator`, `range-alloc`.

      ---

      ## 3. Fehler & Logging

      - Q1: `ArenaExhausted` (B2), `UnsupportedDType` (Adapter), `StaleMapping` (intern, bei gen-Mismatch).
      - Q2: Arena-Auslastung und alloc/free als Debug-Events (die lauten Betriebslogs liegen in D3/E3).

      ## 4. Teststrategie (Q3, ohne GPU)

      - **DType:** `size_bytes` für alle Varianten; `TryFrom`-Roundtrip candle↔eigen (hinter `candle`-Feature) inkl. Fehler bei Dummy-Variante.
      - **Region/gen:** Alloc→Free→Realloc erhöht `gen`; altes Handle wird als stale erkannt.
      - **Free-List:** Alloc/Free-Sequenzen, Coalescing benachbarter Blöcke, Fragmentierung, `ArenaExhausted` bei Überbelegung (reine Logik, mit Fake-Backend).
      - **Layout:** Byte-Strides-Berechnung; Adapter-Umrechnung Elemente↔Bytes.
      - No-Pointer-Invariante: Compile-Test, dass keine öffentliche Signatur candle-/Pointer-Typen nennt (Doc-Test/`trybuild` optional).

      ## 5. Offene Punkte (→ A3)

      1. Byte- vs. Elemente-Konvention für `Layout::strides` final.
      2. Eigenes `Region`-Struct vs. `slotmap`/`thunderdome` für Slot-Verwaltung.
      3. Free-List-Variante (BTreeMap/TLSF/Buddy) und Crate-Pin (`offset-allocator` u. a.).
      4. candle-Version (0.10.x/0.11.x) für den Adapter.

      ## 6. Definition of Done

      - `Region`, `Layout`, `DType` (+`size_bytes`, +`TryFrom`-Adapter hinter `candle`), `DeviceKind`, `compute_dtype`, `MemoryManager`-Trait + backend-agnostische Free-List implementiert.
      - Kein `Tensor`/Pointer in öffentlicher API; gen-Entwertung + `ArenaExhausted` getestet.
      - Baut ohne `cuda`/`candle`-Feature.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/resident_plan_b3.md
    mode: replace
    content: |
      # Umsetzungsplan B3 · Operation-/Op-Modell

      | | |
      |---|---|
      | **Paket** | B3 (Gruppe B) |
      | **Abhängt von** | B1 (Queue/Node), B2 (`Region`/`DeviceKind`) |
      | **Blockiert** | C1 (Fassade), C2 (Processor/Op-Queues), C3 (Planner-Ausgabe) |
      | **Ergebnis** | Op-/Value-Vertrag: `Op`-Trait, `OpParams`, `Requirements`, `Value`, `Lease`, impliziter Graph via Reihenfolge |
      | **Quellen** | `../resident_planner.md` (Ausgabe `Operation`), `../resident_subengine.md` (Task ≠ Op, §4.2 Region), `../resident_processor.md` (Datenübergabe/Leases), `../context/context_b.md` §B3 (12–14) |

      ---

      ## 1. Zweck & Abgrenzung

      `Operation` ist die **datenhafte Beschreibung** eines Schritts auf einem Device. Sie trägt unveränderliche `params` mit `requires()` (Ressourcenart, Modell, Index, erlaubte Engine), ein `materialize: bool` und `Value`-Ein/Ausgaben. Der **Graph ist implizit** durch die Reihenfolge der Operationen eines Tasks.

      **Scharfe Trennung (`resident_subengine.md`):**
      - **Task** = Nutzeranfrage (Host-Typen, Daten + Parameter), Queue-Knoten mit `Arc<TaskState>` (B1).
      - **Op** = Schritt auf einem Device (Regions + Layouts). Ein **Planner** (C3) expandiert Task → Pipeline aus Ops. Op bleibt **reine Beschreibung/Daten**.

      **Nicht Teil von B3:** Expansionsregeln (C3), Worker-Loop/Completion/Backpressure (C2). B3 liefert die Typen, die Planner erzeugt und Processor/Subengine konsumieren.

      ---

      ## 2. Zu implementierende Typen

      ### 2.1 Op-Trait (Op als Daten)

      ```rust
      pub trait Op: Send + Sync {
          fn kind(&self) -> OpKind;        // stabiler Bezeichner
          fn params(&self) -> &OpParams;   // unveränderlich; Clone + Hash + Eq
          fn requires(&self) -> Requirements;
      }

      #[non_exhaustive]
      pub enum OpKind { Tokenize, Embed, Pool, L2Norm, Similarity, TopK, WriteIndex, Infer /* … */ }
      ```
      Muster (Vorbild candle `UnaryOp`/`BinaryOp`): Op als **Enum** oder `Arc<dyn Op>` + `params`. Der Scheduler matcht `requires() ⊆ worker.capabilities()` (Node-Selector-Prinzip).

      ### 2.2 Requirements & ResourceKind

      ```rust
      #[non_exhaustive]
      pub struct Requirements {
          pub resource: ResourceKind,       // Tokenizer | Encoder | Scorer | Index | CrossEncoder …
          pub model:    Option<ModelId>,
          pub index:    Option<IndexId>,
          pub engine:   EngineSelector,     // Cpu | Gpu | Either (requires() bestimmt Engine eindeutig)
          pub caps:     CapabilitySet,      // aus B6 (leer erlaubt)
          pub affinity: Option<Affinity>,   // Erweiterungspunkt, s. u.
      }

      #[non_exhaustive]
      pub enum ResourceKind { Tokenizer, Normalizer, Encoder, Scorer, Index, CrossEncoder }

      #[non_exhaustive]
      pub enum EngineSelector { Cpu, Gpu, Either }
      ```
      - Device-Wahl (Planner, C3): Stage ohne Device-Festlegung trägt nur die **Ressourcenart**; die Zuordnung entscheidet der **Claim** der Subengine. Derzeit: zwei Engines, jede Ressourcenart höchstens einmal pro Device → `requires()` bestimmt die Engine eindeutig.
      - **Affinität = Erweiterungspunkt** (keine aktive Logik in Phase 1):
        ```rust
        #[non_exhaustive]
        pub enum Affinity { Handle(Region), Device(DeviceId) }
        ```
        Scheduler fragt nur `requires()`; neue Varianten ändern **Fassade/Task nicht** (context_b §B3.14).

      ### 2.3 Value & Lease (RAII)

      ```rust
      pub enum Value { Host(HostData), Device(DeviceHandle) }
      ```
      - **Kein `Tensor` im `Value`.** `DeviceHandle` ist eine **Lease** auf eine `Region` (B2), kein Tensor.
      - Transfers sind **explizit** als eigene Op modelliert (`into_host`/Device→Host nur über eine Op), um ungewollte Rücktransfers in Hot-Paths zu vermeiden.
      - **Lease als RAII** (bevorzugt vor manuellen Zählern):
        ```rust
        pub struct Lease { handle: Region, pin: Arc<PinCount> }   // Drop dekrementiert PinCount
        ```
        - Evict nur bei `pin == 0` (Processor/Subengine). Erzeugung prüft `gen` (Debug-Assertion beim Claim, kein eigener Pfad).
        - **`Drop` darf nicht blockieren/awaiten:** asynchrone Freigabe über eine Freigabe-Queue (Channel), nicht Arbeit im `Drop`.
        - Offenes Handle = Lease auf Arena **und** Ressource, Use-Zähler > 0 → Ressource wird nicht evicted. Lease endet mit der letzten konsumierenden Operation oder im Cancel-/Timeout-Pfad.

      ### 2.4 Operation-Knoten (Instanziierung von B1)

      ```rust
      pub struct OperationNode {
          pub op:       Arc<dyn Op>,
          pub input:    Value,
          pub output:   OnceLock<Value>,     // gesetzt beim Fertigwerden
          pub materialize: bool,             // Ergebnis in Host zurückholen (true) oder Device-Handle halten (false)
          pub task:     Arc<TaskCtx>,        // gemeinsamer Zähler offener Ops + Task-ID/Priorität
      }
      ```
      - `materialize` setzt der Planner (C3): `true`, wenn CPU-Nachfolger / Task-Ende / Index-Spiegelung; `false`, wenn GPU-Nachfolger (Handle bleibt im VRAM).
      - Abschluss: `TaskCtx` führt den Zähler offener Operationen; bei 0 → Task `Done`. Erster Fehler → `Cancelled` + `Err`, übrige Ops scheitern beim Claim.
      - Als Queue-Payload: `Queue<OpParams, Value>` (B1), eingereiht in die passende Op-Queue (CPU/GPU/shared) durch C2.

      ---

      ## 3. Fehler & Logging

      - Q1: `Incompatible` (Capability nicht erfüllbar), Config-/Planner-Fehler (ungültige Params). B3 definiert die Varianten, die C3 wirft.
      - Q2: `tracing`-Span pro Task-ID; Events: Op erzeugt (kind, requires), materialize-Entscheidung, Lease auf/zu.

      ## 4. Teststrategie (Q3, ohne GPU)

      - `requires()` ⊆ Capabilities: Match true/false (nutzt B6 `CapabilitySet::satisfies`).
      - `OpParams`: `Clone`/`Hash`/`Eq` stabil (für Cache-/Graph-Schlüssel).
      - **Lease-RAII:** Drop dekrementiert `PinCount`; Evict erst bei 0; Early-Return/Panic lässt keinen Pin hängen.
      - `Value`: kein impliziter Device→Host-Transfer; Transfer nur über explizite Op.
      - Affinität: neue `Affinity`-Variante bricht Fassaden-/Task-API nicht (`#[non_exhaustive]`-Testfall).
      - Op-Zähler: Pipeline aus n Ops → Task erst bei 0 `Done`; erster Fehler cancelt Rest.

      ## 5. Offene Punkte (→ A3 / C3)

      1. `Op` als geschlossenes Enum vs. `Arc<dyn Op>` (Dispatch-Kosten vs. Offenheit) — context_b empfiehlt datenhaftes Enum, `dyn` nur für Laufzeitwahl.
      2. Handle-/Lease-Repräsentation final an B2 `Region`+`gen` koppeln.
      3. Freigabe-Queue-Mechanik für async Lease-Drop (Channel-Typ) mit C2 abstimmen.

      ## 6. Definition of Done

      - `Op`-Trait, `OpKind`, `OpParams`, `Requirements`/`ResourceKind`/`EngineSelector`/`Affinity`, `Value`, `Lease`, `OperationNode` implementiert.
      - Kein `Tensor`/Pointer in `Value`/öffentlicher API; Lease-RAII getestet.
      - Erweiterungspunkt Affinität vorhanden, aber inaktiv; baut ohne `cuda`/`candle`.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/resident_plan_b4.md
    mode: replace
    content: |
      # Umsetzungsplan B4 · Index-Trait & -Vertrag

      | | |
      |---|---|
      | **Paket** | B4 (Gruppe B) — **eigenständig vom Kern**, parallel zu B1–B3 bearbeitbar |
      | **Abhängt von** | A4 |
      | **Blockiert** | C1 (Index-Übergabe), E1/E2/E5/E6 (Manager/Scope/Sync), G2 (konkretes Index-Schema) |
      | **Ergebnis** | Index-Trait-Spezifikation: `Index`-Trait, Entry/Stamp-Semantik, `Scope`, `AccessMethod`, Schreibreihenfolge, Level-Signal `watch<u64>` |
      | **Quellen** | `../resident_index.md` (Index-Vertrag, Scope, Synchronisation), `../context/context_b.md` §B4 (15–20) |

      ---

      ## 1. Zweck & Abgrenzung

      Der **Index-Vertrag** ist bewusst vom Kern entkoppelt: Jeder Index implementiert sein **eigenes Modell** (Dense, ColBERT, Datei/Gleitfenster, AST, Chunks, kaskadierend). Der Kern kennt diese Modelle nicht. Der Index muss **nur Änderungen melden** und Struktur/Stamps liefern — **keine** Geometrie (kein Abstand, Radius, Top-k, Präfix-Match) und **keine** Scope-Auflösung.

      Die einzige harte Bindung ist **Modell-ID und Dimension** im Index-Header. Persistenzformat und -zeitpunkte liegen **außerhalb** (beim Index-Objekt, siehe G2), nicht in B4.

      **Nicht Teil von B4:** `StreamedIndexManager`, Scope-**Resolver**, Arena, Residency, Kommunikationsslot (alles Gruppe E); konkretes Parquet-Schema (G2). B4 = reiner **Trait + Datentypen + Stamp-Mechanik**.

      ---

      ## 2. Der Index-Trait

      ```rust
      trait Index {
          fn stamp(&self, entry: EntryId) -> Option<Stamp>;            // None = entfernt
          fn stamps(&self) -> impl Iterator<Item = (EntryId, Stamp)>;  // OHNE Vektordaten zu lesen
          fn read(&self, entry: EntryId) -> EntryData<'_>;             // Referenz, keine Kopie
          fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp;       // Stamp DES EIGENEN Writes
          fn changed(&self) -> watch::Receiver<u64>;                   // Level-Signal
          fn capabilities(&self) -> &[AccessMethod];                   // Centroid, Prefix, Subtree, Range, …
          fn children(&self, node: NodeId) -> EntrySet;                // reine Struktur
          fn members(&self, cell: CellId) -> EntrySet;                 // Inverted List o. Ä.
          fn summary(&self, kind: AccessMethod) -> Option<SummaryRef>; // z. B. Zentroid-Tabelle
          fn prefetch(&self, set: &EntrySet);                          // optional, darf no-op sein
      }
      ```

      ### 2.1 Begleittypen

      ```rust
      pub struct EntryId(/* opak, u64 o. Ä. */);
      pub struct Stamp(u64);                 // Vergleich NUR mit != , NIE mit <
      #[non_exhaustive]
      pub enum AccessMethod { Centroid, Prefix, Subtree, Range /* erweiterbar */ }
      pub enum Scope { All, Range(/*..*/), Set(/*..*/), Subtree(NodeId), Prefix(/*..*/),
                       Centroid { k: u32, radius: Option<f32> } }
      pub struct EntrySet(/* kompakt: Ranges oder Bitmap, NICHT ID-Liste */);
      pub struct EntryData<'a>(/* Referenz auf Vektorbereich variabler Länge */);
      pub struct SummaryRef(/* z. B. Zentroid-Tabelle als Vektordaten */);
      ```
      - **Entry** = Vektorbereich **variabler Länge** (Dense = 1 Vektor, ColBERT/Datei/Fenster = viele). Entries dürfen überlappen / dieselben Vektoren referenzieren.
      - `EntrySet` ist **kompakt** (Ranges/Bitmap), nicht ID-Liste.

      ---

      ## 3. Stamp-Mechanik (Kern von B4)

      ### 3.1 Monotoner Zähler
      ```rust
      static NEXT: AtomicU64 = AtomicU64::new(1);
      fn next_stamp() -> Stamp { Stamp(NEXT.fetch_add(1, Ordering::Relaxed)) }
      ```
      - Eindeutigkeit genügt `Relaxed`. **Sichtbarkeit** der Daten zum Stamp über Release-Store/Lock beim Veröffentlichen (Stamp innerhalb der Schreib-Serialisierung vergeben).
      - Entfernen + Wiederanlegen derselben ID ⇒ **neuer** Stamp (ABA-Schutz). Vergleich ausschließlich `!=`.
      - Über Neustarts: High-Water-Mark im Index-Header persistieren, beim Laden `max(stored, file_max) + 1`.

      ### 3.2 Schreibreihenfolge (verbindlich)
      **erst Daten, dann Stamp, dann Signal.** Falsche Reihenfolge trägt einen neueren Stamp ein als die Daten → veraltete Daten gelten als aktuell (einzige unsichere Richtung).

      ### 3.3 Echo-Rückgabe
      `apply` gibt den Stamp **des eigenen Writes** zurück. Der Manager (E6) trägt genau diesen als bekannt ein (Gleichheitsvergleich) → Echo-Unterdrückung. **Der Rückgabewert, nicht ein späteres Lesen.**

      ### 3.4 Fallback-Stamp (dateigestützte mmap-Indizes)
      Stufen: **Stamp → size+mtime → Content-Hash.**
      - **Racy-Problem (git-Vorbild):** Liegt `mtime` innerhalb der Zeitauflösung **vor** dem letzten Scan (`mtime >= T_scan − Auflösung`), gilt der Entry als unsicher → Content-Hash-Prüfung. Stamp zusammen mit Referenzzeit `T_scan` merken.
      - Konservatives Racy-Fenster **2 s** (FAT-Worst-Case) oder FS-spezifisch. Fallback-Stamp-Tupel: `(size, mtime_ns, ctime/inode falls verfügbar)` → bei Racy/Abweichung Content-Hash.
      - **Hash:** `xxhash-rust` **xxh3_64** (schnell, nicht kryptografisch) als Default; `xxh3_128` bei Kollisionssorge; `blake3` nur bei Manipulationssicherheit. **Hash-Algorithmus + Version im Index-Header festhalten.**
        > `// TODO(A3): Racy-Fenster (FS-Auflösung) und xxh3_64 vs. 128 festlegen (context_b offener Punkt 5).`

      ---

      ## 4. Benachrichtigung — Level-Signal `watch<u64>`

      - `tokio::sync::watch<u64>`: behält **nur den letzten Wert** (Watermark = Zählerwert), jeder Receiver verfolgt separat. Zusammengefasste/verlorene Signale sind **harmlos**, weil der Abgleich **zustandsbasiert** ist (E6 vergleicht Stamps).
      - Konsumentenschleife mit `borrow_and_update()` (markiert als gesehen) — **nicht** `borrow()` (würde doppelt laufen):
      ```rust
      loop {
          let stamp = *rx.borrow_and_update();
          reconcile(stamp).await;
          if rx.changed().await.is_err() { break; } // alle Sender gedroppt
      }
      ```
      - `wait_for`/Zwischenwerte: gehen verloren — für Ereignisströme ungeeignet, für zustandsbasierten Abgleich akzeptabel.

      ---

      ## 5. Mehrschreiber-Serialisierung

      Der Index **serialisiert `apply`** und garantiert die Sequenz (Last-Writer-Wins = Dequeue-Reihenfolge). Zwei Engines dürfen denselben Entry gleichzeitig aktualisieren; wer zuerst kommt, wird ggf. überschrieben.

      **Default-Muster:** Single-Writer-Task + `mpsc` + `oneshot`-Antwort:
      ```rust
      // apply(cmd) -> oneshot::Receiver<Stamp>
      // Stamp unmittelbar VOR Veröffentlichung vergeben, danach watch::Sender::send(stamp)
      ```
      Alternative: kurzer sync-`Mutex` um `apply` (nie über `.await` halten; sonst `tokio::sync::Mutex`).

      ---

      ## 6. Stamp-Spalte ohne Vektorseiten (Scan-Billigkeit)

      `stamps()` **darf keine Vektorseiten einpagen.** Umsetzung für Parquet/Arrow-gestützte Indizes:
      - `ArrowReaderBuilder::with_projection(ProjectionMask::leaves(…, [stamp_idx]))` liest nur die Stamp-Spalte; Vektorspalte `FixedSizeList<f16,384>` bleibt ungelesen (Spaltenchunks pro Row-Group getrennt).
      - **Robusteste Variante:** Stamp-Spalte als **separate Sidecar-Datei** (kleines Parquet/Arrow-IPC) → garantiert null Vektor-IO, entkoppelt Rewrites.
      - mmap: Reader braucht `ChunkReader`; `memmap2`-Mapping als `bytes::Bytes` einhängen, nur Metadaten + Stamp-Chunk anfassen.
        > `// TODO(A3): parquet Float16/FixedSizeList-Support + Leaf-Index-Ermittlung über Spaltenpfad am Testfall prüfen (context_b offener Punkt 2).`

      *(Das konkrete Schema ist G2; B4 definiert nur die Vertragsanforderung „Stamp ohne Vektor-IO“.)*

      ---

      ## 7. Lazy / blockierende Reads

      `read` darf auf Page-Faults und Platten-I/O **blockieren**. Der Manager (E) ruft blockierende Reads im Blocking-/Worker-Pool auf, nie im async-Runtime-Thread, mit Prefetch-Hinweis (`madvise(WILLNEED)`). B4 dokumentiert dies im Trait-Kontrakt.

      ---

      ## 8. Fehler & Logging

      - Q1: `IndexUnavailable` (Index nicht nutzbar), `StaleMapping` (intern), ABA-/Reihenfolge-Verletzungen als `debug_assert!`.
      - Q2: `tracing` — `apply` (Entry, zurückgegebener Stamp), Signal-Sende-Watermark, Fallback-Hash-Trigger (racy).

      ## 9. Teststrategie (Q3, ohne GPU)

      - **Stamp:** monoton/eindeutig; Entfernen+Neuanlage derselben ID ⇒ anderer Stamp (ABA); Vergleich nur `!=`.
      - **Schreibreihenfolge:** Daten→Stamp→Signal; Test der unsicheren Richtung (Stamp vor Daten) als Negativfall dokumentiert.
      - **Echo:** `apply`-Rückgabe-Stamp = bekannter Stamp (kein Reload-Trigger); fremder Stamp ⇒ Reload (Manager-Testanker für E6).
      - **watch:** verlorenes/zusammengefasstes Signal → nächster Abgleich findet alle Änderungen; `borrow_and_update` läuft nicht doppelt.
      - **Fallback:** mtime innerhalb Racy-Fenster ⇒ Hash-Pfad; size/mtime-Abweichung ⇒ Hash; stabiler Entry ⇒ Stat-Vergleich genügt.
      - **stamps() ohne Vektor-IO:** mit Fake-Backend verifizieren, dass keine Vektorspalte gelesen wird (Zähl-Mock / Sidecar).
      - **Mehrschreiber:** Single-Writer-Task serialisiert; Last-Writer-Wins = Dequeue-Reihenfolge; Stamps streng aufsteigend vergeben.

      ## 10. Offene Punkte (→ A3 / E / G2)

      1. Racy-Fenster & Hash-Variante (xxh3_64/128) final.
      2. parquet Float16/FixedSizeList + Leaf-Index am Testfall.
      3. Stamp-Spalte inline (Projection) vs. Sidecar-Datei.
      4. Mehrschreiber: Single-Writer-Task vs. kurzer Mutex (A3).

      ## 11. Definition of Done

      - `Index`-Trait + alle Begleittypen (`EntryId`, `Stamp`, `Scope`, `AccessMethod`, `EntrySet`, `EntryData`, `SummaryRef`, `NodeId`, `CellId`) definiert.
      - Stamp-Mechanik (monoton, ABA, Echo-Rückgabe, Fallback mit Racy+Hash, High-Water-Mark) implementiert + getestet.
      - `watch<u64>`-Level-Signal-Kontrakt + `borrow_and_update`-Muster dokumentiert.
      - Referenz-In-Memory-Index (Testdouble) implementiert; eigenständig vom Kern; baut ohne `cuda`/`candle`.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/resident_plan_b5.md
    mode: replace
    content: |
      # Umsetzungsplan B5 · Kernel-Abstraktion

      | | |
      |---|---|
      | **Paket** | B5 (Gruppe B) |
      | **Abhängt von** | B2 (`Region`/Spans) |
      | **Blockiert** | D6 (Kernels & Scorer), E4 (Manager-Disziplin), E5 (Kommunikationsslot), E8 (Kernel schreibt Segmente) |
      | **Ergebnis** | Kernel-Trait + Mock-Vertrag: `SegmentDesc`, `SegmentLayout`, `KernelHit`, Kernel-Trait (Batch-Start/-Ende, Ergebnisse, geschriebene Bereiche, Fortschritt) + Mock |
      | **Quellen** | `../resident_index.md` (Kernel-Schnittstelle, Rückabbildung), `../resident_subengine.md` §4.5, `../context/context_b.md` §B5 (21–23) |

      ---

      ## 1. Zweck & Abgrenzung

      Der **Kernel** kennt nur **Segmente, Adressen und Offsets** — keine Entry-Identität, keine Traversierung, keine Kommunikation während der Berechnung. Er liefert `(Adresse/Offset, Score)` bzw. geschriebene Bereiche. Die **Rückabbildung Adresse → Entry** macht der **Manager** (E), nicht der Kernel.

      B5 liefert eine **adress-/offset-agnostische** Trait-Abstraktion plus einen **handgeschriebenen Mock** für Tests ohne GPU. Reale CUDA-Implementierung (cudarc/NVRTC) ist D6/E hinter `feature = "cuda"`.

      **Namensklärung:** B5 benutzt `SegmentLayout` (nicht `Layout`, das ist B2) für `Fixed{stride}|Variable{offsets}`.

      ---

      ## 2. Zu implementierende Typen

      ```rust
      pub enum SegmentLayout {
          Fixed    { stride: u32 },        // Entry-Grenzen per Arithmetik
          Variable { offsets: OffsetRef }, // Offset-Tabelle (Prefix-Summen), vom Manager gebaut
      }

      pub struct SegmentDesc { pub base: u64 /* Arena-Offset */, pub count: u32, pub layout: SegmentLayout }
      pub struct KernelHit   { pub offset: u64 /* Arena-Offset */, pub score: f32 }
      pub struct WrittenRange { pub offset: u64, pub len: u64 }
      pub struct RegionSpan   { pub offset: u64, pub len: u64 } // adress-agnostisch; Pointer-Auflösung NUR im Backend
      ```
      - `OffsetRef` ist im Logik-Build ein Host-Slice/Region-Handle; im `cuda`-Build ein Device-Pointer-Äquivalent. B5 bleibt agnostisch (Trait nimmt **Spans**, keine Pointer).
      - Eine **Kandidatenmenge** ist ein `Variable`-Layout; der Kernel rechnet darüber, egal ob die Tabelle auf kompakte Kopien oder bestehende Segmente zeigt.

      ## 3. Kernel-Trait (real + Mock)

      ```rust
      pub trait Kernel: Send + Sync {
          fn begin(&self, batch: &BatchDesc) -> Result<BatchId, Error>;
          fn run(&self, batch: BatchId, input: &[RegionSpan]) -> Result<KernelOutput, Error>;
          fn progress(&self, batch: BatchId) -> u64;                   // monoton; Atomic/Event
          fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, Error>; // geschriebene Bereiche
      }

      pub struct BatchDesc { /* Segmente (SegmentDesc), Query-Batch, ParamBlock-Verweis, Zeitbudget */ }
      pub struct BatchId(u64);
      pub enum KernelOutput { Hits(Vec<KernelHit>), Partial /* Bereich erschöpft, Rest folgt */ }
      ```
      - **Zwei Kernel-Rollen** (beide über denselben Trait):
        - **Scoring (MaxSim):** `run` liefert `Hits`. MaxSim aggregiert pro Entry über `stride`/Offset-Tabelle, **ohne** Entry-Identität.
        - **Indexaufbau (Embedding):** Kernel schreibt per **Cursor** in einen vorab vergebenen Bereich und **allokiert nie**; bei Bereichsmangel `Partial` → Manager vergibt nächsten Bereich. `end` meldet belegte Bereiche.
      - **Fortschritt:** `progress` monoton (Atomic im Logik-Mock; im Ringbuffer-Modus Zähler in gemapptem Host-Speicher — H1/E).
      - **Keine Kommunikation während der Berechnung**; Mutation nur im Kommunikationsslot (E5).

      ## 4. Rückabbildung-Helfer (Zuständigkeit: Manager)

      B5 stellt die reine Funktion bereit, die E8 nutzt (Kernel selbst traversiert nie):
      ```rust
      // Fixed: addr → index per Division (addr - base) / stride
      // Variable: offsets.len() == n+1; Entry i belegt [offsets[i], offsets[i+1])
      fn entry_of(offsets: &[u64], addr: u64) -> Option<usize> {
          let i = offsets.partition_point(|&o| o <= addr); // erstes o > addr
          (i > 0 && i < offsets.len()).then(|| i - 1)
      }
      ```
      - Aufbau per exklusiver Präfixsumme (`scan`) in O(n); Suche O(log n) via `slice::partition_point`. Dynamisch alternativ `BTreeMap<u64, EntryId>` (`range(..=addr).next_back()`).

      ## 5. cudarc-Bezug (nur Doku für D6, hinter `feature = "cuda"`)

      - cudarc 0.19.x: `CudaContext::new(ordinal)` → `default_stream()`/`new_stream()`; `nvrtc::compile_ptx` → `load_module` → `load_function`; `stream.launch_builder(&func).arg(..).launch(cfg)` (`unsafe`). Daten `CudaSlice`/`CudaView(Mut)`; `clone_htod`/`memcpy_htod`/`clone_dtoh`; Events für Fortschritt; `LaunchConfig{grid_dim,block_dim,shared_mem_bytes}`.
      - **Kritisch:** candle nutzt dieselbe cudarc-Basis → **Stream-/Kontext-Gleichheit** beachten (A3-Prototyp). Kernel-Args müssen `DeviceRepr` sein; Slice-Lebensdauer bis `synchronize`; Kernel-Fehler erscheinen **asynchron** beim späteren Sync.
      - `sm_75` als `--gpu-architecture` beim NVRTC-Compile.

      ## 6. Mock-Kernel (Pflicht, CI ohne GPU)

      Handgeschriebenes Fake statt Mock-Framework:
      - Deterministische Ausgabe als **reine Funktion der Eingabe** (z. B. xxh3 der Bytes → Score).
      - Geschriebene Bereiche aus der Eingabe berechnet; `Partial` simulierbar (Bereich künstlich knapp).
      - Fortschritt als `AtomicU64`; Fehlerinjektion `fail_after(n)`; Aufruf-Log `Mutex<Vec<Call>>` für Assertions.
      - Trait-Generics (`fn run<K: Kernel>`) vermeiden Dispatch-Kosten; `Arc<dyn Kernel>` für Laufzeitwahl. `mockall` nur für reine Interaktions-Tests.

      ## 7. Fehler & Logging

      - Q1: `TransferFailed` (H2D/D2H), `ArenaExhausted` (Bereichsmangel → aber Default ist `Partial`, nicht Fehler), Kernel-/Launch-Fehler.
      - Q2: `tracing`-Span pro Batch (`BatchId`): begin/run/end, Hits-Zahl, WrittenRanges, Partial-Ereignisse.

      ## 8. Teststrategie (Q3, ohne GPU)

      - **entry_of:** Fixed (Division) und Variable (partition_point) inkl. Rändern (addr == base, letzter Offset, out-of-range → None).
      - **Mock-Determinismus:** gleiche Eingabe → gleiche Hits/Ranges.
      - **Partial-Pfad:** knapper Bereich → `Partial`; Manager vergibt Folgebereich; Summe der WrittenRanges deckt Eingabe.
      - **Fortschritt:** `progress` monoton steigend über `run`.
      - **Fehlerinjektion:** `fail_after(n)` liefert `Err`; Batch sauber beendbar.
      - **Adress-Agnostik:** Trait-API nennt keine Pointer; Spans genügen (Compile-Check).

      ## 9. Offene Punkte (→ A3 / D6 / E)

      1. `OffsetRef` im `cuda`-Build (Device-Pointer) vs. Logik-Build (Host-Slice) — Abstraktion final.
      2. MaxSim: Kachel-Kette (Candle-Ops) vs. fused NVRTC-Kernel messen (D6/A3-Prototyp).
      3. cudarc-Version == candle-interne cudarc-Version (A3, sonst inkompatible Typen).
      4. Rückabbildung dynamisch: `partition_point` (statisch) vs. `BTreeMap`/Intervall-Map (bei laufenden Änderungen).

      ## 10. Definition of Done

      - `SegmentDesc`, `SegmentLayout`, `KernelHit`, `WrittenRange`, `RegionSpan`, `Kernel`-Trait, `entry_of`-Helfer implementiert.
      - `MockKernel` mit Determinismus, Partial, Fortschritt, Fehlerinjektion, Aufruf-Log.
      - Trait ist adress-/offset-agnostisch (keine Pointer in öffentlicher API); Rückabbildung als Manager-Aufgabe dokumentiert.
      - Testmatrix §8 grün; baut ohne `cuda`/`candle`.
  - path: /home/user/xyan/xy.ai.workbench/project/resident/plan/resident_plan_b6.md
    mode: replace
    content: |
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
  reason: "Creating the Stufe-2 implementation plans for Group B: a group overview plus one plan per package (B1–B6), synthesizing the planplan package definitions, the Group B research context, and the referenced source specs, each with concrete Rust types/traits, resolved decisions, open points, error/logging conventions, tests, and a Definition of Done."
```
/answer 0f9c-f allow
Result Stats: id=657c2698-a929-41ee-beaa-769df39f0712, total: 219458, in: 125, out: 42512, reason: 8605, read: 111051, write: 176821
ReasoningToken: 8586
ReasoningToken: 19
Fertig. Ich habe die Umsetzungspläne für Gruppe B (Stufe 2) in `/home/user/xyan/xy.ai.workbench/project/resident/plan/` erstellt:

| Datei | Inhalt |
|---|---|
| `resident_plan_b.md` | **Gruppenübersicht**: Reihenfolge B1→B6, Abhängigkeits- und Typ-Eigentümer-Tabellen, Modulzuordnung (`resident-core`), gemeinsame Q1–Q4-Konventionen, Gruppen-DoD |
| `resident_plan_b1.md` | **Queue & Task-Primitive**: lock-freie Liste (`arc-swap`), `TaskState`-Zustandsautomat, Node/Queue mit Sentinel, Unlink-Entscheidung, iterativer Drop, Notify-Topologie, Wiederverwendung für Op-Queues |
| `resident_plan_b2.md` | **Region/Layout/Speicher**: `Region{arena,offset,len,gen}`, eigenes `DType` (`#[non_exhaustive]` + candle/safetensors-`TryFrom`), `Layout`, `MemoryManager`-Trait, backend-agnostische Free-List, `compute_dtype` (GPU f16/CPU f32) |
| `resident_plan_b3.md` | **Op-Modell**: `Op`-Trait (Op als Daten), `Requirements`/`ResourceKind`, `Value::Host\|Device`, RAII-`Lease`, `materialize`, `Affinity` als Erweiterungspunkt, Task≠Op |
| `resident_plan_b4.md` | **Index-Trait**: vollständiger Trait, Stamp-Mechanik (monoton/ABA/Echo-Rückgabe/Racy-Fallback+xxh3), Schreibreihenfolge, `watch<u64>`, Mehrschreiber-Serialisierung, Stamp-Spalte ohne Vektor-IO |
| `resident_plan_b5.md` | **Kernel-Abstraktion**: `SegmentDesc`/`SegmentLayout`/`KernelHit`, Kernel-Trait (begin/run/progress/end), `entry_of`-Rückabbildung, cudarc-Bezug, handgeschriebener Mock |
| `resident_plan_b6.md` | **CapabilitySet & Profile**: `AxisId`/`ValueId`, `CapabilitySet::satisfies`, unbenannte `Profile{position,config}`, Pareto-/Bogenlängen-Helfer, sm75-Achsen (ohne bf16-Rechenpfad) |

**Vorgehen / Prinzipien jedes Plans:** Jeder Paketplan führt Metadaten (Abhängt von / Blockiert / Ergebnis / Quellen), konkrete Rust-Typen/Traits, die aus `context_b.md` bereits **aufgelösten Entscheidungen**, offene `A3`-Punkte, Fehler (Q1) + Logging (Q2) + Teststrategie (Q3, ohne GPU) + Feature-Flags (Q4) und eine Definition of Done.

Beachtet: `resident_index.md` wurde ausschließlich als Index-**Vertrag/Vektorindex**-Spezifikation (B4/B5) verwendet, nicht als Verzeichnisindex. Die Namenskollision „Layout“ wurde durch `SegmentLayout` (B5) aufgelöst, und die Metaplan-Hinweise (Stamp nur `!=`, `DType` `#[non_exhaustive]`) sind eingearbeitet.

Hinweis: Das Verzeichnis-Probing wurde mit „matching is not required“ abgelehnt — ich habe deshalb eine eigene, konsistente Namens-/Dateistruktur (`resident_plan_b*.md`) gewählt, statt ein vorhandenes Format zu rekonstruieren.
