# Spezifikation: Pooling und Normalisierung (Build-Order 42)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Rust-Modul `pooling_norm`. Es führt auf den Hidden States des Encoders zwei Schritte aus:
1. Mean-Pooling über unmaskierte Token oder kein Pooling.
2. Danach einmal L2-Normalisierung mit Epsilon-Clamp. Es zählt die degenerierten Vektoren.

**Einschränkungen:**
- Das Modul ist reine CPU-Berechnung auf `f32`-Slices. Es hat keine I/O, keinen globalen Zustand, keine Fremd-Crates und kennt weder Tensor-Typen noch Device, Tokenizer oder Gewichte.
- Es unterstützt nur Mean und kein Pooling. CLS-Pooling gehört nicht dazu.
- Es normalisiert nicht mehrfach. Der Aufrufer ruft es genau einmal am Encoder-Ende auf, beim Schreiben. Query und Dokument werden gleich behandelt, ohne Unterscheidung im Modul.
- Es führt keine Dimensions- oder Index-Prüfung gegen den Index durch. Es prüft nur die Slice-Längen.

## 2. Kontext
- Der Expander konfiguriert die Pipeline aus dem `ModelDescriptor` (Modul `model_descriptor`). Das Modul übernimmt daraus `pooling`, `normalization` und `dimension`.
- Der Encoder liefert Hidden States `[batch, seq, dim]` und eine Attention-Mask `[batch, seq]`. Dieses Modul macht daraus Embeddings, entweder je Sequenz oder je Token.
- Das Dot-Product normalisierter Vektoren entspricht dem Cosinus.
- Die Rechnung läuft in `f32`. Die Konvertierung in den Storage-Datentyp liegt beim Aufrufer.

## 3. Interface
```rust
use crate::model_descriptor::{ModelDescriptor, Normalization, Pooling};

/// Untergrenze der Norm (Epsilon-Clamp).
pub const NORM_EPSILON: f32 = 1e-12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PoolingKind { Mean, None }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolNormError {
    /// Descriptor-Pooling wird nicht unterstützt (Cls). Anzeige: "nicht unterstütztes Pooling: {0}"
    UnsupportedPooling(String),
    /// Slice-Länge passt nicht zu den Dimensionen. Anzeige: "ungültige Form: {0}"
    ShapeMismatch(String),
    /// dim == 0. Anzeige: "dimension muss größer als 0 sein"
    ZeroDimension,
}
impl std::fmt::Display for PoolNormError { /* s. o. */ }
impl std::error::Error for PoolNormError {}

#[derive(Debug, Clone, PartialEq)]
pub struct PoolNormOutput {
    /// Row-major. Mean: batch*dim Werte. None: batch*seq*dim Werte.
    pub vectors: Vec<f32>,
    /// Anzahl Vektoren in `vectors` (Mean: batch, None: batch*seq).
    pub num_vectors: usize,
    /// Anzahl degenerierter Vektoren (Norm < NORM_EPSILON). Bei Normalization::None immer 0.
    pub degenerate_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolNorm { pub pooling: PoolingKind, pub normalization: Normalization, pub dim: usize }

impl PoolNorm {
    pub fn new(pooling: PoolingKind, normalization: Normalization, dim: usize) -> Result<Self, PoolNormError>;
    /// Mean -> Mean, Cls -> Err(UnsupportedPooling). dim = descriptor.dimension.
    pub fn from_descriptor(d: &ModelDescriptor) -> Result<Self, PoolNormError>;

    /// hidden: batch*seq*dim, mask: batch*seq (0 = Padding, !=0 = Token).
    pub fn pool(&self, hidden: &[f32], mask: &[u8], batch: usize, seq: usize) -> Result<Vec<f32>, PoolNormError>;

    /// Normalisiert jeden dim-langen Vektor in place gemäß self.normalization. Gibt degenerierte Anzahl zurück.
    pub fn normalize(&self, vectors: &mut [f32]) -> Result<usize, PoolNormError>;

    /// pool, danach normalize.
    pub fn apply(&self, hidden: &[f32], mask: &[u8], batch: usize, seq: usize) -> Result<PoolNormOutput, PoolNormError>;
}
```
`PoolNorm`, `PoolNormOutput` und `PoolNormError` sind `Send + Sync + 'static`.

## 4. Verhalten und Invarianten
**Pooling Mean:**
- Der Vektor einer Sequenz ist die Summe der Hidden-State-Vektoren aller Positionen mit `mask != 0`, geteilt durch deren Anzahl.
- Positionen mit `mask == 0` gehen nicht in das Ergebnis ein, auch nicht mit ihrem Hidden-State-Wert. Das gilt auch für NaN oder Inf an Padding-Positionen.
- Eine Sequenz mit null unmaskierten Token ergibt einen Nullvektor, ohne Division durch null und ohne NaN.
- Die Summation läuft in `f64` und wird am Ende nach `f32` gerundet. Das Ergebnis ist deterministisch.
- Die Ausgabelänge ist `batch*dim`.

**Pooling None:**
- `pool` gibt `hidden` unverändert als Kopie zurück (`batch*seq*dim` Werte), einschließlich Padding-Positionen. Die Mask wird nicht ausgewertet. Ihre Länge wird trotzdem geprüft.

**Normalisierung L2:**
- Für jeden Vektor v gilt: `n = sqrt(Σ v_i²)` (Akkumulation in `f64`), `n_c = max(n, NORM_EPSILON)`, `v_i ← v_i / n_c`.
- Ist `n < NORM_EPSILON`, gilt der Vektor als degeneriert. Er wird durch `NORM_EPSILON` geteilt und bleibt praktisch ein Nullvektor, ohne NaN. `degenerate_count` zählt ihn.
- Ein nicht degenerierter Vektor hat danach die Norm 1 (Toleranz ca. 1e-6).
- Ein Vektor mit NaN in einem Element hat Norm NaN. Er zählt nicht als degeneriert und wird nicht verändert.
- Bei `None` bleibt alles unverändert, und der Rückgabewert ist 0.
- Bei Pooling `None` werden alle `batch*seq` Vektoren normalisiert und gezählt, auch Padding-Positionen. Padding-Nullvektoren zählen daher als degeneriert. Der Aufrufer filtert über die Mask.
- Ein leerer Slice ergibt `Ok(0)`.

**`apply`:**
- Es ruft `pool` und danach `normalize` genau einmal auf.
- Normalisierung ist idempotent bis auf Rundung. Der Aufrufer darf sie dennoch nicht ein zweites Mal anwenden.

**Fehler:**
- `new` mit `dim == 0` liefert `ZeroDimension`.
- `from_descriptor` mit `Pooling::Cls` liefert `UnsupportedPooling`. Der Text nennt `"Cls"`.
- `pool` und `apply` liefern `ShapeMismatch`, wenn `hidden.len() != batch*seq*dim` oder `mask.len() != batch*seq`.
- `normalize` liefert `ShapeMismatch`, wenn `vectors.len() % dim != 0`.
- `batch == 0` oder `seq == 0` ist gültig und ergibt eine leere Ausgabe mit Zählung 0. Mean mit `seq == 0` ergibt `batch` Nullvektoren.
- Das Modul paniert nie bei gültigen oder ungültigen Eingaben.

**Concurrency:** Alle Funktionen sind rein und nehmen `&self`. Ohne internen Zustand und ohne Synchronisation dürfen sie parallel von mehreren Threads aufgerufen werden. `normalize` verändert nur den übergebenen `&mut`-Slice.

## 5. Relied-on elements
Aus `model_descriptor`, verbatim:
```rust
pub enum Pooling { Mean /* über unmaskierte Token */, Cls }
pub enum Normalization { L2, None }   // derive Debug, Clone, Copy, PartialEq, Eq, Hash
pub struct ModelDescriptor { /* … */ pub dimension: usize, pub pooling: Pooling, pub normalization: Normalization, /* … */ }
```
Die Descriptor-Typen sind `Send + Sync + 'static`. Alle Felder sind öffentlich. `e5_small_default` liefert `pooling = Mean`, `normalization = L2` und `dimension = 384`. `from_descriptor` liest nur `pooling`, `normalization` und `dimension`.