# Spezifikation: Kernel-Batchlauf-Schnittstelle (Build-Order 57)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `kernel_batch`. Es stellt die pointerfreie Schnittstelle zwischen Host und Such-Kernels bereit. Der Lebenszyklus eines Batches ist: Begin → (Ranges zuteilen) → Run → (bei Partial: Ranges zuteilen → Run)* → End. Das Modul bietet außerdem:
- Segmentbeschreibung (fest oder variabel) mit Adress→Entry-Abbildung.
- Cursor-Schreiben in vorab vergebene Bereiche.
- Fortschrittsabfrage.

**Nicht Teil des Moduls:**
- Kernel-Code, Start von GPU-Arbeit und Device-Speicher. Das erledigt der Aufrufer über das Trait `BatchExecutor`.
- Auflösung von `RegionHandle` auf Adressen und Allokation. `kernel_batch` allokiert nie Bereichsspeicher.
- Eigene Event-, Lane- oder Handle-Typen. Diese werden unverändert aus den Abhängigkeiten verwendet.
- Keine Raw-Pointer, Referenzen auf Device-Speicher oder `unsafe`. Adressen sind reine `u64`-Werte.

Abhängigkeiten: `Event`, `EventSignal`, `DeviceHandle` (Modul Device/Lane/Event) und `RegionHandle`, `Generation` (Modul `region_handle`).

## 2. Context

- Das Memory-Backend vergibt `RegionHandle`s. Es löst sie auf und implementiert `BatchExecutor`.
- Der Query-Planer ruft `BatchInterface` auf. Er teilt Ausgabebereiche (Embeddings) zu und fragt Fortschritt ab.
- Im Normalmodus dient ein `Event` je Batch als Laufzustand.
- Im Persistent-Modus (dauerlaufender Kernel) liest ein Progress-Counter in gemapptem Host-Memory (`ArenaKind::MappedHost`) den Fortschritt. Der Zugriff darauf läuft über `ProgressCounter`.
- Segmente sind Index-Segmente. Ihre Basisadresse ist ein `u64`-Wert, den der Kernel versteht.

## 3. Interface

```rust
use region_handle::RegionHandle;
use device_lane_event::{Event, EventSignal};   // Event, EventSignal
use std::{sync::Arc, time::Duration};

pub type DeviceAddr = u64;                     // reiner Zahlenwert, kein Pointer

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BatchId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentLayout {
    Fixed { stride: u64 },                     // Bytes je Entry
    /// Offset-Tabelle mit entries+1 u64-Werten (relativ zu base, nicht fallend) in dieser Region.
    Variable { offsets: RegionHandle },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentDesc {
    pub base: DeviceAddr,
    pub entries: u64,
    pub layout: SegmentLayout,
}
impl SegmentDesc {
    /// Fest: offsets ignoriert. Variabel: offsets = Inhalt der Offset-Tabelle (Länge entries+1).
    pub fn entry_of(&self, addr: DeviceAddr, offsets: &[u64]) -> Option<u64>;
    /// Variabel: offsets.len()==entries+1 und nicht fallend. Fest: immer true.
    pub fn offsets_valid(&self, offsets: &[u64]) -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BatchSpec {
    pub segments: Vec<SegmentDesc>,
    pub query_count: u32,
    pub params: Option<RegionHandle>,          // optionaler Parameter-Bereich
    pub time_budget: Duration,
}

/// Lesezugriff auf den Progress-Counter im gemappten Host-Memory (Backend implementiert).
pub trait ProgressCounter: Send + Sync + std::fmt::Debug {
    fn load(&self) -> u64;                     // abgeschlossene Queries, nicht-blockierend
}

#[derive(Clone, Debug)]
pub enum BatchMode {
    Normal,                                    // Laufzustand über Event
    Persistent { counter: Arc<dyn ProgressCounter> },
}

/// Schreib-Cursor in einem vorab vergebenen Bereich. Allokiert nie.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeCursor { /* region: RegionHandle, pos: u64 */ }
impl RangeCursor {
    pub fn new(region: RegionHandle) -> RangeCursor;
    pub fn region(&self) -> RegionHandle;
    pub fn used(&self) -> u64;                 // pos
    pub fn remaining(&self) -> u64;            // region.len() - pos
    /// Reserviert len Bytes; Ok(Offset relativ zum Bereichsanfang) und pos += len.
    /// Reicht der Rest nicht: Err(RangeFull), pos unverändert.
    pub fn advance(&mut self, len: u64) -> Result<u64, RangeFull>;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeFull { pub requested: u64, pub remaining: u64 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsedRange { pub region: RegionHandle, pub used: u64 }

#[derive(Clone, Debug)]
pub struct BatchInputs { pub regions: Vec<RegionHandle> }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchOutcome {
    Hits { hits: u64 },                                         // alle Queries fertig
    Partial { queries_done: u32, hits: u64 },                   // Bereich reichte nicht
}
pub type RunOutcome = LaunchOutcome;

/// Wird vom Backend implementiert; führt den Kernel synchron bis Ende oder Partial aus.
pub trait BatchExecutor: Send + Sync {
    /// `queries_done`: bereits abgeschlossene Queries (Wiederaufnahme nach Partial).
    /// Schreibt Embeddings nur über `ranges` (Cursor, in Zuteilungsreihenfolge).
    fn launch(&self, id: BatchId, spec: &BatchSpec, inputs: &BatchInputs,
              queries_done: u32, ranges: &mut [RangeCursor]) -> Result<LaunchOutcome, String>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BatchError {
    UnknownBatch(BatchId),
    InvalidSpec(String),             // z. B. query_count == 0
    Busy,                            // Run läuft bereits
    NeedsRanges,                     // kein Bereich zugeteilt / nach Partial nicht neu zugeteilt
    AlreadyFinished,                 // Run nach vollständigem Ergebnis
    Executor(String),
}
impl std::fmt::Display for BatchError {}
impl std::error::Error for BatchError {}

pub struct BatchInterface<E: BatchExecutor> { /* Mutex<HashMap<BatchId, State>>, Id-Zähler */ }
impl<E: BatchExecutor> BatchInterface<E> {
    pub fn new(executor: E) -> Self;
    pub fn begin(&self, spec: BatchSpec, mode: BatchMode) -> Result<BatchId, BatchError>;
    pub fn assign_ranges(&self, id: BatchId, ranges: &[RegionHandle]) -> Result<(), BatchError>;
    pub fn run(&self, id: BatchId, inputs: &BatchInputs) -> Result<RunOutcome, BatchError>;
    pub fn progress(&self, id: BatchId) -> Result<u64, BatchError>;
    pub fn event(&self, id: BatchId) -> Result<Event, BatchError>;   // Normal: Batch-Event; Persistent: Err(InvalidSpec)
    pub fn end(&self, id: BatchId) -> Result<Vec<UsedRange>, BatchError>;
}
// BatchInterface<E>: Send + Sync, wenn E: BatchExecutor (Send + Sync)
```

## 4. Behavior and invariants

**Adress→Entry (`entry_of`)**
- Fest: `None` bei `stride == 0`, bei `addr < base` oder bei `idx = (addr-base)/stride >= entries`. Sonst `Some(idx)`. Die Rechnung ist überlaufsicher und panikfrei.
- Variabel: `rel = addr - base`. Gesucht ist `i` mit `offsets[i] <= rel < offsets[i+1]` (Binärsuche).
  - Leere Entries (gleiche Offsets) werden nie getroffen.
  - `None` bei `addr < base`, bei `rel >= offsets[entries]` oder wenn `!offsets_valid`.
- `entries == 0` ergibt immer `None`.

**Cursor**
- Ein Cursor beginnt bei `pos = 0`. `advance(len)` gelingt genau dann, wenn `len <= remaining`, und `len == 0` gelingt immer.
- Der Rückgabewert ist der alte `pos`. `pos` überschreitet `region.len()` nie, und Überlauf wird mit `checked_add` geprüft (dann `RangeFull`).
- Cursor allokieren nie.

**Begin**
- `begin` gibt eine eindeutige, aufsteigende `BatchId` zurück.
- `query_count == 0` ergibt `InvalidSpec`.
- Segmente mit `Variable` werden hier nicht gegen Offset-Inhalte geprüft.
- Im Normalmodus entsteht ein pending `Event::cpu()`, dessen `EventSignal` intern gehalten wird.
- `time_budget` wird unverändert an den Executor weitergereicht.

**Ranges**
- `assign_ranges` ist erlaubt vor dem ersten `run` und nach jedem `Partial`. Es fügt neue Cursor (`pos = 0`) hinzu.
- Bereits genutzte Bereiche bleiben mit ihrem `used` erhalten, damit `end` sie meldet.
- Während `run` läuft, ergibt es `Busy`.
- Nach vollständigem Ergebnis ergibt es `AlreadyFinished`.
- Neue Bereiche sind die Cursor, die der nächste `launch` bekommt. `launch` erhält nur Cursor der noch nicht gefüllten Bereiche, also die seit dem letzten Partial neu zugeteilten.

**Run**
- Ohne zugeteilten, unbenutzten Bereich ergibt `run` `NeedsRanges`, und es wird nichts gestartet. Das gilt auch nach `Partial` ohne neue Zuteilung.
- `run` ruft `launch` ohne gehaltenes Map-Lock auf. `progress`, `event` und andere Batches bleiben währenddessen bedienbar.
- Ein zweites gleichzeitiges `run` auf dieselbe ID ergibt `Busy`.
- `Hits` setzt den Batch auf fertig und signalisiert das Event (Normalmodus). Weitere `run` ergeben `AlreadyFinished`.
- `Partial` signalisiert das Event nicht. Es merkt `queries_done` für die Wiederaufnahme. Der nächste `run` übergibt dieses `queries_done` an `launch`.
- Ein Fehler des Executors ergibt `Executor(msg)`. Der Batch bleibt bestehen und kann per `end` beendet werden.
- Der Zustand ändert sich nicht durch einen Fehler. Das Event bleibt pending.

**Progress**
- `progress` liefert die Anzahl abgeschlossener Queries und nimmt nie ab:
  - Normalmodus: 0 vor dem ersten Run, nach `Partial` `queries_done`, nach `Hits` `query_count`.
  - Persistent: `counter.load()`.
  - Beide Modi liefern `max(zuletzt gemeldet, aktueller Wert)` und höchstens `query_count`.
- `progress` blockiert nie.

**End**
- `end` entfernt den Batch und gibt für jeden zugeteilten Bereich in Zuteilungsreihenfolge `UsedRange{region, used}` zurück. Unbenutzte Bereiche erscheinen mit `used = 0`.
- Danach ergibt jede Operation auf die ID `UnknownBatch`.
- `end` während `run` ergibt `Busy`.
- `end` ohne fertigen Batch ist erlaubt (Abbruch).

**Nebenläufigkeit.** `BatchInterface` ist `Send + Sync`. Der Zustand liegt hinter einem Mutex. Eine vergiftete Sperre wird übernommen, es gibt keine Panics. Alle Fehler sind `BatchError`.

## 5. Relied-on elements

Aus Device/Lane/Event (unverändert):
```rust
#[derive(Clone)] pub struct Event;                       // Send + Sync; Klone teilen Zustand
impl Event {
    pub fn cpu() -> (Event, EventSignal);
    pub fn mock() -> (Event, MockEventControl);
    pub fn is_complete(&self) -> Result<bool, EventError>;   // nicht-blockierend
    pub fn wait(&self) -> Result<(), EventError>;
}
#[derive(Clone)] pub struct EventSignal; impl EventSignal { pub fn signal(&self); } // idempotent, Send + Sync
```
Aus `region_handle` (unverändert):
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegionHandle { /* arena, offset, len, generation */ }
impl RegionHandle { pub fn len(&self) -> u64; pub fn arena(&self) -> ArenaKind; pub fn generation(&self) -> Generation; }
```
`RegionHandle` ist `Copy + Send + Sync`, pointerfrei, und die Auflösung auf Adressen erfolgt nur im Memory-Backend.