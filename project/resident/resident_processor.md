# Resident – Processor (Orchestrierung)

Soft-Prompt-Notizen. Besitzt Queues, Worker-Parkplatz, Notify und Backpressure. Ruft den Planner für die Expansion.

## Struktur
- Processor hält: Task-Queue, drei Operations-Queues (CPU, GPU, shared), Worker-Parkplatz, Starved-Hints.
- Shared Queue ist angelegt, aber leer und ohne Logik. Subengine prüft erst die eigene Queue, dann shared.
- Subengines besitzen ihre Ressourcen (Tokenizer, Encoder, Scorer, Index-Segmente) und claimen Operationen per `claim`.
- Tokenizer ist Ressource der CPU-Subengine, genau eine Instanz.

## Queues
- Flache einfach verkettete Liste, nur Append und Unlink per CAS, Reihenfolge = Alter, kein Lock.
- Node: `params` (unveränderlich), `state: Arc<TaskState>`, `payload: Mutex<Option<Data>>`, `next: ArcSwapOption<Node>`.
- `claim` = CAS `Queued → Taken`, Fertigstellung `Taken → Done`, `cancel` nur auf `Queued` wirksam.
- Priorität, Ressource, Modell sind Metadaten, die Queue kennt sie nicht.
- Drop iterativ, kein `unsafe` außerhalb von `arc-swap`.

## Worker
- Anzahl = CPU-Kerne für die gesamte Engine, eine Obergrenze für Kontextwechsel, keine Kernzuordnung.
- Worker parken im Processor und wachen per Notify bei jeder Zustandsänderung auf.
- CPU-Engine: Worker führt die Operation synchron aus und kommt mit dem Ergebnis zurück.
- GPU-Engine: Worker startet Operation oder Kernel und kehrt zum Processor zurück. Nach dem Notify der Completion kommt ein freier Worker und verarbeitet das Ergebnis.
- Blockiert ein Worker (z. B. in Candle/rayon), ist das das natürliche Rückdrucksignal, keine Sonderbehandlung.

## Worker-Loop (Priorität, nach jeder Operation neu prüfen)
1. GPU-Ergebnisse verarbeiten (Index zurückspiegeln).
2. GPU-Ressourcen füttern (Index in VRAM synchronisieren, Launch bei freier Lane).
3. CPU-Ressourcen füttern, claimen, ausführen, Result zurückgeben (auch Indizes).
4. Neue Tasks expandieren (Planner).

- GPU-Vorrang ist selbstlimitierend: Schritt 2 greift nur bei freier Lane, Schritt 4 nur bei Hint oder leerer Queue.

## GPU-Completion
- Callback am Stream-Ende (`cuLaunchHostFunc`) tut nur: Completion-Eintrag ablegen und Notify auslösen. Keine CUDA-API im Callback.
- Schritt 1 des Workers prüft zusätzlich per `cuEventQuery`, übernimmt per CAS eine fertige Lane. Deckt verpasste Callbacks ab.

## Datenübergabe
- Operation-Output ist `Host(data)` oder `Device(handle)`, `materialize` entscheidet (vom Planner gesetzt).
- Handle = `Region` mit `gen`, gehört dem Task-Kontext per Lease.
- Offenes Handle = Lease auf Arena und Ressource, Use-Zähler > 0, Ressource wird nicht evicted.
- Gen-Prüfung beim Claim nur als Debug-Assertion, kein eigener Pfad.
- Lease endet mit der letzten konsumierenden Operation oder im Cancel-Pfad. Task-Timeout bricht blockierte Tasks ab und gibt Leases frei.

## Backpressure und Expansion
- Subengine-Iteration ohne Pick bei freier Ressource erzeugt `Starved(resource_kind, params)`.
- Hints liegen als Set pro Engine-Typ (keine Queue), verfallen bei Pick oder belegter Ressource.
- `expand()` läuft nur bei Hint oder leerer Operations-Queue, bevorzugt Tasks, deren erste Operation die Ressource braucht.
- Mindestpuffer an Operationen als Parameter, damit die Pipeline nicht trocken läuft.
- Keine feste Füllschwelle.

## Lazy Init, Batching, Eviction
- Lazy Init nur durch die Iteration, nie durch das Einreihen. `get_or_try_init`, Limit paralleler Inits per Semaphore.
- Ressource `Loading`: Operation bleibt in der Queue, Iteration überspringt.
- Zustände: `Uninit | Loading | Ready | Backoff`, ressourcenintern `Unloaded → Loading → Resident → Leased(n) → Idle → Evicting`, plus `Failed`.
- Batching ist Aufgabe der Subengine: ab dem Seed alle kompatiblen Operationen derselben Ressource claimen, Packing/Bucketing nach Token-Budget.
- Eviction bei `in_flight == 0`, offenen Leases 0 und abgelaufener TTL, VRAM deutlich länger als RAM.
- Init-/Launch-Fehler: `Err` in die Task-Results des Batchs, Ressource in `Backoff`. GPU-Initfehler: GPU als nicht verfügbar markieren, wiederholbar.

## Abschluss und Fehler
- Task-Zähler offener Operationen, bei 0 `Done`, Ergebnis über Oneshot plus Notify/Event.
- Erster Fehler: `Cancelled` und `Err`, Handles freigeben.
- Logging mit `tracing`, Span pro Task-ID und Ressource, Fehler mit `thiserror`.

## Nicht umgesetzt / Erweiterungen
- Affinität einer Folgeoperation an ein Handle (Erweiterungspunkt in `requires()`).
- GPU-Ringbuffer-Modus, gleitende Indizierung, Graph-Capture.
- Candle-CPU begrenzen (rayon) bleibt Parameter, keine Sonderlogik.

## Prüfen
- [ ] Callback und `cuEventQuery` zusammen: keine doppelte Verarbeitung einer Lane.
- [ ] Worker-Zahl vs. launch-bound GPU-Worker unter Last messen.
- [ ] Starved-Hint-Verfall unter wechselnder Last testen.