# Spezifikation: Precision, Kalibrierung und Profile (Build-Order 80)

## 1. Aufgabe, Einschränkungen, Grenzen
Rust-Modul `precision_profiles`. Es hat zwei Aufgaben:
1. Es bildet einen Precision-Wunsch `p ∈ [0,1]` auf ein **Profil** je Subengine ab. Ein Profil ist die Kombination aus Device und Elementtyp.
2. Es kalibriert per Benchmark im Hintergrund, welche Profile wie schnell sind.

Einschränkungen:
- **Wunsch statt Zusicherung:** `p` ist ein Wunsch. `0` bedeutet schnellste Kombination, `1` bedeutet präziseste Kombination.
- **Keine Garantien:** Das Modul gibt keine Rounding-, Determinismus- oder Genauigkeitsgarantie. Hardwareabhängiges Rounding ist erlaubt.
- **Kein CPU/GPU-Vertrag:** Zwischen CPU- und GPU-Profilen gibt es keinen Rounding- und keinen Datentyp-Vertrag. Das Modul konvertiert keine Werte.
- **Kalibrierung ist unsichtbar:** Sie unterbricht oder blockiert den Nutzer nie. Kein öffentlicher Aufruf dieses Moduls wartet auf sie.
- **Nicht Teil des Moduls:** Kernel, Scoring, Speicher, Planung, Persistenz der Kalibrierung, Profilnamen für Nutzer, Konfigurationsdateien. Ebenso nicht der Storage-Datentyp von Embeddings (`ModelDescriptor::storage_dtype`) und `ModelDescriptor::compute_dtypes`. Beide werden weder gelesen noch verändert.
- **Abhängigkeiten:** nur `std` und die Module aus Abschnitt 5. Keine Fremd-Crates. `thiserror` ist nur erlaubt, wenn es ohnehin im Projekt ist. Sonst implementiere `Display` und `Error` von Hand.

## 2. Kontext
- **Unterstützte Precisions:** `scoring_dtypes(kind)` liefert die Typen mit Scoring-Kernel. CPU hat `[F32]`, GPU hat `[F32, F16]`. Nur diese Typen werden Kandidaten.
- **Device und Lane und Event:** `DeviceHandle` adressiert das Device. `Event` meldet das Ende einer Messung oder der Kalibrierung, ohne zu blockieren.
- **Batch-Compute-Schnittstelle:** Kann ein Aufrufer als Benchmark-Last dienen (`BatchBench`). Sie rechnet nur `f32`.
- **Layout und Elementtypen:** Liefert `DType`.
- **Modellbeschreibung:** Hat keine Schnittstelle zu diesem Modul (siehe Abschnitt 1).
- **Aufrufer:** Subengines registrieren ihre Kandidaten und fragen pro Operation `select(id, p)` ab. Das ist ein reiner, nicht blockierender Lesezugriff.

## 3. Interface
```rust
use std::sync::Arc;
use std::time::Duration;
use crate::device::{DeviceHandle, Event};
use crate::layout::DType;
use crate::batch_compute::BatchCompute;

/// Precision-Wunsch in [0,1] (endlich). 0 = schnellste, 1 = präziseste Kombination.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Precision(f32);
impl Precision {
    pub const FASTEST: Precision;        // 0.0
    pub const MOST_PRECISE: Precision;   // 1.0
    pub fn new(p: f32) -> Result<Precision, PrecisionError>;  // NaN oder außerhalb [0,1] => Err
    pub fn clamped(p: f32) -> Precision;                      // klemmt auf [0,1]; NaN => 0.5
    pub fn get(self) -> f32;
}
impl Default for Precision { /* 0.5 */ }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrecisionError { OutOfRange }   // Display auf Deutsch

/// Kennung einer Subengine (Schlüssel der Profiltabellen).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubengineId(pub &'static str);

/// Device + Elementtyp. Reine Wahl, keine Zusicherung über Rounding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Profile { pub device: DeviceHandle, pub dtype: DType }

impl Profile {
    /// Genauigkeitsstufe des Typs: F16 => 1, F32 => 2. Andere Typen => None (kein Kandidat).
    pub fn precision_level(dtype: DType) -> Option<u8>;
}

/// Alle Kandidaten für die Devices: je Device alle Typen aus `scoring_dtypes(device.kind())`,
/// in Reihenfolge der Devices, dann der Typen. Doppelte Devices werden einmal gezählt.
pub fn candidates_for(devices: &[DeviceHandle]) -> Vec<Profile>;

#[derive(Clone, Debug, PartialEq)]
pub struct ProfileEntry {
    pub profile: Profile,
    pub level: u8,                     // precision_level
    pub cost: Option<Duration>,        // gemessene Zeit je Messlauf (Median); None = ungemessen (Default-Tabelle)
}

/// Geordnete, unveränderliche Tabelle: Pareto-Front, aufsteigend nach `level`,
/// innerhalb gleicher Front streng aufsteigend nach Zeit. Nie leer.
#[derive(Clone, Debug, PartialEq)]
pub struct ProfileTable { /* entries: Vec<ProfileEntry> */ }
impl ProfileTable {
    pub fn entries(&self) -> &[ProfileEntry];
    pub fn calibrated(&self) -> bool;              // true, wenn aus Messung entstanden
    pub fn pick(&self, p: Precision) -> Profile;   // s. Abschnitt 4
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProfileError {
    UnknownSubengine(SubengineId),
    NoCandidates,                      // register mit leerer / nur ungültiger Kandidatenliste
}
impl std::fmt::Display for ProfileError {}
impl std::error::Error for ProfileError {}

/// Gemeinsame Registry. `Send + Sync`; üblicherweise in `Arc`.
pub struct ProfileRegistry { /* RwLock<HashMap<SubengineId, Arc<ProfileTable>>> */ }
impl ProfileRegistry {
    pub fn new() -> ProfileRegistry;
    /// Setzt die Default-Tabelle (ungemessen) aus den Kandidaten. Ersetzt eine vorhandene Tabelle.
    pub fn register(&self, id: SubengineId, candidates: &[Profile]) -> Result<(), ProfileError>;
    /// Nicht blockierend (nur kurzer Lesezugriff). Nutzt die aktuelle Tabelle (Default oder kalibriert).
    pub fn select(&self, id: SubengineId, p: Precision) -> Result<Profile, ProfileError>;
    pub fn table(&self, id: SubengineId) -> Option<Arc<ProfileTable>>;
}
impl Default for ProfileRegistry {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BenchError { Unsupported, Failed(String) }
impl std::fmt::Display for BenchError {}
impl std::error::Error for BenchError {}

/// Vom Besitzer der Subengine bereitgestellte Messlast. Muss `Send + Sync` sein.
pub trait BenchTarget: Send + Sync {
    /// Führt eine repräsentative Operation mit dem Profil aus.
    /// `Ok(Some(event))`: asynchrone Arbeit; der Kalibrator wartet im Hintergrund-Thread mit `Event::wait`.
    /// `Ok(None)`: die Arbeit ist bei Rückkehr fertig.
    fn run(&self, profile: &Profile) -> Result<Option<Event>, BenchError>;
}

pub struct CalibrationJob {
    pub id: SubengineId,
    pub candidates: Vec<Profile>,
    pub target: Arc<dyn BenchTarget>,
}

#[derive(Clone, Debug)]
pub struct CalibrationConfig {
    pub warmup_runs: u32,                  // Default 1
    pub measured_runs: u32,                // Default 5, mindestens 1 wirksam
    pub budget_per_candidate: Duration,    // Default 200 ms; wird zwischen Läufen geprüft
}
impl Default for CalibrationConfig {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CalibrationStatus { Running, Finished, Cancelled, Failed(String) }

/// Handle der Hintergrundkalibrierung. `Send + Sync + Clone`. Drop bricht nicht ab und wartet nicht.
#[derive(Clone)]
pub struct CalibrationHandle { /* Arc */ }
impl CalibrationHandle {
    pub fn status(&self) -> CalibrationStatus;   // nicht blockierend
    pub fn done(&self) -> Event;                 // CPU-Event; signalisiert bei Finished, Cancelled oder Failed
    pub fn cancel(&self);                        // kooperativ, nicht blockierend, idempotent
}

pub struct Calibrator;
impl Calibrator {
    /// Startet einen Hintergrund-Thread und kehrt sofort zurück. Gibt nie einen Fehler zurück.
    pub fn start(registry: Arc<ProfileRegistry>, jobs: Vec<CalibrationJob>, cfg: CalibrationConfig) -> CalibrationHandle;
}

/// Fertige Messlast über `BatchCompute`: MatMul auf festen f32-Batches.
/// Profile mit `dtype != F32` oder ohne zugeordnetes Backend => `BenchError::Unsupported`.
pub struct BatchBench { /* cpu: Arc<dyn BatchCompute>, gpu: Option<(u32, Arc<dyn BatchCompute>)> */ }
impl BatchBench {
    pub fn new(cpu: Arc<dyn BatchCompute>, gpu: Option<(u32, Arc<dyn BatchCompute>)>) -> BatchBench;
}
impl BenchTarget for BatchBench {}
```
Alle öffentlichen Typen sind `Send + Sync + 'static` (Compile-Time-Assertion im Test).

## 4. Verhalten und Invarianten

**Precision**
- `new(p)` liefert `Ok` genau für endliche `p` mit `0.0 <= p <= 1.0`, sonst `Err(OutOfRange)`.
- `clamped` paniert nie.
- `select` und `pick` nehmen nur gültige `Precision` entgegen.

**Kandidaten**
- `candidates_for(&[Cpu])` ergibt `[Cpu/F32]`.
- `candidates_for(&[Cpu, Gpu(0)])` ergibt `[Cpu/F32, Gpu(0)/F32, Gpu(0)/F16]`.
- Es gibt kein BF16 und keinen Typ ohne `precision_level`.
- `register` und `Calibrator` verwerfen Kandidaten ohne `precision_level` und Duplikate.
- Bleibt nichts übrig, liefert `register` `Err(NoCandidates)`.

**Tabelle (Aufbau)**
- *Default-Tabelle (ungemessen):*
  - Sortiere nach `level` aufsteigend. Bei gleichem Level kommt ein GPU-Kandidat vor einem CPU-Kandidaten, dann gilt die Eingabereihenfolge.
  - Je `level` bleibt nur der erste Kandidat.
  - `cost = None`, `calibrated() == false`.
- *Kalibrierte Tabelle:* Aus den gemessenen Kandidaten (mit `cost`) wird die Pareto-Front gebildet.
  - Ein Kandidat A wird verworfen, wenn ein anderer Kandidat B mit `level_B >= level_A` und `cost_B <= cost_A` existiert und mindestens eine Ungleichung strikt ist.
  - Bei völlig gleichem `level` und `cost` bleibt der erste in der Eingabereihenfolge.
  - Die Front wird aufsteigend nach `level` sortiert. Die Kosten sind dabei streng aufsteigend, weil sonst ein Eintrag dominiert wäre.
  - Der schnellste Kandidat ist immer Mitglied der Front. Unter den präzisesten Kandidaten ist es der schnellste.
  - `calibrated() == true`.
- Eine Tabelle ist nie leer und nach dem Bau unveränderlich (`Arc`).

**Abbildung `pick(p)`**
- Bei `n` Einträgen ist der Index `round(p * (n-1))`, mit Rundung half-away-from-zero und Begrenzung auf `n-1`.
- `p == 0` ergibt den ersten Eintrag (schnellste Kombination der Tabelle).
- `p == 1` ergibt den letzten Eintrag (präziseste Kombination der Tabelle).
- Bei `n == 1` ergibt jedes `p` denselben Eintrag.
- `pick` ist monoton: Ein größeres `p` liefert nie einen Eintrag mit niedrigerem `level`.
- `pick` ist rein und gleich für gleiche Eingaben. Die Tabelle selbst kann sich aber nach einer Kalibrierung ändern. Das Modul sichert daher keine Wiederholbarkeit über die Zeit zu.

**Registry**
- `select` auf eine nicht registrierte Subengine ergibt `Err(UnknownSubengine(id))`.
- `select` blockiert nie auf Kalibrierung, Messungen oder I/O. Es hält nur kurz den Lesezugriff.
- Vor und während der Kalibrierung liefert `select` aus der Default-Tabelle. Danach liefert es aus der kalibrierten Tabelle.
- Der Austausch einer Tabelle ist atomar. Ein `select` sieht entweder die alte oder die neue Tabelle, nie einen Mischzustand.
- `register` ersetzt die Tabelle auch dann, wenn sie schon kalibriert war. Eine laufende Kalibrierung kann sie danach wieder überschreiben.
- Ein vergifteter Lock wird übernommen (innerer Zustand), nicht propagiert. Das Modul paniert dadurch nie.

**Kalibrierung**
- `Calibrator::start` spawnt einen Hintergrund-Thread und kehrt sofort zurück, ohne zu warten. Das Handle meldet `Running`.
- Kann der Thread nicht gestartet werden, ist der Status `Failed(text)`, `done()` ist signalisiert, und die Tabellen bleiben unverändert. Es gibt kein Panic und keinen Fehler an den Aufrufer.
- Die Jobs laufen nacheinander im Hintergrund-Thread, in der Reihenfolge von `jobs`. Pro Job und Kandidat gilt:
  1. `warmup_runs` Läufe ohne Zeitnahme.
  2. Bis zu `measured_runs` Läufe mit Zeitnahme, insgesamt höchstens `budget_per_candidate` (Prüfung zwischen den Läufen). Mindestens ein gemessener Lauf wird ausgeführt.
  3. Gibt `run` ein `Event` zurück, wird mit `Event::wait` auf dessen Ende gewartet, und die Zeit umfasst diese Wartezeit.
  4. `cost` ist der Median der gemessenen Läufe.
- Ein Kandidat mit `Err(BenchError)` oder `Event`-Fehler, oder dessen Lauf paniert (`catch_unwind`), wird für diesen Job aus der Messung ausgeschlossen. Die Kalibrierung läuft weiter und der Fehler wird nicht an den Nutzer gemeldet.
- Nach jedem Job wird die Tabelle atomar in die Registry geschrieben. Zuvor war die Subengine nicht notwendigerweise registriert, denn der Job schreibt auch bei unbekannter `id` (er registriert sie damit).
  - Ist mindestens ein Kandidat gemessen, wird die kalibrierte Tabelle aus den gemessenen Kandidaten gebaut.
  - Ist keiner gemessen, bleibt eine vorhandene Tabelle unverändert. Fehlt sie, wird die Default-Tabelle aus den Kandidaten gesetzt.
- `cancel()` setzt ein Flag. Der Thread prüft es zwischen Läufen und zwischen Kandidaten und beendet sich dann mit Status `Cancelled`. Ein bereits laufender Einzellauf wird nicht unterbrochen. Bereits geschriebene Tabellen bleiben. Der Job, der gerade läuft, schreibt nichts.
- `done()` liefert klonbare Events desselben Markers. `done().is_complete()` ist ab dem Endstatus `Ok(true)`. `done().wait()` blockiert nur den Aufrufer, der es selbst ruft. Das Modul ruft es nie auf dem Nutzer-Pfad auf.
- Ein Drop von `CalibrationHandle` oder `Calibrator` bricht den Thread nicht ab und joint ihn nicht.
- Die Kalibrierung benutzt keine Lane des Nutzers und verändert keinen Zustand der Aufrufer. Sie darf nebenläufig zu `select` laufen. Mehrere Kalibrierungen dürfen parallel laufen, wobei die letzte Schreibende gewinnt.
- Ohne Jobs: Status sofort `Finished`, `done()` signalisiert.

**BatchBench**
- `run` führt `BatchKernel::MatMul` auf zwei fest erzeugten `f32`-Batches ([64,64]) mit dem zum `profile.device` gehörenden Backend aus.
- Zuordnung: `Cpu` nutzt `cpu`, `Gpu(n)` nutzt `gpu` mit passendem Ordinal. Sonst, oder bei `dtype != F32`, kommt `Err(Unsupported)`.
- Ein `ComputeError` wird zu `Err(Failed(text))`.
- Die Rückgabe ist `Ok(None)`.
- Folge: `Gpu/F16` wird mit `BatchBench` nicht gemessen und fehlt in der kalibrierten Tabelle.

**Garantien und Nicht-Garantien**
- Das gewählte Profil ist ein Wunsch. Das Modul garantiert weder Rounding noch Determinismus noch Genauigkeit. Hardwareabhängiges Rounding ist erlaubt.
- Zwischen CPU- und GPU-Profilen gibt es keinen Rounding- und Datentyp-Vertrag. Ergebnisse verschiedener Profile dürfen sich unterscheiden.
- Kein öffentlicher Aufruf (`select`, `table`, `register`, `start`, `status`, `cancel`) blockiert auf die Kalibrierung.

## 5. Relied-on elements
- **Device und Lane und Event** (`device`):
  - `#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)] pub enum DeviceHandle { Cpu, Gpu(u32) }` mit `kind() -> DeviceKind` und `ordinal() -> Option<u32>`.
  - `DeviceKind { Cpu, Gpu }` (`#[non_exhaustive]`, Matches brauchen einen Wildcard-Zweig).
  - `Event` (`Clone`, `Send + Sync`): `Event::cpu() -> (Event, EventSignal)`, `is_complete(&self) -> Result<bool, EventError>` (nicht blockierend), `wait(&self) -> Result<(), EventError>` (blockierend).
  - `EventSignal::signal(&self)` (idempotent, `Send + Sync + Clone`).
  - `Event::mock()` für Tests, mit `MockEventControl::{complete, fail, reset}`.
- **Unterstützte Precisions** (`precisions`): `pub fn scoring_dtypes(kind: DeviceKind) -> &'static [DType]`. CPU liefert `[F32]`, GPU liefert `[F32, F16]`, eine unbekannte Art liefert `[]`.
- **Layout und Elementtypen** (`layout`): `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub enum DType { F32, F16, BF16, U8, U32, I64 }`.
- **Batch-Compute-Schnittstelle** (`batch_compute`):
  - `trait BatchCompute: Send + Sync { fn kind(&self) -> BackendKind; fn run(&self, kernel: BatchKernel, inputs: &[&FloatBatch]) -> Result<FloatBatch, ComputeError>; }`.
  - `FloatBatch::new(rows, cols, Vec<f32>) -> Result<Self, ComputeError>`.
  - `BatchKernel::MatMul` (zwei Eingaben, `[m,k] x [k,n]`).
  - `ComputeError` (`Display`).
  - `run` ist nebenläufig auf derselben Instanz aufrufbar.
- **Modellbeschreibung:** Es wird nichts verwendet (siehe Abschnitt 1).