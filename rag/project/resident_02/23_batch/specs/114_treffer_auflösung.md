# Spezifikation: Treffer-Auflösung (Build-Order 114)

Crate-Modul `hit_resolution`.

## 1. Task, restrictions and boundaries

**Aufgabe**
- Kernel-Treffer (`KernelHit`) in Segment-Generation, Slot und Score umrechnen (`locate`).
- Über ein `Manifest` und die Slot-Zuordnung des Segments zu einer `EntryRef` auflösen.
- Den neuesten gültigen Ort eines Entries ermitteln.
- Treffer älterer Manifest-Generationen überführen oder verwerfen.
- Aufgelöste Treffer mit Entry-Referenz, Score und Flag `stale` liefern.

**Grenzen**
- Das Modul ist zustandslos und rein lesend. Es hat keine Locks, keinen Cache, kein `unsafe` und keine Panics bei gültigen Eingaben.
- Es führt keinen Kernel aus und publiziert keine Manifeste.
- Es ändert weder `SlotAssignment` noch `SlotStates` und hält keine Leases. Die Lease auf das Manifest hält der Aufrufer.
- Es definiert keine Typen der Abhängigkeiten neu.
- Es erfindet keine Policy. Es gibt keine Deduplizierung, keine Sortierung und kein Top-k.

## 2. Context

- Der Kernel läuft über die Segmenttabelle eines Manifests (`Manifest::kernel_table`) und meldet `KernelHit { addr, score }`.
- Jedes Segment (`SegmentSpec`) hat `id`, Segment-Generation, Startadresse, Slot-Anzahl und Slot-Abstand. Abstand 0 bedeutet variables Layout.
- Pro Segment gibt es eine `SlotAssignment` (Slot ↔ `EntryRef`) und `SlotStates` (Valid-Bits). Beide gehören dem Manager.
- Dieses Modul erhält sie über den Trait `SegmentDirectory`, den der Manager implementiert.
- Ein Treffer kann aus einem Kernel-Lauf auf einem älteren Manifest stammen. Dann ist `hit_generation < current.generation()`. Das Segment kann inzwischen ersetzt oder der Slot invalidiert sein.

## 3. Interface

```rust
use std::sync::Arc;
use kernel_layout::{KernelHit, Slot};
use manifest::{Manifest, ManifestGeneration, SegmentId};
use region_handle::Generation;
use slot_layout::{EntryRef, SlotAssignment, SlotStates};

/// Slot-Zuordnung und Valid-Bits eines Segments.
#[derive(Clone)]
pub struct SegmentBinding {
    pub assignment: Arc<SlotAssignment>,
    pub states: Arc<SlotStates>,
}

/// Vom Manager implementiert. Muss nicht blockieren.
pub trait SegmentDirectory: Send + Sync {
    /// None, wenn für die ID nichts bekannt ist.
    fn binding(&self, id: SegmentId) -> Option<SegmentBinding>;
}

/// Umgerechneter Treffer: Segment-Generation, Slot, Score (unverändert).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitLocation { pub generation: Generation, pub slot: Slot, pub score: f32 }

/// Aufgelöster Treffer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedHit { pub entry: EntryRef, pub score: f32, pub stale: bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    /// Display: "Slot außerhalb des Bereichs"
    SlotOutOfRange,
    /// Display: "stale Generation"
    StaleGeneration,
    /// Display: "fehlende Zuordnung" (Text enthält Generation und Slot)
    MissingMapping { generation: Generation, slot: Slot },
}
impl std::fmt::Display for ResolveError {}
impl std::error::Error for ResolveError {}

/// Ergebnis eines Laufs; `hits` in Eingabereihenfolge.
#[derive(Debug, Clone, PartialEq)]
pub struct Resolution { pub hits: Vec<ResolvedHit>, pub discarded: usize }

/// Adresse + Score -> Segment-Generation, Slot, Score, bezogen auf `manifest`
/// (das Manifest, dessen Tabelle der Kernel gelesen hat).
pub fn locate(manifest: &Manifest, dir: &dyn SegmentDirectory, hit: KernelHit)
    -> Result<HitLocation, ResolveError>;

/// Segment-Generation + Slot -> EntryRef im Manifest.
pub fn resolve_location(manifest: &Manifest, dir: &dyn SegmentDirectory,
                        generation: Generation, slot: Slot)
    -> Result<EntryRef, ResolveError>;

/// Neuester gültiger Ort des Entries im Manifest.
pub fn find_current_location(manifest: &Manifest, dir: &dyn SegmentDirectory, entry: EntryRef)
    -> Option<(Generation, Slot)>;

/// Überführt Treffer aus einem Lauf auf Manifest-Generation `hit_generation`.
pub fn transfer(current: &Manifest, previous: Option<&Manifest>, dir: &dyn SegmentDirectory,
                hit_generation: ManifestGeneration, locations: &[HitLocation]) -> Resolution;

/// `locate` auf `hit_manifest`, danach `transfer` mit `hit_generation = hit_manifest.generation()`.
/// Treffer, die `locate` ablehnt, zählen als verworfen.
pub fn resolve_hits(hit_manifest: &Manifest, current: &Manifest, previous: Option<&Manifest>,
                    dir: &dyn SegmentDirectory, hits: &[KernelHit]) -> Resolution;
```

`SegmentBinding`, `HitLocation`, `ResolvedHit`, `ResolveError` und `Resolution` sind `Send + Sync`.

## 4. Behavior and invariants

**`locate`**
- Es sucht in `manifest.segments()` das Segment, dessen Adressbereich `addr` enthält.
  - Fest (`slot_stride() > 0`): Bereich `[start, start + slot_count*stride)`, gerechnet in u128. Slot = `(addr - start) / stride`.
  - Variabel (`slot_stride() == 0`): Das Modul holt `dir.binding(id)`. Der Bereich ist `[start, start + bytes())`. Der Slot ist der größte Slot `i < min(slot_count, assignment.slot_count())` mit `device_addr(i) <= addr`, per Binary Search. Er zählt nur, wenn `addr < device_addr(i) + Länge(i)`. Die Länge ist `byte_range(i).end - byte_range(i).start`. Slots mit Länge 0 werden nie geliefert. Fehlt das Binding, wird das Segment übersprungen.
- Ergebnis: `HitLocation { generation: spec.generation(), slot, score: hit.score }`. Der Score bleibt bitgleich, auch bei NaN.
- Liegt die Adresse in keinem Segment, kommt `Err(SlotOutOfRange)`. Das gilt auch bei leerem Manifest oder `slot_count == 0`.
- Es gibt keinen Panic bei Adressen nahe `u64::MAX`.

**`resolve_location`**
- Es sucht das erste Segment im Manifest mit `spec.generation() == generation`. Gibt es keines, kommt `Err(StaleGeneration)`. Das gilt auch, wenn `dir.binding(id)` `None` liefert.
- Dann gilt `assignment.lookup(slot)`:
  - `Some(e)` ergibt `Ok(e)`.
  - `None` ergibt `Err(MissingMapping { generation, slot })`. Das gilt auch für `slot >= slot_count`.
- Es ändert nichts.

**`find_current_location`**
- Es durchläuft die Segmente absteigend nach `generation().0`. Bei gleicher Generation hat das spätere Segment im Manifest Vorrang.
- Ein Segment liefert den Ort, wenn alle drei Bedingungen gelten:
  - `dir.binding` ist `Some`.
  - `assignment.slot_of_entry(entry)` ist `Some(slot)`.
  - `states.is_valid(slot)` ist `true`.
- Das erste solche Segment gewinnt: `Some((spec.generation(), slot))`. Segmente ohne Binding, ohne Zuordnung oder mit ungültigem Slot werden übersprungen. Ein neueres Segment mit ungültigem Slot blockiert also ein älteres gültiges nicht.
- Sonst ist das Ergebnis `None`.

**`transfer`**
- `stale = hit_generation < current.generation()`. Das Flag gilt für alle Treffer des Aufrufs.
- Pro Location, in Eingabereihenfolge, ohne Umsortierung und ohne Deduplizierung:
  1. Auflösung per `resolve_location(current, …)`. Bei jedem `Err` folgt, falls `previous` `Some` ist, `resolve_location(previous, …)`. Hat auch das `Err`, oder ist `previous` `None`, wird der Treffer verworfen.
  2. Das Binding des Segments (per ID aus dem Manifest, in dem aufgelöst wurde) ist die Grundlage der Gültigkeitsprüfung. Ist `states.is_valid(slot)`, bleibt der Treffer: `ResolvedHit { entry, score, stale }`.
  3. Sonst sucht `find_current_location(current, dir, entry)`:
     - `Some(_)` ergibt `ResolvedHit { entry, score, stale }` mit unverändertem Score. Die Entry-Referenz bleibt gleich. Nur der Ort ist neu.
     - `None` heißt verworfen.
- `discarded` ist die Zahl der Verworfenen. Es gilt `hits.len() + discarded == locations.len()`.
- Ist `hit_generation == current.generation()`, laufen dieselben Schritte mit `stale = false`.
- Ein Segment, das nur in `previous` liegt, wird aufgelöst und auf Gültigkeit geprüft. Ist der Slot dort ungültig, wird der aktuelle Ort gesucht.
- `transfer` hat keine Nebenwirkungen und ist beliebig parallel aufrufbar.

**`resolve_hits`**
- Das Modul ruft für jeden Treffer `locate(hit_manifest, …)` auf. Bei `Err` zählt der Treffer als verworfen.
- Die übrigen Locations gehen in Eingabereihenfolge durch `transfer`. `discarded` ist die Summe beider Stufen.

**Concurrency**
- Alle Funktionen sind rein lesend. Sie halten nur kurz die Lese-Locks von `SlotAssignment` und nehmen keine Locks selbst.
- Zwischen Auflösung und Gültigkeitsprüfung kann sich der Zustand nebenläufig ändern. Das Ergebnis ist ein Schnappschuss ohne Atomizität über mehrere Aufrufe.
- Der Aufrufer hält Leases auf `hit_manifest`, `current` und `previous` für die Dauer des Aufrufs.

## 5. Relied-on elements

- **`manifest`**
  - `Manifest`: `generation() -> ManifestGeneration (u64)`, `segments() -> &[SegmentSpec]` (Reihenfolge wie übergeben).
  - `SegmentSpec` (Copy): `id() -> SegmentId`, `generation() -> Generation`, `start() -> Addr`, `slot_count() -> u32`, `slot_stride() -> u32` (0 = variabel), `bytes() -> u64`.
  - `SegmentId(pub u64)`: Copy, Eq, Ord.
- **`slot_layout`**
  - `SlotAssignment`: `lookup(Slot) -> Option<EntryRef>` (`None` für ungebunden und für ungültig), `slot_of_entry(EntryRef) -> Option<Slot>`, `device_addr(Slot) -> Result<Addr, SlotError>`, `byte_range(Slot) -> Result<Range<u64>, SlotError>`, `slot_count() -> u32`. Send + Sync, `&self`.
  - `SlotStates`: `is_valid(Slot) -> bool` (außerhalb false), Send + Sync.
  - `EntryRef { entry: EntryId, section: u32 }`: Copy, Eq, Hash, Ord.
- **`kernel_layout`**: `Addr(pub u64)`, `Slot(pub u32)` (Copy, Eq, Ord), `KernelHit { addr: Addr, score: f32 }` (Copy).
- **`region_handle`**: `Generation(pub u64)`, Copy, Eq.
- **`index_contract`**: `EntryId(pub u64)`, indirekt über `EntryRef`.