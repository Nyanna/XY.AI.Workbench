# Spezifikation: Top-k-Selektion (Build-Order 81)

Crate-Modul `topk`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- Pro Query eine begrenzte Treffermenge (`TopK`) aus (Offset, Score) führen.
- Die Ausgabe ist nach Score absteigend sortiert. Bei gleichem Score gilt der niedrigere Offset zuerst.
- `RangeScorer`: Roh-Scores mit Slot-Offsets entgegennehmen. Je Workspace lokal reduzieren und die Teilergebnisse auf der CPU mergen. Mehrere Bereiche werden nacheinander verarbeitet.

**Grenzen**
- Dieses Modul enthält kein Scoring und keinen Kernel. Es greift nicht auf Device-Speicher zu und enthält kein `unsafe`.
- Es berechnet keine Scores. Die Roh-Scores kommen vom Aufrufer als `f32`.
- Es definiert `SegmentId`, `DocSource`, `Workspace`, `QueryBatch`, `SourceError` und `check_slot_range` nicht neu. Es nutzt sie aus `query_batch`.
- Es dedupliziert keine Treffer. Gleiche Offsets werden nicht zusammengeführt.

## 2. Context

- Das Scoring liefert pro Workspace (halboffener Slot-Bereich, ein Kernel-Launch) Roh-Scores für alle Queries des Batches.
- Der Treffer-Offset ist der Byte-Offset des Slots im Segment (`DocSource::slot_offset`, `u64`).
- Dieses Modul reduziert je Workspace auf höchstens k Treffer je Query und merged die Teilergebnisse über Workspaces und Bereiche hinweg.
- Es liefert das Endergebnis je Query als absteigend sortierte Trefferliste.

## 3. Interface

```rust
use query_batch::{DocSource, SegmentId, SourceError, Workspace};

/// Ein Treffer: Byte-Offset des Slots und Score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit { pub offset: u64, pub score: f32 }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopKError {
    Source(SourceError),                                  // aus check_slot_range
    ScoreLen { expected: usize, got: usize },             // scores.len() != num_queries * ws.len()
    Mismatch { what: &'static str },                      // merge: unterschiedliche k oder Query-Anzahl
}
// Display + std::error::Error + Send + Sync; From<SourceError>.

/// Begrenzte Treffermenge einer Query.
#[derive(Debug, Clone)]
pub struct TopK { /* k, Heap/Puffer */ }
impl TopK {
    pub fn new(k: usize) -> TopK;
    pub fn k(&self) -> usize;                 // Soll-Größe
    pub fn len(&self) -> usize;               // Anzahl gehaltener Treffer, <= k
    pub fn is_empty(&self) -> bool;
    /// true, wenn der Treffer aufgenommen wurde (auch durch Verdrängen). false, wenn verworfen.
    pub fn push(&mut self, offset: u64, score: f32) -> bool;
    pub fn extend<I: IntoIterator<Item = (u64, f32)>>(&mut self, items: I);
    /// Nimmt alle Treffer von other auf. k von self bleibt.
    pub fn merge(&mut self, other: TopK);
    /// Absteigend sortiert, verbraucht self.
    pub fn into_sorted_vec(self) -> Vec<Hit>;
}

/// Top-k je Query eines Batches über mehrere Bereiche.
#[derive(Debug, Clone)]
pub struct RangeScorer { /* Vec<TopK> */ }
impl RangeScorer {
    pub fn new(num_queries: usize, k: usize) -> RangeScorer;
    pub fn num_queries(&self) -> usize;
    pub fn k(&self) -> usize;
    /// scores: query-major. Der Score von Query q für Slot ws.start()+i steht bei scores[q * ws.len() + i].
    /// Der Offset kommt aus src.slot_offset(segment, Slot(ws.start()+i)).
    pub fn add_range(&mut self, src: &dyn DocSource, segment: SegmentId, ws: Workspace, scores: &[f32])
        -> Result<(), TopKError>;
    /// Merged ein anderes Teilergebnis. Gleiches k und gleiche Query-Anzahl sind Pflicht.
    pub fn merge(&mut self, other: RangeScorer) -> Result<(), TopKError>;
    /// Je Query absteigend sortierte Treffer, Index = Query-Index.
    pub fn into_results(self) -> Vec<Vec<Hit>>;
}
```

## 4. Behavior and invariants

**Ordnung und Auswahl**
- Treffer a schlägt Treffer b, wenn `a.score > b.score`, oder bei gleichem Score, wenn `a.offset < b.offset`. Das ist eine totale Ordnung auf nicht-NaN-Werten. `0.0` und `-0.0` sind gleich. `±inf` sind gültig.
- `TopK` hält immer die besten `min(k, Anzahl gültiger Kandidaten)` Treffer. Das Ergebnis hängt nicht von der Eingabereihenfolge ab, auch nicht bei Aufnahme in Teilen oder Merge.
- NaN-Scores werden verworfen (`push` → `false`, Zustand unverändert).
- `k == 0`: `push` nimmt nichts auf und gibt `false` zurück. `len() == 0`, `is_empty() == true`, `k() == 0`.
- Ist die Menge voll, wird ein Kandidat nur aufgenommen, wenn er den aktuell schlechtesten Treffer schlägt. Dieser wird dann verdrängt. Ein gleichwertiger Kandidat (gleicher Score und gleicher Offset) verdrängt nichts.
- Treffer mit gleichem Offset und Score dürfen mehrfach vorkommen.
- `into_sorted_vec` ist absteigend nach Ordnung (bester zuerst). Bei weniger als k Kandidaten ist die Ausgabe entsprechend kürzer, bei null Kandidaten leer.
- `extend` ist äquivalent zu `push` je Element in Reihenfolge. `merge(other)` ist äquivalent zu `push` aller Treffer von `other`.
- Allokation ist durch k begrenzt, nicht durch die Kandidatenzahl. Es darf nicht vorab k Elemente reserviert werden, wenn das bei sehr großem k unverhältnismäßig wäre. Ein Heap, der mit der Aufnahme wächst, ist erlaubt.

**RangeScorer**
- `add_range` prüft zuerst `check_slot_range(src, segment, ws.range())`. Bei Fehler → `TopKError::Source`.
- Danach prüft es `scores.len() == num_queries * ws.len()`, mit überlaufsicherer Rechnung. Bei Abweichung → `ScoreLen { expected, got }`. Bei einem Fehler bleibt der Zustand unverändert.
- Ein leerer Workspace ist mit `scores.is_empty()` Ok und ändert nichts. Bei `num_queries == 0` ist `scores` leer, sonst gilt dieselbe Längenregel.
- Lokale Reduktion: Je Workspace wird pro Query zuerst ein lokales `TopK(k)` gebildet. Danach wird es mit `merge` in das Gesamt-`TopK` der Query übernommen. Das Ergebnis ist identisch zur direkten Aufnahme aller Kandidaten.
- Mehrere Bereiche (auch verschiedener Segmente) werden durch wiederholte `add_range`-Aufrufe nacheinander verarbeitet. Die Reihenfolge der Aufrufe ändert das Ergebnis nicht, außer bei identischen (Offset, Score)-Paaren, die ohnehin ununterscheidbar sind.
- Der Offset ist der rohe `slot_offset`. Es gibt keine Segmentkodierung und keine Umrechnung. Offsets verschiedener Segmente können kollidieren. Das ist Sache des Aufrufers.
- Ein Slot ohne Vektoren hat laut Source denselben Offset wie der Nachfolger. Er wird nicht gesondert behandelt. Ob er aufgenommen wird, entscheidet nur der übergebene Score (NaN → verworfen).
- `merge` mit abweichendem `k` oder abweichender Query-Anzahl → `Mismatch`. Bei einem Fehler bleibt der Zustand unverändert.
- `into_results().len() == num_queries()`. Jede Liste hat höchstens k Treffer.

**Concurrency und Fehler**
- Alle Typen sind `Send + Sync`. Es gibt keinen globalen Zustand und keine Locks.
- Der Aufrufer synchronisiert selbst. Parallele Workspaces nutzen je eigene `RangeScorer` und werden danach mit `merge` zusammengeführt.
- Es gibt keine Panics bei gültigen Eingaben und keine stillen Überläufe.

## 5. Relied-on elements

Aus `query_batch`, unverändert:
- `SegmentId(pub u64)`, `Copy`.
- `Workspace`: `start() -> u32`, `end() -> u32`, `len() -> u32`, `is_empty()`, `range() -> Range<u32>`. Halboffen, `end >= start`.
- `trait DocSource: Send + Sync` mit `slot_offset(&self, SegmentId, Slot) -> Option<u64>`. Das ist der Byte-Offset des Slots und dient als Treffer-Offset. Es gibt `None` bei unbekanntem Segment oder `slot >= slot_count`.
- `check_slot_range(src: &dyn DocSource, segment: SegmentId, range: Range<u32>) -> Result<(), SourceError>`. Ok genau dann, wenn `start <= end <= slot_count`. Bei unbekanntem Segment ist nur `0..0` Ok. Es gibt keine stille Kürzung.
- `SourceError::SlotRange { segment, start, end, slot_count }`: `Display + Error + Send + Sync`, `Clone, PartialEq, Eq`.
- `kernel_layout::Slot(pub u32)`: für `Slot(ws.start() + i)`.

Nach erfolgreichem `check_slot_range` liefert `slot_offset` für jeden Slot im Bereich `Some`. Ein `None` darf trotzdem nicht zu einem Panic führen: Der Slot wird dann übersprungen.