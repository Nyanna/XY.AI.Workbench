# Spezifikation: Work-Chunks (Build-Order 91)

Crate-Modul `work_chunks`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- `ChunkSizer`: Länge der Planungseinheit (Chunk) aus dem exponentiell geglätteten Durchsatz so bestimmen, dass ein Launch etwa die Zieldauer dauert.
- `PreemptionGate` und `TaskClass`: Query-Tasks überholen Indexing-Chunks. Preemption ist nur an Chunk-Grenzen möglich.
- `run_chunks`: einen Slot-Bereich chunkweise abarbeiten, mit Zeitbudget und Preemption. Ergebnis sind erledigte Slots, Restbereich und Grund.
- `score_range`: jeden Tile gegen alle Queries des Batches scoren und in den `RangeScorer` übernehmen.

**Grenzen**
- Kein Kernel, kein Scoring, keine Device-Zugriffe, kein `unsafe`. Das Scoring liefert der Aufrufer über das Trait `TileScorer`.
- `Workspace`, `QueryBatch`, `DocSource`, `SegmentId`, `RangeScorer`, `TopKError` werden nicht neu definiert, sondern genutzt.
- Kein Scheduler, kein Thread-Pool und keine Task-Warteschlange. Der Aufrufer ruft `run_chunks` auf und hält die Tasks.

## 2. Context

- `query_batch::Workspace` ist ein halboffener Slot-Bereich, der einem Kernel-Launch entspricht. `split_front(max_len)` zerlegt ihn.
- Das Scoring liest jeden Tile einmal und rechnet ihn gegen alle wartenden Queries (`QueryBatch`). Das amortisiert die Lesekosten.
- `topk::RangeScorer::add_range` übernimmt die Roh-Scores je Workspace.
- Indexing-Chunks und Query-Chunks laufen konkurrierend. Eine Query-Task wartet nicht auf das Ende eines langen Indexing-Bereichs. Der Indexing-Lauf endet an der nächsten Chunk-Grenze mit dem Grund `Preemption` und liefert den Restbereich zur späteren Fortsetzung.

## 3. Interface

```rust
use query_batch::{DocSource, QueryBatch, SegmentId, Workspace};
use topk::{RangeScorer, TopKError};
use std::sync::atomic::AtomicUsize;
use std::time::Duration;

pub const DEFAULT_MIN_LEN: u32 = 16;
pub const DEFAULT_MAX_LEN: u32 = 1 << 20;            // rund eine Million Slots
pub const DEFAULT_TARGET: Duration = Duration::from_millis(2);
pub const EMA_ALPHA: f64 = 0.3;

/// Dimensioniert Chunks aus geglättetem Durchsatz (Slots/s).
#[derive(Debug, Clone)]
pub struct ChunkSizer { /* min_len, max_len, target, throughput: Option<f64> */ }
impl ChunkSizer {
    /// Defaults: 16, 1<<20, 2 ms, kein Durchsatz.
    pub fn new() -> ChunkSizer;
    /// min_len wird auf mindestens 1 angehoben; max_len auf mindestens min_len.
    pub fn with_limits(min_len: u32, max_len: u32, target: Duration) -> ChunkSizer;
    /// Länge des nächsten Chunks.
    pub fn next_len(&self) -> u32;
    /// Geglätteter Durchsatz in Slots pro Sekunde; None vor der ersten Messung.
    pub fn throughput(&self) -> Option<f64>;
    /// Beobachtung eines Launches: `slots` in `elapsed`.
    pub fn observe(&mut self, slots: u32, elapsed: Duration);
    pub fn min_len(&self) -> u32;
    pub fn max_len(&self) -> u32;
    pub fn target(&self) -> Duration;
}
impl Default for ChunkSizer { /* = new() */ }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskClass { Query, Indexing }

/// Gemeinsamer Zustand für Vorrang von Query-Tasks. Send + Sync, nur &self.
#[derive(Debug, Default)]
pub struct PreemptionGate { /* waiting_queries: AtomicUsize */ }
impl PreemptionGate {
    pub fn new() -> PreemptionGate;
    /// Meldet eine wartende/laufende Query-Task an; Abmeldung beim Drop des Guards.
    pub fn enter_query(&self) -> QueryGuard<'_>;
    pub fn waiting_queries(&self) -> usize;
    /// true nur für Indexing, wenn waiting_queries() > 0. Query ist nie preemptierbar.
    pub fn should_preempt(&self, class: TaskClass) -> bool;
}
#[derive(Debug)]
pub struct QueryGuard<'a> { /* &'a PreemptionGate */ }   // Drop: Zähler −1

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason { TimeBudget, Preemption }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkOutcome {
    pub done_slots: u32,       // Summe der vollständig gelaufenen Chunk-Längen
    pub rest: Workspace,       // unbearbeiteter Restbereich (rest.end == range.end)
    pub stop: Option<StopReason>, // None = Bereich vollständig erledigt
}

/// Wird je Chunk aufgerufen.
pub fn run_chunks<E, F>(
    sizer: &mut ChunkSizer,
    range: Workspace,
    budget: Duration,
    class: TaskClass,
    gate: &PreemptionGate,
    run_chunk: F,
) -> Result<ChunkOutcome, E>
where F: FnMut(Workspace) -> Result<(), E>;

/// Scoring eines Tiles gegen alle Queries des Batches.
pub trait TileScorer: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    /// Liefert Scores query-major: scores[q * tile.len() + i] für Slot tile.start()+i.
    fn score_tile(&self, batch: &QueryBatch, segment: SegmentId, tile: Workspace)
        -> Result<Vec<f32>, Self::Error>;
}

#[derive(Debug)]
pub enum ChunkError<E> { Score(E), TopK(TopKError) }
// Display + std::error::Error (wenn E: Error), From<TopKError>.

/// Verarbeitet `range` chunkweise: je Chunk score_tile über alle Queries, dann rs.add_range.
pub fn score_range<S: TileScorer>(
    sizer: &mut ChunkSizer,
    scorer: &S,
    src: &dyn DocSource,
    batch: &QueryBatch,
    segment: SegmentId,
    range: Workspace,
    budget: Duration,
    class: TaskClass,
    gate: &PreemptionGate,
    rs: &mut RangeScorer,
) -> Result<ChunkOutcome, ChunkError<S::Error>>;
```

## 4. Behavior and invariants

**ChunkSizer**
- Ohne Messung gilt `throughput() == None` und `next_len() == min_len()`.
- Mit Messung gilt `next_len() = clamp(round(throughput * target_secs), min_len, max_len)`. Die Berechnung erfolgt in `f64`, die Umwandlung ist sättigend, und es gibt kein Überlaufen von `u32`.
- Invarianten: `min_len >= 1`, `min_len <= next_len() <= max_len`.
- `observe(slots, elapsed)`:
  - Bei `slots == 0` wird die Beobachtung ignoriert (Zustand unverändert).
  - `elapsed` wird nach unten auf 1 µs begrenzt.
  - Sample = `slots / elapsed_secs`.
  - Die erste Messung setzt den Durchsatz = Sample.
  - Danach gilt `t = 0.3 * sample + 0.7 * t`.
- Es gibt keine Panics, auch nicht bei `Duration::ZERO`.

**PreemptionGate / Vorrang**
- `should_preempt(Indexing)` ist genau dann true, wenn mindestens ein `QueryGuard` lebt. `should_preempt(Query)` ist immer false.
- Das Gate nutzt Atomics ohne Locks. Der Zähler läuft nie unter 0.
- Query-Tasks überholen Indexing-Chunks nur an Chunk-Grenzen. Ein laufender Chunk wird nie abgebrochen.

**run_chunks**
- Leerer `range`: es läuft kein Chunk. Ergebnis `done_slots = 0`, `rest = range`, `stop = None`.
- Schleife:
  1. `chunk, rest = rest.split_front(sizer.next_len())`.
  2. Chunk-Zeit messen und `run_chunk(chunk)` aufrufen.
  3. Bei Erfolg `sizer.observe(chunk.len(), elapsed)` und `done_slots += chunk.len()`.
  4. Nur wenn danach `rest` nicht leer ist (nach jedem Chunk außer dem letzten), die Abbruchprüfung machen: erst Zeitbudget (`elapsed_total >= budget` → `TimeBudget`), dann `gate.should_preempt(class)` (→ `Preemption`).
- Das Zeitbudget hat Vorrang, wenn beide Gründe zutreffen.
- Mindestens ein Chunk läuft immer vollständig, auch bei `budget == 0` oder aktiver Preemption.
- Der letzte Chunk wird nie abgebrochen. Der Rest ist dann leer, und `stop = None`.
- Invarianten: `done_slots + rest.len() == range.len()`, die erledigten Chunks sind lückenlos ab `range.start()`, und `rest.start() == range.start() + done_slots`.
- Fehler von `run_chunk` beenden sofort mit `Err(e)`. Der fehlgeschlagene Chunk wird nicht gezählt und nicht in `observe` aufgenommen. Es folgen keine weiteren Chunks.

**score_range**
- Je Chunk: `scorer.score_tile(batch, segment, tile)` und danach `rs.add_range(src, segment, tile, &scores)`.
- Ein Fehler von `score_tile` → `ChunkError::Score`, von `add_range` (inkl. `SlotRange`, `ScoreLen`) → `ChunkError::TopK`. Beide beenden sofort. Der `RangeScorer` enthält dann nur Chunks, die vorher vollständig erfolgreich waren.
- Der Aufrufer fordert `rs.num_queries() == batch.len()`. Eine Abweichung zeigt sich über `ScoreLen` aus `add_range`.
- Der Rest aus `ChunkOutcome` liegt im selben Segment. Der Aufrufer setzt damit später fort.
- Es gibt keine stille Kürzung. Der Bereichscheck liegt bei `add_range` je Chunk.

**Concurrency**
- `ChunkSizer` ist pro Task veränderlich (`&mut`) und `Send`. `PreemptionGate` ist `Send + Sync` und wird per Referenz oder `Arc` geteilt.
- Es gibt keinen globalen Zustand. Alle Typen sind `Send + Sync`. Es gibt keine Panics bei gültigen Eingaben.

## 5. Relied-on elements

Aus `query_batch`:
- `Workspace`: `new`, `start() -> u32`, `end() -> u32`, `len() -> u32`, `is_empty()`, `range()`, `split_front(self, max_len: u32) -> (Workspace, Workspace)`. Dabei ist Vorderteil ++ Rest = self, `front.len() <= max_len`, und `max_len == 0` ergibt einen leeren Vorderteil. Deshalb ist `next_len() >= 1` zwingend, damit die Schleife terminiert.
- `QueryBatch` mit `len()`, `SegmentId(pub u64)`, `trait DocSource: Send + Sync`.
- `check_slot_range` wird nicht direkt aufgerufen, sondern über `add_range`.

Aus `topk`:
- `RangeScorer::add_range(&mut self, src: &dyn DocSource, segment: SegmentId, ws: Workspace, scores: &[f32]) -> Result<(), TopKError>`.
  - `scores` ist query-major: `scores[q * ws.len() + i]`.
  - Die Länge ist `num_queries * ws.len()`.
  - Bei Fehler bleibt der Zustand unverändert.
  - Ein leerer Workspace ist mit leeren Scores Ok.
- `RangeScorer::num_queries()`.
- `TopKError` (`Source`, `ScoreLen`, `Mismatch`): `Display + Error + Send + Sync`.