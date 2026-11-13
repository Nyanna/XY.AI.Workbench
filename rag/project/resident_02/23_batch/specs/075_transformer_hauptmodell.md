# Spezifikation: Transformer-Hauptmodell (Build-Order 75)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Rust-Modul `transformer_model`. Es prüft eine BERT-Konfiguration und lädt die Gewichte per mmap im gewählten Compute-Datentyp auf CPU oder erstes GPU-Device. Der Forward-Pass macht aus Token-IDs und Mask die Hidden-States aller Token als flache `Vec<f32>`.

**Einschränkungen:**
- Das Modul macht kein Tokenizing, kein Pooling, keine Normalisierung und kein Batching. Es hat keine Policy für die Device-Wahl und keine Lanes oder Events. `encode` ist synchron.
- Das Modul bietet nur BERT. Es hat keine Projection-Layer, keinen Pooler und keinen Fallback auf ein anderes Device oder einen anderen Datentyp.
- Das Modul ändert `Cargo.toml` nicht und nutzt nur Crates, die dort stehen (erwartet `candle-core`, `candle-nn`, `candle-transformers`, `serde`, `serde_json`).
- Candle-Typen (`Tensor`, `Device`, `DType`, `BertModel`) bleiben intern. Die öffentliche API enthält nur `usize`, `u32`, `f32`, Slices, `Vec` und Typen der Nachbarmodule.
- GPU-Code steht hinter `#[cfg(feature = "cuda")]`.
- Das Modul hat keinen globalen Zustand.

## 2. Kontext
- `model_descriptor` liefert `ModelDescriptor` mit `architecture`, `dimension`, `max_seq_len`, `config_path`, `weight_paths` und `compute_dtypes`. Der Aufrufer (Expander/Pipeline) ruft `TransformerModel::load` mit Descriptor und `DeviceClass` auf. Danach ruft er `encode` mit Token-IDs und Mask auf und pooled/normalisiert selbst.
- `weight_source::WeightFile` öffnet die safetensors-Dateien (mmap, Header-Validierung). Das Modul nutzt es, um Tensornamen zu prüfen.
- `library_adapter` liefert `to_lib_dtype` für die Abbildung auf den Backend-Datentyp. Der Adapter bleibt die einzige Stelle, die `DType` in Candle-`DType` übersetzt. Candle-Modelltypen (BERT-Netz, `VarBuilder`) verwendet dieses Modul direkt und intern.
- `layout` liefert `DType`. `device` liefert `DeviceHandle`.

## 3. Interface
```rust
use crate::layout::DType;
use crate::model_descriptor::{DeviceClass, ModelDescriptor};
use crate::device::DeviceHandle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformerError {
    /// Anzeige: "inkompatible Modellkonfiguration: {reason}"
    Incompatible { reason: String },
    /// Anzeige: "Gewichtsname passt nicht: {name}"
    WeightNameMismatch { name: String },
    /// Anzeige: "Load-Error: {reason}"
    Load { reason: String },
    /// Anzeige: "Encoding fehlgeschlagen: {reason}"
    EncodingFailed { reason: String },
}
impl std::fmt::Display for TransformerError {}
impl std::error::Error for TransformerError {}

/// Geladenes BERT-Modell. Send + Sync + 'static (Compile-Time-Assertion im Test).
pub struct TransformerModel { /* BertModel, Candle-Device, Config, DeviceHandle, DType, hidden, max_seq_len */ }

impl TransformerModel {
    pub fn load(desc: &ModelDescriptor, class: DeviceClass) -> Result<TransformerModel, TransformerError>;

    /// Ausgabedimension (= hidden_size = desc.dimension).
    pub fn dimension(&self) -> usize;
    pub fn max_seq_len(&self) -> usize;
    /// DeviceHandle::Cpu oder DeviceHandle::Gpu(0).
    pub fn device(&self) -> DeviceHandle;
    /// Gewählter Compute-Datentyp.
    pub fn compute_dtype(&self) -> DType;

    /// ids und mask: je rows*seq_len Werte, zeilenweise (row-major).
    /// mask: 0 = Padding, ungleich 0 = gültig.
    /// Ergebnis: rows*seq_len*dimension() FP32-Werte, row-major [rows][seq_len][dim].
    pub fn encode(&self, ids: &[u32], mask: &[u32], rows: usize, seq_len: usize)
        -> Result<Vec<f32>, TransformerError>;
}
```

## 4. Behavior and invariants

**Reihenfolge von `load`.** Die erste Verletzung gewinnt:
1. **Config lesen.**
   - `desc.config_path` wird gelesen und als JSON geparst.
   - Fehler beim Lesen, ungültiges JSON, fehlende oder ungültige Pflichtfelder: `Load{reason}`. Die Begründung nennt Pfad und Ursache.
   - Pflichtfelder sind `model_type`, `hidden_size`, `num_hidden_layers`, `num_attention_heads`, `intermediate_size`, `vocab_size`, `max_position_embeddings`.
   - Die übrigen Felder der Candle-BERT-Config (z. B. `hidden_act`, `layer_norm_eps`) erhalten die HF-Defaults.
2. **Modelltyp.** `model_type != "bert"` ergibt `Incompatible`. Der `reason` nennt den gefundenen Typ, z. B. `modeltyp muss "bert" sein (gefunden: "xlm-roberta")`.
3. **Größen.**
   - `hidden_size != desc.dimension` ergibt `Incompatible`. Der `reason` nennt beide Werte.
   - `desc.max_seq_len > max_position_embeddings` ergibt `Incompatible`. Der `reason` nennt beide Werte.
   - Auch `desc.architecture != "bert"` ergibt `Incompatible` und nennt den gefundenen Wert.
4. **Device.**
   - `Cpu` ist immer ladbar (`Device::Cpu`).
   - `Gpu` ohne Feature `cuda` ergibt `Incompatible`. Der `reason` weist darauf hin, dass die GPU-Unterstützung nicht einkompiliert ist.
   - `Gpu` mit Feature `cuda` nutzt GPU-Ordinal 0 (`DeviceHandle::Gpu(0)`). Schlägt das Anlegen des Devices fehl, ergibt das `Load{reason}`.
5. **Datentyp.**
   - `dt = desc.compute_dtype(class)`, übersetzt mit `library_adapter::to_lib_dtype`.
   - Nur `F32`, `F16` und `BF16` sind Compute-Datentypen. `U8`, `U32` und `I64` ergeben `Incompatible`, mit Datentyp im `reason`.
6. **Gewichte prüfen und laden.**
   - Jede Datei in `desc.weight_paths` wird mit `WeightFile::open` geöffnet. Fehler ergeben `Load{reason}`. Eine leere `weight_paths` ergibt `Load`.
   - Namenspräfix: Existiert `embeddings.word_embeddings.weight` in keiner Datei, aber `bert.embeddings.word_embeddings.weight`, gilt das Präfix `bert.`. Sonst gilt kein Präfix.
   - Vor dem Laden wird geprüft, dass folgende Tensoren in mindestens einer Datei existieren (Namen mit Präfix). Der erste fehlende ergibt `WeightNameMismatch{name}` mit dem vollen erwarteten Namen.
     - `embeddings.{word_embeddings,position_embeddings,token_type_embeddings}.weight`
     - `embeddings.LayerNorm.{weight,bias}`
     - je Layer `i` in `0..num_hidden_layers`, unter `encoder.layer.{i}.`:
       - `attention.self.{query,key,value}.{weight,bias}`
       - `attention.output.dense.{weight,bias}`
       - `attention.output.LayerNorm.{weight,bias}`
       - `intermediate.dense.{weight,bias}`
       - `output.dense.{weight,bias}`
       - `output.LayerNorm.{weight,bias}`
   - Das Laden erfolgt per mmap (z. B. `VarBuilder::from_mmaped_safetensors`) mit Zieldatentyp `dt` und Ziel-Device. Die Dateien werden nie komplett in einen Heap-Puffer gelesen. Die Konvertierung in `dt` und der Transfer auf das Device geschehen je Tensor.
   - Ein Candle-Fehler mit erkennbar fehlendem Tensor ergibt `WeightNameMismatch{name}`. Alle übrigen Ladefehler (auch der Aufbau des Netzes) ergeben `Load{reason}`.
7. **Ergebnis.** Das Modell ist unveränderlich. `dimension() == desc.dimension` und `max_seq_len() == desc.max_seq_len`.

**`encode`:**
- Die Prüfungen laufen vor jeder Rechnung. Eine Verletzung ergibt `EncodingFailed{reason}`:
  - `ids.len() != rows*seq_len` oder `mask.len() != rows*seq_len`.
  - `seq_len > max_seq_len()`.
  - ein ID-Wert `>= vocab_size`.
  - Überlauf von `rows*seq_len`.
- `rows == 0` oder `seq_len == 0` ergibt `Ok(vec![])`, ohne Device-Zugriff.
- Token-Type-IDs sind überall 0. Die Mask wirkt als Attention-Mask. Positionen mit Mask 0 werden als Keys ignoriert, additiv über `layout::MASK_VALUE` (−10000.0, endlich, nicht −inf). Positions-IDs sind `0..seq_len` je Zeile.
- Das Ergebnis sind die Hidden-States der letzten Encoder-Schicht für alle Token, auch für Padding-Positionen. Die Werte sind endlich.
- Das Ergebnis ist immer `Vec<f32>` der Länge `rows*seq_len*dimension()`, unabhängig vom Compute-Datentyp (Rückkonvertierung nach F32 am Ende). Die Anordnung ist `[row][token][dim]`.
- Jeder Fehler im Netz oder Device (auch Transfer, Rückkopie) ergibt `EncodingFailed{reason}`. Es gibt keine Panics bei beliebigen Eingaben.
- `encode` kehrt erst zurück, wenn das Ergebnis im Host-Speicher steht. Es gibt keine ausstehende Device-Arbeit.
- Dieselben Eingaben liefern auf demselben Modell dasselbe Ergebnis (deterministisch bis auf Device-Rundung).

**Nebenläufigkeit:**
- `TransformerModel` ist `Send + Sync`. `encode` nimmt `&self` und ist von mehreren Threads gleichzeitig aufrufbar. Das Modell hat keinen veränderlichen Zustand.
- `load` ist zustandslos. Mehrere Modelle (z. B. CPU und GPU) dürfen parallel existieren und teilen sich nur das read-only-Mapping.

**Tests:**
- CPU-Tests laufen überall, mit kleiner synthetischer Config und kleinen safetensors-Gewichten.
- GPU-Tests laufen nur mit Feature `cuda` und einem Device. Sonst brechen sie sauber ab.
- Pflichtfälle:
  - falscher Modelltyp
  - falsche Hidden-Size
  - zu großes `max_seq_len`
  - GPU ohne Feature
  - nicht-float Datentyp
  - fehlender Tensor mit Namen
  - unlesbare Config
  - Ausgabelänge und Anordnung
  - Mask-Wirkung (Änderung eines maskierten Tokens ändert die Hidden-States gültiger Token nicht)
  - F16/BF16 liefern `Vec<f32>`
  - Send/Sync-Assertion

## 5. Relied-on elements
- `model_descriptor` (verbatim):
  - `ModelDescriptor { architecture: String, dimension: usize, max_seq_len: usize, compute_dtypes: ComputeDTypes, config_path: PathBuf, weight_paths: Vec<PathBuf>, .. }` mit öffentlichen Feldern.
  - `ModelDescriptor::compute_dtype(&self, class: DeviceClass) -> DType`.
  - `enum DeviceClass { Cpu, Gpu }`.
- `layout`:
  - `enum DType { F32, F16, BF16, U8, U32, I64 }`.
  - `pub const MASK_VALUE: f32 = -10_000.0;`.
- `device`: `enum DeviceHandle { Cpu, Gpu(u32) }`.
- `weight_source`:
  - `WeightFile::open(path: &Path) -> Result<WeightFile, WeightError>`.
  - `WeightFile::info(&self, name: &str) -> Option<&WeightInfo>`.
  - `WeightFile::tensors(&self) -> &[WeightInfo]`.
  - `WeightError` ist `Display`.
  - `WeightFile` ist `Send + Sync`.
- `library_adapter`: `pub fn to_lib_dtype(d: DType) -> candle_core::DType`. Es bildet F32→F32, F16→F16, BF16→BF16 usw. ab.