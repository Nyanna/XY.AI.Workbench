# Spezifikation: Launch-Modi (Modul `launch_modes`, Build-Order 108)

## 1. Task, restrictions and boundaries

**Task.** Das Modul führt Kernel-Launches in den drei bekannten Modi aus oder lehnt sie ab:
- **Direkter Launch:** wird ausgeführt.
- **Graph-Capture (CUDA Graphs) und Persistent Kernel (Ring-Buffer):** werden als "nicht verfügbar" abgelehnt, mit dem Grund, dass der Modus nicht aktiviert ist.

Zusätzlich bildet es den `LaunchKey` und stellt den Warm-up-Hook vor dem ersten produktiven Launch bereit.

**Nicht Teil des Moduls:**
- Scoring. Das Modul ist vom Scorer getrennt und importiert ihn nicht.
- Kernel-Laden und -Kompilieren (das macht `kernel_register`), Lane-Erzeugung, Event-Verwaltung, Batch-Lebenszyklus (`kernel_batch`) und Publication oder Reclaim von Tabellen.
- Eine eigene Graph-Capture- oder Persistent-Implementierung. Diese Modi liefern nur den Fehler.
- Eigene Region-, Lane-, Event-, DType- oder Variant-Typen. Sie werden unverändert aus den Abhängigkeiten verwendet.
- Kein `unsafe`, kein GPU-Linking. Tests laufen mit Fake-`LaunchLane`, `Event::mock()` und Fake-`RootSlot`/`TableSink`.
- Logging über die `log`-Crate.

## 2. Context

- Die Lane (`device::Lane`) hat selbst keine Submit-Methode. Die Ausführung läuft deshalb hinter dem Trait `LaunchLane`, den der Aufrufer oder das GPU-Binding implementiert. Das Work-Item ist ein generischer Typ `W`. Das Modul interpretiert es nie.
- Jeder normale Launch bekommt zwei Kernel-Argumente aus `table_swap`: den Root-Pointer (Arena-Offset der aktuellen Segmenttabelle, `u64`) und die Epoche der Segmenttabelle. Ein Table-Swap wirkt ab dem nächsten Launch.
- Der `LaunchKey` ist der Schlüssel für spätere Captures. Eine geänderte Generation einer beteiligten Region ergibt einen neuen Key. Damit läuft keine stale Capture je.
- Die Reader-Verwaltung (`register_reader`, `set_quiescent`, `report_epoch`) liegt beim Aufrufer. Das Modul erhält eine vorhandene `ReaderId`.

## 3. Interface

```rust
use std::{collections::HashSet, sync::{Arc, Mutex}};
use device_lane_event::LaneKind;
use kernel_register::Variant;                 // { dim: Option<u32>, tile: Option<u32> }, Copy+Eq+Hash
use layout::DType;
use region_handle::RegionHandle;              // enthält Generation
use table_swap::{Epoch, ReaderId, TableSwap, TableSwapError, ROOT_SENTINEL};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LaunchMode { Direct, GraphCapture, Persistent }

/// Shape-Klasse: Zweierpotenz in 1..=2^31.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShapeClass(u32);
impl ShapeClass {
    pub const MAX: ShapeClass = ShapeClass(1 << 31);
    /// Nächsthöhere (bzw. gleiche) Zweierpotenz, mindestens 1; bei Overflow (> 2^31) 2^31.
    pub fn of(extent: u64) -> ShapeClass;
    pub fn get(self) -> u32;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LaunchKey {
    pub operation: String,
    pub shape: ShapeClass,
    pub regions: Vec<RegionHandle>,   // Reihenfolge und Generation signifikant
    pub dtype: DType,
    pub variant: Variant,
    pub lane_class: LaneKind,
}
impl LaunchKey {
    pub fn new(operation: impl Into<String>, extent: u64, regions: &[RegionHandle],
               dtype: DType, variant: Variant, lane_class: LaneKind) -> LaunchKey;
}

/// Kernel-Argumente jedes normalen Launches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LaunchArgs { pub root: u64, pub epoch: Epoch }

/// Vom Aufrufer/Binding implementiert: reicht das Work-Item unverändert an die Lane.
pub trait LaunchLane<W>: Send + Sync {
    fn launch(&self, item: &W, args: &LaunchArgs) -> Result<(), String>;   // Err = Lane-Fehler (Text)
}

/// Warm-up-Hook. Default-Implementierung tut nichts.
pub trait WarmUp: Send + Sync {
    fn warm_up(&self, _key: &LaunchKey) -> Result<(), String> { Ok(()) }
}
pub struct NoWarmUp;                 // impl WarmUp, tut nichts

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LaunchError {
    /// Display: "nicht verfügbar: <reason>"
    NotAvailable { mode: LaunchMode, reason: String },
    /// Lane-Fehler als Kernel-Error; Display enthält den Lane-Text.
    Kernel(String),
    /// Warm-up-Hook gescheitert.
    WarmUp(String),
    /// Zugriff auf Table-Swap gescheitert (außer NoTable).
    Table(TableSwapError),
}
impl std::fmt::Display for LaunchError {}
impl std::error::Error for LaunchError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LaunchReport { pub args: LaunchArgs, pub productive: bool }

pub struct Launcher { /* swap: Arc<TableSwap>, reader: ReaderId, warm: Arc<dyn WarmUp>, seen: Mutex<HashSet<LaunchKey>> */ }
// Send + Sync (statisch geprüft)

impl Launcher {
    pub fn new(swap: Arc<TableSwap>, reader: ReaderId, warm_up: Arc<dyn WarmUp>) -> Launcher;

    /// Einziger Einstieg. Siehe Abschnitt 4.
    pub fn launch<W>(&self, mode: LaunchMode, key: &LaunchKey,
                     lane: &dyn LaunchLane<W>, item: &W) -> Result<LaunchReport, LaunchError>;

    /// Konsistentes (Root, Epoche) für den nächsten Launch.
    pub fn current_args(&self) -> Result<LaunchArgs, LaunchError>;

    /// Wurde für diesen Key bereits gelaunched (Warm-up-Phase vorbei)?
    pub fn is_warm(&self, key: &LaunchKey) -> bool;
}
```

## 4. Behavior and invariants

**Shape-Klasse**
- `of(0) == 1` und `of(1) == 1`.
- `of(n)` ist die kleinste Zweierpotenz ≥ n. Zum Beispiel `of(3) == 4`, `of(4) == 4`, `of(5) == 8`.
- `of(2^31) == 2^31`. Für `n > 2^31` (Overflow) gilt `2^31`. Es gibt keine Panic.

**Launch-Key**
- Der Key besteht aus Operation, Shape-Klasse (aus `extent`), Regionen (mit Generation), Datentyp, Variante und Lane-Klasse. Gleichheit und Hash gelten über alle Felder.
- Gleiche Eingaben ergeben gleiche Keys. Eine abweichende Generation einer Region ergibt einen ungleichen Key.
- Die Regionen-Reihenfolge bleibt unverändert und zählt.
- Das Modul vergleicht nie Generationen mit einem aktuellen Wert. Es bildet nur Keys.

**Moduswahl in `launch`**
- `GraphCapture` gibt `Err(NotAvailable{mode: GraphCapture, reason: "Graph-Capture ist nicht aktiviert"})` zurück.
- `Persistent` gibt `Err(NotAvailable{mode: Persistent, reason: "Persistent-Modus ist nicht aktiviert"})` zurück.
- Die Ablehnung hat keine Seiteneffekte. Es gibt keinen Table-Zugriff, keinen Warm-up, keinen Lane-Aufruf und keine Änderung an `seen`.

**Direkter Launch (`Direct`)**
1. Warm-up-Prüfung:
   - Ist der Key neu (Einfügen in `seen` gelingt), ruft der Launcher genau einmal `warm_up(key)` auf. Der Aufruf geschieht ohne gehaltene Sperre.
   - Schlägt der Hook fehl, wird der Key wieder entfernt, `Err(WarmUp(text))` zurückgegeben und die Lane nicht aufgerufen.
   - Dieser erste Launch gilt als nicht produktiv: `LaunchReport.productive == false`.
   - Jeder weitere Launch mit demselben Key ist `productive == true`.
   - Bei gleichzeitigen Erstlaunches desselben Keys ist genau einer nicht produktiv.
   - Mit `NoWarmUp` bleibt der Hook wirkungslos. Der erste Launch wird dennoch als nicht produktiv gemeldet.
2. Argumente: `current_args()`. Das Modul ruft `swap.acquire(reader)` auf und nimmt `root = guard.root()` und `epoch = guard.epoch()`. Beides stammt aus einem einzigen konsistenten Tripel. Der Guard wird nach dem Lane-Aufruf gedroppt.
   - Bei `Err(NoTable)` (noch nichts publiziert) gilt `LaunchArgs{root: ROOT_SENTINEL, epoch: 0}`. Das ist kein Fehler.
   - Jeder andere `TableSwapError` ergibt `Err(Table(e))`, und die Lane wird nicht aufgerufen.
3. Das Work-Item geht unverändert (dieselbe Referenz, keine Kopie oder Änderung) mit `&args` an `lane.launch`.
4. Ein Lane-Fehler `Err(text)` ergibt `Err(Kernel(text))`. Der Fehler wird nicht verschluckt, nicht wiederholt und nicht umgedeutet.
5. Tracing: `log::trace!` je Launch mit Operation, Shape-Klasse, Epoche, Root und `productive`. Auf Info-Level oder höher entstehen keine Launch-Logs.

**Table-Swap-Semantik**
- Jeder Launch liest Root und Epoche neu. Ein `publish` zwischen zwei Launches wirkt ab dem nächsten Launch.
- Ein bereits übergebener Launch behält seine Argumente.
- Das Modul ruft nie `publish`, `reclaim` oder `set_quiescent` auf. Quiescence und `report_epoch` für das Ende laufender Batches liegen beim Aufrufer (Reader-Eigner).
- Die `ReaderId` muss beim `TableSwap` registriert sein, sonst gilt `Err(Table(UnknownReader))`.

**Nebenläufigkeit**
- `Launcher` ist `Send + Sync`. `launch` darf parallel aus mehreren Threads laufen.
- Die einzige gemeinsame Sperre ist der `seen`-Mutex. Er wird nie während `warm_up` oder `lane.launch` gehalten.
- Eine vergiftete Sperre wird übernommen, es gibt keine Panic.

**Fehlerbilder**

| Situation | Ergebnis |
|---|---|
| `GraphCapture` / `Persistent` | `NotAvailable{mode, reason}` |
| Lane-Fehler | `Kernel(text)` |
| Warm-up-Hook scheitert | `WarmUp(text)` |
| Table-Swap-Fehler außer `NoTable` | `Table(e)` |

## 5. Relied-on elements

- **`table_swap::TableSwap`** (`Send + Sync`):
  - `acquire(&self, ReaderId) -> Result<TableGuard<'_>, TableSwapError>`. Der Guard liefert `epoch() -> Epoch` und `root() -> u64` (Arena-Offset). Er liefert `Err(NoTable)`, solange nichts publiziert ist.
  - `Drop` des Guards meldet die Epoche des Readers.
  - `ROOT_SENTINEL: u64 = u64::MAX`, `Epoch = u64`, `ReaderId(pub u32)`.
  - `TableSwapError` (`Clone, Debug, PartialEq, Eq`) mit den Varianten `NoTable`, `UnknownReader(ReaderId)` u. a.
  - `acquire` setzt den Reader auf nicht ruhend. Quiescence verwaltet der Aufrufer.
- **`region_handle::RegionHandle`:** `Copy, Eq, Hash`. Er umfasst Arena, Offset, Länge und Generation. Gleichheit gilt über alle vier Felder, und unterschiedliche Generationen ergeben unterschiedliche Keys.
- **`device_lane_event::LaneKind`:** `{Query, Insert, Update, CopyH2D, CopyD2H}` (`Copy, Eq, Hash`, `#[non_exhaustive]`).
- **`kernel_register`:** `Variant { dim: Option<u32>, tile: Option<u32> }` (`Copy, Eq, Hash, Default`).
- **`layout::DType`:** `{F32, F16, BF16, U8, U32, I64}` (`Copy, Eq, Hash`).
- **`kernel_batch`:** wird nicht verwendet. Der Persistent-Modus hier ist nur ein abgelehnter Launch-Modus, nicht `BatchMode::Persistent`.
- **Logging:** `log`-Crate (`trace!`).