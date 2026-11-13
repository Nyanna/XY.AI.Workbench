# Spezifikation: Similarity-Report (Build-Order 64)

## 1. Task, restrictions and boundaries

Das Modul `similarity_report` macht aus einem Operation-Ergebnis (`ScoredHitResult`) einen Bericht. Der Bericht enthält die sortierte Top-k-Rangliste, Zähler und Kennzahlen.

**In scope**
- Ungültige Treffer verwerfen, sortieren, auf Top-k kürzen.
- Source-Links per Lookup ermitteln.
- Zähler berechnen: Kandidaten, Verworfene, stale.
- Kennzahlen berechnen: Statistik, Gap, Spread, z-Score, Softmax, Top-Masse, Entropie, Knee-Point.

**Out of scope**
- Entscheidungen, Thresholds, Bewertung der Kennzahlen und Folge-Tasks. Das macht eine äußere Schicht anhand der Fakten.
- Scoring, Index-Zugriff, Scheduling, Persistenz, Serialisierung, Task-Zustandsübergänge.
- Globaler Zustand und `unsafe`.
- Default-Werte für die Temperatur.

Abhängigkeiten sind nur `std`, `operation` und `similarity_task`. Task-Modell liefert nur das Vokabular (`TaskId` kommt aus `operation`).

## 2. Context

- Die `Similarity`-Operation liefert ein `ScoredHitResult` mit rohen, unsortierten Treffern. Dieses Modul rankt und fasst zusammen. Die Operation rankt nicht.
- Der Bericht trägt Task-ID und `SimilarityParams` (aus `similarity_task`), die den Auftrag beschreiben. Er enthält nur Fakten und Rohscores.
- Der Executor ruft die Funktion auf. Fehlen die Suchparameter (`None`), liefert sie `Err`, und der Task endet mit diesem Fehler. Dieses Modul ändert keinen Task-Zustand.
- `EntryId` ist hier `operation::EntryId` (`EntryId(pub u64)`), nicht `index_contract::EntryId`.

## 3. Interface

```rust
use operation::{EntryId, ScoredHitResult, TaskId};
use similarity_task::SimilarityParams;

/// Eingestellt vom Aufrufer. Kein Default.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReportOptions {
    /// Softmax-Temperatur. Gültig nur, wenn endlich und > 0.
    pub temperature: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReportHit {
    pub entry_id: EntryId,
    pub score: f32,
    pub stale: bool,
    pub source_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Softmax {
    pub temperature: f32,
    /// Gleiche Reihenfolge wie `hits`. Summe ≈ 1.
    pub probabilities: Vec<f64>,
    /// == probabilities[0]
    pub top_probability: f64,
    /// Nats, >= 0.
    pub entropy: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Knee {
    /// 0-basierter Rang in `hits`.
    pub rank: usize,
    pub entry_id: EntryId,
    /// Vertikaler Abstand unterhalb der Verbindungslinie, > 0.
    pub depth: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Metrics {
    pub count: usize,
    pub best: f32,
    pub worst: f32,
    pub mean: f64,
    /// Grundgesamtheit (Division durch n).
    pub std_dev: f64,
    pub gap: Option<f64>,
    pub spread: Option<f64>,
    pub z_score: Option<f64>,
    pub softmax: Option<Softmax>,
    pub knee: Option<Knee>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimilarityReport {
    pub task_id: TaskId,
    pub params: SimilarityParams,
    /// = input.scored_count
    pub candidate_count: usize,
    /// = input.dropped_count + lokal verworfene Treffer
    pub dropped_count: usize,
    /// stale unter den zurückgegebenen Treffern
    pub stale_count: usize,
    pub hits: Vec<ReportHit>,
    /// None bei leeren `hits`
    pub metrics: Option<Metrics>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportError { MissingSearchParams }
impl std::fmt::Display for ReportError {}
impl std::error::Error for ReportError {}

pub fn build_report(
    task_id: TaskId,
    params: Option<&SimilarityParams>,
    input: &ScoredHitResult,
    options: ReportOptions,
    source_link: &dyn Fn(EntryId) -> Option<String>,
) -> Result<SimilarityReport, ReportError>;

/// Reine Kennzahlenberechnung. `scores` ist absteigend sortiert und endlich.
/// `ids` hat dieselbe Länge wie `scores`. Leer ergibt None.
pub fn compute_metrics(scores: &[f32], ids: &[EntryId], temperature: f32) -> Option<Metrics>;
```

Alle Typen sind `Send + Sync + 'static`. Dafür gibt es eine Compile-Time-Assertion.

## 4. Behavior and invariants

**Parameter und Fehler**
- `params == None` ergibt `Err(MissingSearchParams)`. Der Fehler kommt vor jeder anderen Arbeit, und `source_link` wird nicht aufgerufen.
- Sonst ist das Ergebnis immer `Ok`. Es gibt keine Panics, auch nicht bei NaN, Inf, leerem Input oder `top_k` größer als die Trefferzahl.
- `report.params` ist ein Klon von `*params`. `report.task_id == task_id`.

**Verwerfen**
- Ein Treffer ist ungültig, wenn `!score.is_finite()` gilt (NaN, ±Inf). Er wird entfernt und zählt als verworfen.
- `dropped_count = input.dropped_count + (Anzahl lokal ungültiger Treffer)`, saturierend.
- `candidate_count = input.scored_count`, unverändert. Es gibt keine Prüfung gegen `hits.len()`.

**Sortieren und Kürzen**
- Der Input darf unsortiert sein und mehr Treffer als `top_k` enthalten.
- Die gültigen Treffer werden nach Score absteigend sortiert. Bei Gleichstand steht die niedrigere `EntryId` vorn.
- Danach werden sie auf `params.top_k` gekürzt.
- `top_k == 0` ergibt leere `hits` und `metrics == None`.
- Die Treffer werden nicht dedupliziert.
- Der Input wird nicht verändert.

**Zähler und Link**
- `stale_count` ist die Anzahl der Einträge in `hits` mit `stale == true`. Das Flag wird je Treffer unverändert übernommen. Verworfene oder abgeschnittene Treffer zählen nicht.
- `source_link` wird genau einmal pro zurückgegebenem Treffer aufgerufen, in Rangfolge. Für abgeschnittene oder verworfene Treffer wird es nicht aufgerufen.
- Das Ergebnis des Lookups ist `ReportHit.source_link`, also `None`, wenn der Link fehlt. Ein `source_link` im Input-`ScoredHit` wird ignoriert.

**Kennzahlen**
- Sie beziehen sich nur auf die zurückgegebenen `hits`, mit deren Scores als Grundlage. `metrics` ist `None`, wenn `hits` leer ist.
- Rechnung in f64. Die Scores werden als `f64::from(f32)` übernommen.
- `count = n`, `best = scores[0]`, `worst = scores[n-1]`, `mean = Σ/n`, `std_dev = sqrt(Σ(s-mean)²/n)`.
- `gap = s0 - s1`. Das gibt es nur bei n ≥ 2, sonst `None`.
- `spread = s0 - s(n-1)`. Das gibt es nur bei n ≥ 2, sonst `None`.
- `z_score = (s0 - mean)/std_dev`. Bei `std_dev < 1e-9` ist er `None`.
- **Softmax**
  - `p_i = exp((s_i - s0)/T) / Σ_j exp((s_j - s0)/T)`. Das ist stabil, weil auf den besten Score bezogen wird.
  - Ist `T` nicht endlich oder `T <= 0`, ist das gesamte `softmax` `None`. Das gilt auch für Top-Masse und Entropie.
  - `top_probability = p_0`.
  - `entropy = -Σ p_i·ln p_i`. Terme mit `p_i == 0` werden übersprungen. Das Ergebnis ist mindestens 0.
  - Bei n = 1 gilt p = [1.0] und Entropie 0.
- **Knee-Point**
  - Er braucht n ≥ 3, sonst `None`.
  - Die Linie geht von (0, s0) nach (n-1, s(n-1)). Für Rang i gilt `line_i = s0 + (s(n-1)-s0)·i/(n-1)`, und die Tiefe ist `depth_i = line_i - s_i`.
  - Berücksichtigt werden die Ränge 1..n-2 mit `depth_i > 0`. Das Maximum gewinnt, bei Gleichstand der frühere Rang.
  - Ist keine Tiefe > 0 (flache Kurve oder kein Punkt unterhalb der Linie), ist er `None`.
- Es werden keine Schwellen angewendet und nichts bewertet.

**Nebenläufigkeit**
- Es gibt keinen geteilten veränderlichen Zustand. Die Funktionen sind rein, bis auf den `source_link`-Aufruf.
- Gleichzeitige Aufrufe sind sicher, soweit der Lookup es ist.
- Bei gleichem Input und deterministischem Lookup ist das Ergebnis deterministisch.

## 5. Relied-on elements

Von `operation`:
- `EntryId(pub u64)` und `TaskId(pub u64)`. Beide sind `Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord`.
- `ScoredHit { pub entry_id: EntryId, pub score: f32, pub stale: bool, pub source_link: Option<String> }`.
- `ScoredHitResult { pub scored_count: usize, pub dropped_count: usize, pub hits: Vec<ScoredHit> }`.
  - `scored_count` zählt die tatsächlich gescorten Entries im aufgelösten Scope.
  - `dropped_count` zählt die bei der Auflösung verworfenen Treffer, etwa weil der Entry entfernt wurde.
  - `hits` sind die verbleibenden Treffer, unsortiert und ungekürzt. Die Struktur rankt nicht.

Von `similarity_task`:
- `SimilarityParams` ist `Debug + Clone`. Sie hat das Feld `pub top_k: usize` (Default 10). Die übrigen Felder sind `index`, `scope`, `scoring`, `precision` und `extensions`. Sie werden unverändert in den Bericht übernommen.
- Höherer Score bedeutet ähnlicher.

Von Task-Modell: Ein Task endet mit einem Fehler, wenn der Executor `Err(ReportError)` als `Failed(E)` abschließt. Dieses Modul hängt nicht von `Task` ab.