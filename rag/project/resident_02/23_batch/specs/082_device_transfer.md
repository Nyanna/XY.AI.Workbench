# Spezifikation: Device-Transfer (Build-Order 82)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `device_transfer`. Es überträgt Index-Entries aus der Index-Arena (Host) in den VRAM, als Segmente der Arena `IndexSegments`, und meldet jedes geladene Segment an eine austauschbare Sink. Es kennt drei Wege:
- **Direktes Kopieren** ganzer, lückenlos gespeicherter Segmente.
- **Gather-Pfad**: Entries werden über Staging-Slots übertragen. Er ist der Fallback für alles, was nicht direkt kopierbar ist.
- **Direktes Mapping**: Sehr kleine Mengen werden zero-copy über die Arena `MappedHost` bereitgestellt.

Alle Transfers laufen im Blocking-Pool (`page_advice::read_blocking`).

**Nicht Teil des Moduls:**
- Keine Eviction, keine Policy (wann oder was geladen wird). Der Aufrufer liefert die Entry-Liste.
- Kein Generationsmanagement und keine Registrierung in `SegmentStore`. Das macht die Sink-Implementierung.
- Keine eigene Lane und kein eigener Stream. Die Copy-Lane gehört dem `GpuBackend` (`upload` läuft asynchron auf dessen Copy-Stream und liefert ein `Event`).
- Kein Retry. Keine Redefinition der Typen aus Abschnitt 5.
- Keine Stamp-Vergabe. Es nutzt `index_contract::Stamp` nur zum Lesen und Vergleichen auf Gleichheit.

**Crates:** `tokio` (über `page_advice`), `log`, `thiserror` optional.

## 2. Condensed Context

- **Aufrufer** (Lader/Planer) übergeben Entry-Listen. Die Sink (z. B. ein Adapter auf `SegmentStore`) erhält `LoadedSegment` und gibt die Region später selbst frei.
- **`GpuBackend`** (Build 73) liefert `allocate`, `free`, `free_after`, `upload`, `mapped_write`, `mapped_device_addr`, `device`. Device-Adressen verlassen es nie, daher arbeitet dieses Modul nur mit `RegionHandle` und absoluten Arena-Offsets.
- **`staging_ring::StagingRing`** (Build 40) ist der Slot-Ring des Gather-Pfads.
  - Er ist nicht `gpu_backend::StagingRing`, das ist ein anderer Typ.
  - Slots werden erst nach Event-Abschluss wiederverwendet.
- **`index_contract::Index`** liefert `stamp`, `read`, `byte_ranges` und `access_methods`.
- **`page_advice`** liefert Advice (`Sequential`, `Random`) und `read_blocking`.
- Die Index-Arena (Host-Speicher des Index) wird über den Trait `IndexArenaSource` abstrahiert.

## 3. Interface

```rust
use std::sync::Arc;
use index_contract::{Index, EntryId, Stamp, ByteRange as IndexByteRange, IndexError};
use region_handle::RegionHandle;
use gpu_backend::{GpuBackend, PinnedBuffer, BackendError};
use staging_ring::{StagingRing, StagingError, TransferError};
use page_advice::AdviceTarget;

/// Bytes eines Index-Bereichs im Host-Speicher.
pub enum ArenaBytes<'a> {
    Ordinary(&'a [u8]),          // gewöhnliches Host-Memory -> HostSource::Pageable
    Pinned(&'a PinnedBuffer),    // Pinned Host-Memory       -> HostSource::Pinned
}
impl ArenaBytes<'_> { pub fn len(&self) -> usize; }

/// Quelle für Segmentbereiche (Index-Arena). Send + Sync.
pub trait IndexArenaSource: Send + Sync {
    fn index(&self) -> &dyn Index;
    /// Bytes genau dieses Bereichs. Berührt höchstens die Seiten; blockiert nicht für I/O,
    /// bevor die Bytes gelesen werden.
    fn range_bytes(&self, range: &IndexByteRange) -> Result<ArenaBytes<'_>, IndexError>;
    /// Ziel für Page-Cache-Hinweise; None = keine Hinweise.
    fn advice_target(&self) -> Option<Arc<dyn AdviceTarget>>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Binding { pub entry: EntryId, pub slot: usize, pub stamp: Stamp }

/// Ein geladenes Segment (Region + Slot-Zuordnung + Bindings).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedSegment {
    pub region: RegionHandle,      // Arena IndexSegments (direct/gather) oder MappedHost (mapping)
    pub entries: Vec<EntryId>,     // aufsteigend; Slot i enthält entries[i]
    pub bounds: Vec<u64>,          // len == entries.len()+1; absolute Arena-Offsets; Slot i = [bounds[i], bounds[i+1]);
                                   // letzter Wert = End-Marker; bounds[0] == region.offset()
    pub bindings: Vec<Binding>,    // nur Entries mit bekanntem Stamp; aufsteigend nach slot
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SinkError(pub String);

/// Austauschbare Sink. Send + Sync. Wird aus dem Blocking-Pool aufgerufen.
pub trait SegmentSink: Send + Sync {
    /// Übernimmt Besitz an Segment und Region. Bei Err bleibt die Region beim Modul (wird freigegeben).
    fn accept(&self, segment: LoadedSegment) -> Result<(), SinkError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransferConfig {
    /// Ausrichtung für Entry-Anfänge; Potenz von 2, 1..=256. Default 16.
    pub align: u64,
}
impl Default for TransferConfig {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TransferReport {
    pub bytes: u64,        // übertragene/geschriebene Bytes (Summe)
    pub segments: usize,   // an die Sink gemeldete Segmente
    pub entries_done: usize, // erledigte Entries (inkl. entfernte)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeviceTransferError {
    /// Abbruch/Panic des Blocking-Tasks, oder Warten auf Event fehlgeschlagen. direction ist immer H2D.
    Transfer(TransferError),
    /// Display: "Gather ohne Fortschritt"
    GatherNoProgress,
    /// Display: "Arena erschöpft"
    ArenaExhausted,
    /// config.align ungültig
    InvalidConfig,
    Backend(BackendError),
    Staging(StagingError),
    Sink(SinkError),
}
impl std::fmt::Display for DeviceTransferError {}
impl std::error::Error for DeviceTransferError {}

/// Send + Sync (static-assert).
pub struct DeviceTransfer { /* source, backend, ring, sink, config */ }

impl DeviceTransfer {
    pub fn new(
        source: Arc<dyn IndexArenaSource>,
        backend: Arc<GpuBackend>,
        ring: Arc<StagingRing>,
        sink: Arc<dyn SegmentSink>,
        config: TransferConfig,
    ) -> Result<DeviceTransfer, DeviceTransferError>;

    /// Ganzes Segment: direktes Kopieren, sonst Gather-Pfad. Läuft im Blocking-Pool.
    pub async fn load_segment(self: &Arc<Self>, entries: Vec<EntryId>)
        -> Result<TransferReport, DeviceTransferError>;
    /// Sehr kleine Menge per direktem Mapping (Fallback: Gather-Pfad). Läuft im Blocking-Pool.
    pub async fn load_mapped(self: &Arc<Self>, entries: Vec<EntryId>)
        -> Result<TransferReport, DeviceTransferError>;

    /// Synchrone Varianten (rufen den Ablauf direkt auf dem aktuellen Thread; nicht aus Async-Threads aufrufen).
    pub fn load_segment_blocking(&self, entries: &[EntryId]) -> Result<TransferReport, DeviceTransferError>;
    pub fn load_mapped_blocking(&self, entries: &[EntryId]) -> Result<TransferReport, DeviceTransferError>;
}
```

## 4. Behavior and invariants

**Allgemein**
- `new`: `align` keine Potenz von 2 oder nicht in 1..=256 → `InvalidConfig`.
- Die async-Methoden führen den Ablauf der `_blocking`-Variante per `page_advice::read_blocking` im Blocking-Pool aus. `BlockingError::Cancelled` und `Panicked` werden zu `Transfer(TransferError{direction: H2D, cause: EventError{device: backend.device(), cause: "cancelled" | "panicked"}})`. Eine Panic propagiert nicht.
- Eingabe: `entries` wird aufsteigend sortiert und dedupliziert. Eine leere Liste ergibt `Ok(TransferReport::default())`, ohne Sink-Aufruf und ohne Backend-Zugriff.
- Der Stamp jedes Entries wird immer vor dem Lesen seiner Daten per `Index::stamp` erfasst. Das Binding trägt diesen Stamp. Ändert sich der Entry danach, ist der Stamp älter als die Daten, nie neuer.
- Nur Entries mit `stamp == Some(_)` erhalten ein Binding.
- Jede Region hat die Länge der vom Backend gerundeten Größe. Offsets in `bounds` sind absolut: `region.offset()` + relativer Offset.
- Wird eine Region nach `allocate` nicht an die Sink übergeben (Fehler oder `Sink`-Fehler), wird sie freigegeben.
  - Fehler vor dem Absenden der Kopie: `backend.free`.
  - Fehler beim Warten auf das Event: `backend.free_after(region, event)`.
- Alle Instanzen sind nebenläufig nutzbar (`&self`). Verschiedene Aufrufe teilen den Ring. Jeder erhält distinkte Slots.

**`load_segment`: Zuordnung und Direktprüfung**

Die Entries werden aufsteigend `e[0..n)` zugeordnet. Direkt kopiert wird nur, wenn alle folgenden Punkte gelten. Sonst folgt ohne Fehler der Gather-Pfad (mit `log::debug!`):
1. `Index::access_methods()` enthält `Range`, und `byte_ranges(e[0], e[n-1])` ist `Ok`.
2. Es kommt genau ein Bereich `r` zurück (lückenlos, gestützt), mit `r.first == e[0]` und `r.count == n`.
3. Nicht leer: `r.len > 0` und `r.count > 0`. `r.len % r.count == 0`, und `stride = r.len / r.count`.
4. Ausgerichtet: `r.offset % align == 0` und `stride % align == 0`.
5. `source.range_bytes(&r)` ist `Ok` und liefert genau `r.len` Bytes.

**Direktes Kopieren** (Reihenfolge strikt)
1. `Advice::Sequential` auf `[r.offset, r.offset + r.len)` über `advice_target()`. Ein Fehler oder fehlendes Ziel wird nur mit `warn!` bzw. ignoriert behandelt.
2. Stamps aller Entries erfassen.
3. `backend.allocate(ArenaKind::IndexSegments, r.len, align)`.
4. `backend.upload(&region, HostSource::Pageable(..) | Pinned(..))` gemäß `ArenaBytes`.
5. `event.wait()` (synchron).
6. Bei Fehler wird die Region freigegeben (siehe Allgemein). Eine Backend-Fehler kommt als `Backend(..)`, ein Warte-Fehler als `Transfer(H2D)`.
7. `LoadedSegment`: `entries = e`, `bounds[i] = region.offset() + i*stride`, End-Marker `bounds[n] = region.offset() + r.len`. Bindings gibt es für Entries mit bekanntem Stamp. Danach `sink.accept`.

Report: `bytes = r.len`, `segments = 1`, `entries_done = n`.

**Gather-Pfad**
1. Stamps aller Entries werden einmal vorab erfasst, vor jedem Datenzugriff. Entries mit `None` gelten als entfernt.
2. Ist `Range` deklariert und liefert `byte_ranges(e[0], e[n-1])` `Ok`, wird je Bereich `Advice::Random` gesetzt. Fehler werden nur geloggt.
3. Wiederholt, bis alle Entries erledigt sind:
   - **Slot holen:** `ring.acquire()`. Das wartet auf freie Slots. `StagingError::Transfer(t)` wird zu `Transfer(t)`, jeder andere Fehler zu `Staging(..)`.
   - **Bepacken:** Die verbleibenden Entries werden in aufsteigender Reihenfolge verarbeitet (Gathering).
     - Entfernte Entries werden übersprungen, ohne Platz zu belegen, und als erledigt gezählt.
     - Sonst wird `Index::read(id).vector` gelesen. Anfang `off = align_up(cur, align)`. Passt der Entry (`off + len <= slot_size`), wird er bei `off` in den Slot geschrieben (`with_payload`). Der nächste Entry beginnt hinter ihm.
     - Beim ersten nicht passenden Entry endet das Bepacken. Dieser Entry bleibt für den nächsten Slot übrig. Sein bereits gelesener Payload darf behalten werden.
     - Passt schon der erste (nicht entfernte) Entry in einen leeren Slot nicht, wird der Slot per `ring.release` freigegeben und es kommt `ArenaExhausted`.
     - Ein Entry mit leerem Vektor belegt 0 Bytes, hat aber einen Slot in `bounds` (Binding bei bekanntem Stamp).
   - **Kopie:**
     - Packt der Lauf keinen Entry (nur entfernte oder keine Daten, Gesamtlänge 0), wird der Slot freigegeben (`release`). Es gibt keine Region, keinen Transfer und kein Segment. Die entfernten Entries zählen als erledigt.
     - Sonst `backend.allocate(IndexSegments, end, align)` mit `end` = Ende des letzten Entries. Bei Fehler `release(slot)` und `Backend(..)`.
     - Dann `backend.upload(&region, HostSource::Pageable(&slot[..end]))`. Die Slot-Bytes werden über `payload_ptr` gelesen. Bei Fehler `release(slot)`, Region freigeben, `Backend(..)`.
     - Mit dem zurückgegebenen Event: `ring.begin_transfer(slot, event.clone())`, dann `event.wait()`. Ein Fehler beim Warten ergibt `Transfer(H2D)`.
   - **Melden:** `LoadedSegment`, `bounds[k]` = absolute Entry-Anfänge, End-Marker `region.offset() + end`. Danach `sink.accept`, und `bytes += end`.
   - **Fortschritt:** Erledigte Entries dieses Durchlaufs = gepackte + entfernte. Sind es 0 (und kein `ArenaExhausted`), kommt `GatherNoProgress`.
4. Der Slot wird erst durch das abgeschlossene Event wieder frei (Ring-Garantie). Das Modul gibt einen InFlight-Slot nie selbst frei.
5. Ein Durchlauf erzeugt ein Segment (`segments += 1`). `entries_done` = n bei Erfolg.

**`load_mapped` (direktes Mapping)**
1. Stamps erfassen, dann `Index::read` der nicht entfernten Entries (aufsteigend). Offsets sind `align`-ausgerichtet (`off_i = align_up(cur)`), und `total` ist das Ende des letzten Entries.
2. Fällt zurück auf den Gather-Pfad (`log::warn!`, ohne Fehler), wenn eines davon zutrifft:
   - `backend.mapped_device_addr()` ist `None`, oder
   - `total == 0`, oder
   - `backend.allocate(ArenaKind::MappedHost, total, align)` scheitert.

   Der Fallback nutzt die Entries der Anfrage, mit neu erfassten Stamps.
3. Sonst wird je Entry `backend.mapped_write(&region, off_i, vector)` aufgerufen. Bei Fehler wird die Region freigegeben (`free`) und `Backend(..)` zurückgegeben.
4. `LoadedSegment` mit `region` aus `MappedHost`, `bounds` wie beim Gather (End-Marker `region.offset() + total`). Danach `sink.accept`.

Report: `bytes = total`, `segments = 1`. Es gibt keinen Staging-Slot, keine Kopie und keinen Event.

**Sink-Fehler:** Die Region wird per `backend.free` freigegeben und `Sink(e)` geliefert. Bereits gemeldete Segmente früherer Gather-Durchläufe bleiben bei der Sink. Der Aufruf bricht ab.

**Tests ohne GPU (müssen laufen):** `Display` der Fehler, Konfigurationsprüfung, `Send + Sync`-Assertions und die reine Zuordnungs- bzw. Packlogik (Direktprüfung, Bounds, Bindings, Entfernte). Tests mit echtem Upload laufen nur auf einem Nvidia-Device.

## 5. Relied-on elements

- **`gpu_backend::GpuBackend`** (`Send + Sync`):
  - `device() -> DeviceHandle`
  - `upload(&self, dst: &RegionHandle, src: HostSource<'_>) -> Result<Event, BackendError>`
    - Kopiert `src.len()` Bytes an den Anfang von `dst`, asynchron auf dem Copy-Stream.
    - `Pageable` darf nach der Rückkehr wiederverwendet werden.
    - `Pinned` muss bis zum Event unverändert bleiben.
  - `mapped_write(&self, &RegionHandle, offset: usize, &[u8]) -> Result<(), BackendError>`
    - Region in `MappedHost`. Sie führt kein Driver-Call aus.
  - `mapped_device_addr() -> Option<u64>`
  - Über `MemoryBackend`: `allocate(arena, size, align) -> Result<RegionHandle, _>`, `free`, `free_after(&RegionHandle, Event)`.
    - `size == 0` ist ein Fehler. `align` ist eine Potenz von 2, 1..=256.
    - Die Region ist auf 256 Byte gerundet und liegt bei 256-Byte-Offsets.
  - `enum HostSource<'a> { Pageable(&'a [u8]), Pinned(&'a PinnedBuffer) }`
  - `PinnedBuffer`: `Send + Sync`.
  - `BackendError`: `Clone + Debug + PartialEq + Eq`.
- **`staging_ring::StagingRing`** (`Send + Sync`):
  - `acquire() -> Result<SlotId, StagingError>` (blockierend).
  - `with_payload`, `payload_ptr(slot) -> Result<*mut u8, _>`. Er ist nur gültig, solange der Slot Occupied oder InFlight ist.
  - `begin_transfer(slot, Event)`: Occupied → InFlight.
  - `release(slot)`: Occupied → Free.
  - `slot_size()`.
  - `StagingError::{Exhausted, InvalidSlot, Transfer(TransferError), InvalidConfig}`.
  - `TransferError{direction: TransferDirection::H2D, cause: EventError}`.
  - Der Slot wird nur nach abgeschlossenem Event wiederverwendet.
- **`device`:**
  - `Event`: `Clone + Send + Sync`. `wait() -> Result<(), EventError>`.
  - `EventError{device: DeviceHandle, cause: String}`.
  - `DeviceHandle`.
- **`region_handle::RegionHandle`:** `Copy + Eq + Hash`, `offset()`, `len()`, `arena()`.
- **`arenas::ArenaKind::{IndexSegments, MappedHost}`.**
- **`index_contract`:**
  - `Index: Send + Sync`
  - `stamp(EntryId) -> Option<Stamp>` (berührt keine Daten)
  - `read(EntryId) -> Payload`
    - Darf blockieren.
    - Bei fehlendem Entry sind beide `SharedBytes` leer.
  - `byte_ranges(first, last) -> Result<Vec<ByteRange{first,count,offset,len}>, IndexError>`
    - Range-Zugriff, nur wenn `AccessMethod::Range` deklariert ist.
    - Sonst `Unsupported`.
  - `access_methods()`
  - `EntryId`: `Copy + Ord + Hash`.
  - `Stamp`: `Copy + Eq`. Er wird hier nur auf Gleichheit genutzt.
- **`page_advice`:**
  - `AdviceTarget`, `Advice::{Sequential, Random}`, `ByteRange{start,end}` (anderer Typ als `index_contract::ByteRange`).
  - `advise(&dyn AdviceTarget, Advice, ByteRange) -> Result<(), AdviceError>`.
  - `read_blocking(f) -> Result<T, BlockingError>`, mit `Cancelled` und `Panicked`.
  - Ein leerer Bereich ist ein No-op.
- **Logging:** `log`-Crate (`debug!`, `warn!`).