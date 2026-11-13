# Spezifikation: Speicherzugriff (Build-Order 32)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Rust-Modul `memory_access` mit zwei Teilen:
1. Eine einheitliche, nebenläufig nutzbare Schnittstelle zum Lesen und Schreiben von Bytes einer Region einer Memory-Domain.
2. `TensorRef`, das Region und `Layout` verbindet. Es ist der einzige Weg, auf dem Tensoren die Encoder-Schnittstelle passieren.

**Einschränkungen:**
- Das Modul enthält keine Allokation, kein Domain-Management, keine Wertekonvertierung und keine Arithmetik auf Elementen. Es kennt keine Tensor-Operationen.
- Es kennt keine konkrete Domain (CPU, GPU usw.). Domains implementieren `RegionBackend` (Abschnitt 3).
- Es gibt keine Abhängigkeit außer `std` und `layout` (`Layout`, `DType`, `LayoutError`).
- Es gibt keine Typen für Tensoren, die an `TensorRef` vorbeiführen. `TensorRef` ist die einzige Tensor-Darstellung an dieser Schnittstelle.

## 2. Kontext
- Eine Region ist ein zusammenhängender Bytebereich fester Länge in einer Memory-Domain. Ihre Bytes werden ausschließlich über dieses Modul gelesen und geschrieben.
- Das Modul `layout` liefert `Layout` (Shape, Byte-Strides, `DType`). Das Layout trägt keinen Byte-Offset. Bei Narrow-Views trägt `TensorRef` den Offset, ihn liefert `Layout::narrow_offset_bytes`.
- Encoder und andere Verbraucher erhalten Tensoren nur als `TensorRef`. Sie lesen und schreiben deren Bytes über `Region`.

## 3. Interface
```rust
use crate::layout::{DType, Layout, LayoutError};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessError {
    /// offset + len > region_len (mit Überlaufprüfung)
    OutOfBounds { offset: usize, len: usize, region_len: usize },
    /// Fehler der Domain (z. B. Transfer fehlgeschlagen); Text von der Domain
    Backend(String),
}
impl std::fmt::Display for AccessError { /* "Zugriff außerhalb der Region: offset={offset} len={len} region_len={region_len}" bzw. "Speicherzugriff fehlgeschlagen: {0}" */ }
impl std::error::Error for AccessError {}

/// Wird von Memory-Domains implementiert. Alle Methoden nehmen &self (nebenläufig aufrufbar).
/// Das Backend darf davon ausgehen, dass offset+len bereits gegen region_len geprüft ist.
pub trait RegionBackend: Send + Sync + std::fmt::Debug {
    fn len(&self) -> usize;
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError>;
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>;
}

/// Billig klonbares Handle auf eine Region. Klone verweisen auf dieselben Bytes.
#[derive(Debug, Clone)]
pub struct Region { /* Arc<dyn RegionBackend> */ }

impl Region {
    pub fn new(backend: Arc<dyn RegionBackend>) -> Region;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    /// Liest dst.len() Bytes ab `offset` in `dst`.
    pub fn read(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError>;
    /// Schreibt src.len() Bytes ab `offset` in die Region.
    pub fn write(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>;
    /// true, wenn beide Handles dasselbe Backend (Arc::ptr_eq) referenzieren.
    pub fn same_region(&self, other: &Region) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TensorRefError {
    Layout(LayoutError),
    /// Der vom Layout (inkl. Offset) belegte Byte-Bereich liegt nicht in der Region.
    ExceedsRegion { required_end: usize, region_len: usize },
}
impl std::fmt::Display for TensorRefError {}
impl std::error::Error for TensorRefError {}
impl From<LayoutError> for TensorRefError {}

/// Tensor-Referenz: Region + Layout + Byte-Offset des Ursprungs in der Region.
#[derive(Debug, Clone)]
pub struct TensorRef { /* region: Region, layout: Layout, offset_bytes: usize; Felder privat */ }

impl TensorRef {
    /// Offset 0.
    pub fn new(region: Region, layout: Layout) -> Result<TensorRef, TensorRefError>;
    pub fn with_offset(region: Region, layout: Layout, offset_bytes: usize) -> Result<TensorRef, TensorRefError>;
    pub fn region(&self) -> &Region;
    pub fn layout(&self) -> &Layout;
    pub fn offset_bytes(&self) -> usize;
    pub fn dtype(&self) -> DType;
    /// Bytes, die der Tensor in der Region maximal erreicht: offset + Spannweite (s. u.), Ende exklusiv.
    pub fn end_bytes(&self) -> usize;
    /// Views teilen dieselbe Region; Offset/Layout werden neu validiert.
    pub fn transpose(&self, dim1: usize, dim2: usize) -> Result<TensorRef, TensorRefError>;
    pub fn narrow(&self, dim: usize, start: usize, len: usize) -> Result<TensorRef, TensorRefError>;
}
```
`Region`, `TensorRef` und `AccessError` sind `Send + Sync + 'static`. Das muss per Compile-Time-Assertion im Test abgesichert sein.

## 4. Verhalten und Invarianten
**Region-Zugriff**
- `read` und `write` prüfen `offset.checked_add(len) <= self.len()`, bevor das Backend aufgerufen wird. Bei Verletzung (auch bei Überlauf) kommt `OutOfBounds { offset, len, region_len }`, und es wird nichts gelesen oder geschrieben. Das Backend wird dann nicht aufgerufen.
- Ein Zugriff mit Länge 0 ist erlaubt, wenn `offset <= len()`. Er ist ein No-op ohne Backend-Aufruf.
- Ein erfolgreiches `read` füllt `dst` vollständig. Bei `Err` ist der Inhalt von `dst` unspezifiziert.
- Ein erfolgreiches `write` ist danach für spätere `read`-Aufrufe sichtbar (Read-your-writes, auch aus demselben Thread über Klone).
- Backend-Fehler werden unverändert als `AccessError::Backend` weitergereicht.
- Die Methoden nehmen `&self`. Mehrere Threads dürfen gleichzeitig lesen und schreiben.
- Konkurrierende Zugriffe auf disjunkte Bytebereiche beeinflussen sich nicht.
- Überlappende Schreib- oder Lesezugriffe serialisiert dieses Modul nicht. Sie sind Sache des Aufrufers und der Domain. Es gibt aber kein Undefined Behavior: das Ergebnis ist eine beliebige Verschränkung von Bytes.
- Das Modul hält keinen eigenen Lock über Aufrufe hinweg und blockiert nicht selbst.
- `Region::len` ist konstant über die Lebensdauer des Handles.
- `Clone` kopiert keine Bytes.

**TensorRef**
- Die Spannweite ist `0`, wenn `layout.elem_count() == 0`. Sonst ist sie `Σ_i (shape[i]-1)*strides[i] + dtype.size_in_bytes()`. Das gilt auch für Rank 0 (Spannweite = Elementgröße) und für nicht contiguous Layouts.
- `end_bytes = offset_bytes + Spannweite`, mit Überlaufprüfung per `checked_*`. Ein Überlauf ergibt `ExceedsRegion { required_end: usize::MAX, region_len }`.
- `new` und `with_offset` liefern `ExceedsRegion { required_end, region_len }`, wenn `end_bytes > region.len()`. Ein Tensor mit `elem_count == 0` ist gültig, wenn `offset_bytes <= region.len()`.
- `transpose` und `narrow` delegieren an `Layout`. Layout-Fehler kommen als `TensorRefError::Layout(..)` zurück.
- `narrow` setzt den neuen Offset auf `offset_bytes + layout.narrow_offset_bytes(dim, start)?`. Danach gilt dieselbe Validierung wie bei `with_offset`.
- `transpose` lässt den Offset unverändert.
- Views kopieren keine Bytes. Sie referenzieren dieselbe Region (`same_region` ist true).
- Ein `TensorRef` ist unveränderlich. Seine Bytes ändern sich nur über `Region::write`.
- Ein `TensorRef` hat keine Möglichkeit, Bytes ohne `Region` zu transportieren. Eine Methode zum Lesen oder Schreiben ganzer Tensoren gehört nicht zu diesem Modul.

## 5. Relied-on elements
Aus `layout` (unverändert nutzen):
- `DType::size_in_bytes(self) -> usize`.
- `Layout::{shape() -> &[usize], strides() -> &[usize] /*Byte*/, dtype() -> DType, rank(), elem_count(), byte_len(), is_contiguous()}`.
- `Layout::transpose(&self, usize, usize) -> Result<Layout, LayoutError>`.
- `Layout::narrow(&self, dim, start, len) -> Result<Layout, LayoutError>`.
- `Layout::narrow_offset_bytes(&self, dim, start) -> Result<usize, LayoutError>`, d. h. `start * strides[dim]`.
- `LayoutError` (`Debug, Clone, PartialEq, Eq, Display, Error`).
- `Layout` enthält keinen Byte-Offset. Den Offset trägt der Aufrufer, hier also `TensorRef`.
- `Layout` und `DType` sind `Send + Sync`.
- `elem_count` ist das Produkt der Shape-Einträge (0 bei einer 0-Dimension).
- Strides sind beliebig erlaubt, `new` prüft nur die Rank-Gleichheit.