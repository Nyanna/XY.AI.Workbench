# Spezifikation: Dateiformat und Modellbindung (Modul `index_format`, Build-Order 113)

## 1. Aufgabe, Einschränkungen, Grenzen

**Aufgabe:** Das Modul legt das Spalten-Schema und den Index-Header der Parquet-Indexdatei fest. Der Header steht als Key-Value-Metadaten im Datei-Footer. Das Modul prüft die Bindung einer Datei an das Modell über Model-ID, Dimension und Pipeline-Merkmale. Es bildet den Tokenizer-Fingerprint und hält Algorithmus und Version des schnellen 64-Bit-Hash fest.

**Grenzen:**
- Es führt keine I/O aus. Es liest und schreibt keine Parquet-Dateien und berechnet keine Content-Hashes von Dateien. Es vergibt keine Stamps.
- Es hat keinen globalen Zustand, keine Threads und kein `unsafe`.
- Der Aufrufer (Parquet-Schreiber/-Leser) überträgt die Key-Value-Paare in den Footer und zurück.
- Abhängigkeiten sind `std`, `layout`, `model_descriptor`, `tokenizer_loader` und die Arrow-Schema-Typen (`Schema`, `Field`, `DataType`) aus den bereits in der `Cargo.toml` stehenden Arrow-Crates. Neue Crates sind verboten.

## 2. Kontext

- `ModelDescriptor` liefert Model-ID, Dimension, Pooling, Normalisierung, Präfixe, Datentypen und `max_seq_len`. Die Model-Revision liefert der Aufrufer als Parameter.
- `LoadedTokenizer` liefert die Tokenizer-Definition für den Fingerprint.
- Der Index (Parquet, Arrow-Schema) legt pro Entry eine Zeile ab. Die Stamps stehen in einer eigenen Spalte, damit das Einlesen der Stamps keine Vektoren lädt.
- Gemischtes Indexieren auf CPU und GPU ist erlaubt. Deshalb sind Compute-Datentyp und Max-Stamp nicht bindend.
- `ChangeStamp` ist opak. Der Header führt den Max-Stamp daher als Rohwert `u64` (`StampCounter::max_raw()`, `None` entspricht 0).

## 3. Interface

```rust
use std::sync::Arc;
use arrow_schema::{Schema, Field, DataType};      // gleiche Typen wie arrow::datatypes
use crate::layout::DType;
use crate::model_descriptor::{ModelDescriptor, Normalization};
use crate::tokenizer_loader::LoadedTokenizer;

pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_ALGORITHM: HashAlgorithm = HashAlgorithm::Fast64;
pub const HASH_VERSION: u64 = 1;
pub const TOKENIZER_FP_ALGORITHM: &str = "fnv1a64";

/// Spaltennamen in Schema-Reihenfolge.
pub const COL_ENTRY_ID: &str = "entry_id";
pub const COL_CHANGE_STAMP: &str = "change_stamp";
pub const COL_VECTOR: &str = "vector";
pub const COL_TARGET_FILE: &str = "target_file";
pub const COL_CHAR_START: &str = "char_start";
pub const COL_CHAR_END: &str = "char_end";
pub const COL_CHECKSUM: &str = "checksum";

/// Header-Keys (Footer-Key-Value).
pub const KEY_SCHEMA_VERSION: &str = "schema_version";
pub const KEY_MAX_STAMP: &str = "max_stamp";
pub const KEY_HASH_ALGORITHM: &str = "hash_algorithm";
pub const KEY_HASH_VERSION: &str = "hash_version";
pub const KEY_MODEL_ID: &str = "model_id";
pub const KEY_MODEL_REVISION: &str = "model_revision";
pub const KEY_DIMENSION: &str = "dimension";
pub const KEY_POOLING: &str = "pooling";
pub const KEY_NORMALIZATION: &str = "normalization";
pub const KEY_QUERY_PREFIX: &str = "query_prefix";
pub const KEY_PASSAGE_PREFIX: &str = "passage_prefix";
pub const KEY_STORAGE_DTYPE: &str = "storage_dtype";
pub const KEY_COMPUTE_DTYPE: &str = "compute_dtype";
pub const KEY_MAX_SEQ_LEN: &str = "max_seq_len";
pub const KEY_TOKENIZER_FINGERPRINT: &str = "tokenizer_fingerprint";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeaderPooling { MaskedMean, PerToken }     // Text: "masked_mean", "per_token"

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HashAlgorithm { Fast64, Fast128, Cryptographic } // Text: "fast_64_bit", "fast_128_bit", "cryptographic"

/// "<algorithmus>:<16 Hex-Stellen, klein, führende Nullen>"
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenizerFingerprint(/* String, privat */);
impl TokenizerFingerprint {
    /// Format: `{algorithm}:{hash:016x}`.
    pub fn new(algorithm: &str, hash: u64) -> TokenizerFingerprint;
    /// Berechnet den Fingerprint: Algorithmus `TOKENIZER_FP_ALGORITHM`, Hash = FNV-1a-64
    /// (offset 0xcbf29ce484222325, prime 0x100000001b3) über die UTF-8-Bytes von
    /// `loaded.tokenizer().to_string(false)`.
    pub fn from_loaded(loaded: &LoadedTokenizer) -> Result<TokenizerFingerprint, FormatError>;
    /// Prüft die Form (nicht leerer Algorithmus ohne ':' vor dem letzten ':', genau 16 Hex-Stellen klein).
    pub fn parse(s: &str) -> Option<TokenizerFingerprint>;
    pub fn as_str(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch { pub key: String, pub expected: String, pub found: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatError {
    /// Header unlesbar/unvollständig/abweichend; alle Abweichungen gesammelt (nie leer). Reihenfolge = Key-Reihenfolge der Struktur.
    SchemaIncompatible { mismatches: Vec<Mismatch> },
    /// Spalten-Schema weicht ab; beide Schemata als Text (Name: Typ, nullable-Flag je Spalte).
    ColumnSchemaMismatch { expected: String, found: String },
    /// Tokenizer ließ sich nicht serialisieren.
    TokenizerDefinition(String),
}
impl std::fmt::Display for FormatError {}   // deutsch; nennt alle Key/erwartet/gefunden bzw. beide Schemata
impl std::error::Error for FormatError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexHeader {
    pub schema_version: u64,
    pub max_stamp: u64,                 // 0 = leer
    pub hash_algorithm: HashAlgorithm,
    pub hash_version: u64,
    pub model_id: String,
    pub model_revision: String,
    pub dimension: usize,
    pub pooling: HeaderPooling,
    pub normalization: Normalization,
    pub query_prefix: String,
    pub passage_prefix: String,
    pub storage_dtype: DType,
    pub compute_dtype: String,          // Text, z. B. "cpu=F32,gpu=F32"
    pub max_seq_len: usize,
    pub tokenizer_fingerprint: TokenizerFingerprint,
}

impl IndexHeader {
    /// Ableitung aus Modellbeschreibung.
    pub fn from_descriptor(d: &ModelDescriptor, model_revision: &str,
                           tokenizer_fingerprint: TokenizerFingerprint) -> Result<IndexHeader, FormatError>;
    pub fn dimension(&self) -> usize;
    /// dimension * 2 (FP16).
    pub fn vector_bytes(&self) -> usize;
    /// Kopie mit anderem Max-Stamp.
    pub fn with_max_stamp(&self, max_stamp: u64) -> IndexHeader;
    /// Verlustfrei: genau 15 Paare in der Key-Reihenfolge der Konstanten oben.
    pub fn to_kv(&self) -> Vec<(String, String)>;
    /// Liest und validiert; unbekannte Zusatz-Keys (z. B. "ARROW:schema") werden ignoriert.
    pub fn from_kv(pairs: &[(String, String)]) -> Result<IndexHeader, FormatError>;
    /// Bindung dieses (Datei-)Headers an `expected` (aus dem aktuellen Modell).
    pub fn check_binding(&self, expected: &IndexHeader) -> Result<(), FormatError>;
    /// Arrow-Schema der Indexdatei für diesen Header (Dimension).
    pub fn column_schema(&self) -> Result<Schema, FormatError>;
}

/// Erwartetes Spalten-Schema für eine Dimension.
pub fn column_schema(dimension: usize) -> Result<Schema, FormatError>;
/// Vergleicht ein gefundenes Schema mit dem erwarteten für `dimension`.
pub fn check_column_schema(found: &Schema, dimension: usize) -> Result<(), FormatError>;
```

Alle Typen sind reine Wertetypen und `Send + Sync + 'static`. Alle Funktionen sind frei von Zustand und Synchronisation.

## 4. Behavior und Invarianten

**Spalten-Schema** (`column_schema(dim)`), in genau dieser Reihenfolge, alle `nullable = false`:

| # | Name | Typ |
|---|---|---|
| 1 | `entry_id` | `UInt64` |
| 2 | `change_stamp` | `UInt64` |
| 3 | `vector` | `FixedSizeList(Field("item", Float16, nullable=false), dim as i32)` |
| 4 | `target_file` | `Utf8` |
| 5 | `char_start` | `UInt64` |
| 6 | `char_end` | `UInt64` |
| 7 | `checksum` | `UInt64` |

- `dim == 0` oder `dim > i32::MAX` ergibt `SchemaIncompatible` mit Key `dimension`.
- `check_column_schema` vergleicht Anzahl, Reihenfolge, Name, Datentyp und Nullability.
- Beim Vergleich werden Schema-Metadaten und Feld-Metadaten ignoriert. Der Name des Kind-Feldes der Liste wird nicht verglichen, wohl aber dessen Typ (`Float16`), Nullability und die Listengröße.
- Jede Abweichung ergibt `ColumnSchemaMismatch`. Darin stehen beide vollständigen Schemata, erwartet und gefunden, als Text.
- `Utf8` wird nur als `Utf8` akzeptiert, nicht als `LargeUtf8`.

**Ableitung** (`from_descriptor`):
- `schema_version = 1`, `max_stamp = 0`, `hash_algorithm = Fast64`, `hash_version = 1`.
- `model_id`, `dimension`, `normalization`, `query_prefix`, `passage_prefix` und `max_seq_len` werden unverändert aus dem Descriptor übernommen (Präfixe ohne Trimmen).
- `model_revision` und `tokenizer_fingerprint` stammen von den Parametern.
- `Pooling::Mean` wird zu `MaskedMean`. `Pooling::Cls` hat keinen Header-Wert und ergibt `SchemaIncompatible` (Key `pooling`).
- `storage_dtype` muss `DType::F16` sein, sonst `SchemaIncompatible` (Key `storage_dtype`, erwartet `F16`).
- `compute_dtype` wird als Text `cpu=<X>,gpu=<Y>` geschrieben. `<X>` und `<Y>` sind `DType::to_safetensors()` von `compute_dtypes.cpu` bzw. `.gpu`.
- `dimension == 0` ergibt `SchemaIncompatible` (Key `dimension`).
- Alle Verstöße werden gesammelt gemeldet. Die Funktion validiert sonst nichts und paniert nie.

**Key-Value-Darstellung:**
- Zahlen stehen als Dezimal-`u64`-Text, ohne Vorzeichen und ohne Leerraum.
- Enum-Texte:
  - Pooling: `masked_mean` oder `per_token`.
  - Normalisierung: `l2` oder `none`.
  - Hash-Algorithmus: `fast_64_bit`, `fast_128_bit` oder `cryptographic`.
  - Datentypen: `DType::to_safetensors()` (`F32`, `F16`, `BF16`, `U8`, `U32`, `I64`), Parsing über `DType::from_safetensors`. Die Groß-/Kleinschreibung zählt.
- Strings (`model_id`, `model_revision`, Präfixe) stehen unverändert, auch leer oder mit Leerzeichen am Ende.
- `from_kv(to_kv(h)) == Ok(h)` gilt für jeden Header, einschließlich leerer Präfixe und Revision. Dafür muss `dimension` und `max_seq_len` in `usize` passen.
- `from_kv` meldet als `Mismatch` und sammelt alle Fälle, nicht nur den ersten:
  - Fehlender Key: `found = "<fehlt>"`, `expected` beschreibt den Typ, z. B. `u64` oder `masked_mean|per_token`.
  - Nicht numerischer Wert bei einem Zahlen-Key.
  - Unbekannter Enum-Wert.
  - Ungültige Form des Fingerprints.
  - `dimension == 0`.
  - `found` enthält jeweils den gelesenen Wert.
- Bei doppeltem Key gewinnt der letzte Eintrag.
- `compute_dtype` wird als beliebiger Text akzeptiert.
- `schema_version` wird beim Lesen nur als Zahl geprüft. Eine Abweichung von 1 erkennt `check_binding`.

**Bindung** (`file.check_binding(&expected)`):
- Verglichen werden Gleichheit von:
  - `schema_version`, `hash_algorithm`, `hash_version`
  - `model_id`, `model_revision`, `dimension`
  - `pooling`, `normalization`, `query_prefix`, `passage_prefix`
  - `storage_dtype`, `max_seq_len`, `tokenizer_fingerprint`
- `max_stamp` und `compute_dtype` werden ignoriert. Unterschiede dort sind nie ein Fehler.
- Bei Abweichungen kommt `SchemaIncompatible` mit einem `Mismatch` je abweichendem Key.
  - `expected` ist der Wert aus `expected`, `found` der Wert aus `self`, jeweils im Key-Value-Text.
  - Alle Abweichungen werden gesammelt, die Reihenfolge entspricht der Key-Reihenfolge.
- Sonst `Ok(())`.
- Model-ID und Dimension sind die einzige harte Modellbindung. Die übrigen Merkmale sind die Pipeline-Merkmale, die Vektoren vergleichbar machen.

**Fingerprint:**
- Format `fnv1a64:` plus genau 16 kleine Hex-Stellen mit führenden Nullen. Beispiel: Hash `0x2a` ergibt `fnv1a64:000000000000002a`.
- `from_loaded` ist deterministisch. Gleiche Tokenizer-Definition ergibt denselben Fingerprint, auch über mehrere Aufrufe und Threads.
- Schlägt `to_string(false)` fehl, kommt `TokenizerDefinition(msg)`.
- `parse(as_str())` ist die Identität.

**Hash-Festlegung:**
- `HASH_ALGORITHM = Fast64` und `HASH_VERSION = 1` gelten für die Content-Prüfung bei Dateiänderungen. Eine Änderung von Algorithmus oder Version bricht die Bindung.
- Dieses Modul berechnet diesen Content-Hash nicht.

**Abgeleitete Werte:**
- `vector_bytes() == dimension * 2`.
- `with_max_stamp(m)` ändert nur `max_stamp`.

**Concurrency:** Alle Funktionen sind rein und nehmen höchstens `&self`. Sie sind beliebig parallel aufrufbar. Es gibt keine Panics bei beliebigen Eingaben.

## 5. Relied-on elements

```rust
// layout (verbatim)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType { F32, F16, BF16, U8, U32, I64 }
impl DType {
    pub const fn to_safetensors(self) -> &'static str;   // "F32","F16","BF16","U8","U32","I64"
    pub fn from_safetensors(dtype: &str, source: &str) -> Result<DType, LayoutError>; // case-sensitiv
}

// model_descriptor (verbatim, genutzte Felder)
pub enum Pooling { Mean, Cls }
pub enum Normalization { L2, None }          // Debug, Clone, Copy, PartialEq, Eq, Hash
pub struct ComputeDTypes { pub cpu: DType, pub gpu: DType }
pub struct ModelDescriptor { pub model_id: String, pub dimension: usize, pub pooling: Pooling,
    pub normalization: Normalization, pub query_prefix: String, pub passage_prefix: String,
    pub storage_dtype: DType, pub compute_dtypes: ComputeDTypes, pub max_seq_len: usize, /* weitere ungenutzt */ }

// tokenizer_loader (verbatim, genutzt)
impl LoadedTokenizer { pub fn tokenizer(&self) -> &tokenizers::Tokenizer; }
// tokenizers::Tokenizer::to_string(pretty: bool) -> Result<String, _>
```

- Der Stamp im Header ist der Rohwert aus `StampCounter::max_raw()`, `None` wird als 0 geschrieben. `ChangeStamp` implementiert kein `Ord`, das Modul vergleicht Stamps nicht.
- Parquet-Indizes haben ein Arrow-Schema. Die Spalte `change_stamp` wird von Index-Implementierungen als `u64` aus `ChangeStamp::as_raw()` gefüllt.
- Der Index-Vertrag hat `EntryId(u64)`. Die `entry_id`-Spalte trägt dessen Rohwert.