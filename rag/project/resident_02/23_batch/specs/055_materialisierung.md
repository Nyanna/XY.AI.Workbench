# Spezifikation: Materialisierung (Build-Order 55)

## 1. Task, restrictions and boundaries

**Task.** Rust-Modul `materialization`. Es führt das vom Planner gesetzte Materialization-Flag für ein fertiges Ergebnis aus. Es gibt zwei Fälle:
- `ToHost`: Das Ergebnis wird ins RAM zurückgeholt.
- `KeepOnDevice`: Das Ergebnis bleibt als Referenz im VRAM.

**Nicht Teil des Moduls:**
- Das Modul entscheidet nicht, welches Flag gilt. Das setzt der Planner.
- Es allokiert und gibt keinen VRAM frei, führt keine Eviction aus und vergibt keine Generationen.
- Es löst `RegionHandle` nicht auf Device-Adressen auf und enthält keine Raw-Pointer, Device-Adressen oder `unsafe`.
- Es führt keine Wertekonvertierung durch und keine Tensor-Arithmetik.
- Es redefiniert weder `RegionHandle`, `Region`, `TensorRef`, `Layout` noch deren Fehlertypen.
- Es gibt keine Abhängigkeit außer `std`, `region_handle` und `memory_access` (mit deren Abhängigkeiten `layout` und `arenas`).

## 2. Context

- Ein Kernel-Ergebnis liegt im VRAM. `RegionHandle` ist die pointerfreie Referenz auf diese Region. `TensorRef` (Region + Layout + Offset) ist der Zugriff auf die Bytes über `Region::read`.
- Der Planner setzt je Ergebnis ein Flag. Dieses Modul setzt es um und liefert das Ergebnis entweder als Host-`TensorRef` oder als unveränderte Device-Referenz.
- Das RAM-Ziel ist ein privater Host-Puffer. Er implementiert `RegionBackend` und wird in eine `Region` verpackt.
- Das Flag ist in diesem Modul als `Materialization` definiert. Der Planner übergibt es als Wert.

## 3. Interface

```rust
use memory_access::{AccessError, Region, TensorRef, TensorRefError};
use region_handle::RegionHandle;

/// Vom Planner gesetztes Flag.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Materialization {
    /// Ergebnis beim Fertigwerden ins RAM kopieren.
    ToHost,
    /// Ergebnis bleibt im VRAM; nur die Referenz wird weitergegeben.
    KeepOnDevice,
}

/// Fertiges Ergebnis im VRAM: Referenz plus Zugriff auf die Bytes.
#[derive(Clone, Debug)]
pub struct DeviceResult {
    /* handle: RegionHandle, tensor: TensorRef; Felder privat */
}
impl DeviceResult {
    pub fn new(handle: RegionHandle, tensor: TensorRef) -> DeviceResult;
    pub fn handle(&self) -> RegionHandle;
    pub fn tensor(&self) -> &TensorRef;
}

/// Ergebnis der Materialisierung.
#[derive(Clone, Debug)]
pub enum Materialized {
    /// Im RAM; unabhängige Kopie.
    Host(TensorRef),
    /// Weiterhin im VRAM; unverändertes DeviceResult.
    Device(DeviceResult),
}
impl Materialized {
    pub fn is_host(&self) -> bool;
    pub fn is_device(&self) -> bool;
    pub fn tensor(&self) -> &TensorRef;
    /// Some nur bei Device.
    pub fn handle(&self) -> Option<RegionHandle>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterializeError {
    Access(AccessError),
    Tensor(TensorRefError),
}
impl std::fmt::Display for MaterializeError { /* ... */ }
impl std::error::Error for MaterializeError {}
impl From<AccessError> for MaterializeError {}
impl From<TensorRefError> for MaterializeError {}

/// Führt das Flag aus. Synchron; blockiert nur für die Dauer des Region-Lesens.
pub fn materialize(flag: Materialization, result: DeviceResult)
    -> Result<Materialized, MaterializeError>;
```

`Materialization`, `DeviceResult`, `Materialized` und `MaterializeError` sind `Send + Sync + 'static`. Das wird per statischem Test geprüft.

## 4. Behavior and invariants

**KeepOnDevice**
- `materialize` gibt `Materialized::Device(result)` zurück, mit demselben Handle und demselben `TensorRef`.
- Es findet kein Lesen, Schreiben oder Kopieren statt. Die Region wird nicht berührt, und das Backend wird nicht aufgerufen.
- Es gibt keine Fehler.

**ToHost**
- Sei `span = tensor.end_bytes() - tensor.offset_bytes()`. `span` ist die Spannweite des Layouts und ist 0 bei `elem_count == 0`.
- Das Modul liest genau die Bytes `[offset_bytes, end_bytes)` per `Region::read` in einen neuen Host-Puffer der Länge `span`.
- Danach liefert es `Materialized::Host(t)`. Dabei gilt:
  - `t` wird per `TensorRef::new(host_region, layout.clone())` mit Offset 0 gebaut.
  - `t.layout()` ist gleich dem Quell-Layout, also gleiche Shape, gleiche Byte-Strides und gleicher DType. Es findet keine Umordnung oder Kompaktierung statt.
  - `t.offset_bytes() == 0`.
  - `host_region.len() == span`.
- Die Bytes von `t` sind identisch mit den Quellbytes im Bereich `[offset_bytes, end_bytes)`. Bytes dazwischen, die zur Spannweite gehören, aber nicht zu Elementen, werden mitkopiert.
- Die Host-Region ist unabhängig von der Quell-Region: `!t.region().same_region(src.region())`. Spätere Schreibzugriffe auf die Quelle ändern die Kopie nicht.
- Bei `span == 0` wird kein `read` aufgerufen. Das Ergebnis ist eine leere Host-Region mit gültigem `TensorRef`.
- Das Handle wird verbraucht und nicht weitergegeben. Das Modul gibt den VRAM nicht frei. Das bleibt Sache des Eigentümers der VRAM-Region.
- Die Generation des Handles wird nicht geprüft, weil das Modul den aktuellen Zähler nicht kennt. Die Prüfung ist Sache des Aufrufers oder des Backends.
- Fehler:
  - Ein `Region::read`-Fehler wird als `MaterializeError::Access(e)` unverändert durchgereicht, auch `AccessError::Backend`.
  - Ein Fehler beim Bauen des Host-`TensorRef` wird als `MaterializeError::Tensor(e)` durchgereicht. Er ist bei konsistentem Eingang nicht erreichbar.
  - Im Fehlerfall wird kein Teilergebnis zurückgegeben.

**Host-Puffer (intern, aber vorgeschrieben)**
- Der Puffer ist ein privater Typ, der `RegionBackend` implementiert (`Send + Sync + Debug`, `len` konstant).
- Er hält die Bytes in einem `Vec<u8>` hinter einem Lock, damit `&self`-Lese- und Schreibzugriffe möglich sind.
- Er erfüllt die `Region`-Garantien: Read-your-writes, `OutOfBounds`-Prüfung liegt bei `Region`, und es gibt kein UB.

**Nebenläufigkeit**
- `materialize` hat keinen globalen Zustand und hält keine Locks über den Aufruf hinaus. Mehrere Threads dürfen gleichzeitig für verschiedene oder dieselben Ergebnisse aufrufen.
- Das Lesen der Quelle ist ein einzelner `Region::read`. Schreibt jemand gleichzeitig auf die Quellbytes, gilt die Verschränkungssemantik von `Speicherzugriff`. Das Modul serialisiert das nicht.
- Es gibt keine Panics bei gültigen Eingaben.

## 5. Relied-on elements

Aus `memory_access` (unverändert):
```rust
pub struct Region; // Clone, Send+Sync
impl Region {
    pub fn new(backend: Arc<dyn RegionBackend>) -> Region;
    pub fn len(&self) -> usize;
    pub fn read(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError>;
    pub fn same_region(&self, other: &Region) -> bool;
}
pub trait RegionBackend: Send + Sync + std::fmt::Debug {
    fn len(&self) -> usize;
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError>;
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>;
}
// AccessError { OutOfBounds{offset,len,region_len}, Backend(String) }
// TensorRefError { Layout(LayoutError), ExceedsRegion{required_end,region_len} }
impl TensorRef {
    pub fn new(region: Region, layout: Layout) -> Result<TensorRef, TensorRefError>; // Offset 0
    pub fn region(&self) -> &Region;
    pub fn layout(&self) -> &Layout;
    pub fn offset_bytes(&self) -> usize;
    pub fn end_bytes(&self) -> usize; // offset + Spannweite, exklusiv
}
```
- Die Spannweite ist 0 bei `elem_count == 0`. Sonst ist sie `Σ (shape[i]-1)*strides[i] + dtype.size_in_bytes()`.
- Ein `read` mit Länge 0 ist ein No-op ohne Backend-Aufruf.
- `Layout` ist `Clone` und `Send + Sync`. Es enthält keinen Byte-Offset.

Aus `region_handle` (unverändert):
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RegionHandle { /* arena, offset, len, generation */ }
```
- `RegionHandle` ist ein pointerfreier `Copy`-Wert, `Send + Sync + 'static`. Er löst keine Adressen auf.
- Die Auflösung auf Device-Adressen ist ausschließlich Sache des Memory-Backends.