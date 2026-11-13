# Spezifikation: Tokenizer (Build-Order 43)

## 1. Aufgabe, Einschränkungen, Grenzen
**Aufgabe:** Rust-Modul `tokenizer`. Es wandelt Text auf der CPU in Token-IDs (`u32`) um. Der Tokenizer ist modellspezifisch, aus einem `ModelDescriptor` geladen, und kann geteilt werden (`Arc`).

**Nicht Teil des Moduls:**
- Es läuft nur auf der CPU und kennt weder GPU noch Device.
- Es gibt kein Gathering, kein Address-Mapping, kein Padding und keine Attention-Masken. Die GPU bekommt nur Token-IDs.
- Es fügt keine Query-/Passage-Präfixe hinzu, denn das tut der Aufrufer (Expander). Es verarbeitet keine Multi-Vector-Marker und führt kein Pooling und keine Embeddings aus.
- Es gibt keinen globalen Zustand und keinen Cache über den Tokenizer hinaus.
- Fremd-Crate: `tokenizers` (HuggingFace) lädt und führt die `tokenizer.json` aus. `std` und `crate::model_descriptor` werden sonst genutzt. Fehler-Typen werden von Hand implementiert (`Display`, `Error`), außer `thiserror` ist ohnehin im Projekt.

## 2. Kontext
- `ModelDescriptor` (Modul `model_descriptor`) ist reine Daten. Er enthält keinen Tokenizer und keinen Tokenizer-Pfad. Er hat `config_path: PathBuf` und `max_seq_len: usize`.
- **Festlegung:** Die Tokenizer-Datei ist `tokenizer.json` im selben Verzeichnis wie `config_path` (`config_path.parent()/tokenizer.json`). Für E5-small ist das `<dir>/tokenizer.json`.
- Der Aufrufer reicht Text samt eventuellem Präfix herein und bekommt `Vec<u32>` für die GPU-Vorbereitung zurück.
- Mehrere Komponenten dürfen denselben Tokenizer über `Arc<Tokenizer>` teilen. Verschiedene Modelle bekommen verschiedene Instanzen.

## 3. Interface
```rust
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::model_descriptor::ModelDescriptor;

/// Unveränderlich nach dem Laden. Send + Sync + 'static.
pub struct Tokenizer { /* intern: tokenizers::Tokenizer, max_len: usize */ }

pub type SharedTokenizer = Arc<Tokenizer>;

pub const TOKENIZER_FILE_NAME: &str = "tokenizer.json";

#[derive(Debug)]
pub enum TokenizerError {
    /// Anzeige: "Tokenizer-Datei nicht ladbar ({path}): {reason}"
    Load { path: PathBuf, reason: String },
    /// Anzeige: "Tokenisierung fehlgeschlagen: {reason}"
    Encode { reason: String },
    /// Anzeige: "ungültige Tokenizer-Konfiguration: {reason}"
    InvalidConfig { reason: String },
}
impl std::fmt::Display for TokenizerError { /* s. o. */ }
impl std::error::Error for TokenizerError {}

impl Tokenizer {
    /// Lädt `config_path.parent()/tokenizer.json`; max_len = descriptor.max_seq_len.
    pub fn from_descriptor(descriptor: &ModelDescriptor) -> Result<Tokenizer, TokenizerError>;
    /// Wie from_descriptor, gibt gleich einen geteilten Handle zurück.
    pub fn shared_from_descriptor(descriptor: &ModelDescriptor) -> Result<SharedTokenizer, TokenizerError>;
    /// Lädt eine explizite Datei; max_len > 0 ist die Obergrenze der Ausgabelänge.
    pub fn from_file(path: &Path, max_len: usize) -> Result<Tokenizer, TokenizerError>;

    pub fn max_len(&self) -> usize;
    pub fn vocab_size(&self) -> usize;

    /// Ein Text -> Token-IDs inklusive Special Tokens des Modells.
    pub fn encode(&self, text: &str) -> Result<Vec<u32>, TokenizerError>;
    /// Batch; Ausgabe i gehört zu Eingabe i. Keine Padding, Längen können verschieden sein.
    pub fn encode_batch(&self, texts: &[&str]) -> Result<Vec<Vec<u32>>, TokenizerError>;
}
```

## 4. Behavior und Invarianten
**Laden**
- `from_descriptor` führt kein `validate()` aus, nutzt aber nur `config_path` und `max_seq_len`.
- Hat `config_path` kein Parent-Verzeichnis, wird der Pfad `tokenizer.json` relativ verwendet.
- Fehlende, nicht lesbare oder ungültige Datei ergibt `Load { path, reason }`. Es wird nie gepanikt.
- `max_len == 0` ergibt `InvalidConfig` (bei `from_descriptor` auch für `max_seq_len == 0`).
- Beim Laden setzt der Tokenizer Truncation auf `max_len` (Strategie `LongestFirst`, Richtung rechts) und deaktiviert Padding, ohne die Special-Token-Konfiguration der Datei zu ändern.

**Encoding**
- Die Ausgabe sind ausschließlich IDs als `u32`, in Textreihenfolge.
- Special Tokens (z. B. BOS/EOS) fügt die Post-Processing-Konfiguration der `tokenizer.json` hinzu. Das Modul fügt keine eigenen hinzu.
- Die Ausgabelänge ist immer `<= max_len`. Bei Kürzung bleiben die abschließenden Special Tokens erhalten, weil die Truncation sie mitzählt.
- `encode("")` liefert nur die Special Tokens des Modells (kein Fehler, nie ein leerer Fehlerfall durch leeren Text).
- Das Ergebnis ist deterministisch: gleicher Text und gleicher Tokenizer ergeben gleiche IDs.
- Jede ID ist `< vocab_size()` (inklusive hinzugefügter Tokens).
- `encode_batch(&[])` liefert `Ok(vec![])`. `encode_batch(texts)[i] == encode(texts[i])`. Der Batch darf intern parallel laufen, die Reihenfolge bleibt erhalten.
- Ein Fehler in einem Element ergibt `Err(Encode)` für den ganzen Aufruf, ohne Teilergebnis.
- Es gibt keine Normalisierung oder Präfixe außerhalb der `tokenizer.json`.

**Concurrency**
- `Tokenizer` ist `Send + Sync`. Alle Methoden nehmen `&self` und sind ohne externe Synchronisation gleichzeitig aufrufbar. Es gibt keinen veränderlichen Zustand nach dem Laden.
- `SharedTokenizer` darf frei zwischen Threads geklont werden.

## 5. Relied-on elements
Aus `model_descriptor`, verbatim:
```rust
pub struct ModelDescriptor {
    pub model_id: String,
    pub architecture: String,
    pub dimension: usize,
    pub pooling: Pooling,
    pub normalization: Normalization,
    pub query_prefix: String,
    pub passage_prefix: String,
    pub storage_dtype: DType,
    pub compute_dtypes: ComputeDTypes,
    pub max_seq_len: usize,
    pub projection_layers: Vec<ProjectionLayer>,
    pub config_path: PathBuf,
    pub weight_paths: Vec<PathBuf>,
    pub multi_vector: Option<MultiVectorConfig>,
}
```
- Der Descriptor ist reine Daten (`Send + Sync + 'static`, alle Felder öffentlich) und führt keine I/O aus.
- Für E5-small gilt: `config_path = dir.join("config.json")` und `max_seq_len = 512`. Daraus ergibt sich `dir/tokenizer.json`.
- Der Tokenizer nutzt nur `config_path` und `max_seq_len`. Er validiert den Descriptor nicht.