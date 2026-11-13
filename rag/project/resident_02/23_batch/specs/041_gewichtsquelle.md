# Spezifikation: Gewichtsquelle (Build-Order 41)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Rust-Modul `weight_source`. Es hat zwei Teile:
1. Es öffnet eine safetensors-Datei per mmap und liefert die Tensor-Rohbytes zero-copy als `&[u8]` auf die Mapping-Seiten.
2. Es kopiert ausgewählte Tensoren dieser Datei in die Weights-Arena einer Subengine, also in eine `Region`. Dabei legt es sie nacheinander ab und liefert für jeden Tensor eine `TensorRef`.

**Einschränkungen:**
- Es gibt keine Wertekonvertierung, keine Dtype-Umwandlung und keine Arithmetik. Die Bytes werden unverändert übertragen.
- Es gibt kein Allokieren oder Anlegen von Arenen. `ArenaSet::ensure` und das Budget gehören dem Aufrufer.
- Es gibt keine Policy, welche Subengine welche Gewichte bekommt. Der Aufrufer wählt über Region und Auswahlfunktion.
- Es gibt keinen Schreibzugriff auf die Datei und kein Parquet.
- Es gibt keine Sharded-Dateien (`*.index.json`). Eine Instanz entspricht genau einer Datei.
- Es gibt keinen Retry und keinen Fallback auf Voll-Lesen. Die Datei wird nie komplett in einen eigenen Heap-Puffer kopiert.
- Cargo.toml wird nicht verändert. Es dürfen nur Crates verwendet werden, die dort schon stehen. Erwartet sind `memmap2` und `serde_json` für den Header. Das `safetensors`-Crate wird nicht benötigt, der Header wird selbst geparst. Die Dtype-Strings übernimmt `DType::from_safetensors`.

## 2. Kontext

- `layout` liefert `DType`, `Layout` und `LayoutError`. `from_safetensors(dtype, source)` ist die einzige Dtype-Abbildung.
- `memory_access` liefert `Region` (`write(&self, offset, &[u8])`) und `TensorRef`. Die Region ist das Ziel der Kopie.
- `arenas` liefert `Arena { kind, capacity }` und `ArenaKind::Weights`. Der Aufrufer legt die Weights-Arena der Subengine mit `ArenaSet::ensure` an (Budgetprüfung dort) und übergibt Arena und die zugehörige Region. Region-Offset 0 entspricht Arena-Offset 0.
- Zielumgebung: Weights sind safetensors, Candle ist die einzige Tensor-Bibliothek. Dieses Modul nutzt Candle nicht.
- Mehrere Subengines laden aus derselben `WeightFile` (Arc). Das Mapping wird geteilt, jede Subengine bekommt ihre eigene Kopie in ihrer Arena.

## 3. Interface

```rust
use crate::arenas::{Arena, ArenaKind};
use crate::layout::{DType, LayoutError};
use crate::memory_access::{AccessError, Region, TensorRef, TensorRefError};
use std::path::{Path, PathBuf};

/// Ausrichtung jedes Tensor-Starts innerhalb der Arena (Byte).
pub const WEIGHT_ALIGN: u64 = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeightInfo {
    pub name: String,
    pub dtype: DType,
    pub shape: Vec<usize>,
    /// Byte-Bereich relativ zum Beginn des Datenbereichs der Datei: [begin, end)
    pub data_begin: u64,
    pub data_end: u64,
}

#[derive(Debug)]
pub enum WeightError {
    Io { path: PathBuf, reason: String },
    /// Header/Struktur ungültig (Länge, JSON, Offsets, Größe passt nicht zu Shape*Dtype, Überlappung, Lücke, Datei zu kurz)
    InvalidFormat { path: PathBuf, reason: String },
    /// Dtype nicht unterstützt (durchgereicht von DType::from_safetensors)
    Dtype(LayoutError),
    NotFound { name: String },
    /// Arena ist nicht Weights, oder Bedarf passt nicht in Arena bzw. Region
    ArenaKindMismatch { got: ArenaKind },
    CapacityExceeded { required_end: u64, arena_capacity: u64, region_len: u64 },
    Access(AccessError),
    TensorRef(TensorRefError),
}
impl std::fmt::Display for WeightError {}
impl std::error::Error for WeightError {}
impl From<AccessError> for WeightError {}
impl From<TensorRefError> for WeightError {}

/// Gemappte safetensors-Datei. Unveränderlich nach `open`.
#[derive(Debug)]
pub struct WeightFile { /* Mmap, path, Vec<WeightInfo> sortiert nach data_begin, Name-Index */ }

impl WeightFile {
    /// Öffnet read-only, mappt die gesamte Datei und validiert den Header. Kopiert keine Tensordaten.
    pub fn open(path: &Path) -> Result<WeightFile, WeightError>;
    pub fn path(&self) -> &Path;
    /// Alle Tensoren in Dateireihenfolge (aufsteigend nach data_begin). `__metadata__` ist nicht enthalten.
    pub fn tensors(&self) -> &[WeightInfo];
    pub fn info(&self, name: &str) -> Option<&WeightInfo>;
    /// Zero-copy: Slice direkt auf das Mapping, Länge = data_end - data_begin.
    pub fn raw_bytes(&self, name: &str) -> Result<&[u8], WeightError>;

    /// Kopiert alle Tensoren, für die `select(name)` true liefert, in `region`
    /// (Weights-Arena einer Subengine), ab Arena-Offset `base_offset`.
    pub fn copy_to_arena(
        &self,
        arena: &Arena,
        region: &Region,
        base_offset: u64,
        select: &dyn Fn(&str) -> bool,
    ) -> Result<LoadedWeights, WeightError>;
}

#[derive(Debug, Clone)]
pub struct LoadedWeights {
    /// In Dateireihenfolge; Layout = Layout::contiguous(shape, dtype); offset_bytes = Ablageposition.
    pub tensors: Vec<(String, TensorRef)>,
    /// Erstes freies Byte nach dem letzten Tensor (ohne Endausrichtung); = base_offset, wenn nichts ausgewählt.
    pub end_offset: u64,
}
impl LoadedWeights {
    pub fn get(&self, name: &str) -> Option<&TensorRef>;
}
```

`WeightFile`, `WeightInfo`, `LoadedWeights` und `WeightError` sind `Send + Sync + 'static`. Das ist per Compile-Time-Assertion im Test abzusichern.

## 4. Verhalten und Invarianten

**Format und `open`**
- Dateiaufbau: 8 Byte `u64` little-endian = Header-Länge `N`, dann `N` Byte JSON (UTF-8), dann der Datenbereich bis Dateiende.
- Der Header ist ein JSON-Objekt. Jeder Eintrag außer `__metadata__` hat die Form `{"dtype": str, "shape": [..], "data_offsets": [begin, end]}`.
- Die Offsets sind relativ zum Datenbereich.
- `InvalidFormat` entsteht in diesen Fällen:
  - Die Datei ist kürzer als 8 Byte.
  - `8 + N` überschreitet die Dateigröße. Die Prüfung ist überlaufsicher.
  - Das JSON ist ungültig oder hat die falsche Struktur.
  - `begin > end`.
  - `end` liegt über der Datenbereichslänge.
  - `end - begin != Produkt(shape) * dtype.size_in_bytes()`.
  - Zwei Tensor-Bereiche überlappen sich.
- Lücken zwischen den Tensoren sind ebenfalls `InvalidFormat`. Die Bereiche müssen sortiert lückenlos bei 0 beginnen und genau am Ende des Datenbereichs enden. Tensoren mit 0 Elementen haben `begin == end`.
- Ein unbekannter Dtype-String ergibt `WeightError::Dtype(UnsupportedDType{dtype, source})`. `source` ist `"{path}:{tensorname}"`.
- Fehler beim Öffnen oder Mappen ergeben `Io`.
- `open` liest nur den Header. Die Tensordaten werden nicht berührt und nicht kopiert.
- Eine leere Tensor-Liste (Header `{}`, Datenbereich leer) ist gültig.
- Die Namen sind eindeutig. Doppelte JSON-Schlüssel sind `InvalidFormat`.

**Zero-copy**
- `raw_bytes` gibt eine Referenz in das Mapping zurück. Die Pointer von `raw_bytes(n)` liegen innerhalb des Mapping-Bereichs und sind bei wiederholten Aufrufen identisch.
- Das Slice lebt so lange wie `&WeightFile`.
- Unbekannter Name ergibt `NotFound`.
- Das Mapping ist read-only. Die Datei darf während der Lebensdauer von `WeightFile` nicht von außen verändert oder gekürzt werden. Das ist eine Aufrufer-Vorgabe und der Grund für das `unsafe` beim Mappen. Es ist am Aufruf zu dokumentieren.

**`copy_to_arena`**
- Zuerst wird ein Plan erstellt, danach wird geschrieben.
  - Die ausgewählten Tensoren werden in Dateireihenfolge nacheinander abgelegt.
  - Der Start jedes Tensors ist `align_up(cursor, WEIGHT_ALIGN)`, mit `cursor` ab `base_offset`.
  - Ein Tensor mit 0 Bytes belegt keinen Platz, bekommt aber einen gültigen Offset (aufgerundeter Cursor).
  - Alle Berechnungen sind überlaufsicher. Ein Überlauf ergibt `CapacityExceeded`.
- Die Prüfungen laufen vor dem ersten Schreibzugriff, in dieser Reihenfolge:
  1. `arena.kind != ArenaKind::Weights` ergibt `ArenaKindMismatch`.
  2. Das Ende des letzten Tensors muss die Bedingung `<= arena.capacity` und `<= region.len()` erfüllen, sonst `CapacityExceeded { required_end, arena_capacity, region_len }`.
- Bei einem Planfehler wird nichts geschrieben.
- Jeder Tensor wird mit einem `Region::write` direkt aus dem Mapping-Slice geschrieben. Es gibt keinen Zwischenpuffer und keine Vollkopie der Datei im Heap.
- Die Bytes werden unverändert übertragen. Ein späteres `region.read` am Offset liefert exakt `raw_bytes(name)`.
- Die Bytes zwischen den Tensoren (Padding) werden nicht geschrieben.
- Ein `AccessError` beim Schreiben bricht ab und wird als `Access` zurückgegeben. Bereits geschriebene Tensoren bleiben unverändert in der Region. Es gibt kein Rollback, und der Aufrufer verwirft die Arena-Belegung.
- Die zurückgegebenen `TensorRef` entstehen mit `TensorRef::with_offset(region.clone(), Layout::contiguous(shape, dtype), offset)`. Ihre Fehler kommen als `TensorRef` zurück.
- `end_offset` ist das Ende des letzten Tensors, ungepadded.
- `select` wird genau einmal pro Tensor aufgerufen, in Dateireihenfolge.
- Auswahl, die nichts trifft: `Ok` mit leerer Liste und `end_offset == base_offset`. Es wird geschrieben nichts.
- Dieselbe `WeightFile` kann beliebig oft und für verschiedene Regionen kopiert werden. Jeder Aufruf ist unabhängig.

**Nebenläufigkeit**
- `WeightFile` ist nach `open` unveränderlich und hat keinen internen Lock. `&self`-Methoden sind von mehreren Threads gleichzeitig nutzbar, z. B. parallele `copy_to_arena`-Aufrufe für verschiedene Subengines (typisch über `Arc<WeightFile>`).
- Gleichzeitige Kopien in dieselbe Region mit überlappenden Bereichen serialisiert dieses Modul nicht. Das ist Sache des Aufrufers (siehe `memory_access`).
- Das Modul hat keinen globalen Zustand.

**Fehler**
- Kein Panic bei beliebigem Dateiinhalt. Der Header ist nicht vertrauenswürdig, alle Längen und Offsets sind mit `checked_*` zu prüfen. Ein riesiges `N` darf keine Allokation von `N` Byte vor der Prüfung auslösen.
- Die `Display`-Texte enthalten Pfad und Grund bzw. den Tensornamen.

## 5. Relied-on elements

Aus `layout`:
- `DType::from_safetensors(dtype: &str, source: &str) -> Result<DType, LayoutError>`. Akzeptiert werden genau `F32`, `F16`, `BF16`, `U8`, `U32`, `I64`, alles andere ergibt `UnsupportedDType`.
- `DType::size_in_bytes(self) -> usize` (F32=4, F16=2, BF16=2, U8=1, U32=4, I64=8).
- `Layout::contiguous(shape: &[usize], dtype: DType) -> Layout`.
- `LayoutError` (`Debug, Clone, PartialEq, Eq, Display, Error`).

Aus `memory_access`:
- `Region::write(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>`. Es prüft die Grenzen, ein Zugriff mit Länge 0 ist ein No-op. `Region::len()` und `Region` sind `Clone` (gleiche Bytes) sowie `Send + Sync`.
- `TensorRef::with_offset(region: Region, layout: Layout, offset_bytes: usize) -> Result<TensorRef, TensorRefError>`.
- `AccessError`, `TensorRefError` (`Debug, Clone, PartialEq, Eq`).

Aus `arenas`:
- `Arena { pub kind: ArenaKind, pub capacity: u64 }` (`Copy`) und `Arena::contains(off: u64, len: u64) -> bool`.
- `ArenaKind::Weights`.
- Die Arena ist ein logischer Offsetraum `[0, capacity)`. Das Anlegen mit Budgetprüfung erfolgt über `ArenaSet::ensure` durch den Aufrufer.

Aus der Zielumgebung:
- Das Gewichtsformat ist safetensors.
- Es dürfen nur Crates aus `/home/user/xyan/xy.ai.workbench/rag/Cargo.toml` verwendet werden.