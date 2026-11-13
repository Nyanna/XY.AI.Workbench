# Spezifikation: Encoder-Aufbau (Build-Order 97)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Rust-Modul `encoder_build`. Es baut aus einem `ModelDescriptor` und einer `DeviceClass` einen fertigen `encoder_interface::Encoder`. Der Encoder besteht aus Hauptmodell, optionaler Projection-Layer, Pooling, Normalisierung und Storage-Datentyp. Dasselbe gilt für Multi-Vector-Modelle (ColBERT-Art): Backbone plus eigener Projection-Head, ohne Pooling.

**Grenzen:**
- Das Modul enthält keine Modell-, Projection-, Pooling- oder Normalisierungslogik und kein Encoding. Es ruft nur `TransformerModel::load`, `Projection::from_weight_file`, `PoolNorm`-Typen und `Encoder::new` der Nachbarmodule auf.
- Es führt kein Tokenizing, kein Batching, kein Scheduling, keine Arena- oder Region-Belegung und keine Lane-Nutzung aus.
- Es gibt keinen Retry und keinen Fallback auf andere Devices, Datentypen oder Architekturen.
- Es gibt keinen globalen Zustand. `Cargo.toml` bleibt unverändert, es werden nur `std` und die Nachbarmodule genutzt.
- Backbones sind genau die, die `TransformerModel::load` akzeptiert. Aktuell ist das nur `architecture == "bert"`. Dieses Modul prüft die Architektur nicht selbst. Lehnt `load` sie ab, reicht das Modul den Fehler weiter. Neue Backbones (ModernBERT usw.) kommen über `TransformerModel`, nicht über dieses Modul.

## 2. Kontext

- Der Expander oder die Pipeline hat einen `ModelDescriptor` und wählt die `DeviceClass`. Er ruft `build_encoder` einmal je Subengine auf. Danach ruft er `Encoder::encode_async` auf.
- `ModelDescriptor.projection_layers` und `weight_paths` stehen im Descriptor. Bei Multi-Vector-Modellen enthält `multi_vector` zusätzlich `dimension`, die Token-Vektor-Dimension nach dem Projection-Head.
- Das Hauptmodell liefert Hidden-States `[rows][seq][dim]` in f32. Die Projection wirkt je Token. Pooling und Normalisierung laufen am Ende genau einmal im `Encoder`.
- Das Tracing nutzt die hinterlegte Device-Klasse und den Compute-Datentyp (Abschnitt 3).

## 3. Interface

```rust
use std::sync::Arc;
use crate::layout::DType;
use crate::model_descriptor::{DeviceClass, ModelDescriptor, Normalization};
use crate::encoder_interface::Encoder;
use crate::transformer_model::TransformerModel;
use crate::projection_layer::Projection;

/// Default, wenn keine Normalisierungsart angegeben ist.
pub const DEFAULT_NORMALIZATION: Normalization = Normalization::L2;
/// None -> DEFAULT_NORMALIZATION (L2), Some(n) -> n.
pub fn resolve_normalization(n: Option<Normalization>) -> Normalization;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncoderBuildError {
    /// Anzeige: "inkompatibel: {reason}"
    Incompatible { reason: String },
    /// Anzeige: "Load-Error: {reason}"
    Load { reason: String },
    /// Anzeige: "Gewichtsname passt nicht: {name}"
    WeightNameMismatch { name: String },
}
impl std::fmt::Display for EncoderBuildError {}
impl std::error::Error for EncoderBuildError {}

/// Zusammengesetzter Encoder plus Tracing-Metadaten. Unveränderlich.
pub struct BuiltEncoder { /* Arc<Encoder>, Arc<TransformerModel>, Option<Arc<Projection>>, DeviceClass, DType, bool multi_vector */ }
impl BuiltEncoder {
    pub fn encoder(&self) -> &Arc<Encoder>;
    pub fn model(&self) -> &Arc<TransformerModel>;
    pub fn projection(&self) -> Option<&Arc<Projection>>;
    /// Device-Klasse, mit der gebaut wurde (für das Tracing).
    pub fn device_class(&self) -> DeviceClass;
    /// Compute-Datentyp des Hauptmodells = desc.compute_dtype(device_class) (für das Tracing).
    pub fn compute_dtype(&self) -> DType;
    pub fn storage_dtype(&self) -> DType;
    pub fn is_multi_vector(&self) -> bool;
}

pub fn build_encoder(desc: &ModelDescriptor, class: DeviceClass) -> Result<BuiltEncoder, EncoderBuildError>;
```

`BuiltEncoder` und `EncoderBuildError` sind `Send + Sync + 'static`. Das wird per Compile-Time-Assertion im Test geprüft.

## 4. Behavior und Invarianten

**Ablauf von `build_encoder`.** Die erste Verletzung gewinnt, und es entsteht kein Teilergebnis:
1. `desc.validate()` schlägt fehl: `Incompatible { reason }` mit dem `reason` des `ModelConfigError`.
2. Pooling bestimmen:
   - `desc.multi_vector.is_some()`: `PoolingKind::None`. Jeder Token-Vektor bleibt erhalten.
   - Sonst `Pooling::Mean` → `PoolingKind::Mean`.
   - Sonst `Pooling::Cls` → `Incompatible` (Reason nennt „Cls“).
3. Projection-Bedarf prüfen:
   - Multi-Vector ohne Eintrag in `projection_layers` ergibt `Incompatible` („Projection-Head fehlt“).
   - Mehr als eine Layer in `projection_layers` ergibt `Incompatible`. Der Encoder unterstützt genau eine Projection.
   - Ist eine Layer nötig und `weight_paths` leer, folgt `Load { reason: "keine Gewichtsdateien" }`. Das passiert vor dem Laden des Hauptmodells.
4. Hauptmodell: `TransformerModel::load(desc, class)`. Fehlerabbildung:
   - `Incompatible` → `Incompatible`.
   - `WeightNameMismatch` → `WeightNameMismatch`.
   - `Load` und `EncodingFailed` → `Load`.
5. Projection (nur wenn `projection_layers` nicht leer ist), mit `layer = projection_layers[0]`:
   - `WeightFile::open(&weight_paths[0])`. Es zählt nur die erste Weight-Datei. Fehler ergeben `Load { reason }` (Display des `WeightError`).
   - `info(layer.tensor_name)` fehlt: `WeightNameMismatch { name: tensor_name }`.
   - Die Shape muss Rank 2 sein, `[output, input]`. Sonst `Incompatible`.
   - Kompatibilität: `input != model.dimension()` ergibt `Incompatible`. Der Reason enthält „inkompatibel“ sowie beide Werte (Projection-Eingabe und Hauptmodell-Ausgabe).
   - Bei Multi-Vector muss zusätzlich `output == multi_vector.dimension` gelten, sonst `Incompatible`.
   - `Projection::from_weight_file(&file, layer, input, output)`:
     - `WeightNameMismatch` → `WeightNameMismatch`.
     - `IncompatibleShape`, `IncompatibleDType` und `InputDimMismatch` → `Incompatible`.
     - `Load` → `Load`.
   - Danach darf die `WeightFile` freigegeben werden, weil die Projection keinen Bezug mehr hält.
6. Normalisierung: `resolve_normalization(Some(desc.normalization))`. Die Beschreibung bestimmt die Art, und es gibt keine eigene Umdeutung. Bei fehlender Angabe gilt L2 (`DEFAULT_NORMALIZATION`).
7. `Encoder::new(Arc<model>, projection.map(Arc::new), pooling, normalization, desc.storage_dtype)`:
   - `EncoderError::Incompatible` (z. B. nicht-float Storage-Datentyp, Projection-Dimension oder `dim == 0`) → `Incompatible`, Reason = Display des Fehlers.
   - Jeder andere `EncoderError` → `Load`.
8. Ergebnis: `BuiltEncoder` mit `device_class = class` und `compute_dtype = desc.compute_dtype(class)`.

**Invarianten des Ergebnisses:**
- `encoder().output_dim()` ist `projection.output_dim()`, sonst `desc.dimension`.
- `encoder().storage_dtype() == desc.storage_dtype`.
- `compute_dtype() == model().compute_dtype()`.
- `is_multi_vector() == desc.multi_vector.is_some()`.
- Ohne Multi-Vector und ohne `projection_layers` gibt es keine Projection. Dann gilt `projection().is_none()`, und die Weight-Dateien werden nur vom Hauptmodell gelesen.

**Fehler und Nebenläufigkeit:**
- Es gibt keine Panics bei beliebigen Descriptor-Werten.
- Das Modul verändert `desc` nicht. Der Aufruf ist zustandslos.
- Mehrere `build_encoder`-Aufrufe dürfen parallel laufen, auch mit demselben Descriptor. Sie teilen sich nur das read-only-Mapping.
- Der gebaute Encoder ist über `Arc` gemeinsam nutzbar, und `encode_async` ist parallel aufrufbar (siehe `encoder_interface`).

## 5. Relied-on elements

- `model_descriptor`:
  - `ModelDescriptor { dimension, pooling: Pooling, normalization: Normalization, storage_dtype: DType, projection_layers: Vec<ProjectionLayer>, weight_paths: Vec<PathBuf>, multi_vector: Option<MultiVectorConfig>, .. }`. Alle Felder sind öffentlich.
  - `ProjectionLayer { tensor_name: String }`.
  - `MultiVectorConfig { dimension: usize, .. }`.
  - `enum Pooling { Mean, Cls }`, `enum Normalization { L2, None }` (`Copy`), `enum DeviceClass { Cpu, Gpu }`.
  - `ModelDescriptor::validate(&self) -> Result<(), ModelConfigError>` mit `ModelConfigError::Incompatible { reason }`.
  - `ModelDescriptor::compute_dtype(&self, DeviceClass) -> DType`.
- `transformer_model`:
  - `TransformerModel::load(&ModelDescriptor, DeviceClass) -> Result<TransformerModel, TransformerError>`.
  - `dimension()`, `compute_dtype()`.
  - `TransformerError { Incompatible{reason}, WeightNameMismatch{name}, Load{reason}, EncodingFailed{reason} }`.
  - `TransformerModel` ist `Send + Sync`.
- `projection_layer`:
  - `Projection::from_weight_file(&WeightFile, &ProjectionLayer, input_dim, output_dim) -> Result<Projection, ProjectionError>`.
  - `input_dim()`, `output_dim()`.
  - Weight-Shape `[output, input]`.
  - `ProjectionError { WeightNameMismatch, IncompatibleShape, IncompatibleDType, Load, InputDimMismatch }`.
  - `Projection` ist `Send + Sync`.
- `weight_source`:
  - `WeightFile::open(&Path) -> Result<WeightFile, WeightError>`.
  - `info(&str) -> Option<&WeightInfo>` mit `WeightInfo.shape: Vec<usize>`.
  - `WeightError: Display`.
- `pooling_norm`: `PoolingKind { Mean, None }`.
- `encoder_interface`:
  - `Encoder::new(Arc<TransformerModel>, Option<Arc<Projection>>, PoolingKind, Normalization, DType) -> Result<Encoder, EncoderError>`.
  - `Encoder::new` verlangt einen float-Storage-Datentyp (F32, F16, BF16) und `projection.input_dim() == model.dimension()`.
  - `output_dim()`, `storage_dtype()`.
  - `EncoderError::{Incompatible{what, expected, actual}, Device(String), Memory(String), Processing(String)}: Display`.
  - `Encoder` ist `Send + Sync + 'static`.
- `layout`: `DType { F32, F16, BF16, U8, U32, I64 }`.