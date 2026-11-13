# Spezifikation: Ergebnisarten (Build-Order 34)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Das Crate-Modul `result_kinds` definiert die Ergebnisart eines Tasks als Datentyp. Genau eine der folgenden Arten ist möglich:
- Embeddings
- Entry-Referenzen
- Treffer
- Similarity-Report
- Score-Liste

Die Ergebnisse sind auf Gleichheit vergleichbar, und die Menge der Arten ist erweiterbar.

**Grenzen:**
- Das Modul enthält keine Ausführung, kein Scheduling, keine Inferenz, keine Suche und keine Serialisierung.
- Es führt keine Konvertierung zwischen den Arten durch.
- Es greift nicht auf einen Index zu. Es nutzt nur die ID-Typen des Index-Vertrags.
- Nur `std` und `index_contract::EntryId`.
- Keine `unsafe`-Blöcke. Alle Typen sind reine Wertetypen.

## 2. Kontext
- Ein Task liefert als Ergebnis genau einen `TaskResult`. Verbraucher unterscheiden die Art per `match` oder per `kind()`.
- Zum Schreiben in den Index: `TaskResult::EntryRefs` enthält die IDs der geschriebenen Entries. Die Daten selbst liegen dann im Index (`index_contract::Index::write`).
- Treffer sind Paare aus `EntryId` und Score.
- Eine Cross-Encoder-Inferenz liefert `Scores`.
- Bei Embeddings ist jeder Vektor ein `Vec<f32>`.
- Das Modul vergibt keine Stamps und trifft keine Index-Entscheidungen.

## 3. Interface
```rust
use index_contract::EntryId;

/// Diskriminante der Ergebnisart. Erweiterbar (non_exhaustive).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResultKind { Embeddings, EntryRefs, Hits, SimilarityReport, Scores }

impl ResultKind {
    /// Stabiler Name: "embeddings" | "entry_refs" | "hits" | "similarity_report" | "scores".
    pub fn name(self) -> &'static str;
}

/// Ein Treffer: Entry-ID und Score.
#[derive(Debug, Clone, Copy)]
pub struct Hit { pub id: EntryId, pub score: f32 }

/// Ein Paar im Similarity-Report.
#[derive(Debug, Clone, Copy)]
pub struct SimilarityPair { pub a: EntryId, pub b: EntryId, pub score: f32 }

/// Similarity-Report: Liste von Paar-Ähnlichkeiten, Reihenfolge erhalten.
#[derive(Debug, Clone, Default)]
pub struct SimilarityReport { pub pairs: Vec<SimilarityPair> }

/// Ergebnis eines Tasks: genau eine Art. Erweiterbar (non_exhaustive).
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum TaskResult {
    Embeddings(Vec<Vec<f32>>),
    EntryRefs(Vec<EntryId>),
    Hits(Vec<Hit>),
    SimilarityReport(SimilarityReport),
    Scores(Vec<f32>),
}

impl TaskResult {
    pub fn kind(&self) -> ResultKind;
    pub fn as_embeddings(&self)  -> Option<&[Vec<f32>]>;
    pub fn as_entry_refs(&self)  -> Option<&[EntryId]>;
    pub fn as_hits(&self)        -> Option<&[Hit]>;
    pub fn as_report(&self)      -> Option<&SimilarityReport>;
    pub fn as_scores(&self)      -> Option<&[f32]>;
}

// Gleichheit (manuell implementiert, siehe §4):
impl PartialEq for Hit {}              impl Eq for Hit {}
impl PartialEq for SimilarityPair {}   impl Eq for SimilarityPair {}
impl PartialEq for SimilarityReport {} impl Eq for SimilarityReport {}
impl PartialEq for TaskResult {}       impl Eq for TaskResult {}
impl std::hash::Hash for TaskResult {} // konsistent mit Eq; ebenso Hit, SimilarityPair, SimilarityReport
```
Alle Typen sind `Send + Sync + 'static`.

## 4. Behavior und Invarianten
**Genau eine Art**
- Ein `TaskResult` hat genau eine Art. `kind()` liefert die zur Variante passende `ResultKind`.
- `as_*` liefert `Some` nur bei der passenden Variante, sonst `None`.
- Leere Listen sind gültige Ergebnisse. Ein leeres `Embeddings` ist ungleich einem leeren `Scores`, weil die Art verschieden ist.

**Gleichheit**
- Zwei `TaskResult` sind gleich genau dann, wenn Art und Inhalt gleich sind.
- Der Vergleich ist Element für Element und reihenfolgeempfindlich. Er berücksichtigt auch die Länge. Bei `Embeddings` müssen die inneren Vektoren ebenfalls gleich lang sein.
- Floats (`f32`) werden bitweise verglichen (`to_bits`).
  - `NaN` mit gleichem Bitmuster ist gleich.
  - `0.0` und `-0.0` sind ungleich.
  - Dadurch sind `Eq` und `Hash` total und reflexiv.
  - `Hash` hasht dieselben Bits. Gleiche Werte haben gleichen Hash.
- `Hit` und `SimilarityPair` sind gleich, wenn alle Felder gleich sind (Score bitweise).
- `clone()` ist gleich dem Original.

**Erweiterbarkeit**
- `ResultKind` und `TaskResult` sind `#[non_exhaustive]`. Neue Arten sind in neuen Varianten ergänzbar, ohne bestehende Varianten oder deren Gleichheit zu ändern.
- Externe `match`-Ausdrücke brauchen einen Wildcard-Zweig.
- Neue Arten müssen `kind()`, `name()`, `PartialEq`, `Eq` und `Hash` mitführen.

**Fehler und Nebenläufigkeit**
- Es gibt keine Fehlerrückgaben und keine Panics, auch nicht bei leeren Listen oder NaN.
- Es gibt keinen geteilten Zustand und keine Interior Mutability. Die Typen sind `Send + Sync`. Die Methoden sind rein.

## 5. Relied-on elements
Aus `index_contract` (Build-Order 33), unverändert genutzt:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntryId(pub u64);
```
`EntryId` ist ein eigener Typ. Er ist nicht mit `NodeId` oder `CellId` austauschbar. Weitere Elemente des Index-Vertrags werden nicht verwendet.