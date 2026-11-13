# Spezifikation: Kalibrierungslauf (Build-Order 38)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Rust-Modul `calibration_run`. Es kalibriert die Hardware einer Subengine per Benchmark für ein Ziel aus Subengine und Model-ID. Es führt diese Schritte aus:
1. Kombinationen aus dem Capability-Set aufzählen.
2. Die Kombinationen interleaved messen, ungültige verwerfen, per Achsenstufe prunen und die Kandidaten im Verifikationsschritt messen.
3. Die Pareto-Front über Zeit und Abweichung bilden.
4. Einen Report erzeugen.

**Einschränkungen:**
- Das Modul führt selbst keine Kernel, Tokenisierung oder Hardware-Zugriffe aus. Messungen laufen ausschließlich über die injizierten Traits `CalibrationBackend` und `Statistics`.
- Das Modul berechnet weder Streuungsmaß noch Schätzer selbst. Beides liefert `Statistics`.
- Das Modul macht kein I/O außer `std::thread::sleep` für die Backoff-Pause und Zeitmessung über `std::time::Instant`.
- Es gibt keine globalen Zustände und keine Fremd-Crates.
- Der Lauf ist sequenziell. Es gibt keine internen Threads.
- Das Modul erfindet keine Persistenz und keine Auswahl über die Front hinaus.

## 2. Kontext
- `Capability-Modell` liefert `CapabilitySet`, `Axis` und `Value`. Das Set beschreibt, was die Subengine kann.
- `Modellbeschreibung` liefert `ModelDescriptor`. Das Modul nutzt `model_id` und `max_seq_len` (obere Sequenzlänge) und ruft `validate()` auf.
- Ein Aufrufer, zum Beispiel die Facade, reicht Backend, Statistik, Konfiguration, Set und Descriptor herein. Er wertet den Report aus.
- Das Modul gehört keinem Tier. Es erzeugt nur den Report.

## 3. Interface
```rust
use std::collections::BTreeMap;
use std::time::Duration;
use capability::{Axis, Value, CapabilitySet};
use model_descriptor::ModelDescriptor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target { pub subengine: String, pub model_id: String }

/// Eine Belegung Achse -> Wert. Geordnet nach Achse.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Combination(pub BTreeMap<Axis, Value>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispersionKind { Cv, RangeOverMedian }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstimatorKind { Median, Minimum }

#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationConfig {
    pub max_deviation: f64,             // 0.05
    pub min_repeats: usize,             // 3; Werte < 3 werden auf 3 angehoben
    pub time_window: Duration,          // 10 ms
    pub stability_threshold: f64,       // 0.05
    pub chunk_budget: Duration,         // 5 ms
    pub backoff: Duration,              // 5 ms
    pub dispersion: DispersionKind,     // Cv
    pub estimator: EstimatorKind,       // Median
    pub max_attempts: usize,            // 40 (harte Obergrenze je Kombination)
    pub max_duration: Option<Duration>, // Some(30 s) (je Kombination, ab erstem Versuch)
    pub total_budget: Option<Duration>, // Some(600 s)
    pub dominance_margin: f64,          // 0.10
    pub separable_axes: Vec<Axis>,      // [batch_size, tile_size, thread_count]
    pub full_product: bool,             // false
    pub seq_start: usize,               // 16
    pub seq_factor: usize,              // 4
}
impl Default for CalibrationConfig { /* Werte wie oben */ }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeError {
    Failed(String),      // Lauf gescheitert -> verwerfen
    Rejected(String),    // sonstiger Verwerfungsgrund
    Interrupted,         // löst Backoff aus, zählt als Versuch
    Unavailable,         // Device nicht verfügbar
    Fatal(String),       // bricht den Lauf ab
}

/// Injizierter Messzugang. Muss Send + Sync sein.
pub trait CalibrationBackend: Send + Sync {
    /// Abweichung der Kombination gegen die Referenz über alle seq_lens (billig, einmal je Kombination).
    fn deviation(&self, c: &Combination, seq_lens: &[usize]) -> Result<f64, ProbeError>;
    /// Eine Zeitprobe (Dauer eines Laufs über alle seq_lens). Das Backend misst mindestens `window`,
    /// in Chunks von höchstens `chunk_budget`, und liefert die Dauer je Lauf.
    fn time(&self, c: &Combination, seq_lens: &[usize], window: Duration, chunk_budget: Duration)
        -> Result<Duration, ProbeError>;
}

/// Injizierte Statistik; Eingaben in Sekunden bzw. Verhältnisse. None = nicht berechenbar.
pub trait Statistics: Send + Sync {
    fn dispersion(&self, samples: &[f64], kind: DispersionKind) -> Option<f64>;
    fn estimate(&self, samples: &[f64], kind: EstimatorKind) -> Option<f64>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage { Axis, Verification, Full }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscardReason { Failed(String), NonFinite, ExceedsMax, Other(String) }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipReason { Budget, DeviceUnavailable }

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Dominated,
    Discarded(DiscardReason),
    Skipped(SkipReason),
    Measured { time: Duration, deviation: f64, stable: bool, samples: usize, uncertainty: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReportEntry { pub stage: Stage, pub combination: Combination, pub outcome: Outcome }

#[derive(Debug, Clone, PartialEq)]
pub struct FrontPoint { pub combination: Combination, pub time: Duration, pub deviation: f64 }

#[derive(Debug, Clone, PartialEq)]
pub struct BudgetStatus { pub limit: Option<Duration>, pub elapsed: Duration, pub exhausted: bool }

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Counters {
    pub measured: usize, pub stable: usize, pub unstable: usize, pub dominated: usize,
    pub discarded: usize, pub skipped: usize, pub attempts: usize, pub interruptions: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Summary { pub front: Vec<FrontPoint>, pub candidate_count: usize }

#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationReport {
    pub target: Target,
    pub entries: Vec<ReportEntry>,
    pub product_size: usize,     // Größe des vollen kartesischen Produkts aller Achsen
    pub budget: BudgetStatus,
    pub runtime: Duration,
    pub summary: Summary,
    pub counters: Counters,
    pub log: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibrationError {
    Capability,                     // leeres Capability-Set
    Model(String),                  // ModelDescriptor::validate() schlug fehl (reason)
    TargetMismatch,                 // target.model_id != model.model_id
    Backend(String),                // ProbeError::Fatal
}
impl std::fmt::Display for CalibrationError {}
impl std::error::Error for CalibrationError {}

pub fn sequence_lengths(max_seq_len: usize, start: usize, factor: usize) -> Vec<usize>;

pub fn run_calibration(
    target: Target,
    capabilities: &CapabilitySet,
    model: &ModelDescriptor,
    backend: &dyn CalibrationBackend,
    stats: &dyn Statistics,
    config: &CalibrationConfig,
) -> Result<CalibrationReport, CalibrationError>;
```
Alle öffentlichen Datentypen sind `Send + Sync + 'static`.

## 4. Behavior und Invarianten

### Vorbereitung
- Ablauf: `config.min_repeats` wird auf `max(3, …)` angehoben. Danach wird `model.validate()` geprüft, bei Fehler `Model(reason)`. Danach `target.model_id == model.model_id`, sonst `TargetMismatch`. `capabilities.is_empty()` führt zu `Err(Capability)`. Alle Prüfungen laufen vor der ersten Messung.
- Leere Achsen werden ignoriert. Das Set kann per Invariante keine haben. Achsen in `separable_axes`, die im Set fehlen, werden ignoriert.
- Wertereihenfolge je Achse: zuerst Werte, die vollständig als `u64` parsen, numerisch aufsteigend. Danach alle übrigen alphabetisch (Byte-Reihenfolge). Der „kleinste Wert" einer Achse ist der erste in dieser Reihenfolge.
- `sequence_lengths(max, start, factor)`:
  - Es erzeugt `start, start·factor, …` solange der Wert `≤ max` ist.
  - Danach hängt es `max` an, falls es nicht schon letzter Wert ist.
  - Ist `max < start`, ist das Ergebnis `[max]`.
  - Default bei `max=512`: `[16,64,256,512]`.
  - Der Lauf nutzt `sequence_lengths(model.max_seq_len, config.seq_start, config.seq_factor)`.
- Kernachsen sind alle Achsen im Set außerhalb von `separable_axes`. Kernkombinationen sind das Produkt der Kernachsen in Achsen- und Werte-Reihenfolge. Ohne Kernachsen gibt es genau eine leere Kernkombination.
- `product_size` ist das Produkt aller Werteanzahlen aller Achsen im Set.
- Der Gesamtbudget-Zeitgeber startet beim Eintritt in `run_calibration`. `runtime` ist die Gesamtdauer des Laufs.
- Budget gilt als erschöpft, sobald `total_budget` gesetzt und `elapsed >= total_budget` ist. Der Status wird im Report vermerkt (`exhausted`).

### Modi
- Normalmodus (`full_product=false`):
  1. Achsenstufe über alle Kernkombinationen, Einträge mit `Stage::Axis`.
  2. Danach der Verifikationsschritt: eine Gruppenmessung über alle deduplizierten Kandidaten, Einträge mit `Stage::Verification`. Die Reihenfolge der Kandidaten ist Kernkombination, dann Achsenwerte.
  3. Die Front stammt nur aus den Verifikationseinträgen.
- Vollmodus: Eine Gruppenmessung über das ganze Produkt in der Reihenfolge Achsen/Werte, `Stage::Full`. Es gibt keine Achsenstufe. Die Front stammt aus diesen Einträgen.

### Achsenstufe
- Das Budget wird vor jeder Kernkombination geprüft. Ist es erschöpft, erhalten diese und alle weiteren Kernkombinationen keine Messung. Je Kombination der Gruppe wird ein `Skipped(Budget)`-Eintrag erzeugt. Der Verifikationsschritt läuft danach ebenfalls mit `Skipped(Budget)` für alle Kandidaten, soweit welche existieren.
- Je Kernkombination und je vorhandener separierbarer Achse wird jeder Wert dieser Achse einzeln variiert. Die anderen separierbaren Achsen stehen auf ihrem kleinsten Wert. Die Kombinationen aller Achsen einer Kernkombination bilden eine Gruppe. Die Kombination mit lauter kleinsten Werten kommt nur einmal vor (Deduplizierung nach `Combination`). Die Gruppe wird mit einer Gruppenmessung gemessen.
- Gewinner einer Achse sind die Pareto-optimalen Werte über `Measured`-Ergebnisse (Zeit, Abweichung): ein Wert gewinnt, wenn keine andere Variation mit höchstens gleicher Zeit und höchstens gleicher Abweichung und in mindestens einem Wert strikt besser existiert. Einwertige Achsen gewinnen automatisch, ohne dass ihr Ergebnis nötig ist. Dominated, verworfene und übersprungene Variationen können nicht gewinnen.
- Hat eine Achse keinen Gewinner, wird die Kernkombination übersprungen: keine Kandidaten, Logzeile.
- Kandidaten einer Kernkombination sind das Produkt der Gewinner je Achse (inklusive Kernwerte). Kandidaten werden dedupliziert, die erste Position bleibt.

### Gruppenmessung (Eingabe: geordnete Liste Kombinationen, Stage)
1. **Abweichungsphase** je Kombination in Reihenfolge, vor jeder: Budgetprüfung (erschöpft → `Skipped(Budget)`).
   - Aufruf `backend.deviation`.
   - `Err(Failed(m))` → `Discarded(Failed(m))`. `Err(Rejected(m))` → `Discarded(Other(m))`.
   - `Err(Unavailable)` → `Skipped(DeviceUnavailable)`.
   - `Err(Interrupted)` → Backoff-Pause, Wiederholung. Jeder Aufruf zählt als Versuch. Nach `max_attempts` Versuchen → `Discarded(Other("interrupted"))`.
   - `Err(Fatal(m))` → Lauf bricht mit `Backend(m)` ab, kein Report.
   - Ein nicht-endlicher Wert → `Discarded(NonFinite)`. Wert `> max_deviation` → `Discarded(ExceedsMax)`. Sonst ist die Kombination aktiv, ihre Abweichung ist fixiert.
2. **Zeitphase** über die aktiven Kombinationen in Runden (Rundenindex ab 0). Jede Runde ruft für jede noch aktive Kombination höchstens einmal `backend.time` auf. Gerade Runden laufen in Gruppenreihenfolge, ungerade Runden in umgekehrter Reihenfolge.
   - **Anchor** ist die erste aktive Kombination in Gruppenreihenfolge. Fällt sie in Runde 0 durch `Unavailable` aus, wird die nächste Anchor.
   - Jeder Aufruf zählt als Versuch (`attempts`, Zähler im Report). `Interrupted` zählt zusätzlich unter `interruptions`, liefert keine Probe und löst die Backoff-Pause aus. `Unavailable` ohne bisherige Probe → `Skipped(DeviceUnavailable)`. Mit Proben wird es wie `Interrupted` behandelt. `Fatal` → Abbruch wie oben.
   - Eine Probe in Sekunden (`f64`) wird gespeichert. Für Nicht-Anchor-Kombinationen mit positiver Anchor-Probe derselben Runde wird zusätzlich das Verhältnis `t / t_anchor` gespeichert. Ohne positive Anchor-Probe entsteht kein Verhältnis.
   - **Stabil** heißt: Reihe hat `≥ min_repeats` Werte und `stats.dispersion(reihe, config.dispersion) = Some(d)` mit `d ≤ stability_threshold`. Für Nicht-Anchor-Kombinationen ist die Reihe die Verhältnisreihe, für den Anchor die eigene Zeitreihe.
   - **Schätzer** (Zeit einer Kombination): `stats.estimate(eigene Proben, config.estimator)`.
   - Nach jeder Runde werden die aktiven Nicht-Anchor-Kombinationen, danach der Anchor, in dieser Reihenfolge geprüft.
3. **Beendigung einer Nicht-Anchor-Kombination** (Priorität in dieser Reihenfolge):
   - **Dominated:** Sie hat `≥ min_repeats` Proben, und es gibt eine andere nicht verworfene, nicht dominierte, nicht übersprungene Kombination der Gruppe mit `≥ min_repeats` Proben, höchstens gleicher Abweichung und Schätzer `<` Schätzer·(1 − `dominance_margin`). Der Anchor kann dominiert nicht werden und dominiert andere normal. Ergebnis `Dominated`.
   - **Stable:** ihre Verhältnisreihe ist stabil. Ergebnis `Measured{stable:true}`.
   - **Unstable an der Obergrenze:** `attempts_der_Kombination >= max_attempts` oder (`max_duration` gesetzt und seit ihrem ersten Versuch vergangen `>= max_duration`). Ergebnis ist der beste Zwischenstand (aktuelle Schätzung der vorhandenen Proben) mit `stable:false`.
   Mit null Proben an der Obergrenze: `Skipped(DeviceUnavailable)`, wenn nur `Unavailable`/`Interrupted` auftraten, sonst `Discarded(Other("no samples"))`.
4. **Beendigung des Anchors** erst, wenn alle Nicht-Anchor-Kombinationen beendet sind (oder keine existiert). Er endet als stable, wenn mindestens ein Mitbewerber `Measured{stable:true}` ist und kein Mitbewerber `Measured{stable:false}`, oder wenn seine eigene Reihe stabil ist. Sonst läuft er weiter bis zur Obergrenze (wie oben) und endet dann unstable.
5. **Messergebnis:** `Measured{ time, deviation, stable, samples, uncertainty }`.
   - `time` ist die Schätzung.
   - `samples` ist die Anzahl der Zeitproben.
   - `uncertainty` ist `stats.dispersion(eigene Proben, config.dispersion)`, ersatzweise `dominance_margin`, wenn nicht berechenbar.
6. **Budgetende** während der Zeitphase: Jede noch aktive Kombination mit `≥ 1` Probe wird `Measured{stable:false}` (Anchor eingeschlossen). Aktive ohne Probe werden `Skipped(Budget)`. Die Budgetprüfung läuft vor jedem Backend-Aufruf.
7. Jede Kombination der Gruppe liefert genau einen `ReportEntry` (Stage, Combination, Outcome), und zwar in Gruppenreihenfolge. Einträge stehen im Report in Laufreihenfolge.

### Front und Report
- Eingabe sind die `Measured`-Einträge der Stufe `Verification` bzw. `Full` (Normalmodus bzw. Vollmodus). Unstable zählen mit. Achsenstufen-Einträge gehen nie in die Front ein.
- Sortierung aufsteigend nach `time`, bei Gleichstand aufsteigend nach `deviation`. Ein Punkt gehört zur Front, wenn seine Abweichung strikt kleiner ist als die des zuletzt aufgenommenen Punkts. Der erste Punkt gehört immer dazu. Die Front hat so aufsteigende Zeit und streng fallende Abweichung.
- Jede Einordnung (Front / ausgeschlossen, mit Combination, Zeit, Abweichung) wird als Logzeile festgehalten. Weitere Logzeilen: übersprungene Kernkombinationen ohne Gewinner, Gruppenbeginn, Budgetende.
- `summary.candidate_count` ist die Zahl der Verifikations- bzw. Vollmodus-Kombinationen.
- `counters`: `measured`, `stable`, `unstable`, `dominated`, `discarded`, `skipped` zählen Einträge aller Stufen. `attempts` und `interruptions` zählen alle Backend-Zeitaufrufe bzw. Unterbrechungen. Die Abweichungsphase zählt Versuche mit.
- `budget`: `limit = config.total_budget`, `elapsed` zum Ende, `exhausted` ob irgendwann erschöpft.

### Fehler und Nebenläufigkeit
- Fehler: nur `CalibrationError`. Das Modul panikt nicht. Backend- und Statistik-Aufrufe erfolgen auf dem aufrufenden Thread, nacheinander.
- Backend und Statistik sind `Send + Sync`. Die Funktion hält keinen geteilten Zustand und ist reentrant. Mehrere Läufe dürfen parallel mit eigenen Argumenten laufen.

## 5. Relied-on elements
Aus Capability-Modell, verbatim:
```rust
pub struct Name(/* Arc<str> */);   // Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug
impl Name { pub fn new(s: impl AsRef<str>) -> Self; pub fn as_str(&self) -> &str; }
pub type Axis = Name;
pub type Value = Name;
pub struct CapabilitySet(/* BTreeMap<Axis, BTreeSet<Value>> */);
impl CapabilitySet {
    pub fn values(&self, axis: &Axis) -> Option<&std::collections::BTreeSet<Value>>;
    pub fn axes(&self) -> impl Iterator<Item = &Axis>;   // aufsteigend
    pub fn is_empty(&self) -> bool;
}
pub mod axes { pub fn batch_size() -> Axis; pub fn tile_size() -> Axis; pub fn thread_count() -> Axis; }
```
- Invariante: keine Achse hat eine leere Wertemenge. `Name` sortiert lexikographisch, nicht numerisch. Dieses Modul sortiert deshalb selbst. `Name`/`CapabilitySet` sind `Send + Sync`.
- Default von `separable_axes`: `[axes::batch_size(), axes::tile_size(), axes::thread_count()]`.

Aus Modellbeschreibung:
```rust
pub struct ModelDescriptor { pub model_id: String, pub max_seq_len: usize, /* weitere Felder */ }
impl ModelDescriptor { pub fn validate(&self) -> Result<(), ModelConfigError>; }
// ModelConfigError::Incompatible { reason: String }, Display: "inkompatible Modellkonfiguration: {reason}"
```
- `Model(String)` in `CalibrationError` trägt `reason` aus `Incompatible`.