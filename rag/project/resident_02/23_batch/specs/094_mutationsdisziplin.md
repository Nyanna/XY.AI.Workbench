# Spezifikation: Mutationsdisziplin (Build-Order 94)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe.** Rust-Modul `mutation`. Es setzt die Regeln, nach denen der Host geteilte Index-Strukturen verändert, während ein Such-Kernel auf ihnen liest. Es erzwingt sie über Typen.

- Es verändert nur innerhalb einer offenen Doorbell-Gap, nie unter dem Kernel.
- Es tauscht atomar ausschließlich einen natürlich ausgerichteten 8-Byte-Pointer.
- Größere Strukturen ersetzt es per Copy-on-Write: neu schreiben, dann den Pointer tauschen.
- Updates in-place in einer Gap sind erlaubt. Im Persistent-Modus akzeptiert und dokumentiert es Torn Reads (gemischte, aber gültige Werte).

**Nicht Teil des Moduls:**
- Kein Allokieren, Freigeben oder Deferred Free. Der Tausch gibt den alten Wert zurück, und der Aufrufer gibt ihn frei, nachdem die Subengine ihn nicht mehr liest.
- Keine Kernel-Starts oder Batches, keine Events. Es erzeugt weder Gaps noch ruft es `Gap::close` auf. Das tut der Aufrufer.
- Kein Aufbau von Segment-Inhalten, Layouts oder Offset-Tabellen. Der Aufrufer liefert sie fertig geschrieben.
- Kein CUDA-Aufruf. Es nutzt nur `std`, `doorbell` und `memory_access`. `Cargo.toml` wird nicht angefasst.
- Keine Locks auf Datenpfaden. Kein Retry, kein Warten, kein Blockieren des Kernels.
- Kein `unsafe` außer in `PtrCell::from_raw` und den atomaren Zugriffen darauf.

## 2. Kontext

- Die Doorbell-Synchronisation liefert `Gap<'_>`, die offene Lücke (acked, completed]. Der Host verändert geteilte Daten nur zwischen dem Abschluss eines Batches und dem Ack. Der Ack erfolgt über `Gap::close` (Release).
- Hierarchie der geteilten Strukturen im Device-sichtbaren Speicher:
  - **Root-Pointer** zeigt auf die **Segmenttabelle**.
  - Die Segmenttabelle verweist auf **Segmente**.
  - Ein Segment besteht aus **Basis, Layout und Offset-Pointer**. Dieser zeigt auf die Offset-Tabelle (`Variable`) und ist bei `Fixed` unbenutzt.
  - Die Typen `SegmentDesc` und `SegmentLayout` stammen aus `kernel_batch`, `SegmentStore` aus `segments`. Dieses Modul definiert sie nicht neu.
- Lookup-Tabellen und ganze Segmente werden per Copy-on-Write ersetzt. Der Pointer wird getauscht, sobald die Subengine dort nicht mehr liest.
- Der Kernel ist entweder batchweise aktiv (zwischen Gaps liest er, in einer Gap ruht er für diese Strukturen) oder persistent. Im Persistent-Modus liest er auch während der Gap weiter.
- Stale Ergebnisse sind kein Fehler.

## 3. Interface

```rust
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use doorbell::Gap;
use memory_access::{Region, AccessError};

/// Laufzustand des Kernels, vom Aufrufer geliefert.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelMode {
    /// Batchweise: In einer Gap liest der Kernel diese Strukturen nicht.
    Batch,
    /// Dauerlaufend: Der Kernel kann auch während der Gap lesen (Torn Reads möglich).
    Persistent,
}

/// Ebene der Hierarchie, nur zur Fehlerzuordnung und Diagnose.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level { Root, SegmentTable, Segment, OffsetTable, LookupTable }

/// Ein natürlich ausgerichteter 8-Byte-Pointer-Slot im geteilten Speicher.
/// Der Inhalt ist eine Device-Adresse oder ein Handle als u64. Send + Sync.
pub struct PtrCell { /* NonNull<AtomicU64> + keepalive Arc, level */ }
impl PtrCell {
    /// Host-only (Heap), Anfangswert `initial`. Für Tests und CPU.
    pub fn new_local(level: Level, initial: u64) -> PtrCell;
    /// # Safety: `ptr` ist 8-aligned, gültig für 8 Byte Lesen/Schreiben, wird nur
    /// atomar zugegriffen und bleibt gültig, solange `keepalive` lebt.
    pub unsafe fn from_raw(level: Level, ptr: NonNull<AtomicU64>,
        keepalive: Arc<dyn std::any::Any + Send + Sync>) -> PtrCell;
    pub fn level(&self) -> Level;
    /// Acquire-Load. Beliebig, auch ohne Gap, lock-frei.
    pub fn load(&self) -> u64;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MutationError {
    /// Die Gap ist leer oder ungültig. Nicht erreichbar mit `Gap`, siehe Verhalten.
    NoGap,
    /// Neuer Pointer ist 0 (Null ist kein gültiges Ziel).
    NullPointer,
    /// Neuer Wert ist nicht auf 8 Byte ausgerichtet.
    Misaligned { value: u64 },
    /// In-place-Schreiben im Batch-Modus mit nicht erlaubtem Zustand, siehe Verhalten.
    ModeViolation { mode: KernelMode },
    Access(AccessError),
}
impl std::fmt::Display for MutationError {}
impl std::error::Error for MutationError {}

/// Beleg, dass eine Gap offen ist. Leiht die Gap nur lesend; schließt sie nie.
/// Send + Sync nicht erforderlich; nicht Clone.
pub struct GapGuard<'g> { /* &'g Gap<'_>, mode */ }

impl<'g> GapGuard<'g> {
    /// Der Aufrufer hält die Gap offen und gibt den Kernel-Modus an.
    pub fn enter(gap: &'g Gap<'_>, mode: KernelMode) -> GapGuard<'g>;
    pub fn mode(&self) -> KernelMode;
    /// Gap-Grenzen, durchgereicht (`first`, `last`).
    pub fn gap_range(&self) -> (u64, u64);

    /// Copy-on-Write-Publish: tauscht den Pointer atomar (8 Byte, Release) auf `new`.
    /// `new` zeigt auf eine bereits vollständig geschriebene neue Struktur.
    /// Gibt den alten Wert zurück; der Aufrufer gibt die alte Struktur erst frei,
    /// wenn die Subengine sie nicht mehr liest.
    pub fn swap_ptr(&self, cell: &PtrCell, new: u64) -> Result<u64, MutationError>;

    /// In-place-Update kleiner Daten (z. B. Vektorwerte, Slot-Bytes) in einer Gap.
    pub fn write_in_place(&self, region: &Region, offset: usize, src: &[u8])
        -> Result<(), MutationError>;
}

const _: () = { fn _a<T: Send + Sync + 'static>() {} fn _b() { _a::<PtrCell>(); _a::<MutationError>(); } };
```

## 4. Behavior und Invarianten

**Gap-Pflicht.**
- `swap_ptr` und `write_in_place` sind nur über einen `GapGuard` erreichbar. Der Guard braucht eine offene `&Gap`, also ist der Aufrufer zwischen Batch-Abschluss und Ack. Außerhalb einer Gap sind Mutationen durch das Typsystem unmöglich.
- `GapGuard` schließt oder acked die Gap nie. Es ruft weder `Gap::close` noch `Doorbell::poll` oder `wait` auf. Es wartet nie, auf nichts.
- Der Guard darf die Gap nicht überleben (Lifetime `'g`).
- `MutationError::NoGap` ist reserviert und wird vom Modul nie erzeugt.

**Pointer-Tausch (`swap_ptr`).**
- Er ist genau ein atomarer 8-Byte-Swap auf einem `AtomicU64` (`swap(new, Release)`). Er ist nie ein Mehr-Wort-Update und nie ein `memcpy`.
- Reihenfolge: Der Aufrufer schreibt die neue Struktur vollständig. Danach tauscht `swap_ptr` mit Release. Ein Leser, der den neuen Pointer per Acquire sieht, sieht auch die vollständige neue Struktur.
- `new == 0` ergibt `NullPointer`. `new % 8 != 0` ergibt `Misaligned { value: new }`. In beiden Fällen bleibt die Zelle unverändert.
- Bei Erfolg gibt `swap_ptr` den bisherigen Wert zurück. Ein Leser sieht entweder den alten oder den neuen Pointer, nie einen Mischwert. Ein Leser kann noch den alten sehen (stale), und das ist kein Fehler.
- Das Modul gibt die alte Struktur nie selbst frei und überschreibt sie nicht. Sie bleibt unverändert gültig, bis der Aufrufer sie freigibt.
- Nur das 8-Byte-Wort wird getauscht. Größere Strukturen (Segmenttabelle, Segment, Offset-/Lookup-Tabelle, ganzes Segment) werden nie in place ersetzt, sondern immer per COW neu angelegt und dann über ihren Pointer (`Root` → Segmenttabelle → Segment → Offset-Pointer) getauscht. Die Ebene wird in `PtrCell` geführt, aber nicht weiter validiert.
- Der Tausch erfolgt in beiden Modi, auch bei `Persistent`. Der Aufrufer tauscht erst, wenn die Subengine an dieser Stelle nicht mehr liest. Das Modul prüft das nicht.

**In-place (`write_in_place`).**
- In einer Gap sind In-place-Updates trivial: Der Guard schreibt `src` per `Region::write` ab `offset`. Grenzprüfung und `AccessError` kommen von `Region`, abgebildet auf `MutationError::Access`. Bei einem Fehler gilt kein Teilschreib-Rollback.
- `src.len() == 0` gelingt ohne Zugriff.
- Im Modus `Batch` ist das Schreiben erlaubt, weil der Kernel in der Gap ruht.
- Im Modus `Persistent` ist es ebenfalls erlaubt. Der Kernel kann dann einen halb aktualisierten Vektor lesen: gemischte, aber gültige Werte (Torn Read). Das wird bewusst akzeptiert und ist kein Fehler. Das Modul synchronisiert nicht dagegen. Kein Lock und keine Wiederholung.
- Konsistenz mehrteiliger Daten darf nicht auf In-place-Schreiben beruhen. Dafür gilt COW mit Pointer-Tausch.
- `ModeViolation` ist reserviert und wird nicht erzeugt.

**Nie unter dem Kernel.**
- Das Modul nimmt nur Regionen und Zellen entgegen, die der Aufrufer als Datenstrukturen übergibt. Es schreibt nie Kernel-Code, Kernel-Parameter oder Strukturen, die der Kernel gerade ausführt. Die API bietet keinen Weg dorthin.

**Nebenläufigkeit.**
- Keine Locks auf Datenpfaden, keine Panics bei Poisoning. `PtrCell::load` und `swap_ptr` sind lock-frei.
- `PtrCell` und `MutationError` sind `Send + Sync + 'static` (Compile-Zeit-Assert).
- Mehrere `swap_ptr` auf dieselbe Zelle sind atomar. Der letzte Tausch gewinnt, und jeder Aufrufer erhält den Wert, den er ersetzt hat.
- Der Host bleibt der einzige Schreiber. Das Modul erzwingt das nicht über Locks.
- Stale Ergebnisse (Leser sieht noch den alten Pointer) sind kein Fehler.

**Tests ohne GPU.**
- `swap_ptr` liefert den alten Wert, `load` den neuen.
- `NullPointer` und `Misaligned` lassen die Zelle unverändert.
- Ein Reader-Thread sieht nur den alten oder den neuen Wert, nie einen anderen (viele Iterationen).
- Release/Acquire: Eine vor dem Swap geschriebene Struktur ist beim Lesen des neuen Pointers vollständig sichtbar.
- Der Guard schließt die Gap nicht: Nach `enter` und `swap_ptr` ist `acked` unverändert, bis der Aufrufer `close` ruft.
- `write_in_place` wirkt auf `Region` (Mock), Grenzfehler ergeben `Access`, Länge 0 gelingt.
- Dokumentierter Torn-Read-Test im Modus `Persistent`: Ein Leser sieht nur Werte aus dem alten oder dem neuen Zustand je Element.

## 5. Relied-on elements

- `doorbell::Gap<'a>`:
  - `#[must_use]`, schließt mit `close(self)` (Release-Ack).
  - Methoden `first() -> u64`, `last() -> u64`, `len() -> u64`.
  - Exklusive Ausleihe von `Doorbell`, also höchstens eine offene Gap gleichzeitig.
  - Drop ohne `close` panict.
  - Dieses Modul ruft `close` nie auf.
- Der Host verändert geteilte Daten (Ergebnisse, Zuordnungen) nur zwischen Batch-Abschluss und Ack. Ein Release-`close` macht die Host-Schreibzugriffe für das Gerät sichtbar, sobald es `acked` per Acquire liest.
- `memory_access::Region`: `Clone + Send + Sync`, `write(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>`. Es prüft Grenzen selbst, und `AccessError` ist `Clone + Eq`.
- `kernel_batch::{SegmentDesc, SegmentLayout}` (`Fixed { stride }`, `Variable { offsets }`) und `segments::SegmentStore`: Strukturen, die über die Hierarchie Root → Segmenttabelle → Segment (Basis, Layout, Offset-Pointer) erreichbar sind. Dieses Modul nutzt sie nicht direkt und definiert sie nicht neu.
- Freigabe alter Strukturen: Allocator-Deferred-Free (`free_deferred(&Region, Event)`) liegt beim Aufrufer, nicht in diesem Modul.