# Spezifikation: Layout und Elementtypen (Build-Order 11)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Rust-Modul (Crate-Modul `layout`), das Folgendes bereitstellt:
- eigene Elementtypen (`DType`)
- Layouts aus Shape, Byte-Strides und Elementtyp
- Views, die nur das Layout ändern
- die Übernahme von Elementtypen aus safetensors
- den festen additiven Mask-Wert

**Einschränkungen:**
- Es gibt keine Abhängigkeiten, auch nicht zu Fremd-Libraries für Typen (kein `half`, kein `candle`, kein `safetensors`-Crate). Nur `std`.
- Das Modul enthält keine Speicher-, Tensor-, Device- oder Datenlogik. Es besitzt und kopiert keine Daten.
- Es gibt keine Konvertierung von Werten zwischen Typen und keine Arithmetik.
- Es gibt kein safetensors-Parsing. Es übernimmt nur den Dtype-String.

## 2. Kontext
Das Modul ist ein Basisbaustein ohne Abhängigkeiten. Andere Module (Tensor/Speicher, Gewichtsladen aus safetensors, Attention/Masken) nutzen `DType`, `Layout` und `MASK_VALUE`.

Views (Transpose, Narrow) sind reine Layout-Operationen. Der Byte-Offset eines Narrow ist nicht Teil des Layouts. Den Offset trägt der Aufrufer, er kann ihn über `narrow_offset_bytes` berechnen.

## 3. Interface
```rust
/// Additiver Mask-Wert für ungültige Positionen (endlich, da -inf bei FP16 NaN-Zeilen ergäbe).
pub const MASK_VALUE: f32 = -10_000.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType { F32, F16, BF16, U8, U32, I64 }

impl DType {
    pub const fn size_in_bytes(self) -> usize;   // F32=4, F16=2, BF16=2, U8=1, U32=4, I64=8
    /// safetensors-Dtype-String ("F32","F16","BF16","U8","U32","I64") -> DType.
    /// `source` beschreibt die Herkunft (z. B. Datei/Tensorname) und geht in den Fehler ein.
    pub fn from_safetensors(dtype: &str, source: &str) -> Result<DType, LayoutError>;
    pub const fn to_safetensors(self) -> &'static str; // inverse Zuordnung
    pub const fn is_float(self) -> bool;               // F32, F16, BF16
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    /// Anzeige: "nicht unterstützter Datentyp: {dtype} (Quelle: {source})"
    UnsupportedDType { dtype: String, source: String },
    RankMismatch { shape_rank: usize, strides_rank: usize },
    DimOutOfRange { dim: usize, rank: usize },
    NarrowOutOfRange { dim: usize, start: usize, len: usize, size: usize },
}
impl std::fmt::Display for LayoutError { /* s. o. */ }
impl std::error::Error for LayoutError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layout { /* shape: Vec<usize>, strides: Vec<usize> (Byte), dtype: DType; Felder privat */ }

impl Layout {
    pub fn new(shape: Vec<usize>, strides: Vec<usize>, dtype: DType) -> Result<Layout, LayoutError>;
    pub fn contiguous(shape: &[usize], dtype: DType) -> Layout;
    pub fn shape(&self) -> &[usize];
    pub fn strides(&self) -> &[usize];      // in Byte
    pub fn dtype(&self) -> DType;
    pub fn rank(&self) -> usize;
    pub fn elem_count(&self) -> usize;
    pub fn byte_len(&self) -> usize;
    pub fn is_contiguous(&self) -> bool;
    pub fn transpose(&self, dim1: usize, dim2: usize) -> Result<Layout, LayoutError>;
    pub fn narrow(&self, dim: usize, start: usize, len: usize) -> Result<Layout, LayoutError>;
    /// Byte-Offset des Narrow-Starts relativ zum Ursprung: start * strides[dim].
    pub fn narrow_offset_bytes(&self, dim: usize, start: usize) -> Result<usize, LayoutError>;
}
```
`DType` und `Layout` sind `Send + Sync` (reine Wertetypen). Es gibt keinen internen Zustand und keine Synchronisation.

## 4. Verhalten und Invarianten
**DType**
- Die Größen sind exakt F32=4, F16=2, BF16=2, U8=1, U32=4, I64=8.
- `from_safetensors` akzeptiert genau die Strings `F32`, `F16`, `BF16`, `U8`, `U32`, `I64` (case-sensitiv).
- Jeder andere String ergibt `UnsupportedDType { dtype: <Eingabe>, source: <source> }`. Das gilt z. B. für `F64`, `I32`, `I16`, `I8`, `U16`, `U64`, `BOOL`, `F8_*` und für den leeren String.
- `from_safetensors(x.to_safetensors(), _) == Ok(x)` gilt für alle Varianten.

**MASK_VALUE**
- `MASK_VALUE` ist genau −10000.0 und endlich. Er liegt im FP16-Bereich (±65504) und ist in BF16 endlich.
- Die Rundung in den Zieltyp übernimmt der Aufrufer. Das Modul konvertiert nicht.

**Layout**
- `contiguous`: Der Stride der letzten Dimension ist `dtype.size_in_bytes()`. Jeder vordere Stride ist `stride[i+1] * max(shape[i+1], 1)`, also das Produkt der hinteren Dimensionen mal Elementgröße. Dimensionen der Größe 0 zählen im Produkt als 1.
  - Beispiel: `[2,3,4]` F32 ergibt Strides `[48,16,4]`.
  - Beispiel: `[2,0,4]` F32 ergibt `[16,16,4]`.
- Rank 0 (Skalar) hat leere Shape und leere Strides. `elem_count` ist 1, `byte_len` ist die Elementgröße, `is_contiguous` ist true.
- `new` prüft nur `shape.len() == strides.len()`, sonst `RankMismatch`. Beliebige Strides sind erlaubt.
- `elem_count` ist das Produkt der Shape-Einträge, im Gegensatz zur Stride-Berechnung mit echter 0. Eine Shape mit einer 0 ergibt also 0.
- `byte_len = elem_count * dtype.size_in_bytes()`. Das ist die logische Bytelänge der Elemente, unabhängig von den Strides.
- `is_contiguous` ist true, wenn für alle Dimensionen mit `shape[i] > 1` gilt: `strides[i]` ist gleich dem Stride aus `Layout::contiguous` für dieselbe Shape. Dimensionen der Größe 0 oder 1 sind für den Stride-Vergleich irrelevant. Ein Layout mit `elem_count == 0` ist contiguous.
- `transpose(d1, d2)` tauscht Shape- und Stride-Einträge der beiden Dimensionen. Dtype und Daten bleiben unverändert.
  - `d1 == d2` ergibt eine Kopie des Layouts.
  - Ein Index `>= rank` ergibt `DimOutOfRange`.
  - Zweimaliges Transponieren ergibt das Original.
- `narrow(dim, start, len)` setzt nur `shape[dim] = len`. Strides und Dtype bleiben unverändert.
  - `dim >= rank` ergibt `DimOutOfRange`.
  - `start + len > shape[dim]` (mit Überlaufprüfung) ergibt `NarrowOutOfRange`.
  - `len == 0` ist erlaubt, solange `start <= shape[dim]`.
  - Ein Narrow auf eine nicht-letzte Dimension macht das Layout nicht contiguous, wenn danach `shape[dim] > 1` ist und Dimensionen dahinter größer als 1 sind. Das folgt aus der `is_contiguous`-Regel.
- `narrow_offset_bytes` liefert `start * strides[dim]` mit denselben Fehlerprüfungen (`start <= shape[dim]`, sonst `NarrowOutOfRange` mit `len: 0`).
- Alle Operationen sind rein, ändern `self` nicht und geben neue Werte zurück. Es gibt keine Panics bei gültigen Eingaben. Überlauf bei `elem_count` oder `byte_len` ist durch Aufrufer-Vorgaben ausgeschlossen, darf aber nicht stillschweigend falsch wrappen (Debug-Panic ist akzeptabel).

## 5. Relied-on elements
Keine. Das Modul hat keine Abhängigkeiten.