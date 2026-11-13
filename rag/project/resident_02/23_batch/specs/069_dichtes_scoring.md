# Spezifikation: Dichtes Scoring (Build-Order 69)

Crate-Modul `dense_scoring`.

## 1. Task, restrictions and boundaries

**Aufgabe.** Dichtes Scoring von Queries mit genau einem Token gegen Slots mit genau einem Vektor. Das Modul bietet:
- Cosine Similarity im Modus `Dot` oder `Exact`.
- Ablehnung leerer Query-Embeddings.
- Einen Score je Query und Slot in Slot-Reihenfolge.
- Top-k-Treffer je Query.
- Scoring einer Query gegen die Centroids der Summary-Struktur eines Index.

**Grenzen**
- Das Modul ist zustandslos und synchron. Es hat keine Threads, keine I/O, keine Device-Zugriffe, kein `unsafe` und keine Fremdcrates.
- Es führt **keine** L2-Normalisierung aus. Die Normalisierung geschieht einmal am Ende des Encoders. `cpu_compute::l2_normalize` wird nicht aufgerufen.
- Es berechnet keine Late-Interaction-Scores und keine Multi-Token-Queries. Das Modul definiert keine Typen der Abhängigkeiten neu.
- Es startet keine Kernel-Batches. `kernel_batch` wird nicht verwendet.
- Es führt keine Slot-Verwaltung durch. Weder `SlotStates` noch `SlotAssignment` werden verwendet. Der Aufrufer reicht nur gültige Slots als Slices durch.
- Es erstellt den Similarity-Report nicht. Es liefert nur die Scores, aus denen der Report seine Kennzahlen ableitet.
- Die Auswahl der nächsten Zellen ist Sache des Managers. Das Modul liefert nur Scores und Top-k.

## 2. Context

- **Aufrufer.** Retrieval bzw. der Manager (Query gegen Slots und gegen Centroids), der Similarity-Report (verbraucht Scores).
- **Vorgelagert.** Der Encoder liefert L2-normalisierte `f32`-Vektoren. Dann ist Dot-Product gleich Cosine.
- **Abweichende Längen.** Bei FP16-Speicherung oder aus anderen Gründen können Vektoren von der Länge 1 abweichen. Dafür dient der Modus `Exact`.
- **Slots.** Der Aufrufer hält die Slot-Vektoren als `&[f32]` der Länge `dim`. `Slot` kommt aus `kernel_layout` (`Slot(pub u32)`), wie in „Slot-Layout und Segment-View“.
- **Centroids.** `Index::summary(node)` liefert `Summary.centroid` als Bytes im `dtype()` des Index.
- **Precisions.** Auf der CPU gibt es einen Scoring-Kernel nur für F32. F16-Centroids werden hier nach F32 gewandelt, und das Rechnen erfolgt in F32 („FP16-Speicherung, FP32-Compute“).

## 3. Interface

```rust
use cpu_compute::dot;                       // intern
use index_contract::{Index, IndexError, NodeId};
use kernel_layout::Slot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimilarityMode { Dot, Exact }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeSource { Query(usize), Slot(usize) }   // Slot(i): i-ter Slot bzw. i-ter Knoten

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DenseError {
    InvalidDim,                                          // dim == 0
    /// Länge != dim oder Länge 0 (leerer Slot/Centroid/Query-Slice)
    Shape { what: ShapeSource, len: usize, dim: usize },
    /// norm² nicht endlich oder < 1e-18; "leeres Embedding"
    EmptyEmbedding { query_index: usize },
    UnsupportedDType(layout::DType),                     // Centroid nicht F32/F16
    Index(IndexError),                                   // Fehler von Index::summary
}
impl std::fmt::Display for DenseError {}   // EmptyEmbedding: Text "leeres Embedding" + Query-Index
impl std::error::Error for DenseError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit { pub slot: Slot, pub score: f32 }

/// Row-major: Index = query * slot_count + slot.
#[derive(Debug, Clone, PartialEq)]
pub struct DenseScores { /* queries, slots, data: Vec<f32> */ }
impl DenseScores {
    pub fn query_count(&self) -> usize;
    pub fn slot_count(&self) -> usize;
    pub fn row(&self, query: usize) -> Option<&[f32]>;       // Länge slot_count, Slot-Reihenfolge
    pub fn as_flat(&self) -> &[f32];
    pub fn top_k(&self, query: usize, k: usize) -> Option<Vec<Hit>>;   // None, wenn query >= query_count
}

/// Score je Query und Slot.
pub fn score(queries: &[&[f32]], slots: &[&[f32]], dim: usize, mode: SimilarityMode)
    -> Result<DenseScores, DenseError>;

/// Top-k einer Score-Zeile.
pub fn top_k(scores: &[f32], k: usize) -> Vec<Hit>;

/// Query(s) gegen die Centroids der angegebenen Knoten (Summary-Struktur).
pub fn score_centroids(index: &dyn Index, nodes: &[NodeId], queries: &[&[f32]],
                       dim: usize, mode: SimilarityMode)
    -> Result<CentroidScores, DenseError>;

#[derive(Debug, Clone, PartialEq)]
pub struct CentroidScores { pub nodes: Vec<NodeId>, pub scores: DenseScores }  // Spalte i entspricht nodes[i]
```

Alle Funktionen sind frei und reentrant. `DenseScores`, `DenseError`, `Hit` und `CentroidScores` sind `Send + Sync`.

## 4. Behavior and invariants

**Score-Formeln** (`d = cpu_compute::dot`, mit seiner festen Summationsreihenfolge)
- `Dot`: `score = d(q, s)`. Es gibt weder eine Norm noch eine Division.
- `Exact`: `denom = sqrt(d(q,q)) * sqrt(d(s,s))` in `f32`.
  - Ist `!(denom > 0.0)` oder `!denom.is_finite()`, dann ist `score = 0.0`.
  - Sonst ist `score = d(q,s) / denom`.
  - Der Slot-Norm wird pro Slot einmal berechnet, der Query-Norm pro Query einmal.
  - Ein Nullvektor als Slot ergibt in `Exact` den Score `0.0`, und kein Fehler entsteht.
- Das Ergebnis ist deterministisch und unabhängig von Thread oder Reihenfolge.
- Nicht endliche Dot-Werte werden unverändert durchgereicht (`Dot`).

**Prüfreihenfolge in `score`.** Alle Prüfungen laufen vor jeder Score-Berechnung. Der erste Fehler bricht ab, und es gibt kein Teilergebnis.
1. `dim == 0` → `InvalidDim`.
2. Jede Query `i` mit `len != dim` (auch `len == 0`) → `Shape { Query(i), len, dim }`, niedrigster Index zuerst.
3. Jede Query `i` mit `n2 = d(q,q)`, `!n2.is_finite() || n2 < 1e-18` → `EmptyEmbedding { query_index: i }`, niedrigster Index zuerst. Das gilt in **beiden** Modi und für **alle** Queries, bevor irgendein Slot bewertet wird.
4. Jeder Slot `j` mit `len != dim` (auch leer) → `Shape { Slot(j), len, dim }`, niedrigster Index zuerst.

**Form**
- Jede Query ist genau ein Token. Ein Query-Slice hat die Länge `dim`.
- Das Ergebnis hat `queries.len()` Zeilen mit je `slots.len()` Scores, in Slot-Reihenfolge.
- Leere `queries` oder leere `slots` ergeben `Ok` mit einer leeren Matrix (entsprechend 0 Zeilen bzw. 0 Spalten).
- Slots und Queries werden nie verändert oder normalisiert.

**Top-k** (`top_k`, `DenseScores::top_k`, in beiden Modi gleich)
- Sortierung absteigend nach `score`. Bei Gleichstand gilt der kleinere Slot-Index zuerst. `Hit.slot = Slot(index in der Zeile)`.
- NaN-Scores werden übersprungen und nie zurückgegeben.
- `k == 0` ergibt `[]`. Die Länge ist `min(k, Anzahl nicht-NaN-Scores)`.
- Die Eingabe wird nicht verändert. Die Laufzeit ist höchstens `O(n log k)` (z. B. Heap), und es gibt keine Panics.

**Centroids** (`score_centroids`)
- Für jeden Knoten wird in Reihenfolge `index.summary(node)` aufgerufen. Fehler werden als `Index(e)` durchgereicht, z. B. `Unsupported(Centroid)` oder `NotFound`.
- Die Centroid-Bytes sind im `index.dtype()` kodiert, little-endian.
  - `F32` wird direkt gelesen.
  - `F16` (IEEE binary16) wird verlustfrei nach `f32` gewandelt, lokal implementiert.
  - Jeder andere Typ ergibt `UnsupportedDType(dtype)`.
- Die Anzahl der Elemente muss `dim` sein. Sonst, auch bei leerem Centroid, ergibt das `Shape { Slot(i), len: Elementzahl, dim }`. Eine Byte-Länge, die kein Vielfaches der Elementgröße ist, zählt ebenfalls als Shape-Fehler.
- Danach gelten dieselbe Prüfreihenfolge und dieselben Score-Formeln wie bei `score`.
  - Die Query-Prüfungen (Schritte 1–3) laufen **vor** den `summary`-Aufrufen.
  - Die `summary`-Aufrufe sind die einzige Interaktion mit dem Index. Es gibt keine Schreibzugriffe.
- Leeres `nodes` ergibt `Ok` mit 0 Spalten.
- `CentroidScores.scores.top_k(q, k)` liefert `Hit.slot` als Position in `nodes`.

**Nebenläufigkeit.** Es gibt keinen globalen oder geteilten Zustand. Beliebig viele Threads dürfen gleichzeitig aufrufen. `score_centroids` ist nur so nebenläufig wie `Index::summary`. Das Modul hält keine Sperren und löst keine Panics aus. Fehler laufen ausschließlich über `DenseError`.

## 5. Relied-on elements

- `cpu_compute::dot(a: &[f32], b: &[f32]) -> f32`.
  - Es entsteht ein Panic bei ungleichen Längen, deshalb wird vorher auf `dim` geprüft.
  - Es verwendet 8 Teilsummen mit fester Reduktion `((l0+l1)+(l2+l3))+((l4+l5)+(l6+l7))`, dazu `+ Rest`.
  - Leere Slices ergeben `0.0`.
- `index_contract`
  - `Index` mit `dtype() -> DType` und `summary(&self, NodeId) -> Result<Summary, IndexError>`. Default ist `Err(Unsupported(Centroid))`.
  - `Summary { node, centroid: SharedBytes (Deref<[u8]>, Vektor im dtype() des Index), count, stamp }`.
  - `NodeId(pub u64)` (`Copy`) und `IndexError`.
- `layout::DType { F32, F16, BF16, U8, U32, I64 }` mit `size_in_bytes()`.
- `kernel_layout::Slot(pub u32)`.
- Unterstützte Precisions: `has_scoring_kernel(Cpu, F32) == true`. Auf der CPU gibt es keinen F16-Scoring-Kernel. F16 wird hier nur als Speicherformat gelesen und in F32 gerechnet. BF16 ist nie ein Compute-Pfad.