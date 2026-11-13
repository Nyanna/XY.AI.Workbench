# Spezifikation: Messung (Build-Order 127), Crate-Modul `messung`

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe**
- Das Modul stellt ein austauschbares Mess-Interface (`Measure`) bereit. Es erfasst je Kombination die Abweichung gegen die FP32-Referenz und eine normierte Zeit-Probe.
- Es liefert zwei Implementierungen: `CpuMeasurer` und `GpuMeasurer`. Beide haben Warm-up, Unterbrechungsprüfung und austauschbare Clock.
- Es beschreibt Kombinationen (`Combination`) mit Textform und bildet das Messergebnis (`MeasurementResult`).

**Grenzen**
- Das Modul enthält keine Suche, keine Auswahl von Kombinationen und keine Persistenz. Es wählt kein `max_deviation`; der Aufrufer liefert es.
- Es enthält keine Kernel, keinen Device-Speicher, keinen Treiberzugriff und keine Crate-Abhängigkeiten außer `std` und den Abhängigkeitsmodulen. Der GPU-Zugriff läuft nur über `GpuKernels`, `Lane` und das Trait `GpuRig`.
- Es definiert keine Typen der Abhängigkeiten neu.
- Es enthält kein `unsafe` und keinen globalen Zustand. Alle Fehler werden zurückgegeben; es gibt keine Panics.

## 2. Kontext

- Ein Tuner oder Planer ruft `Measure` je Kombination auf: erst `deviation`, dann mehrfach `time_probe`. Er verdichtet die Samples mit `MeasurementResult::from_samples`.
- Der Aufrufer bricht eine Kombination ab, wenn `deviation` `Incompatible` liefert.
- `bench_workload` liefert Workload, FP32-Referenz, `query_count` und `Assessment`.
- `cpu_compute::scan_slots` ist der CPU-Scorer.
- `gpu_kernel::GpuKernels::dense_dot` ist der GPU-Scorer. Cosine entsteht, weil Workload-Vektoren L2-normalisiert sind (Dot = Cosine).
- `device_lane_event` liefert `Lane` und `Event`.
- `capability` liefert `Axis`, `Value`, `CapabilitySet` und die Achsennamen.
- Das GPU-Binding implementiert `GpuRig`. `GpuBackend` gibt keine Device-Adressen heraus, daher übernimmt das Rig Upload, Adressen, Cache-Flush, Event-Zeitmessung und Doorbell-Ack.
- Die Zielplattform ist auch ohne GPU testbar, mit Mock-`GpuRig`, Mock-`GpuLauncher`, Mock-`Clock` und Mock-`PendingTasks`.

## 3. Interface

```rust
use capability::{Axis, CapabilitySet, Value};
use bench_workload::{BenchError, Validity, Workload, WorkloadConfig};
use device_lane_event::{DeviceHandle, Event, Lane};
use gpu_kernel::{GpuKernelError, GpuKernels, KernelParams};
use layout::DType;
use std::{collections::BTreeMap, ops::Range, sync::{Arc, atomic::AtomicBool}};

// ---- Konstanten (Defaults) ----
pub const DEFAULT_WINDOW_NS: u64 = 10_000_000;        // 10 ms
pub const DEFAULT_CHUNK_BUDGET_NS: u64 = 5_000_000;   // 5 ms
pub const CPU_DOCS: usize = 4_000;
pub const GPU_FLUSH_BYTES: u64 = 64 * 1024 * 1024;    // 64 MiB
pub const GPU_WARMUP_NS: u64 = 100_000_000;           // 100 ms
pub const MIN_REPS: u32 = 1;
pub const MAX_REPS: u32 = 10_000;
pub const STABLE_MAX_SPREAD: f64 = 0.05;

// ---- Kombination ----
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombinationError {
    InvalidName(String),               // leer, enthält ',' oder '=', oder Rand-Whitespace
    DuplicateAxis(Axis),
    NotSingleValued { axis: Axis, count: usize },
    Parse(String),
}  // Display + Error + Send + Sync

/// Ein Wert je Achse + Sequenzlängen des Test-Sets (nur Verarbeitbarkeitsprüfung).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Combination { /* BTreeMap<Axis, Value> */ pub seq_lens: Vec<usize> }
impl Combination {
    pub fn new() -> Self;
    pub fn set(&mut self, axis: impl Into<Axis>, value: impl Into<Value>) -> Result<(), CombinationError>; // ersetzt vorhandenen Wert
    pub fn with(self, axis: impl Into<Axis>, value: impl Into<Value>) -> Result<Self, CombinationError>;
    pub fn with_seq_lens(self, lens: Vec<usize>) -> Self;
    pub fn value(&self, axis: &Axis) -> Option<&Value>;
    pub fn iter(&self) -> impl Iterator<Item = (&Axis, &Value)>;   // aufsteigend nach Achse
    /// Jede Achse muss genau einen Wert haben, sonst NotSingleValued. seq_lens bleibt leer.
    pub fn from_capability(set: &CapabilitySet) -> Result<Self, CombinationError>;
    pub fn to_capability(&self) -> CapabilitySet;
    pub fn to_text(&self) -> String;
}
impl std::fmt::Display for Combination {}            // == to_text()
impl std::str::FromStr for Combination { type Err = CombinationError; } // seq_lens leer

// ---- Austauschbare Quellen ----
pub trait Clock: Send + Sync { fn now_ns(&self) -> u64; }   // monoton, nicht fallend
pub struct MonotonicClock;                                    // std::time::Instant, Epoche = Erzeugung
impl Clock for MonotonicClock {}

/// Abfrage wartender produktiver Aufgaben, die das Device anfordern.
pub trait PendingTasks: Send + Sync { fn productive_waiting(&self) -> bool; }
pub struct NeverPending;                                      // immer false (Default)
impl PendingTasks for NeverPending {}
/// Gemeinsamer Schalter: Clone teilt den Zustand; Default = false.
#[derive(Clone, Default)]
pub struct PendingFlag(/* Arc<AtomicBool> */);
impl PendingFlag { pub fn new() -> Self; pub fn set(&self, waiting: bool); pub fn get(&self) -> bool; }
impl PendingTasks for PendingFlag {}

#[derive(Clone)]
pub struct Hooks { pub clock: Arc<dyn Clock>, pub pending: Arc<dyn PendingTasks> }
impl Default for Hooks {}                                     // MonotonicClock + NeverPending

#[derive(Debug, Clone)]
pub struct MeasureConfig {
    pub window_ns: u64,          // Default DEFAULT_WINDOW_NS, muss > 0
    pub chunk_budget_ns: u64,    // Default DEFAULT_CHUNK_BUDGET_NS, muss > 0
    pub max_deviation: f64,      // Pflichtangabe des Aufrufers (siehe MeasureConfig::new)
    pub top_k: usize,            // Default 5 (an DeviationConfig)
}
impl MeasureConfig { pub fn new(max_deviation: f64) -> Self; }

// ---- Ergebnisse ----
#[derive(Debug, Clone, PartialEq)]
pub enum DeviationOutcome {
    /// Validity::Valid; deviation = worst_deviation.
    Measured { deviation: f64, validity: Validity },
    /// "unverträglich": jede andere Validity oder nicht verarbeitbare Kombination.
    Incompatible { validity: Validity },
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeProbe {
    /// Normierte Zeit in ns je Arbeitseinheit (= je Query über das gesamte Dokument-Set).
    Time(f64),
    /// Produktive Aufgabe fordert das Device; Probe verworfen.
    Interrupted,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stability { Stable, Unstable }

#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementResult {
    pub robust_time_ns: f64,          // Median der Samples
    pub deviation: Option<f64>,       // None bei Incompatible
    pub validity: Validity,
    pub stability: Stability,
    pub samples: usize,
    pub rel_uncertainty: f64,
}
impl MeasurementResult {
    /// Err(NoSamples) bei leeren Samples oder nicht-endlichen/negativen Samples.
    pub fn from_samples(samples: &[f64], deviation: &DeviationOutcome) -> Result<Self, MeasureError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum MeasureError {
    Capability(String),               // keine GPU-Unterstützung
    Config(String),
    NoSamples,
    Bench(BenchError),
    Gpu(GpuKernelError),
    Rig(String),                      // Fehlertext des GpuRig
    Compute(String),                  // CPU-Scan/Pool-Fehler
}  // Display + Error + Send + Sync; From<BenchError>, From<GpuKernelError>

// ---- Mess-Interface ----
pub trait Measure: Send + Sync {
    /// Abweichung gegen die Referenz; Warm-up beim ersten Gebrauch.
    fn deviation(&self, combo: &Combination) -> Result<DeviationOutcome, MeasureError>;
    /// Eine Zeit-Probe (skaliert auf das Zeitfenster). Warm-up beim ersten Gebrauch.
    fn time_probe(&self, combo: &Combination) -> Result<TimeProbe, MeasureError>;
}

// ---- CPU ----
#[derive(Debug, Clone)]
pub struct CpuConfig { pub common: MeasureConfig, pub threads: usize /* Default 1, muss >= 1 */, pub seed: u64 }
impl CpuConfig { pub fn new(max_deviation: f64) -> Self; }
pub struct CpuMeasurer { /* Workload(4000 Docs), eigener Pool, Mutex-State */ }
impl CpuMeasurer {
    pub fn new(cfg: CpuConfig, hooks: Hooks) -> Result<Self, MeasureError>;   // baut Workload + Pool
}
impl Measure for CpuMeasurer {}

// ---- GPU ----
pub struct TimingMark { pub event: Event, pub id: u64 }

/// Vom GPU-Binding implementiert. Hält Dokumente in F32 UND F16 auf dem Device (aus dem Workload).
pub trait GpuRig: Send + Sync {
    /// Params für dense_dot: erste `n_queries` Workload-Queries (F32), Slot-Ring über `slots`,
    /// Output f32[n_queries * slots.len()] (Index q*slots.len()+s). dtype: F32 oder F16.
    fn kernel_params(&self, dtype: DType, n_queries: usize, slots: Range<u32>) -> Result<KernelParams, String>;
    /// Pollt die Doorbell, schließt abgeschlossene Batches (Gap::close) und liefert `Doorbell::acked()`.
    fn ack_completed(&self) -> Result<u64, String>;
    /// Schreibt `bytes` Cache-Flush-Arbeit auf den Stream der Lane; Event deckt sie ab.
    fn flush_cache(&self, lane: &mut Lane, bytes: u64) -> Result<Event, String>;
    /// Zeichnet eine Zeitmarke (Event) auf dem Stream auf.
    fn mark(&self, lane: &Lane) -> Result<TimingMark, String>;
    /// GPU-Zeit zwischen zwei Marken in ns; nur gültig, wenn `end.event` abgeschlossen ist.
    fn elapsed_ns(&self, start: &TimingMark, end: &TimingMark) -> Result<f64, String>;
    /// Scores des letzten Launches über den vollen Slot-Bereich, Länge n_queries * docs, Index q*docs+d.
    fn read_scores(&self, n_queries: usize) -> Result<Vec<f32>, String>;
}

pub struct GpuMeasurer<const DIM: usize> { /* Workload, kernels, rig, Mutex<Lane>, Plan-Map, Warm-up-Flag */ }
impl<const DIM: usize> GpuMeasurer<DIM> {
    /// Err(Capability), wenn lane.device() nicht Gpu ist oder lane.stream() None ist.
    /// Err(Config), wenn workload.config().dim != DIM, window/chunk == 0 oder max_deviation ungültig.
    pub fn new(workload: Workload, kernels: Arc<GpuKernels<DIM>>, rig: Arc<dyn GpuRig>,
               lane: Lane, cfg: MeasureConfig, hooks: Hooks) -> Result<Self, MeasureError>;
}
impl<const DIM: usize> Measure for GpuMeasurer<DIM> {}
```

`CpuMeasurer`, `GpuMeasurer<DIM>`, `Combination`, `MeasurementResult`, `MeasureError`, `Hooks` und `PendingFlag` sind `Send + Sync`.

## 4. Behavior and invariants

**Combination**
- Der Text besteht aus `achse=wert`-Paaren, aufsteigend nach Achse sortiert und mit `", "` getrennt. Eine leere Kombination ergibt `""`. `seq_lens` ist nicht Teil des Textes.
- `FromStr(to_text(c)) == c` gilt, wenn `c.seq_lens` leer ist. `FromStr` trimmt Whitespace um Paare. Es trennt am ersten `=`. Doppelte Achse ergibt `DuplicateAxis`. Ein Paar ohne `=` oder mit leerem Teil ergibt `Parse`.
- Gleiche Paare ergeben gleichen Text und gleichen Hash, unabhängig von der Einfügereihenfolge.

**Gemeinsame Vorprüfung** (in `deviation` und `time_probe`, vor jedem Warm-up oder Device-Zugriff). Der erste Fehler entscheidet.
1. Jede `seq_lens`-Zahl muss ≥ 1 sein, sonst `Incompatible`. Die Längen beeinflussen weder Zeit noch Abweichung.
2. Die Query-Zahl `n` ergibt sich aus `bench_workload::query_count_from_value(combo.value(batch_size).map(Name::as_str), max_queries)`. Ein `BatchError` führt zu `Incompatible`.
3. Die Precision-Prüfung steht unten je Messmittel.
4. Eine Vorprüfung, die scheitert, führt in `deviation` zu `Incompatible{validity}` mit `kind = Degenerate` und nicht leerem Text, der die Ursache nennt. In `time_probe` führt sie zu `Err(MeasureError::Config(text))`.
5. Achsen, die oben nicht genannt sind, werden ignoriert.

**deviation**
- Sie ruft `Workload::evaluate(n, Ok(scores) | Err(text), &DeviationConfig)` auf. Die `DeviationConfig` hat `max_deviation` und `top_k` aus `MeasureConfig`; ungültige Werte ergeben `Err(Bench(InvalidConfig))`.
- Die Referenz wird je `n` einmal berechnet und im Measurer zwischengespeichert. Das Ergebnis ist gleich dem frischen `reference_scores(n)`.
- `Validity::Valid` ergibt `Measured{deviation: worst_deviation, validity}`. Jede andere Validity ergibt `Incompatible{validity}`; das gilt auch für `ExceedsMax`, `NonFinite`, `Degenerate` und `ExecutionFailed`.
- Scheitert die Ausführung selbst (Launch-, Rig- oder Scan-Fehler), ist das Ergebnis `Incompatible{kind: ExecutionFailed}` mit dem Fehlertext. Das ist kein `Err`.
- Warm-up beim ersten Gebrauch läuft vor der Messung.

**time_probe, gemeinsam**
- Ein Sample ist `Gesamtzeit / (Wiederholungen · n)` in ns, als f64. Es ist endlich und > 0 (untere Grenze 1e-3 ns, falls die Clock 0 meldet).
- Die Clock kommt ausschließlich aus `Hooks.clock`. Die Unterbrechungsprüfung ruft `Hooks.pending.productive_waiting()` auf.
- Liefert die Prüfung `true`, endet die Probe sofort mit `Ok(TimeProbe::Interrupted)`. Bereits gemessene Zeiten werden verworfen. Das Warm-up ist davon unberührt, außer wo unten anders angegeben.
- Mit `NeverPending` wird nie unterbrochen.
- Es ist garantiert, dass jede Probe, die nicht unterbrochen wurde, ein `Time` liefert.

**CpuMeasurer**
- Workload: `WorkloadConfig::new(seed)` mit `docs = CPU_DOCS` (4000); die übrigen Felder sind Defaults.
- Precision: nur FP32. `scoring` und `operands`, sofern vorhanden, müssen `fp32` sein. Sonst ist das Ergebnis `Incompatible`, mit Text, der die Achse nennt.
- `threads == 0` ergibt `Config`.
- Der Pool gehört dem Measurer: `threads` Worker, erzeugt in `new`, beendet beim Drop. Er ist nicht global und nutzt keine Fremd-Pools.
- Die Berechnung teilt den Dokumentbereich `0..docs` in `threads` zusammenhängende Teilbereiche. Jeder Teilbereich wird mit `cpu_compute::scan_slots` über `source.slot_vectors(SegmentId(0), Slot(i))` gerechnet. Die Teilergebnisse werden zu einer query-major-Matrix `q*docs+d` zusammengesetzt.
- Das Ergebnis ist unabhängig von `threads` gleich dem seriellen Scan. Mit `threads == 1` läuft der Scan ohne Teilung.
- Warm-up: ein ungemessener Durchlauf (n Queries, volles Dokument-Set) beim ersten Gebrauch je Kombination (Schlüssel `to_text()` und `n`). Er läuft einmal je Kombination; auch ein zweites `time_probe` oder `deviation` auf derselben Kombination wärmt nicht erneut auf.
- Ablauf einer Probe:
  1. Unterbrechungsprüfung.
  2. Ein gemessener Durchlauf. Aus ihm ergibt sich `t1`.
  3. `passes = clamp(ceil(window_ns / t1), 1, MAX_REPS)`. Der erste Durchlauf zählt mit.
  4. Zwischen Durchläufen, nicht davor und nicht danach, folgt eine Unterbrechungsprüfung.
- Sample = Summe der Durchlaufzeiten / (passes · n). Die Zeit wird je Durchlauf gemessen.
- Der Chunk-Budget-Parameter gilt für CPU nur als Grenze der Prüfabstände; ein Durchlauf wird nicht geteilt.

**GpuMeasurer**
- Precision: `dtype` ergibt sich aus der Achse `scoring`, sonst aus `operands`. Erlaubt sind `fp32` (→ `DType::F32`) und `fp16` (→ `DType::F16`). Fehlen beide Achsen, ist es `fp32`. Sind beide vorhanden und verschieden, oder ist ein Wert nicht erlaubt, ist die Kombination `Incompatible`.
- Der Kernel ist `GpuKernels::<DIM>::dense_dot(&mut lane, dtype, &params, acked)`. Ein Cosine-Pfad ist es, weil die Vektoren normalisiert sind.
- Jede Launch-Folge hält die Ack-Regel von `GpuKernels` ein. Vor jedem Launch muss der Vorgänger abgeschlossen sein (`Event::wait`). Danach gilt `acked = rig.ack_completed()`. Der erste Launch nach dem Erzeugen braucht keinen Ack. Ein `AckBehind`-Fehler ist ein Bug des Aufrufers und wird als `Err(Gpu(..))` gemeldet.
- Zeitmessung: Jeder Launch liegt zwischen zwei `rig.mark`-Marken auf dem Stream. Seine Zeit ist `rig.elapsed_ns(start, end)`, nachdem `end.event.wait()` zurückkehrte. Die Messung per Wanduhr (Clock) wird nur für die Zeitfenster-Steuerung genutzt (Warm-up-Dauer).
- Cache-Flush: `rig.flush_cache(lane, GPU_FLUSH_BYTES)` vor jeder Probe, einmal je Probe. Das Event wird abgewartet, bevor gemessen wird. Der Flush zählt nicht zur Zeit.
- Warm-up: Beim ersten Gebrauch des Measurers, je Instanz nur einmal, wird Dense-Dot-Arbeit mindestens `GPU_WARMUP_NS` lang (gemessen mit der Clock) ausgeführt. Das geschieht in Gruppen, die je höchstens das Chunk-Budget füllen; zwischen Gruppen folgt eine Unterbrechungsprüfung. Wird unterbrochen, ist `time_probe` `Interrupted` und das Warm-up gilt als nicht erledigt (es beginnt später von vorn). Das Warm-up läuft mit der Precision der Kombination, die es auslöst. `deviation` führt das Warm-up ebenfalls aus; dort wird nicht unterbrochen.
- Wiederholungen (Plan je Kombination, Schlüssel `to_text()` samt `n`):
  1. Beim ersten `time_probe` nach dem Warm-up wird ein Kalibrierungsdurchlauf über das volle Dokument-Set mit Event-Zeit `t_pass` gemessen.
  2. `reps = clamp(ceil(window_ns / t_pass), MIN_REPS, MAX_REPS)`, also 1 bis 10 000.
  3. Der Plan (`reps`, `slots_per_launch`) wird gemerkt und in späteren Proben derselben Kombination wiederverwendet. Es gibt keine Neukalibrierung.
- Einzel-Launches im Chunk-Budget:
  - Eine Wiederholung ist ein Durchlauf über alle Dokumente.
  - Ist `t_pass <= chunk_budget_ns`, ist es ein Launch über `0..docs`.
  - Sonst wird der Durchlauf in Launches über Slot-Teilbereiche zerlegt. `slots_per_launch = max(1, floor(docs · chunk_budget_ns / t_pass))`, über `rig.kernel_params(dtype, n, range)`. Die Teilbereiche sind lückenlos und decken `0..docs` genau ab.
- Gruppen:
  - Gruppen fassen Wiederholungen so zusammen, dass die erwartete Gruppenzeit das Chunk-Budget nicht überschreitet: `group_reps = max(1, floor(chunk_budget_ns / t_pass))`.
  - Vor der ersten Gruppe und zwischen Gruppen (nicht nach der letzten) wird auf Unterbrechung geprüft.
- Sample = Summe aller Launch-Zeiten / (reps · n). Der Kalibrierungsdurchlauf zählt nicht dazu.
- Abweichung: ein Launch über `0..docs` mit `n` Queries und der gewählten Precision; danach `Event::wait`, `rig.read_scores(n)` und `Workload::evaluate`. Die Dokumente liegen in beiden Precisions auf dem Device; das Rig wählt per `dtype`.
- Fehler in Rig oder Kernel in `time_probe` ergeben `Err(Rig(text))` bzw. `Err(Gpu(e))`. Der Plan wird dabei nicht verändert.
- Capability-Error: `new` liefert `Err(Capability)`, wenn die Lane nicht zu einem GPU-Device gehört oder keinen Stream hat. Der Fehlertext nennt die Ursache.

**MeasurementResult::from_samples**
- `robust_time_ns` = Median (bei gerader Anzahl Mittel der zwei mittleren Werte).
- `spread = 1.4826 · MAD / median`, MAD = Median der absoluten Abweichungen vom Median. Bei `samples < 2` ist `spread = 0`.
- `rel_uncertainty = spread / sqrt(n)` für `n >= 2`, sonst `f64::INFINITY`.
- `stability = Stable`, wenn `n >= 2` und `spread <= STABLE_MAX_SPREAD`, sonst `Unstable`.
- `deviation` und `validity` stammen aus `DeviationOutcome` (`Measured` → `Some`, `Incompatible` → `None`).
- Leere Samples ergeben `Err(NoSamples)`.

**Nebenläufigkeit**
- Alle `Measure`-Aufrufe einer Instanz werden serialisiert (interner Mutex; Lane, Pool und Pläne sind exklusiv). Vergiftete Locks werden übernommen, ohne Panic.
- Verschiedene Instanzen sind unabhängig. `PendingFlag` und die Clock dürfen von anderen Threads gleichzeitig gesetzt bzw. gelesen werden.
- Ein Measurer hält keine Locks über Aufrufe von `pending` oder `clock`.
- Eine `GpuMeasurer`-Instanz ist an eine Lane und einen `GpuKernels` gebunden; mehrere Lanes pro `GpuKernels` sind unzulässig (siehe `GpuKernels`).

## 5. Relied-on elements

- `bench_workload`:
  - `Workload::{build(WorkloadConfig), config(), source(), segment() = SegmentId(0), queries(), reference_scores(n), evaluate(n, Result<&[f32], String>, &DeviationConfig) -> Result<Assessment, BenchError>}`.
  - `WorkloadConfig::new(seed)` mit öffentlichen Feldern `seed, docs, dim, max_queries, top_k`.
  - `DeviationConfig::new(max_deviation) -> Result<_, BenchError>`, `.with_top_k(k)`.
  - `Assessment{validity, rows, worst_deviation: Option<f64>}`.
  - `Validity{kind: ValidityKind, text}`. `ValidityKind::{Valid, NonFinite, Degenerate, ExceedsMax, ExecutionFailed}`; `text` ist immer nicht leer.
  - `query_count_from_value(Option<&str>, max_queries) -> Result<usize, BatchError>`. `None` ergibt `min(8, max)`. Nicht numerisch, 0, zu groß oder uneindeutig ist ein Fehler.
  - Die Workload-Vektoren sind L2-normalisiert; das Layout ist query-major, `q*docs+d`.
- `cpu_compute::scan_slots(queries: &[&[f32]], slots: &[Option<&[f32]>], range: Range<usize>, dim: usize) -> Result<Vec<f32>, CpuComputeError>`; Index `q*range.len() + (slot - range.start)`.
- `query_batch`:
  - `DocSource::slot_vectors(SegmentId, Slot) -> Option<&[f32]>`.
  - `SegmentId(pub u64)`, `Query::data()`, `QueryBatch::{len, get, data}`.
  - `kernel_layout::Slot(pub u32)`.
- `gpu_kernel`:
  - `GpuKernels<DIM>::{new, launched(), dense_dot(&self, &mut Lane, DType, &KernelParams, acked: u64) -> Result<Event, GpuKernelError>}`.
  - `dense_dot`: Output `f32[query_len * slot_total]`, Index `q*slot_total+s`. Es unterstützt nur F32 und F16, sonst `UnsupportedDType`.
  - Ab dem zweiten Batch muss `acked >= launched()` gelten, sonst `AckBehind`.
  - Ein `KernelParams`-Block (72 Byte) wird von `dense_dot` in die Lane geschrieben. Die Slots einer Launch-Gruppe stehen im Ring zwischen `head` und `tail`.
- `device_lane_event`:
  - `Lane::{device(), stream(), params(), params_mut(), record_event()}`.
  - `Event::{wait() -> Result<(), EventError>, is_complete()}`.
  - `DeviceHandle::{Cpu, Gpu(n)}`; ein Event deckt alle zuvor auf der Lane eingereihte Arbeit ab. `Event` hat keine Zeitmessung, daher `GpuRig::mark/elapsed_ns`.
- `capability`: `Axis`/`Value` = `Name` (`Name::as_str()`, `From<&str>`, `From<u64>`), `CapabilitySet::{values(&Axis), iter(), add}`, `axes::{batch_size, scoring, operands}()`, `values::{fp32, fp16}()`.
- Unterstützte Precisions: GPU-Scoring nur F32 und F16, CPU nur F32; BF16 nie. Dieses Modul prüft Precisions selbst gegen diese Menge.
- `layout::DType::{F32, F16}`.