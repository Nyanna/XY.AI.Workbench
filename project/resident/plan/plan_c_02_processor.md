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
