# Spezifikation: Similarity-Task (Build-Order 53)

## 1. Task, restrictions and boundaries

Das Crate-Modul `similarity_task` beschreibt einen Similarity-Suchauftrag und führt ihn aus.
- Es nimmt eine Liste von Query-Vektoren, eine Index-ID, Top-k, Scope, Scoring-Verfahren, Precision und Zusatzparameter entgegen.
- Es liefert pro Query die Treffer als Entry, Score und stale-Markierung.
- Reranking ist keine eigene Task-Art. Es ist eine zweite `SimilarityParams` mit anderem Scoring auf einem verengten Scope, dessen Kandidaten aus einem früheren Ergebnis stammen.

Grenzen:
- Das Modul rechnet nur auf den Kandidaten, die die `CandidateSource` liefert. Es garantiert weder Vollständigkeit noch Größe der Kandidatenmenge noch Ergebnisqualität. Genauigkeitsverluste durch Chunking, Query-Formulierung, Index-Struktur, Reranking oder Kandidatenzahl liegen bei den äußeren Schichten.
- Es löst Scopes nicht selbst auf. Das ist Sache der `CandidateSource`.
- Es hat keinen Index-Zugriff, kein Scheduling, keine Queue, keine Persistenz und keine Serialisierung.
- Stale Treffer sind kein Fehler.
- Es hat keinen globalen Zustand und keine `unsafe`-Blöcke.
- Abhängigkeiten: nur `std` und die Module Ergebnisarten (`result_kinds`), Erweiterbare Zusatzparameter (`Extensions`) und Scopes und Entry-Mengen (`Scope`, `EntrySet`).

## 2. Context

- Ein Task-Executor ruft `run_similarity` auf. Die Kandidaten (Entry-ID, Vektor, stale-Flag) kommen über das Trait `CandidateSource`. Das implementiert die äußere Schicht, die den Index kennt.
- Die Index-ID ist verpflichtend. Die Source leitet daraus das Modell ab.
- `TaskResult::Hits` (aus `result_kinds`) trägt kein stale-Flag. Deshalb hat dieses Modul einen eigenen Ergebnistyp `SimilarityOutput` mit stale. `into_task_result()` konvertiert verlustbehaftet zu `TaskResult::Hits`.
- Ein Similarity-Task kann den Scope eines Folge-Tasks treiben: `SimilarityOutput::narrowed_scope()` liefert `Scope::Set` aus den Treffer-IDs. `SimilarityParams::rerank` baut daraus den Folgeauftrag.
- Das Modul benutzt zwei `EntryId`-Darstellungen:
  - `result_kinds::EntryId`, also `index_contract::EntryId(pub u64)`, ist der Typ aller Treffer und der Candidate-API.
  - Im Scopes-Modul ist `EntryId` nur ein Alias für `u64`.
  - Die Umwandlung geschieht über `.0` und `EntryId(x)`.

## 3. Interface

```rust
use std::sync::Arc;
use index_contract::EntryId;             // EntryId(pub u64)
use result_kinds::{Hit, TaskResult};
use scopes::{Scope, EntrySet};           // Scopes und Entry-Mengen
use extensions::Extensions;              // Erweiterbare Zusatzparameter

/// Verbindliche, opake Index-ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexId(pub u64);

/// Scoring-Verfahren. Höherer Score = ähnlicher.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Scoring {
    #[default] Cosine,
    DotProduct,
    NegEuclidean,   // -(L2-Distanz)
}

/// Genauigkeit der Kandidaten-Vektoren, die von der Source angefordert wird.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Precision { #[default] F32, F16, Int8 }

#[derive(Debug, Clone)]
pub struct SimilarityParams {
    pub index: IndexId,            // verpflichtend
    pub top_k: usize,              // Default 10
    pub scope: Scope,              // Default Scope::All
    pub scoring: Scoring,          // Default Cosine
    pub precision: Precision,      // Default F32
    pub extensions: Extensions,    // Default leer
}

impl SimilarityParams {
    /// Defaults: top_k=10, Scope::All, Cosine, F32, leere Extensions.
    pub fn new(index: IndexId) -> Self;
    pub fn with_top_k(self, k: usize) -> Self;
    pub fn with_scope(self, s: Scope) -> Self;
    pub fn with_scoring(self, s: Scoring) -> Self;
    pub fn with_precision(self, p: Precision) -> Self;
    pub fn with_extensions(self, e: Extensions) -> Self;
    /// Reranking: gleicher Index, gleiche Precision und Extensions wie `self`,
    /// scope = prev.narrowed_scope(), scoring = `scoring`, top_k = self.top_k.
    pub fn rerank(&self, prev: &SimilarityOutput, scoring: Scoring) -> SimilarityParams;
}

/// Ein Kandidat aus der Source.
#[derive(Debug, Clone)]
pub struct Candidate { pub id: EntryId, pub vector: Vec<f32>, pub stale: bool }

/// Von der äußeren Schicht implementiert. Löst den Scope auf.
pub trait CandidateSource: Send + Sync {
    /// Kandidaten von `index` innerhalb `scope`, Vektoren in `precision`
    /// (als f32 geliefert). Keine Vollständigkeitsgarantie.
    fn candidates(&self, index: IndexId, scope: &Scope, precision: Precision, extensions: &Extensions)
        -> Result<Vec<Candidate>, SourceError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceError(pub String);

/// Treffer nach außen: Entry, Score, stale.
#[derive(Debug, Clone, Copy)]
pub struct SimilarityHit { pub entry: EntryId, pub score: f32, pub stale: bool }

/// Ergebnis: eine Trefferliste je Query, in Query-Reihenfolge.
#[derive(Debug, Clone, Default)]
pub struct SimilarityOutput { pub per_query: Vec<Vec<SimilarityHit>> }

impl SimilarityOutput {
    /// Scope::Set mit der Vereinigung aller Treffer-IDs (stale eingeschlossen).
    pub fn narrowed_scope(&self) -> Scope;
    /// TaskResult::Hits: alle Treffer, Query-Reihenfolge, dann Rang; stale entfällt.
    pub fn into_task_result(self) -> TaskResult;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimilarityError {
    /// Ein Query-Vektor ist leer (Länge 0).
    EmptyQueryVector { query: usize },
    Source(SourceError),
}
impl std::fmt::Display for SimilarityError {}
impl std::error::Error for SimilarityError {}

/// Führt die Suche aus. Reine Funktion bis auf den Source-Aufruf.
pub fn run_similarity(
    source: &dyn CandidateSource,
    queries: &[Vec<f32>],
    params: &SimilarityParams,
) -> Result<SimilarityOutput, SimilarityError>;
```

Alle Typen sind `Send + Sync + 'static`. Dafür gibt es eine Compile-Time-Assertion.

## 4. Behavior and invariants

**Ablauf**
- `queries` ist leer: Ergebnis `Ok` mit leerem `per_query`. Die Source wird nicht aufgerufen.
- Sonst wird zuerst jede Query geprüft. Hat eine Query Länge 0, kommt `Err(EmptyQueryVector{query: index})`, ohne Source-Aufruf.
- Dann ruft `run_similarity` `source.candidates` genau einmal pro Aufruf auf. Alle Queries teilen sich die Kandidatenmenge. Dabei werden `params.index`, `&params.scope`, `params.precision` und `&params.extensions` übergeben.
- Ein Source-Fehler wird als `SimilarityError::Source` zurückgegeben.
- `per_query.len() == queries.len()`.

**Scoring** (f32)
- `Cosine` ist `dot / (|a|·|b|)`. Hat ein Vektor die Norm 0, ist der Score `0.0`.
- `DotProduct` ist das Skalarprodukt.
- `NegEuclidean` ist `-sqrt(Σ(a-b)²)`.
- Hat ein Kandidat eine andere Dimension als die Query, wird er für diese Query übersprungen. Das ist kein Fehler.

**Auswahl**
- Pro Query werden die Kandidaten nach Score absteigend sortiert. Bei Gleichstand gewinnt die kleinere `EntryId`. Die ersten `top_k` bleiben übrig.
- NaN-Scores werden hinter allen anderen Scores einsortiert (untereinander nach `EntryId`).
- `top_k == 0` ergibt je Query eine leere Liste.
- Gibt es weniger als `top_k` Kandidaten, werden alle geliefert. Das ist kein Fehler.
- Kommt dieselbe `EntryId` mehrfach in den Kandidaten vor, wird nur das erste Vorkommen bewertet.

**stale**
- `SimilarityHit.stale` ist das `stale` des Kandidaten, unverändert.
- Stale Treffer werden weder gefiltert noch als Fehler behandelt.
- Es gibt keine Garantie zu Aktualität, Vollständigkeit oder Qualität.

**Reranking und Folge-Scope**
- `narrowed_scope()` liefert `Scope::Set(EntrySet::from_ids(..))` mit den `.0`-Werten aller Treffer. Bei leerem Output ist das eine leere Menge.
- `rerank` ändert `self` nicht. Die übrigen Parameter bleiben gleich, nur `scope` und `scoring` ändern sich.
- Die Source wird beim Rerank mit dem verengten Scope aufgerufen. Das Modul prüft nicht, ob sie ihn einhält.

**Defaults**
- `new` setzt `top_k = 10`, `Scope::All`, `Scoring::Cosine`, `Precision::F32` und leere `Extensions`.

**Nebenläufigkeit**
- Es gibt keinen geteilten veränderlichen Zustand. Gleichzeitige Aufrufe sind sicher, solange die Source es ist.
- `run_similarity` ist deterministisch bei deterministischer Source.
- Es gibt keine Panics bei NaN, leeren Kandidaten oder `top_k` größer als die Kandidatenzahl.

**`into_task_result`**
- Ergebnis ist `TaskResult::Hits(Vec<Hit>)`, wobei `Hit{id: entry, score}` gilt.

## 5. Relied-on elements

- `index_contract::EntryId`: `#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct EntryId(pub u64);`. Es ist ein eigener Typ.
- `result_kinds`:
  - `pub struct Hit { pub id: EntryId, pub score: f32 }`.
  - `TaskResult` ist `#[non_exhaustive]` und hat die Variante `Hits(Vec<Hit>)`.
- `Scope` ist `#[non_exhaustive]`. Die Variante `Scope::Set(EntrySet)` und `Scope::All` werden genutzt. `Scope` ist `Clone + Debug`.
- `EntrySet::from_ids<I: IntoIterator<Item = u64>>(ids) -> EntrySet`. Im Scopes-Modul ist `EntryId = u64`.
- `Extensions` ist `Clone + Default + Send + Sync`. Beim Klonen werden die Werte geteilt, und die Werte werden nie interpretiert.