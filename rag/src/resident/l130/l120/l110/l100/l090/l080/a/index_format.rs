use std::collections::HashMap;
use std::fmt;
use arrow::datatypes::{DataType, Field, Schema};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::tokenizer_loader::LoadedTokenizer;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::{
    ModelDescriptor, Normalization, Pooling,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
pub const SCHEMA_VERSION: u64 = 1;
pub const HASH_ALGORITHM: HashAlgorithm = HashAlgorithm::Fast64;
pub const HASH_VERSION: u64 = 1;
pub const TOKENIZER_FP_ALGORITHM: &str = "fnv1a64";
pub const COL_ENTRY_ID: &str = "entry_id";
pub const COL_CHANGE_STAMP: &str = "change_stamp";
pub const COL_VECTOR: &str = "vector";
pub const COL_TARGET_FILE: &str = "target_file";
pub const COL_CHAR_START: &str = "char_start";
pub const COL_CHAR_END: &str = "char_end";
pub const COL_CHECKSUM: &str = "checksum";
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
pub const KEY_PREPROCESSING_ID: &str = "preprocessing_id";
const MISSING: &str = "<fehlt>";
const EXPECTED_U64: &str = "u64";
const EXPECTED_POSITIVE: &str = "u64 > 0";
const EXPECTED_STRING: &str = "string";
const EXPECTED_POOLING: &str = "masked_mean|per_token";
const EXPECTED_NORMALIZATION: &str = "l2|none";
const EXPECTED_HASH_ALGORITHM: &str = "fast_64_bit|fast_128_bit|cryptographic";
const EXPECTED_DTYPE: &str = "F32|F16|BF16|U8|U32|I64";
const EXPECTED_FINGERPRINT: &str = "<algorithm>:<16 lowercase hex digits>";
const EXPECTED_VECTOR_DIMENSION: &str = "1..=2147483647";
pub const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const FINGERPRINT_HEX_DIGITS: usize = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeaderPooling {
    MaskedMean,
    PerToken,
}
impl HeaderPooling {
    fn as_text(self) -> &'static str {
        match self {
            HeaderPooling::MaskedMean => "masked_mean",
            HeaderPooling::PerToken => "per_token",
        }
    }
    fn from_text(s: &str) -> Option<HeaderPooling> {
        match s {
            "masked_mean" => Some(HeaderPooling::MaskedMean),
            "per_token" => Some(HeaderPooling::PerToken),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    Fast64,
    Fast128,
    Cryptographic,
}
impl HashAlgorithm {
    fn as_text(self) -> &'static str {
        match self {
            HashAlgorithm::Fast64 => "fast_64_bit",
            HashAlgorithm::Fast128 => "fast_128_bit",
            HashAlgorithm::Cryptographic => "cryptographic",
        }
    }
    fn from_text(s: &str) -> Option<HashAlgorithm> {
        match s {
            "fast_64_bit" => Some(HashAlgorithm::Fast64),
            "fast_128_bit" => Some(HashAlgorithm::Fast128),
            "cryptographic" => Some(HashAlgorithm::Cryptographic),
            _ => None,
        }
    }
}
fn normalization_text(n: Normalization) -> &'static str {
    match n {
        Normalization::L2 => "l2",
        Normalization::None => "none",
    }
}
fn normalization_from_text(s: &str) -> Option<Normalization> {
    match s {
        "l2" => Some(Normalization::L2),
        "none" => Some(Normalization::None),
        _ => None,
    }
}
fn dtype_from_text(s: &str) -> Option<DType> {
    DType::from_safetensors(s, KEY_STORAGE_DTYPE).ok()
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenizerFingerprint(String);
impl TokenizerFingerprint {
    pub fn new(algorithm: &str, hash: u64) -> TokenizerFingerprint {
        TokenizerFingerprint(format!("{algorithm}:{hash:016x}"))
    }
    pub fn from_loaded(
        loaded: &LoadedTokenizer,
    ) -> Result<TokenizerFingerprint, FormatError> {
        let definition = loaded
            .tokenizer()
            .to_string(false)
            .map_err(|e| FormatError::TokenizerDefinition(e.to_string()))?;
        Ok(
            TokenizerFingerprint::new(
                TOKENIZER_FP_ALGORITHM,
                fnv1a64(definition.as_bytes()),
            ),
        )
    }
    pub fn parse(s: &str) -> Option<TokenizerFingerprint> {
        let (algorithm, hex) = s.rsplit_once(':')?;
        if algorithm.is_empty() || algorithm.contains(':') {
            return None;
        }
        let hex_ok = hex.len() == FINGERPRINT_HEX_DIGITS
            && hex.bytes().all(|b| matches!(b, b'0'..= b'9' | b'a'..= b'f'));
        hex_ok.then(|| TokenizerFingerprint(s.to_owned()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
fn fnv1a64(bytes: &[u8]) -> u64 {
    fnv1a64_extend(FNV_OFFSET, bytes)
}
/// Continues an FNV-1a 64 hash from `state` over `bytes`.
pub fn fnv1a64_extend(state: u64, bytes: &[u8]) -> u64 {
    bytes.iter().fold(state, |hash, &b| (hash ^ u64::from(b)).wrapping_mul(FNV_PRIME))
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch {
    pub key: String,
    pub expected: String,
    pub found: String,
}
impl Mismatch {
    fn new(key: &str, expected: &str, found: &str) -> Mismatch {
        Mismatch {
            key: key.to_owned(),
            expected: expected.to_owned(),
            found: found.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatError {
    SchemaIncompatible { mismatches: Vec<Mismatch> },
    ColumnSchemaMismatch { expected: String, found: String },
    TokenizerDefinition(String),
}
impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormatError::SchemaIncompatible { mismatches } => {
                write!(f, "Indexdatei nicht kompatibel:")?;
                for m in mismatches {
                    write!(
                        f, " [{}: erwartet '{}', gefunden '{}']", m.key, m.expected, m
                        .found
                    )?;
                }
                Ok(())
            }
            FormatError::ColumnSchemaMismatch { expected, found } => {
                write!(
                    f,
                    "Spalten-Schema weicht ab: erwartet [{expected}], gefunden [{found}]"
                )
            }
            FormatError::TokenizerDefinition(msg) => {
                write!(f, "Tokenizer-Definition nicht serialisierbar: {msg}")
            }
        }
    }
}
impl std::error::Error for FormatError {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexHeader {
    pub schema_version: u64,
    pub max_stamp: u64,
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
    pub compute_dtype: String,
    pub max_seq_len: usize,
    pub tokenizer_fingerprint: TokenizerFingerprint,
    pub preprocessing_id: String,
}
impl IndexHeader {
    pub fn from_descriptor(
        d: &ModelDescriptor,
        model_revision: &str,
        tokenizer_fingerprint: TokenizerFingerprint,
        preprocessing_id: &str,
    ) -> Result<IndexHeader, FormatError> {
        let mut mismatches = Vec::new();
        if d.dimension == 0 {
            mismatches.push(Mismatch::new(KEY_DIMENSION, EXPECTED_POSITIVE, "0"));
        }
        let pooling = match d.pooling {
            Pooling::Mean => Some(HeaderPooling::MaskedMean),
            Pooling::Cls => {
                mismatches.push(Mismatch::new(KEY_POOLING, EXPECTED_POOLING, "cls"));
                None
            }
        };
        if d.storage_dtype != DType::F16 {
            mismatches
                .push(
                    Mismatch::new(
                        KEY_STORAGE_DTYPE,
                        DType::F16.to_safetensors(),
                        d.storage_dtype.to_safetensors(),
                    ),
                );
        }
        let Some(pooling) = pooling else {
            return Err(FormatError::SchemaIncompatible {
                mismatches,
            });
        };
        if !mismatches.is_empty() {
            return Err(FormatError::SchemaIncompatible {
                mismatches,
            });
        }
        Ok(IndexHeader {
            schema_version: SCHEMA_VERSION,
            max_stamp: 0,
            hash_algorithm: HASH_ALGORITHM,
            hash_version: HASH_VERSION,
            model_id: d.model_id.clone(),
            model_revision: model_revision.to_owned(),
            dimension: d.dimension,
            pooling,
            normalization: d.normalization,
            query_prefix: d.query_prefix.clone(),
            passage_prefix: d.passage_prefix.clone(),
            storage_dtype: d.storage_dtype,
            compute_dtype: format!(
                "cpu={},gpu={}", d.compute_dtypes.cpu.to_safetensors(), d.compute_dtypes
                .gpu.to_safetensors()
            ),
            max_seq_len: d.max_seq_len,
            tokenizer_fingerprint,
            preprocessing_id: preprocessing_id.to_owned(),
        })
    }
    pub fn dimension(&self) -> usize {
        self.dimension
    }
    pub fn vector_bytes(&self) -> usize {
        self.dimension.saturating_mul(2)
    }
    pub fn with_max_stamp(&self, max_stamp: u64) -> IndexHeader {
        IndexHeader {
            max_stamp,
            ..self.clone()
        }
    }
    pub fn to_kv(&self) -> Vec<(String, String)> {
        let pair = |key: &str, value: String| (key.to_owned(), value);
        vec![
            pair(KEY_SCHEMA_VERSION, self.schema_version.to_string()),
            pair(KEY_MAX_STAMP, self.max_stamp.to_string()), pair(KEY_HASH_ALGORITHM,
            self.hash_algorithm.as_text().to_owned()), pair(KEY_HASH_VERSION, self
            .hash_version.to_string()), pair(KEY_MODEL_ID, self.model_id.clone()),
            pair(KEY_MODEL_REVISION, self.model_revision.clone()), pair(KEY_DIMENSION,
            self.dimension.to_string()), pair(KEY_POOLING, self.pooling.as_text()
            .to_owned()), pair(KEY_NORMALIZATION, normalization_text(self.normalization)
            .to_owned()), pair(KEY_QUERY_PREFIX, self.query_prefix.clone()),
            pair(KEY_PASSAGE_PREFIX, self.passage_prefix.clone()),
            pair(KEY_STORAGE_DTYPE, self.storage_dtype.to_safetensors().to_owned()),
            pair(KEY_COMPUTE_DTYPE, self.compute_dtype.clone()), pair(KEY_MAX_SEQ_LEN,
            self.max_seq_len.to_string()), pair(KEY_TOKENIZER_FINGERPRINT, self
            .tokenizer_fingerprint.as_str().to_owned(),), pair(KEY_PREPROCESSING_ID, self
            .preprocessing_id.clone()),
        ]
    }
    pub fn from_kv(pairs: &[(String, String)]) -> Result<IndexHeader, FormatError> {
        let mut r = KvReader {
            map: pairs.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect(),
            mismatches: Vec::new(),
        };
        let schema_version = r.number(KEY_SCHEMA_VERSION);
        let max_stamp = r.number(KEY_MAX_STAMP);
        let hash_algorithm = r
            .choice(
                KEY_HASH_ALGORITHM,
                EXPECTED_HASH_ALGORITHM,
                HashAlgorithm::from_text,
            );
        let hash_version = r.number(KEY_HASH_VERSION);
        let model_id = r.text(KEY_MODEL_ID);
        let model_revision = r.text(KEY_MODEL_REVISION);
        let dimension = r.dimension(KEY_DIMENSION);
        let pooling = r.choice(KEY_POOLING, EXPECTED_POOLING, HeaderPooling::from_text);
        let normalization = r
            .choice(KEY_NORMALIZATION, EXPECTED_NORMALIZATION, normalization_from_text);
        let query_prefix = r.text(KEY_QUERY_PREFIX);
        let passage_prefix = r.text(KEY_PASSAGE_PREFIX);
        let storage_dtype = r.choice(KEY_STORAGE_DTYPE, EXPECTED_DTYPE, dtype_from_text);
        let compute_dtype = r.text(KEY_COMPUTE_DTYPE);
        let max_seq_len = r.size(KEY_MAX_SEQ_LEN);
        let tokenizer_fingerprint = r
            .choice(
                KEY_TOKENIZER_FINGERPRINT,
                EXPECTED_FINGERPRINT,
                TokenizerFingerprint::parse,
            );
        let preprocessing_id = r.text(KEY_PREPROCESSING_ID);
        let header = (|| {
            Some(IndexHeader {
                schema_version: schema_version?,
                max_stamp: max_stamp?,
                hash_algorithm: hash_algorithm?,
                hash_version: hash_version?,
                model_id: model_id?,
                model_revision: model_revision?,
                dimension: dimension?,
                pooling: pooling?,
                normalization: normalization?,
                query_prefix: query_prefix?,
                passage_prefix: passage_prefix?,
                storage_dtype: storage_dtype?,
                compute_dtype: compute_dtype?,
                max_seq_len: max_seq_len?,
                tokenizer_fingerprint: tokenizer_fingerprint?,
                preprocessing_id: preprocessing_id?,
            })
        })();
        match header {
            Some(h) if r.mismatches.is_empty() => Ok(h),
            _ => {
                Err(FormatError::SchemaIncompatible {
                    mismatches: r.mismatches,
                })
            }
        }
    }
    pub fn check_binding(&self, expected: &IndexHeader) -> Result<(), FormatError> {
        let mismatches: Vec<Mismatch> = self
            .to_kv()
            .into_iter()
            .zip(expected.to_kv())
            .filter(|((key, _), _)| key != KEY_MAX_STAMP && key != KEY_COMPUTE_DTYPE)
            .filter(|((_, found), (_, wanted))| found != wanted)
            .map(|((key, found), (_, wanted))| Mismatch {
                key,
                expected: wanted,
                found,
            })
            .collect();
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(FormatError::SchemaIncompatible {
                mismatches,
            })
        }
    }
    pub fn column_schema(&self) -> Result<Schema, FormatError> {
        column_schema(self.dimension)
    }
}
struct KvReader<'a> {
    map: HashMap<&'a str, &'a str>,
    mismatches: Vec<Mismatch>,
}
impl<'a> KvReader<'a> {
    fn raw(&mut self, key: &str, expected: &str) -> Option<&'a str> {
        let found = self.map.get(key).copied();
        if found.is_none() {
            self.mismatches.push(Mismatch::new(key, expected, MISSING));
        }
        found
    }
    fn text(&mut self, key: &str) -> Option<String> {
        self.raw(key, EXPECTED_STRING).map(str::to_owned)
    }
    fn number(&mut self, key: &str) -> Option<u64> {
        let raw = self.raw(key, EXPECTED_U64)?;
        let parsed = parse_u64(raw);
        if parsed.is_none() {
            self.mismatches.push(Mismatch::new(key, EXPECTED_U64, raw));
        }
        parsed
    }
    fn size(&mut self, key: &str) -> Option<usize> {
        let raw = self.raw(key, EXPECTED_U64)?;
        let parsed = parse_u64(raw).and_then(|n| usize::try_from(n).ok());
        if parsed.is_none() {
            self.mismatches.push(Mismatch::new(key, EXPECTED_U64, raw));
        }
        parsed
    }
    fn dimension(&mut self, key: &str) -> Option<usize> {
        let raw = self.raw(key, EXPECTED_POSITIVE)?;
        let parsed = parse_u64(raw)
            .filter(|&n| n > 0)
            .and_then(|n| usize::try_from(n).ok());
        if parsed.is_none() {
            self.mismatches.push(Mismatch::new(key, EXPECTED_POSITIVE, raw));
        }
        parsed
    }
    fn choice<T>(
        &mut self,
        key: &str,
        expected: &str,
        parse: impl FnOnce(&str) -> Option<T>,
    ) -> Option<T> {
        let raw = self.raw(key, expected)?;
        let parsed = parse(raw);
        if parsed.is_none() {
            self.mismatches.push(Mismatch::new(key, expected, raw));
        }
        parsed
    }
}
fn parse_u64(s: &str) -> Option<u64> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}
fn dimension_error(dimension: usize) -> FormatError {
    FormatError::SchemaIncompatible {
        mismatches: vec![
            Mismatch::new(KEY_DIMENSION, EXPECTED_VECTOR_DIMENSION, & dimension
            .to_string(),)
        ],
    }
}
pub fn column_schema(dimension: usize) -> Result<Schema, FormatError> {
    let size = i32::try_from(dimension)
        .ok()
        .filter(|&n| n > 0)
        .ok_or_else(|| dimension_error(dimension))?;
    let item = Field::new("item", DataType::Float16, false);
    Ok(
        Schema::new(
            vec![
                Field::new(COL_ENTRY_ID, DataType::UInt64, false),
                Field::new(COL_CHANGE_STAMP, DataType::UInt64, false),
                Field::new(COL_VECTOR, DataType::FixedSizeList(item.into(), size),
                false,), Field::new(COL_TARGET_FILE, DataType::Utf8, false),
                Field::new(COL_CHAR_START, DataType::UInt64, false),
                Field::new(COL_CHAR_END, DataType::UInt64, false),
                Field::new(COL_CHECKSUM, DataType::UInt64, false),
            ],
        ),
    )
}
pub fn check_column_schema(found: &Schema, dimension: usize) -> Result<(), FormatError> {
    let expected = column_schema(dimension)?;
    let fields_match = expected.fields().len() == found.fields().len()
        && expected
            .fields()
            .iter()
            .zip(found.fields().iter())
            .all(|(e, f)| {
                e.name() == f.name() && e.is_nullable() == f.is_nullable()
                    && data_types_match(e.data_type(), f.data_type())
            });
    if fields_match {
        Ok(())
    } else {
        Err(FormatError::ColumnSchemaMismatch {
            expected: schema_text(&expected),
            found: schema_text(found),
        })
    }
}
fn data_types_match(expected: &DataType, found: &DataType) -> bool {
    match (expected, found) {
        (DataType::FixedSizeList(e, en), DataType::FixedSizeList(f, fn_)) => {
            en == fn_ && e.is_nullable() == f.is_nullable()
                && data_types_match(e.data_type(), f.data_type())
        }
        _ => expected == found,
    }
}
fn data_type_text(data_type: &DataType) -> String {
    match data_type {
        DataType::FixedSizeList(item, size) => {
            format!(
                "FixedSizeList({}, nullable={}, size={size})", data_type_text(item
                .data_type()), item.is_nullable()
            )
        }
        other => format!("{other:?}"),
    }
}
fn schema_text(schema: &Schema) -> String {
    schema
        .fields()
        .iter()
        .map(|f| {
            format!(
                "{}: {}, nullable={}", f.name(), data_type_text(f.data_type()), f
                .is_nullable()
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}
#[cfg(test)]
#[path = "test/index_format.rs"]
mod tests;
