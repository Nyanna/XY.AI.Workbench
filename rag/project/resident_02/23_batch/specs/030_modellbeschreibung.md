# Spezifikation: Modellbeschreibung (Build-Order 30)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Rust-Modul `model_descriptor`. Es stellt einen Descriptor bereit, die reine Beschreibung eines Embedding-Modells. Es validiert den Descriptor und leitet den Default für `intfloat/multilingual-e5-small` ab.

**Einschränkungen:**
- Der Descriptor ist reine Daten: ohne Ressourcen, ohne Dateizugriff, ohne Gewichte, ohne Tokenizer, ohne Device. Er besitzt nichts außer eigenen Werten.
- Das Modul führt keine I/O aus. Es parst weder `config.json` noch safetensors und prüft nicht, ob Pfade existieren.
- Das Modul führt keine Pipeline aus. Der Expander nutzt den Descriptor nur als Konfiguration.
- Es gibt keine globalen Zustände. Abhängig ist nur das Modul `layout` (`DType`) und `std`. Es gibt keine Fremd-Crates, außer `thiserror` ist ohnehin im Projekt. Sonst implementiere `Display` und `Error` von Hand.

## 2. Kontext
- Model-ID und `dimension` sind die einzige harte Bindung zum Index. Der Index vergleicht beide, sonst nichts.
- Der Expander liest Präfixe, Pooling, Normalisierung, `max_seq_len` und Datentypen aus dem Descriptor, um die Pipeline zu konfigurieren.
- Compute-Datentypen (Hauptmodell) und Storage-Datentyp (gespeicherte Embeddings) sind unabhängig wählbar.
- Die Multi-Vector-Parameter (Query-/Dokumentlänge, Marker, Dimension, Skiplist) stammen aus der Checkpoint-Konfiguration. Der Aufrufer, der diese liest, füllt `MultiVectorConfig`. Dieses Modul hartkodiert keine Multi-Vector-Werte.

## 3. Interface
```rust
use std::path::{Path, PathBuf};
use crate::layout::DType;

pub const E5_SMALL_MODEL_ID: &str = "intfloat/multilingual-e5-small";
pub const E5_SMALL_REPO_DIR: &str = "/home/user/xyan/xy.ai.workbench/rag/model/multilingual-e5-small";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceClass { Cpu, Gpu }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pooling { Mean /* über unmaskierte Token */, Cls }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Normalization { L2, None }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComputeDTypes { pub cpu: DType, pub gpu: DType }
impl ComputeDTypes {
    pub const fn for_device(&self, class: DeviceClass) -> DType; // Cpu -> cpu, Gpu -> gpu
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionLayer { pub tensor_name: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiVectorConfig {
    pub max_query_len: usize,
    pub max_document_len: usize,
    pub query_marker: String,      // Marker-Token für Queries
    pub document_marker: String,   // Marker-Token für Dokumente
    pub dimension: usize,          // Dimension der Token-Vektoren
    pub skiplist: Vec<String>,     // Token, die nicht eingebettet werden
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDescriptor {
    pub model_id: String,
    pub architecture: String,          // Architektur-ID, z. B. "bert"
    pub dimension: usize,
    pub pooling: Pooling,
    pub normalization: Normalization,
    pub query_prefix: String,          // "" = kein Präfix
    pub passage_prefix: String,
    pub storage_dtype: DType,          // Embeddings im Speicher/Index
    pub compute_dtypes: ComputeDTypes, // Hauptmodell
    pub max_seq_len: usize,
    pub projection_layers: Vec<ProjectionLayer>,
    pub config_path: PathBuf,
    pub weight_paths: Vec<PathBuf>,
    pub multi_vector: Option<MultiVectorConfig>,
}

impl ModelDescriptor {
    pub fn compute_dtype(&self, class: DeviceClass) -> DType;
    pub fn validate(&self) -> Result<(), ModelConfigError>;
    /// Default für intfloat/multilingual-e5-small aus Verzeichnis und Storage-Datentyp.
    pub fn e5_small_default(dir: &Path, storage_dtype: DType) -> ModelDescriptor;
    /// Wie e5_small_default mit dir = E5_SMALL_REPO_DIR.
    pub fn e5_small_repo_default(storage_dtype: DType) -> ModelDescriptor;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelConfigError {
    /// Anzeige: "inkompatible Modellkonfiguration: {reason}"
    Incompatible { reason: String },
}
impl std::fmt::Display for ModelConfigError { /* s. o. */ }
impl std::error::Error for ModelConfigError {}
```
`ModelDescriptor` und alle Hilfstypen sind `Send + Sync + 'static` (reine Wertetypen). Alle Felder sind öffentlich.

## 4. Verhalten und Invarianten
**e5_small_default(dir, storage_dtype)** liefert:
- `model_id = "intfloat/multilingual-e5-small"`, `architecture = "bert"`, `dimension = 384`
- `pooling = Mean`, `normalization = L2`
- `query_prefix = "query: "`, `passage_prefix = "passage: "`
- `max_seq_len = 512`
- `compute_dtypes = { cpu: F32, gpu: F32 }`
- `storage_dtype` wie übergeben, unabhängig von den Compute-Datentypen
- `projection_layers` leer, `multi_vector = None`
- `config_path = dir.join("config.json")`, `weight_paths = [dir.join("model.safetensors")]`
- Das Ergebnis besteht `validate()` immer.
- Die Funktion ist rein, paniert nicht und greift nicht aufs Dateisystem zu. `dir` muss nicht existieren.
- `e5_small_repo_default(d)` ist gleich `e5_small_default(Path::new(E5_SMALL_REPO_DIR), d)`.

**compute_dtype(class)** gibt `compute_dtypes.cpu` bzw. `.gpu` zurück.

**validate()** prüft in dieser Reihenfolge und liefert beim ersten Verstoß `Incompatible { reason }`:
1. `dimension > 0`
2. `max_seq_len > 0`
3. `query_prefix` ist leer oder endet mit `' '` (U+0020)
4. `passage_prefix` ist leer oder endet mit `' '`
- Ein leeres Präfix ist gültig. Ein Präfix nur aus `" "` ist gültig.
- Die Begründung nennt das verletzte Feld und die Regel auf Deutsch, z. B. `"dimension muss größer als 0 sein"`, `"max_seq_len muss größer als 0 sein"`, `"query_prefix muss mit einem Leerzeichen enden (war: \"query:\")"`. Der Präfix-Fehler nennt den tatsächlichen Wert. Die Tests prüfen mindestens, dass der Feldname im `reason` vorkommt.
- Es gibt keine weiteren Prüfungen. Weder Storage-/Compute-Datentypen noch `multi_vector` noch Pfade werden validiert. Die Multi-Vector-Parameter werden unverändert übernommen.
- `validate` ändert nichts und ist thread-sicher, weil es nur `&self` liest.

**Bindung zum Index:** Nur `model_id` und `dimension` sind für den Index verbindlich. Andere Felder dürfen sich ändern, ohne den Index zu invalidieren.

**Concurrency:** Alle Funktionen sind rein. Der Descriptor darf geklont und geteilt werden (`Arc<ModelDescriptor>`). Es gibt keine interne Synchronisation.

## 5. Relied-on elements
Aus `layout`, verbatim:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType { F32, F16, BF16, U8, U32, I64 }
```
`DType` ist `Send + Sync`. Es wird nur als Wert genutzt, ohne Konvertierung und ohne Arithmetik.