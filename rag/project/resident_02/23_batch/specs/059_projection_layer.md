# Spezifikation: Projection-Layer (Build-Order 59)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Rust-Modul `projection_layer`. Es bildet jeden Token-Vektor linear von `input_dim` auf `output_dim` ab, z. B. 768 → 128, vor allem für Late Interaction. Es lädt Weight (`[output, input]`) und optionalen Bias (`[output]`) aus safetensors, prüft Shape und Datentyp und hält beide als FP32.

**Einschränkungen:**
- Das Modul ist nur CPU-FP32 und hat keinen Zustand außer den geladenen Gewichten.
- Es nutzt weder Arena noch `Region` noch `TensorRef` und kein Candle.
- Es gibt keine Normalisierung, keine Aktivierung, kein Batching, kein Padding und keine Maskenlogik. Das Modul führt keine Multi-Vector-Pipeline aus.
- Es gibt kein Training und keine Quantisierung. Es werden nur FP32, FP16 und BF16 gelesen.
- Nur Crates aus der bestehenden Cargo.toml sind erlaubt (`serde_json` ist erwartet). Cargo.toml wird nicht verändert.
- Es gibt kein `half`-Crate. FP16 und BF16 werden selbst nach FP32 konvertiert.
- Es gibt keine Sharded-Dateien, keinen Retry und keinen Fallback.

## 2. Kontext

- `model_descriptor`: `ProjectionLayer { tensor_name }` nennt den Weight-Tensor. Der Aufrufer leitet `input_dim` und `output_dim` ab (typisch `ModelDescriptor.dimension` bzw. `MultiVectorConfig.dimension`) und übergibt sie explizit. Dieses Modul liest den Descriptor nicht selbst.
- `weight_source`: `WeightFile` mappt eine safetensors-Datei read-only. Dieses Modul nutzt `WeightFile::open`, `info` und `raw_bytes` für den mmap-Pfad.
- Der Pfad "im Speicher" parst den safetensors-Header selbst, weil `WeightFile` nur Dateien öffnet.
- `layout`: `DType` und `DType::from_safetensors`.
- Aufrufer ist ein Multi-Vector-Embedder. Er ruft `apply` auf den Token-Vektoren eines Textes auf. Das Modul wird von mehreren Threads gleichzeitig genutzt (`Arc<Projection>`).

## 3. Interface

```rust
use crate::model_descriptor::ProjectionLayer;
use crate::weight_source::WeightFile;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct Projection { /* weight: Vec<f32> (output*input, zeilenweise), bias: Option<Vec<f32>>, input_dim, output_dim; privat */ }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    /// Anzeige: "Gewichtsname passt nicht: {name}"
    WeightNameMismatch { name: String },
    /// Anzeige: "inkompatibel: Tensor {tensor} hat Shape {actual:?}, erwartet {expected:?}"
    IncompatibleShape { tensor: String, actual: Vec<usize>, expected: Vec<usize> },
    /// Anzeige: "inkompatibel: Tensor {tensor} hat nicht unterstützten Typ {dtype}"
    IncompatibleDType { tensor: String, dtype: String },
    /// Datei-/Format-/Mapping-Fehler. Anzeige: "Load-Error: {reason}" (mit Pfad, falls vorhanden)
    Load { path: Option<PathBuf>, reason: String },
    /// Eingabevektor hat falsche Länge. Anzeige: "Eingabedimension {actual} statt {expected}"
    InputDimMismatch { expected: usize, actual: usize },
}
impl std::fmt::Display for ProjectionError {}
impl std::error::Error for ProjectionError {}

impl Projection {
    /// Aus bereits geöffneter (gemappter) Datei. Tensor-Name = layer.tensor_name.
    pub fn from_weight_file(
        file: &WeightFile, layer: &ProjectionLayer, input_dim: usize, output_dim: usize,
    ) -> Result<Projection, ProjectionError>;

    /// Öffnet die Datei per `WeightFile::open` (mmap) und lädt wie `from_weight_file`.
    pub fn from_file(
        path: &Path, layer: &ProjectionLayer, input_dim: usize, output_dim: usize,
    ) -> Result<Projection, ProjectionError>;

    /// Aus safetensors-Bytes im Speicher (gesamter Dateiinhalt).
    pub fn from_bytes(
        bytes: &[u8], layer: &ProjectionLayer, input_dim: usize, output_dim: usize,
    ) -> Result<Projection, ProjectionError>;

    pub fn input_dim(&self) -> usize;
    pub fn output_dim(&self) -> usize;

    /// Projiziert jeden Vektor: out[o] = dot(weight[o, :], v) + bias[o].
    pub fn apply(&self, vectors: &[Vec<f32>]) -> Result<Vec<Vec<f32>>, ProjectionError>;
}

/// Bias-Name zum Weight-Namen (rein).
pub fn bias_tensor_name(weight_name: &str) -> String;
```

`Projection` und `ProjectionError` sind `Send + Sync + 'static`. Das ist per Compile-Time-Assertion im Test abzusichern.

## 4. Behavior und Invarianten

**Namen**
- `bias_tensor_name`: Endet der Name auf `"weight"`, wird diese Endung durch `"bias"` ersetzt. Beispiele: `"proj.weight"` → `"proj.bias"` und `"weight"` → `"bias"`. Sonst wird `"bias"` ohne Trennzeichen angehängt, z. B. `"proj"` → `"projbias"`.

**Laden (alle drei Wege gleich, nur die Quelle der Bytes unterscheidet sich)**
1. Der Weight-Tensor `layer.tensor_name` wird gesucht. Fehlt er, kommt `WeightNameMismatch { name }`.
2. Die Weight-Shape muss genau `[output_dim, input_dim]` sein, sonst `IncompatibleShape { tensor, actual, expected }`. `expected` ist `[output_dim, input_dim]`.
3. Der Weight-Dtype muss F32, F16 oder BF16 sein, sonst `IncompatibleDType { tensor, dtype }`. `dtype` ist der safetensors-String, z. B. `"I64"` oder `"F64"`.
4. Der Bias-Tensor `bias_tensor_name(tensor_name)` ist optional. Fehlt er, ist `bias = None` und es entsteht kein Fehler. Ist er vorhanden, gelten dieselben Prüfungen mit erwarteter Shape `[output_dim]`. Bei Fehlern ist `tensor` der Bias-Name.
5. Prüfreihenfolge: Weight-Name, Weight-Shape, Weight-Dtype, dann Bias (Shape, dann Dtype). Der erste Fehler gewinnt. Shape wird vor Dtype geprüft.
6. Die Werte werden in FP32 konvertiert. Die Bytes sind little-endian, das Speicherlayout ist zeilenweise `[output][input]`.
   - F32: unverändert.
   - F16: IEEE-754-half → f32, exakt. Subnormale, ±Inf und NaN bleiben erhalten.
   - BF16: die 16 Bit werden als obere Hälfte eines f32 gesetzt (`bits << 16`).
7. `input_dim == 0` oder `output_dim == 0` ist kein Sonderfall. Es gelten dieselben Shape-Regeln mit Nulldimension.
8. Die Konvertierung kopiert höchstens die Tensoren Weight und Bias. Die restliche Datei wird nicht in den Heap kopiert.

**Quellen und Fehler**
- `from_weight_file`:
  - Es nutzt `WeightFile::info` für Shape und Dtype (`DType`) und `raw_bytes` für die Daten.
  - Ein Tensor, den `info` nicht kennt, ergibt `WeightNameMismatch`.
- `from_file`:
  - Es ruft `WeightFile::open`.
  - Jeder `WeightError` daraus (`Io`, `InvalidFormat`, `Dtype`, …) wird zu `Load { path: Some(path), reason: <Display des Fehlers> }`.
  - Eine nicht existierende Datei ergibt `Load`.
  - Ein unbekannter Dtype-String eines beliebigen Tensors der Datei ergibt ebenfalls `Load`, weil `open` schon scheitert.
- `from_bytes`:
  - Format wie in safetensors: 8 Byte `u64` LE Header-Länge `N`, dann `N` Byte JSON-Objekt, dann Daten.
  - Jeder Eintrag außer `__metadata__` hat `{dtype, shape, data_offsets:[begin,end]}`. Die Offsets sind relativ zum Datenbereich.
  - Ungültige Struktur ergibt `Load { path: None, reason }`. Das gilt für:
    - weniger als 8 Byte,
    - `8+N` über der Länge,
    - ungültiges JSON,
    - `begin > end`,
    - `end` hinter dem Datenbereich,
    - Bytelänge ungleich `Produkt(shape) * Elementgröße` für einen gelesenen Tensor.
  - Alle Berechnungen nutzen `checked_*`. Es gibt kein Panic bei beliebigen Bytes und keine Allokation von `N` vor der Prüfung.
  - Dtype-Strings anderer Tensoren, die nicht geladen werden, sind irrelevant. Es werden nur der Weight- und der Bias-Eintrag ausgewertet.
  - Der Dtype der geladenen Einträge wird als String geprüft (`F32`, `F16`, `BF16` zulässig, alles andere `IncompatibleDType`).
- Bei `from_weight_file` und `from_file` meldet ein `WeightError::NotFound` aus `raw_bytes` sich als `WeightNameMismatch`. Andere `WeightError` aus `raw_bytes` werden `Load`.

**Dimensionen**
- `input_dim()` und `output_dim()` liefern die bei der Konstruktion übergebenen Werte (bei Erfolg identisch zur Weight-Shape).

**apply**
- Für jeden Eingabevektor `v` der Länge `input_dim` gilt `out[o] = Σ_i weight[o*input_dim + i] * v[i] + bias[o]`. Ohne Bias entfällt der Summand. Die Akkumulation läuft in f32, aufsteigend über `i`, ohne Skalierung oder Aktivierung.
- Die Ausgabe hat dieselbe Anzahl und Reihenfolge von Vektoren wie die Eingabe. Jeder Ausgabevektor hat die Länge `output_dim`.
- Eine leere Eingabe ergibt `Ok(vec![])`.
- Hat ein Vektor nicht die Länge `input_dim`, kommt `InputDimMismatch { expected: input_dim, actual }`. Es wird keine Teilausgabe geliefert.
- Die Eingabe wird nicht verändert. Das Ergebnis ist deterministisch.

**Concurrency**
- `Projection` ist nach der Konstruktion unveränderlich. `apply` nimmt `&self`, hat keinen Lock und keinen globalen Zustand. Parallele Aufrufe sind erlaubt.
- Nach dem Laden hält `Projection` keinen Bezug mehr auf Datei oder Mapping. Die `WeightFile` darf danach freigegeben werden.

## 5. Relied-on elements

Aus `model_descriptor`:
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionLayer { pub tensor_name: String }
```

Aus `weight_source`:
- `WeightFile::open(path: &Path) -> Result<WeightFile, WeightError>`, `Send + Sync`, read-only gemappt, unveränderlich.
- `WeightFile::info(&self, name: &str) -> Option<&WeightInfo>` mit `WeightInfo { name: String, dtype: DType, shape: Vec<usize>, data_begin: u64, data_end: u64 }`.
- `WeightFile::raw_bytes(&self, name: &str) -> Result<&[u8], WeightError>`: zero-copy, Länge = `data_end - data_begin`. Ein unbekannter Name ergibt `WeightError::NotFound { name }`.
- `WeightError` hat die Varianten `Io { path, reason }`, `InvalidFormat { path, reason }`, `Dtype(LayoutError)`, `NotFound { name }`, `ArenaKindMismatch`, `CapacityExceeded`, `Access`, `TensorRef`. Es implementiert `Display + Error`.
- `open` validiert den Header. Alle Dtype-Strings müssen bekannt sein, sonst `WeightError::Dtype`.

Aus `layout`:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType { F32, F16, BF16, U8, U32, I64 }
impl DType {
    pub const fn size_in_bytes(self) -> usize; // F32=4, F16=2, BF16=2, U8=1, U32=4, I64=8
    pub const fn to_safetensors(self) -> &'static str; // "F32","F16","BF16","U8","U32","I64"
}
```
`DType::to_safetensors` liefert den String für `IncompatibleDType.dtype` beim Datei-Pfad.