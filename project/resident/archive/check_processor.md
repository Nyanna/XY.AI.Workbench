# Aufgabe: Design-Dokument mit Implementierung abgleichen

Gleiche das Design-Dokument `/home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md` mit der Implementierung in `/home/user/xyan/xy.ai.workbench/rag/src/resident/processor` ab. Liefere zwei Ergebnisse:
1. Die README (`/home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md`) der Implementierung, überarbeitet.
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
SystemInit:  id=7d032b21-33cc-48aa-bf14-f986b43cbdbb, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: b503-27
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident/resident_processor.md
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
  reason: Read design and existing README
```
/answer b503-27 allow
Control Request:
```yaml
id: b503-28
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
  reason: List implementation files
```
/answer b503-28 allow
Control Request:
```yaml
id: b503-29
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor
      entries:
      - ".:"
      - README.md
      - completion.rs
      - completion_cuda.rs
      - config.rs
      - error.rs
      - hub.rs
      - mod.rs
      - ops.rs
      - resource.rs
      - starved.rs
      - tests.rs
      - worker.rs
      - "./test:"
      - completion.rs
      - completion_cuda.rs
      - error.rs
      - ops.rs
      - resource.rs
      - starved.rs
```
/answer b503-29 allow
Control Request:
```yaml
id: b503-2a
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/mod.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/hub.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/worker.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/ops.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/starved.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/resource.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/completion.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/completion_cuda.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/config.rs
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/error.rs
  reason: Read all implementation code
```
/answer b503-2a allow
ReasoningToken: 2999
Control Request:
```yaml
id: b503-2b
toolName: search
arguments:
  items:
  - pattern: starved\(|OrchestrationError::(Cancelled|Backoff|CompletionLost|Shutdown)|Semaphore|get_or_try_init|candle_rayon_threads|push_shared
    directory:
    - /home/user/xyan/xy.ai.workbench/rag/src
    include:
    - '*.rs'
    exclude:
    - target/**
    limit: 40
  reason: "Check whether unused/outside-package usages exist for hints, errors, semaphore, rayon config"
```
/answer b503-2b allow
Control Request:
```yaml
id: b503-2c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src
      matches:
      - path: resident/processor/worker.rs
        matches:
        - "Step::Idle(Some(kind)) => hub.starved(side).mark(kind),"
      - path: resident/processor/tests.rs
        matches:
        - assert!(hub.starved(Side::Cpu).snapshot().is_empty());
        - hub.starved(Side::Cpu).mark(ResourceKind::Tokenizer);
        - hub.starved(Side::Cpu).mark(ResourceKind::Encoder);
        - let snap = hub.starved(Side::Cpu).snapshot();
      - path: resident/processor/config.rs
        matches:
        - "pub candle_rayon_threads: Option<usize>,"
        - "candle_rayon_threads: None,"
        - "pub fn with_candle_rayon_threads(mut self, n: usize) -> Self {"
        - self.candle_rayon_threads = Some(n);
      - path: resident/processor/hub.rs
        matches:
        - "pub fn starved(&self, side: Side) -> &StarvedSet {"
        - self.starved(side).clear(job.params().requires.resource);
        - let hint = self.starved(side).snapshot();
        - self.starved(side).clear_all(hint);
      - path: resident/processor/test/error.rs
        matches:
        - "assert!(OrchestrationError::Backoff { resource: ResourceKind::Encoder }.is_retryable());"
        - "assert!(!OrchestrationError::Cancelled { task: 1 }.is_retryable());"
        - "let e: ResidentError = OrchestrationError::Shutdown.into();"
        - "assert!(matches!(e, ResidentError::Orchestration(OrchestrationError::Shutdown)));"
      - path: resident/processor/test/ops.rs
        matches:
        - "q.push_shared(job(&run, ResourceKind::Scorer, EngineSelector::Either));"
        - "q.push_shared(job(&run, ResourceKind::Encoder, EngineSelector::Gpu));"
        - "q.push_shared(job(&run, ResourceKind::Tokenizer, EngineSelector::Cpu));"
      - path: resident/processor/ops.rs
        matches:
        - "pub fn push_shared(&self, job: OpJob) {"
      - path: resident/subengine/lifecycle.rs
        matches:
        - "use tokio::sync::{OnceCell, Semaphore};"
        - pub struct InitLimiter(Arc<Semaphore>);
        - Self(Arc::new(Semaphore::new(max_parallel.max(1))))
        - "async fn acquire(&self) -> Result<tokio::sync::SemaphorePermit<'_>, SubengineError> {"
        - ".get_or_try_init(|| async move {"
      - path: resident/manager/streamed.rs
        matches:
        - ".get_or_try_init(|| async {"
```
/answer b503-2c allow
ReasoningToken: 232
Control Request:
```yaml
id: b503-2d
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    id: ""
    old_text: "- **One worker pool, two sides.** Workers belong to the CPU or the GPU side."
    new_text: "- **One worker pool, two sides.** Workers belong to the CPU or the GPU side; their counts are configured per side, parked on one wake-up per side and woken by every submit and every operation enqueue (GPU workers also by a lane signal). An operation that blocks a worker is the natural backpressure signal and gets no special handling."
    replaceAll: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    id: ""
    old_text: Expansion runs only for a hint or when the operation buffer is below its minimum. No fixed fill threshold.
    new_text: "Hints are a set per side (CPU, GPU), not a queue; a pick of the kind or an expansion that consumed the hint clears them, otherwise they expire after a TTL. Expansion runs only for a hint or when the queued operations are below the configured minimum, and prefers a task whose first operation needs the hinted resource. No fixed fill threshold."
    replaceAll: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    id: ""
    old_text: A deadline fails overdue tasks.
    new_text: A deadline fails overdue tasks with a timeout; it is checked when one of the task's operations is claimed or enqueues follow-ups. A device handle at claim is only checked by a debug assertion (region check set on the hub).
    replaceAll: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    id: ""
    old_text: "Completion arrives by a host callback (push, no CUDA calls inside) or by polling an event; each lane is taken over exactly once."
    new_text: "Completion arrives by a host callback (push: sets a flag and wakes, no CUDA calls inside) or by polling an event (`cuEventQuery`) in the take-over step, which covers callbacks that never fire; callback and poll race on a CAS, so each lane is taken over exactly once."
    replaceAll: false
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    id: ""
    old_text: "A synchronous CAS lifecycle cell (unloaded → loading → ready → evicting) with in-flight count, pins and TTL eviction."
    new_text: "A synchronous CAS lifecycle cell (uninit → loading → ready → evicting, failed loads go to backoff and may retry after a delay) with in-flight count, pins and TTL eviction; eviction only at no operation in flight, no pins and an expired TTL."
    replaceAll: false
  reason: Integrate design/implementation facts into existing README structure
```
/answer b503-2d allow
Control Request:
```yaml
id: b503-2e
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
      error: No node matched the selector.
      id: ""
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
      error: No node matched the selector.
      id: ""
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
      error: No node matched the selector.
      id: ""
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
      error: No node matched the selector.
      id: ""
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
      error: No node matched the selector.
      id: ""
```
/answer b503-2e allow
ReasoningToken: 33
Control Request:
```yaml
id: b503-2f
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
  reason: Get node ids for editing
```
/answer b503-2f allow
Control Request:
```yaml
id: b503-30
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
      nodes:
      - id: 8eab5a
        signature: processor
        children:
        - id: 8eab5a.YIalZm|ifIk3L
          signature: "Orchestration: owns the worke…"
        - id: 8eab5a.c05baf
          signature: Concept
        - id: 8eab5a.6ce6c5
          signature: Files
          children:
          - id: 8eab5a.6ce6c5.QQdRaj|abi4Ho
            signature: "Tests: `test/` per file; `tes…"
```
/answer b503-30 allow
ReasoningToken: 29
Control Request:
```yaml
id: b503-31
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    id: 8eab5a.c05baf
    old_text: Workers belong to the CPU or the GPU side.
    new_text: "Workers belong to the CPU or the GPU side; their counts are configured per side. They park on one wake-up per side, woken by submits, operation enqueues and GPU lane signals. A blocking operation is the natural backpressure signal and gets no special handling."
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    id: 8eab5a.c05baf
    old_text: Expansion runs only for a hint or when the operation buffer is below its minimum. No fixed fill threshold.
    new_text: "Hints are a set per side, not a queue; a pick of the kind or an expansion that used the hint clears them, otherwise they expire after a TTL. Expansion runs only for a hint or when the queued operations are below the configured minimum, and prefers a task whose first operation needs the hinted resource. No fixed fill threshold."
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    id: 8eab5a.c05baf
    old_text: A deadline fails overdue tasks.
    new_text: A deadline fails overdue tasks with a timeout; it is checked when one of the task's operations is claimed or enqueues follow-ups. A device handle at claim is checked only by a debug assertion (region check set on the hub).
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    id: 8eab5a.c05baf
    old_text: or by polling an event; each lane is taken over exactly once.
    new_text: "or by polling an event in the take-over step, which covers callbacks that never fire; both race on a CAS, so each lane is taken over exactly once."
  - path: /home/user/xyan/xy.ai.workbench/rag/src/resident/processor/README.md
    id: 8eab5a.c05baf
    old_text: "(unloaded → loading → ready → evicting) with in-flight count, pins and TTL eviction."
    new_text: "(uninit → loading → ready → evicting; a failed load goes to backoff and may retry after a delay) with in-flight count, pins and TTL eviction at no operation in flight, no pins and an expired TTL."
  reason: Integrate design/implementation facts into the Concept section
```
/answer b503-31 allow
Result Stats: id=7d032b21-33cc-48aa-bf14-f986b43cbdbb, total: 41303, in: 14, out: 8137, reason: 0, read: 160248, write: 33152, start: 1793188321130, end: 1793188334564, time: 13356, total: 66104, io: 0, control: 78, sent: 0, recv: 360
Ich habe die README überarbeitet. Die Abweichungen stehen unten, weil du keinen Zielpfad für eine Abweichungsdatei genannt hast. Ich habe nichts davon ausgeführt oder getestet. Die Tests habe ich nicht gelesen, denn alles Nötige ging aus dem Code hervor.

## 1. README
Die Änderungen stehen in `rag/src/resident/processor/README.md`, im Abschnitt „Concept“ und ohne neue Abschnitte. Sie betreffen diese Punkte:
- Wake-ups pro Seite und Rückdruck durch blockierende Operationen.
- Starved-Hints als Set pro Seite mit Verfall und Bevorzugung beim Expandieren.
- Task-Deadline und Region-Check beim Claim (nur Debug-Assertion).
- Completion per Callback oder Event-Poll mit CAS.
- Lebenszyklus der Ressource mit Backoff und Eviction-Bedingungen.

Tokenizer, Lease/Handle, Batching, Lazy Init per Semaphore und Queue-Interna gehören in andere Pakete (`subengine`, `op`/`memory`, `queue`). Ich habe sie deshalb nicht aufgenommen.

## 2. Abweichungen (Englisch, für agentische Verarbeitung)

Ich habe sie nicht in eine Datei geschrieben. Sag mir den gewünschten Pfad, dann lege ich sie dort ab.

```
# Deviations: resident_processor.md vs rag/src/resident/processor

1. Worker pool split
   - Design: one pool sized to CPU cores for the whole engine; any free worker processes GPU completions.
   - Code: separate `cpu_workers` / `gpu_workers` (WorkerConfig); each worker is bound to one side/engine for life. Default is 1/0. No derivation from core count.

2. Starved hint lacks params
   - Design: `Starved(resource_kind, params)`.
   - Code: only a per-kind bit (StarvedSet/StarvedSnapshot); no params are stored or passed to the expander.

3. Hint expiry rules
   - Design: hints expire on pick or occupied resource.
   - Code: cleared on pick (Hub::claim) and after any expansion that used the hint (clear_all). No clearing on "resource occupied" within this package.

4. Expansion trigger
   - Design: hint or empty operation queue.
   - Code: hint or total queued ops (CPU+GPU+shared, not per side) < `min_op_buffer` (default 2).

5. Task timeout coverage
   - Design: timeout aborts blocked tasks and releases leases.
   - Code: the deadline is only checked on claim (Counted::claim) and in complete_op with follow-ups. It is not applied to tasks still in the task queue, to a running (blocked) operation, or to operations stuck unclaimed (e.g. resource Loading). Lease release is not performed in this package.

6. Resource states
   - Design: Uninit|Loading|Ready|Backoff, internally Unloaded→Loading→Resident→Leased(n)→Idle→Evicting, plus Failed.
   - Code (ResourceSlot): Uninit, Loading, Ready, Backoff, Evicting only. Leases are a pin counter and in_flight is a counter, not states. No `Failed` state.

7. GPU init failure
   - Design: mark GPU as unavailable (retryable).
   - Code: GPU availability is fixed at start (`gpu_workers > 0` and a gpu engine given). `EngineUnavailable` is returned at enqueue; there is no runtime mark and no retry.

8. First-error result
   - Design: task gets `Cancelled` and `Err`.
   - Code: TaskRun::fail completes the task with `TaskError::Engine(err)`; remaining ops are skipped at claim. Handle release is not done in this package.

9. Eviction TTL ("VRAM much longer than RAM")
   - Code: ResourceSlot::try_evict takes ttl as a parameter; no distinction in the processor package.

10. Span per task-id and resource
    - Design: span per task ID and resource.
    - Code: the task span exists only around expansion (telemetry::task_span) and a worker span per thread. No resource span in this package.

11. Unused config
    - `WorkerConfig.candle_rayon_threads` is stored but not used in this package.

12. Unused error variants
    - OrchestrationError::{Cancelled, Backoff, CompletionLost, Shutdown} are not raised anywhere in this package.
```

Die Abweichungen 9, 11 und 12 sind unsicher: Die Werte oder Varianten könnten außerhalb des Pakets genutzt werden. Mein Suchlauf hat dort nichts gefunden. Soll ich 9, 11 und 12 trotzdem in der Datei lassen?
