# Spezifikation: Referenz-Scorer-Vergleich (Build-Order 90)

Crate-Modul `reference_compare`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- Zwei Scorer (Kandidat und Referenz) über dieselben Workspaces laufen lassen.
- Ihre Top-k-Ergebnisse je Query mit Toleranz vergleichen, nicht auf exakte Gleichheit.
- Abweichungen klassifizieren, als Warnung mit Segment loggen und im Report zurückgeben.
- Abweichungen führen nie zum Abbruch. Fehler der Scorer und der Abhängigkeiten werden unverändert weitergereicht.

**Grenzen**
- Das Modul enthält keinen Kernel, kein eigenes Scoring, kein `unsafe` und keine Device-Zugriffe.
- Es definiert `SegmentId`, `DocSource`, `Workspace`, `QueryBatch`, `SourceError`, `Hit`, `RangeScorer` und `TopKError` nicht neu.
- Es zerlegt keine Bereiche. Der Aufrufer liefert die Workspaces.
- Es wählt die Precision nicht selbst. Der Aufrufer gibt sie an.

## 2. Context

- `query_batch` liefert `QueryBatch`, `DocSource`, `SegmentId` und `Workspace` (ein Workspace ist ein halboffener Slot-Bereich und entspricht einem Kernel-Launch).
- `topk` liefert `RangeScorer`. Er nimmt query-major Roh-Scores je Workspace an und liefert je Query eine absteigend sortierte `Vec<Hit>`.
- Dieses Modul sitzt darüber. Es ruft einen Kandidaten-Scorer und einen Referenz-Scorer über dieselben Workspaces auf. Es reduziert beide Ergebnisse mit je einem `RangeScorer` und vergleicht die beiden Trefferlisten.
- Die Scorer-Implementierungen (GPU-Kernel, CPU-Referenz) liegen außerhalb. Sie implementieren das hier definierte Trait `Scorer`.

## 3. Interface

```rust
use query_batch::{DocSource, QueryBatch, SegmentId, Workspace};
use topk::{Hit, TopKError};

/// Numerische Genauigkeit des Kandidaten; bestimmt die absolute Toleranz.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precision { Fp32, Fp16 }
impl Precision {
    /// Fp32 → 1e-5, Fp16 → 1e-3 (absolut).
    pub fn tolerance(self) -> f32;
}

/// Fehler eines Scorers (opak, wird nur weitergereicht).
pub type ScorerError = Box<dyn std::error::Error + Send + Sync>;

/// Scorer über einen Workspace.
pub trait Scorer: Send + Sync {
    /// Roh-Scores query-major: Score von Query q für Slot ws.start()+i steht bei
    /// out[q * ws.len() + i]. Länge = batch.len() * ws.len().
    fn score_range(&self, src: &dyn DocSource, segment: SegmentId, ws: Workspace, batch: &QueryBatch)
        -> Result<Vec<f32>, ScorerError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side { Candidate, Reference }

#[derive(Debug, Clone, PartialEq)]
pub enum DeviationKind {
    /// Unterschiedliche Trefferanzahl: ab Rang `from_rank` (= Länge der kürzeren Liste)
    /// hat `missing_on` keine weiteren Ränge dieser Query mehr.
    CountMismatch { candidate_len: usize, reference_len: usize, from_rank: usize, missing_on: Side },
    /// Score an gleichem Rang weicht um mehr als die Toleranz ab.
    ValueMismatch { rank: usize, candidate: f32, reference: f32, diff: f32 },
    /// Gleicher Rang, Score innerhalb Toleranz, aber anderer Offset.
    RankMismatch { rank: usize, candidate_offset: u64, reference_offset: u64,
                   /// Rang des Kandidaten-Offsets in der Referenzliste; None, wenn dort nicht enthalten.
                   reference_rank_of_candidate: Option<usize> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deviation { pub segment: SegmentId, pub query: usize, pub kind: DeviationKind }

#[derive(Debug, Clone, PartialEq)]
pub struct CompareReport {
    pub candidate: Vec<Vec<Hit>>,   // Index = Query
    pub reference: Vec<Vec<Hit>>,
    pub deviations: Vec<Deviation>, // sortiert nach query, dann rank; CountMismatch zuletzt je Query
}
impl CompareReport { pub fn is_clean(&self) -> bool; }  // deviations.is_empty()

#[derive(Debug)]
pub enum CompareError {
    Candidate(ScorerError),
    Reference(ScorerError),
    TopK(TopKError),
}
// Display + std::error::Error + Send + Sync; From<TopKError>.

/// Läuft beide Scorer über `workspaces` (in Reihenfolge), reduziert auf Top-k und vergleicht.
pub fn compare_scorers(
    candidate: &dyn Scorer, reference: &dyn Scorer,
    src: &dyn DocSource, segment: SegmentId,
    workspaces: &[Workspace], batch: &QueryBatch,
    k: usize, precision: Precision,
) -> Result<CompareReport, CompareError>;

/// Reiner Vergleich zweier Trefferlisten einer Query (ohne Logging).
pub fn compare_hits(segment: SegmentId, query: usize, candidate: &[Hit], reference: &[Hit],
                    tol: f32) -> Vec<Deviation>;
```

## 4. Behavior and invariants

**Ablauf von `compare_scorers`**
- Es legt zwei `RangeScorer::new(batch.len(), k)` an, einen je Scorer.
- Für jeden Workspace in Reihenfolge ruft es erst `candidate.score_range`, dann `reference.score_range` mit denselben `src`, `segment`, `ws` und `batch` auf. Die Scores gehen sofort per `add_range` in den zugehörigen `RangeScorer`.
- Leere Workspaces werden übersprungen (kein Scorer-Aufruf).
- Danach ruft es `into_results()` auf beiden und `compare_hits` je Query.
- Abweichungen werden mit `log::warn!` geloggt, eine Zeile je Deviation. Jede Zeile enthält Segment (`segment.0`), Query-Index und Art mit den Werten (bei ValueMismatch Rang und Differenz, bei RankMismatch die Offsets).
- Der Rückgabewert ist `Ok(CompareReport)`, auch wenn Abweichungen vorliegen. Abweichungen erzeugen nie ein `Err` und nie ein Panic. Ein sauberer Lauf loggt nichts.

**Fehler**
- Ein Fehler von `candidate.score_range` ergibt sofort `Err(CompareError::Candidate(e))`. Bei der Referenz gilt `Reference(e)`. Der Fehler wird unverändert weitergereicht, ohne Umdeuten und ohne Warnung.
- Fehler von `add_range` (`Source`, `ScoreLen`) ergeben `Err(CompareError::TopK(e))`. Dazu gehört eine falsche Score-Länge eines Scorers.
- Die Verarbeitung bricht beim ersten Fehler ab. Es gibt keinen Teilreport.

**Toleranz**
- `Precision::Fp32.tolerance() == 1e-5`, `Fp16.tolerance() == 1e-3`, jeweils absolut.
- Zwei Scores gelten als gleich, wenn `|c − r| <= tol` (Differenz in f64 gerechnet).
- Es gibt keine exakte Gleichheitsprüfung auf Scores.

**`compare_hits`: Klassifikation** (c = Kandidatenliste, r = Referenzliste, n = min(len c, len r))
- Für jeden Rang `i < n` wird höchstens eine Deviation erzeugt:
  1. Gilt `|c[i].score − r[i].score| > tol`, entsteht `ValueMismatch { rank: i, candidate, reference, diff }`, mit `diff = |c − r|`.
  2. Sonst, bei `c[i].offset != r[i].offset`, entsteht `RankMismatch`, außer es liegt ein Beinahe-Gleichstand vor (nächster Punkt).
- Beinahe-Gleichstand: Der Kandidaten-Offset steht in der Referenzliste an Rang `j`. Dabei gilt für jedes benachbarte Referenzpaar zwischen `min(i,j)` und `max(i,j)`: `|r[m].score − r[m+1].score| <= tol`. Dann wird keine Rangabweichung gemeldet.
- `reference_rank_of_candidate` ist der kleinste Rang `j` in r mit `r[j].offset == c[i].offset`, sonst `None`. Bei `None` gilt kein Beinahe-Gleichstand, es wird gemeldet.
- Bei `len c != len r` entsteht zusätzlich genau eine `CountMismatch { candidate_len, reference_len, from_rank: n, missing_on }`. `missing_on` ist die Seite mit der kürzeren Liste. Für die Ränge `>= n` entstehen keine Value- oder RankMismatch.
- Gleiche Längen und alle Ränge gleich (innerhalb Toleranz) ergeben eine leere Liste.
- Beide Listen leer ergeben eine leere Liste.
- `Deviation.segment` und `Deviation.query` werden aus den Argumenten gesetzt.
- Die Ausgabe ist deterministisch und hängt nur von den Eingaben ab.

**Workspaces**
- Beide Scorer sehen exakt dieselben Workspaces, dieselbe Reihenfolge, dieselbe `DocSource` und denselben `QueryBatch`. Das Modul ändert Workspaces nicht.

**Concurrency**
- Alle Typen sind `Send + Sync`. Es gibt keinen globalen Zustand außer dem `log`-Aufruf und keine Locks.
- Die Scorer werden sequentiell aufgerufen (Kandidat, dann Referenz je Workspace). Mehrere Aufrufe von `compare_scorers` dürfen parallel laufen.

**Edge Cases**
- `k == 0` ergibt leere Listen und einen sauberen Report.
- Ein leerer Batch ergibt leere Vektoren in `candidate`/`reference` und einen sauberen Report.
- NaN-Scores werden schon von `topk` verworfen und tauchen im Vergleich nicht auf.

## 5. Relied-on elements

Aus `query_batch`:
- `SegmentId(pub u64)` (`Copy`).
- `QueryBatch::len() -> usize`.
- `Workspace` (`start()`, `end()`, `len() -> u32`, `is_empty()`, `range()`).
- `trait DocSource: Send + Sync`.

Aus `topk`:
- `Hit { pub offset: u64, pub score: f32 }` (`Copy`, `PartialEq`).
- `RangeScorer::new(num_queries, k)`.
- `RangeScorer::add_range(&mut self, src: &dyn DocSource, segment: SegmentId, ws: Workspace, scores: &[f32]) -> Result<(), TopKError>`. Die Scores sind query-major: `scores[q * ws.len() + i]`.
- `RangeScorer::into_results(self) -> Vec<Vec<Hit>>`. Die Listen sind absteigend sortiert, bei gleichem Score zuerst der niedrigere Offset, höchstens k Treffer, Index = Query.
- `TopKError` (`Source`, `ScoreLen`, `Mismatch`): `Display + Error + Send + Sync`.
- Der Offset ist der rohe `slot_offset` (Byte-Offset). `add_range` prüft `check_slot_range` selbst. Es gibt keine stille Kürzung.