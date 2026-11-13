# Spezifikation: Manifeste und Publication (Build-Order 102)

Crate-Modul `manifest`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- Manifeste als unveränderliche Snapshots der Segmente führen: Generation, Kernel-Segmenttabelle, lock-freier Reader-Zugriff, Leases.
- Das aktuelle Manifest atomar ersetzen. Dabei stale Generationen ablehnen, Konkurrenz per Retry auflösen und Störungen für Tests auslösbar machen.
- Manifeste hinter dem Event ihres Kopiervorgangs vormerken und der Reihe nach publizieren.
- Nicht mehr gehaltene Segmente samt offenen Completion-Markern zurückgeben (`reclaim`).
- Change-Notifications an beliebig viele Subscriber senden.

**Grenzen**
- Das Modul allokiert, kopiert und gibt keinen Speicher frei. `reclaim` liefert nur `RegionHandle`s. Die Rückgabe an `Allocator`/`Bookkeeping` und das Abbilden auf `bookkeeping::Region` machen die Aufrufer.
- Es führt keine Epochen und keinen Root-Pointer (das macht `TableSwap`). Es definiert keine Typen der Abhängigkeiten neu und enthält keine Policy, wann publiziert wird.
- Es führt keine Kopier-Submission aus. Der Aufrufer reiht den Copy-Auftrag per Closure auf der Lane ein.
- Es enthält kein GPU-Linking. Tests nutzen `Event::mock()`.
- Es enthält kein `unsafe`, außer was ein lock-freier Pointer-Swap zwingend braucht. Ein bewährtes Crate (z. B. `arc-swap`) ist erlaubt.

## 2. Context

- Ein Segment ist eine `SegmentView` (Slot-Layout) mit Segment-Generation. Der Kernel bekommt die Segmenttabelle als Binärblock und den Start jedes Segments als Adresse.
- Ein Manifest ist die Menge der Segmente, die für Reader zu einem Zeitpunkt gilt. Reader sichern es per `Lease`. Die Manifest-Generation steigt je Publication um genau 1.
- Ein Kopiervorgang (CopyH2D-Lane) erzeugt ein Event. Das Folge-Manifest wird erst publiziert, wenn dieses Event abgeschlossen ist.
- Ein Segment wird zurückgegeben, sobald kein Manifest (aktuell, vorgemerkt, vorheriges beim Aufrufer) und keine Lease es mehr hält. Seine Completion-Marker (Events von Readern und deferred Writes) entscheiden, ob die Region schon wiederverwendbar ist.

## 3. Interface

```rust
use std::sync::Arc;
use std::time::Duration;
use device::{Event, Lane};
use kernel_layout::Addr;
use region_handle::{Generation, RegionHandle};
use slot_layout::{SegmentView, SlotError};

pub type ManifestGeneration = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SegmentId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestError {
    /// Display: "stale Generation". Angebotene Generation <= aktuelle.
    StaleGeneration { current: ManifestGeneration, got: ManifestGeneration },
    DuplicateSegment(SegmentId),
    GenerationOverflow,
}
impl std::fmt::Display for ManifestError {}
impl std::error::Error for ManifestError {}

/// Sync-Error beim Vormerken hinter einem Kopier-Event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncError { pub cause: String }
impl std::fmt::Display for SyncError {}
impl std::error::Error for SyncError {}

/// Beschreibung eines Segments (reine Daten, Clone).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentSpec { /* id, generation, region, start, slot_count, slot_stride, bytes */ }
impl SegmentSpec {
    /// Festes Layout: generation = view.generation(), region = view.region(),
    /// (start, slot_count, slot_stride) = view.descriptor(device_base)?, bytes = view.total_bytes().
    pub fn from_view(id: SegmentId, view: &SegmentView, device_base: Addr) -> Result<SegmentSpec, SlotError>;
    /// Variables Layout: slot_stride = 0, bytes = region.len().
    pub fn variable(id: SegmentId, generation: Generation, region: RegionHandle,
                    start: Addr, slot_count: u32) -> SegmentSpec;
    pub fn id(&self) -> SegmentId;
    pub fn generation(&self) -> Generation;
    pub fn region(&self) -> RegionHandle;
    pub fn start(&self) -> Addr;
    pub fn slot_count(&self) -> u32;
    pub fn slot_stride(&self) -> u32;   // 0 = variabel
    pub fn bytes(&self) -> u64;
}

/// Unveränderlicher Snapshot. Clone teilt den Zustand (Arc). Send + Sync.
#[derive(Clone)]
pub struct Manifest { /* Arc<inner> */ }
impl Manifest {
    /// Generation 0, keine Segmente.
    pub fn empty() -> Manifest;
    pub fn generation(&self) -> ManifestGeneration;
    pub fn segments(&self) -> &[SegmentSpec];       // Reihenfolge wie übergeben
    pub fn len(&self) -> usize;
    pub fn total_bytes(&self) -> u64;               // Summe SegmentSpec::bytes(), saturierend
    pub fn lease_count(&self) -> usize;             // aktuell gehaltene Leases
    /// 24 Byte je Segment, Little-Endian, Segmentreihenfolge; leer => leerer Vec:
    /// [0..8) Segment-Generation u64 | [8..16) Startadresse u64 | [16..20) Slot-Anzahl u32 | [20..24) Slot-Abstand u32 (0 = variabel)
    pub fn kernel_table(&self) -> Vec<u8>;
}

/// Sichert ein Manifest. Drop gibt es frei. Send + Sync.
pub struct Lease { /* Manifest */ }
impl Lease {
    pub fn manifest(&self) -> &Manifest;
    pub fn generation(&self) -> ManifestGeneration;
    /// Registriert `marker` bei allen Segmenten des gesicherten Manifests.
    /// Auch für deferred Writes. Bereits abgeschlossene Marker (`is_complete() == Ok(true)`) werden verworfen.
    pub fn register_completion(&self, marker: &Event);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind { Added, Changed }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeEvent {
    pub kind: ChangeKind,
    pub segment_id: SegmentId,
    pub segment_generation: Generation,      // im neuen Manifest
    pub manifest_generation: ManifestGeneration,  // des neuen Manifests
}

/// Send. Nicht blockierender Empfang für den Producer.
pub struct Subscriber { /* Receiver */ }
impl Subscriber {
    pub fn try_recv(&self) -> Option<ChangeEvent>;
    pub fn recv_timeout(&self, d: Duration) -> Option<ChangeEvent>;
    pub fn dropped(&self) -> u64;           // wegen vollem Buffer verworfene Events
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreConfig { pub subscriber_buffer: usize }   // Default 64; 0 wird wie 1 behandelt
impl Default for StoreConfig {}

/// Hält aktuelles Manifest, vorgemerkte Manifeste, Subscriber und Rückgabe-Warteschlange. Send + Sync, alle Methoden &self.
pub struct ManifestStore { /* ... */ }
impl ManifestStore {
    /// Startet mit Manifest::empty() (Generation 0).
    pub fn new(config: StoreConfig) -> ManifestStore;

    /// Lock-freier Zugriff auf das aktuelle Manifest.
    pub fn current(&self) -> Manifest;
    /// Sichert die aktuelle Generation (lock-frei).
    pub fn lease(&self) -> Lease;

    /// Baut das Nachfolger-Manifest von `base`: Generation = base.generation() + 1, Segmente = `specs`.
    pub fn successor(&self, base: &Manifest, specs: Vec<SegmentSpec>) -> Result<Manifest, ManifestError>;

    /// Ersetzt das aktuelle Manifest atomar, sendet Notifications und liefert das vorherige zurück.
    pub fn publish(&self, next: Manifest) -> Result<Manifest, ManifestError>;

    /// Merkt `next` hinter `event` vor (Ende der Warteschlange).
    pub fn stage(&self, next: Manifest, event: Event);
    /// Reiht den Copy-Auftrag auf der Copy-Lane ein, zeichnet danach `lane.record_event()` auf und merkt `next` vor.
    pub fn stage_copy<F>(&self, lane: &mut Lane, enqueue: F, next: Manifest) -> Result<(), SyncError>
        where F: FnOnce(&mut Lane) -> Result<(), String>;
    /// Publiziert vorgemerkte Manifeste der Reihe nach, solange das vorderste Event abgeschlossen ist. Liefert die Anzahl publizierter.
    pub fn publish_ready(&self) -> usize;
    pub fn staged_count(&self) -> usize;

    pub fn subscribe(&self) -> Subscriber;

    /// Gibt Regionen nicht mehr gehaltener Segmente zurück, deren Marker alle abgeschlossen sind. Der Rest bleibt zurückgehalten.
    pub fn reclaim(&self) -> Vec<RegionHandle>;
    /// Nicht mehr gehaltene, noch nicht zurückgegebene Segmente.
    pub fn retired_count(&self) -> usize;

    // Testhilfen (Störungen)
    /// Die nächsten `n` Swap-Versuche scheitern künstlich und lösen einen Retry aus.
    pub fn inject_swap_conflicts(&self, n: u32);
    pub fn swap_retries(&self) -> u64;      // Zähler aller Retries
}
```

`Manifest`, `Lease`, `SegmentSpec`, `ChangeEvent`, `Subscriber`, `ManifestStore`, `ManifestError`, `SyncError` sind `Send + Sync` (statisch geprüft).

## 4. Behavior and invariants

**Manifest**
- `Manifest::empty()` hat Generation 0, keine Segmente, `total_bytes() == 0`, leere `kernel_table()`.
- `successor(base, specs)` hat genau Generation `base.generation() + 1`. Bei Überlauf kommt `GenerationOverflow`. Doppelte `SegmentId` in `specs` ergeben `DuplicateSegment(id)` (erste doppelte). `base` bleibt unverändert.
- Segmentzustand (Region + Marker) wird zwischen Manifesten geteilt. Eine Spec, deren `id` in `base` vorkommt und deren Region (`offset()`, `len()`, `generation()`) gleich ist, übernimmt den Zustand samt Markern. Sonst entsteht ein neuer Zustand. Die Segment-Generation allein erzeugt keinen neuen Zustand.
- `total_bytes()` ist die Summe der `bytes()`. `generation()`, `segments()`, `total_bytes()` und `lease_count()` sind jederzeit abfragbar und blockieren nie.
- `kernel_table()`:
  - Länge `24 * len()`.
  - Segment i belegt Bytes `[24i, 24i+24)`: Generation (`Generation.0`), Start (`Addr.0`), Slot-Anzahl, Slot-Abstand, alles Little-Endian.
  - Variables Layout hat Abstand 0.
  - Für Views gilt: Eintrag = `Descriptor { start, slot_count, stride }` der View.

**Aktuelles Manifest und `publish`**
- `current()` und `lease()` sind lock-frei und wartefrei gegenüber `publish`. Reader sehen immer ein vollständiges Manifest, nie einen Zwischenzustand.
- `publish(next)` ist eine Compare-and-Swap-Schleife:
  1. Lade das aktuelle Manifest `cur`.
  2. Ist `next.generation() <= cur.generation()`, folgt `Err(StaleGeneration { current: cur.generation(), got })`. Zustand und Subscriber bleiben unverändert.
  3. Ein künstlicher Konflikt (`inject_swap_conflicts`) oder ein verlorener CAS (anderer Publisher war schneller) zählt `swap_retries` hoch und wiederholt ab Schritt 1.
  4. Erfolg: `next` ist aktuell, das vorherige Manifest wird zurückgegeben.
- Konkurrierende `publish` mit derselben Generation: höchstens eines gelingt, die anderen enden nach Retry mit `StaleGeneration`.
- Es gibt keine Lücke: Eine Generation > cur + 1 ist erlaubt, nur `<=` wird abgelehnt.
- Nach Erfolg werden die Change-Notifications gesendet (siehe unten), und zwar mit dem tatsächlich ersetzten Manifest als Vorgänger.
- Mit `inject_swap_conflicts(n)` gelingt `publish` trotzdem, mit mindestens `n` zusätzlichen Retries. Das Ergebnis bleibt korrekt, auch gegen stale Generationen.

**Leases**
- `lease()` sichert die zum Zeitpunkt aktuelle Generation: `lease_count()` dieses Manifests steigt um 1, `Drop` der Lease senkt es um 1. Ein Manifest, das nicht mehr aktuell ist, bleibt mit seinen Segmenten gültig, solange eine Lease existiert.
- `lease.manifest()` bleibt für die Lebensdauer der Lease unverändert (Generation, Segmente, Tabelle).
- `register_completion(&marker)` hängt den Marker (Clone des `Event`) an den Zustand jedes Segments des gesicherten Manifests. Mehrfache Registrierung desselben Markers ist erlaubt. Ist `marker.is_complete() == Ok(true)`, wird er nicht gespeichert. Ein Marker mit `Err` oder pending wird gespeichert.
- Deferred-Write-Events werden auf demselben Weg registriert und sind von Reader-Markern ununterscheidbar.

**Rückgabe von Segmenten (`reclaim`)**
- Ein Segmentzustand gilt als nicht mehr gehalten, sobald kein `Manifest` (aktuelles, vorgemerktes, vom Aufrufer gehaltenes vorheriges) und keine `Lease` ihn referenziert. Dann wandert er mit seiner Region und seinen offenen Markern in die Rückgabe-Warteschlange (`retired_count()` steigt). Das passiert beim Drop des letzten Halters.
- Der Zustand wandert nicht in die Warteschlange, solange er in einem anderen Manifest weiterlebt (z. B. unveränderte Segmente im Nachfolger).
- `reclaim()` liefert die Regionen aller Warteschlangeneinträge, bei denen jeder Marker `is_complete() == Ok(true)` ist. Das schließt Einträge ohne Marker ein.
  - Einträge mit mindestens einem Marker, der pending ist oder `Err` liefert, bleiben unverändert zurückgehalten. Ein `Err`-Marker gibt nie frei.
  - Rückgegebene Einträge werden entfernt. Jede Region wird höchstens einmal geliefert.
  - Reihenfolge ist die der Zurückstellung.
- `reclaim()` ist idempotent (ohne neue Abschlüsse liefert ein zweiter Aufruf leer), blockiert nie und ist parallel zu allem anderen sicher. Parallele Aufrufe liefern jede Region genau einmal.
- `reclaim` ruft weder `Allocator` noch `Bookkeeping` auf.

**Vormerken und Publizieren hinter Copy-Event**
- `stage(next, event)` hängt `(next, event)` an die Warteschlange an. Die Reihenfolge ist die Aufrufreihenfolge. Die Generation wird erst beim Publizieren geprüft.
- `stage_copy(lane, enqueue, next)`:
  1. Ruft `enqueue(lane)` auf, um den Copy-Auftrag auf der Copy-Lane einzureihen. `Err(cause)` ergibt `Err(SyncError { cause })`.
  2. Ruft `lane.record_event()` auf. `Err(e)` ergibt `Err(SyncError { cause: e.to_string() })`.
  3. Merkt `(next, event)` vor.
  - Bei jedem Fehler wird nichts vorgemerkt.
- `publish_ready()` läuft serialisiert (ein Aufrufer gleichzeitig) und arbeitet so lange, bis das vorderste Event nicht abgeschlossen ist:
  - `front.event.is_complete()`:
    - `Ok(true)`: Eintrag entfernen und `publish(next)` aufrufen.
      - Erfolg: zählt als publiziert.
      - `Err(StaleGeneration)`: Eintrag verworfen, `log::warn!`, nicht gezählt, weiter mit dem nächsten.
    - `Ok(false)` oder `Err(_)`: Stopp. Der Eintrag bleibt vorn (bei `Err` mit `log::warn!`). Spätere Aufrufe fragen erneut.
  - Ein abgeschlossenes Event hinter einem noch offenen vordersten Event wird nicht überholt.
  - Rückgabe ist die Anzahl tatsächlich publizierter Manifeste. Ohne Vorgemerkte ist sie 0.
  - Das vom `publish` zurückgegebene vorherige Manifest wird intern verworfen (dabei können Segmente in die Rückgabe-Warteschlange gelangen).
- `staged_count()` ist die Zahl noch vorgemerkter Einträge.

**Change-Notifications**
- Nach einer erfolgreichen Publication, in Segmentreihenfolge des neuen Manifests, je Segment höchstens ein Event:
  - `Added`, wenn die `SegmentId` im vorherigen Manifest nicht vorkommt.
  - `Changed`, wenn die ID vorkommt und die Segment-Generation sich unterscheidet.
  - Kein Event bei gleicher ID und gleicher Segment-Generation.
- `ChangeEvent.segment_generation` ist die Generation im neuen Manifest. `manifest_generation` ist die Generation des neuen Manifests. Entfernte Segmente erzeugen kein Event.
- Die Events einer Publication werden jedem Subscriber zusammenhängend zugestellt.
- `subscribe()` liefert einen neuen Subscriber mit eigenem Buffer der Größe `StoreConfig::subscriber_buffer` (0 wie 1). Er erhält nur Events nachfolgender Publications. Beliebig viele Subscriber sind erlaubt.
- Senden blockiert nie und scheitert nie:
  - Ohne Subscriber ist es ein No-op, `publish` gelingt.
  - Ist der Buffer eines Subscribers voll, wird das neue Event für ihn verworfen und `dropped()` erhöht. Andere Subscriber sind nicht betroffen.
  - Gedroppte Subscriber werden still entfernt.

**Concurrency und Fehler**
- Lesezugriffe (`current`, `lease`, Manifest-Abfragen) nehmen keinen Lock, den `publish` hält. Die Warteschlange der Vormerkungen, die Rückgabe-Warteschlange und die Subscriberliste liegen hinter kurzen Mutexen. Es werden keine Events oder Callbacks (`is_complete` ausgenommen) unter einem Lock außerhalb dieser Strukturen aufgerufen.
- Lock-Reihenfolge: Staging-Lock → (CAS/Publish) → Subscriber-Lock; Rückgabe-Warteschlangen-Lock ist ein Blatt. Vergiftete Locks werden übernommen (`into_inner`), kein Panic.
- Es gibt keine Panics bei gültigen Eingaben. Überläufe werden geprüft.

## 5. Relied-on elements

- **`slot_layout`**
  - `SegmentView` (Copy): `region() -> RegionHandle`, `generation() -> Generation`, `total_bytes() -> u64`, `descriptor(device_base: Addr) -> Result<Descriptor, SlotError>`.
  - `Descriptor { start: Addr, slot_count: u32, stride: u32 }`.
  - `SlotError` (Debug, Clone, PartialEq, Eq).
- **`region_handle`**
  - `RegionHandle` ist `Copy`, mit `len() -> u64`, `offset() -> u64`, `generation()`.
  - `Generation(pub u64)` ist `Copy + Eq`.
- **`kernel_layout`**: `Addr(pub u64)`, Copy.
- **`device`**
  - `Event` ist `Clone + Send + Sync`, Klone teilen den Zustand.
  - `Event::is_complete(&self) -> Result<bool, EventError>` blockiert nie.
  - `Lane::record_event(&self) -> Result<Event, LaneError>`, `LaneError: Display`.
  - Tests nutzen `Event::mock()` (`complete()`, `fail(..)`, `reset()`) und `Event::cpu()`.
- **Nicht aufgerufen (nur Abgrenzung):** `Allocator::free`/`free_deferred` und `TableSwap` nutzt dieses Modul nicht. Das Weiterreichen der von `reclaim` gelieferten Regionen ist Sache des Aufrufers.
- **Logging:** `log`-Crate (`warn!`).