//! Arrow schema, footer header keys and schema versioning of the persisted index.
use std::collections::HashMap;
use std::sync::Arc;
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use parquet::file::metadata::KeyValue;
use super::{content_hash, HashAlgo, IndexHeader};
use crate::resident::error::IndexError;
use crate::resident::memory::DType;
use crate::resident::model::{ModelSpec, Normalize, Pooling};
pub const SCHEMA_VERSION: u32 = 1;
pub const COL_ENTRY_ID: usize = 0;
pub const COL_STAMP: usize = 1;
pub const COL_VECTOR: usize = 2;
pub const COL_TARGET_FILE: usize = 3;
pub const COL_CHAR_START: usize = 4;
pub const COL_CHAR_END: usize = 5;
pub const COL_CHECKSUM: usize = 6;
pub const NAME_VECTOR: &str = "vector";
const KEY_SCHEMA_VERSION: &str = "resident.schema_version";
const KEY_HIGH_WATER: &str = "resident.high_water";
const KEY_HASH_ALGO: &str = "payload.hash_algo";
const KEY_HASH_VERSION: &str = "payload.hash_version";
const KEY_MODEL_ID: &str = "embedding.model_id";
const KEY_MODEL_REVISION: &str = "embedding.model_revision";
const KEY_DIM: &str = "embedding.dim";
const KEY_POOLING: &str = "embedding.pooling";
const KEY_NORMALIZE: &str = "embedding.normalize";
const KEY_PREFIX_QUERY: &str = "embedding.prefix_query";
const KEY_PREFIX_PASSAGE: &str = "embedding.prefix_passage";
const KEY_STORAGE_DTYPE: &str = "embedding.storage_dtype";
const KEY_COMPUTE_DTYPE: &str = "embedding.compute_dtype";
const KEY_MAX_SEQ_LEN: &str = "embedding.max_seq_len";
const KEY_TOKENIZER_SHA: &str = "tokenizer.sha";
/// Column layout; the stamp lives in its own column so that scans never page in vectors.
pub fn arrow_schema(dim: usize) -> SchemaRef {
    let item = Arc::new(Field::new("item", DataType::Float16, false));
    Arc::new(
        Schema::new(
            vec![
                Field::new("entry_id", DataType::UInt64, false), Field::new("stamp",
                DataType::UInt64, false), Field::new(NAME_VECTOR,
                DataType::FixedSizeList(item, dim as i32), false),
                Field::new("target_file", DataType::Utf8, false),
                Field::new("char_start", DataType::UInt64, false), Field::new("char_end",
                DataType::UInt64, false), Field::new("checksum", DataType::UInt64,
                false),
            ],
        ),
    )
}
/// Name, type and nullability must match; field metadata is ignored.
pub fn check_fields(found: &Schema, expected: &Schema) -> Result<(), IndexError> {
    let describe = |s: &Schema| {
        s.fields()
            .iter()
            .map(|f| {
                format!(
                    "{}:{}{}", f.name(), f.data_type(), if f.is_nullable() { "?" } else {
                    "" }
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    };
    let same = found.fields().len() == expected.fields().len()
        && found
            .fields()
            .iter()
            .zip(expected.fields().iter())
            .all(|(a, b)| {
                a.name() == b.name() && a.data_type() == b.data_type()
                    && a.is_nullable() == b.is_nullable()
            });
    if same {
        Ok(())
    } else {
        Err(IndexError::IncompatibleSchema {
            expected: describe(expected),
            found: describe(found),
        })
    }
}
/// Everything that binds an index file to the model configuration that produced it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaHeader {
    pub index: IndexHeader,
    pub schema_version: u32,
    pub model_revision: String,
    pub pooling: Pooling,
    pub normalize: Normalize,
    pub prefix_query: String,
    pub prefix_passage: String,
    pub storage_dtype: DType,
    /// Informational: mixed CPU/GPU indexing is allowed, so this is not compared.
    pub compute_dtype: String,
    pub max_seq_len: usize,
    pub tokenizer_sha: String,
}
impl SchemaHeader {
    pub fn from_spec(
        spec: &ModelSpec,
        model_revision: impl Into<String>,
        tokenizer_sha: impl Into<String>,
    ) -> Result<Self, IndexError> {
        if spec.dtype != DType::F16 {
            return Err(IndexError::IncompatibleSchema {
                expected: format!("{KEY_STORAGE_DTYPE}=f16"),
                found: format!("{KEY_STORAGE_DTYPE}={}", dtype_name(spec.dtype)),
            });
        }
        Ok(Self {
            index: IndexHeader {
                model_id: spec.model_id.clone(),
                dim: spec.dim as u32,
                high_water: 0,
                hash_algo: HashAlgo::Xxh3_64,
                hash_version: 1,
            },
            schema_version: SCHEMA_VERSION,
            model_revision: model_revision.into(),
            pooling: spec.pooling,
            normalize: spec.normalize,
            prefix_query: spec.prefixes.query.clone(),
            prefix_passage: spec.prefixes.passage.clone(),
            storage_dtype: spec.dtype,
            compute_dtype: format!(
                "cpu={};gpu={}", dtype_name(spec.compute.cpu), dtype_name(spec.compute
                .gpu)
            ),
            max_seq_len: spec.max_seq_len,
            tokenizer_sha: tokenizer_sha.into(),
        })
    }
    pub fn dim(&self) -> usize {
        self.index.dim as usize
    }
    pub fn vector_bytes(&self) -> usize {
        self.dim() * DType::F16.size_bytes()
    }
    pub fn with_high_water(&self, high_water: u64) -> Self {
        let mut h = self.clone();
        h.index.high_water = high_water;
        h
    }
    /// Keys that must be equal for an index to be usable with the active model.
    fn binding_entries(&self) -> Vec<(&'static str, String)> {
        vec![
            (KEY_SCHEMA_VERSION, self.schema_version.to_string()), (KEY_HASH_ALGO,
            hash_algo_name(self.index.hash_algo).into()), (KEY_HASH_VERSION, self.index
            .hash_version.to_string()), (KEY_MODEL_ID, self.index.model_id.clone()),
            (KEY_MODEL_REVISION, self.model_revision.clone()), (KEY_DIM, self.index.dim
            .to_string()), (KEY_POOLING, pooling_name(self.pooling).into()),
            (KEY_NORMALIZE, normalize_name(self.normalize).into()), (KEY_PREFIX_QUERY,
            self.prefix_query.clone()), (KEY_PREFIX_PASSAGE, self.prefix_passage
            .clone()), (KEY_STORAGE_DTYPE, dtype_name(self.storage_dtype).into()),
            (KEY_MAX_SEQ_LEN, self.max_seq_len.to_string()), (KEY_TOKENIZER_SHA, self
            .tokenizer_sha.clone()),
        ]
    }
    pub fn to_key_values(&self) -> Vec<KeyValue> {
        let mut entries = self.binding_entries();
        entries.push((KEY_HIGH_WATER, self.index.high_water.to_string()));
        entries.push((KEY_COMPUTE_DTYPE, self.compute_dtype.clone()));
        entries.into_iter().map(|(k, v)| KeyValue::new(k.to_string(), v)).collect()
    }
    pub fn from_key_values(kvs: &[KeyValue]) -> Result<Self, IndexError> {
        let map: HashMap<&str, &str> = kvs
            .iter()
            .filter_map(|kv| kv.value.as_deref().map(|v| (kv.key.as_str(), v)))
            .collect();
        let get = |key: &'static str| -> Result<&str, IndexError> {
            map.get(key)
                .copied()
                .ok_or_else(|| IndexError::IncompatibleSchema {
                    expected: format!("{key}=<present>"),
                    found: format!("{key}=<missing>"),
                })
        };
        let parse = |key: &'static str, what: &str| -> IndexError {
            IndexError::IncompatibleSchema {
                expected: format!("{key}=<{what}>"),
                found: format!("{key}={}", map.get(key).copied().unwrap_or_default()),
            }
        };
        let num = |key: &'static str| -> Result<u64, IndexError> {
            get(key)?.parse::<u64>().map_err(|_| parse(key, "integer"))
        };
        let hash_algo = hash_algo_parse(get(KEY_HASH_ALGO)?)
            .ok_or_else(|| parse(KEY_HASH_ALGO, "known algorithm"))?;
        Ok(Self {
            index: IndexHeader {
                model_id: get(KEY_MODEL_ID)?.to_string(),
                dim: num(KEY_DIM)? as u32,
                high_water: num(KEY_HIGH_WATER)?,
                hash_algo,
                hash_version: num(KEY_HASH_VERSION)? as u32,
            },
            schema_version: num(KEY_SCHEMA_VERSION)? as u32,
            model_revision: get(KEY_MODEL_REVISION)?.to_string(),
            pooling: pooling_parse(get(KEY_POOLING)?)
                .ok_or_else(|| parse(KEY_POOLING, "known pooling"))?,
            normalize: normalize_parse(get(KEY_NORMALIZE)?)
                .ok_or_else(|| parse(KEY_NORMALIZE, "known normalization"))?,
            prefix_query: get(KEY_PREFIX_QUERY)?.to_string(),
            prefix_passage: get(KEY_PREFIX_PASSAGE)?.to_string(),
            storage_dtype: dtype_parse(get(KEY_STORAGE_DTYPE)?)
                .ok_or_else(|| parse(KEY_STORAGE_DTYPE, "known dtype"))?,
            compute_dtype: get(KEY_COMPUTE_DTYPE)?.to_string(),
            max_seq_len: num(KEY_MAX_SEQ_LEN)? as usize,
            tokenizer_sha: get(KEY_TOKENIZER_SHA)?.to_string(),
        })
    }
    /// `self` is the header read from a file, `expected` the one derived from the active model.
    pub fn check_against(&self, expected: &SchemaHeader) -> Result<(), IndexError> {
        let (mut want, mut have) = (Vec::new(), Vec::new());
        for ((key, e), (_, f)) in expected
            .binding_entries()
            .into_iter()
            .zip(self.binding_entries())
        {
            if e != f {
                want.push(format!("{key}={e}"));
                have.push(format!("{key}={f}"));
            }
        }
        if want.is_empty() {
            Ok(())
        } else {
            Err(IndexError::IncompatibleSchema {
                expected: want.join("; "),
                found: have.join("; "),
            })
        }
    }
}
/// Fingerprint of `tokenizer.json` for the `tokenizer.sha` header key.
pub fn tokenizer_fingerprint(tokenizer_json: &[u8]) -> String {
    format!("xxh3_64:{:016x}", content_hash(tokenizer_json))
}
fn dtype_name(d: DType) -> &'static str {
    match d {
        DType::F32 => "f32",
        DType::F16 => "f16",
        DType::Bf16 => "bf16",
        DType::U8 => "u8",
        DType::U32 => "u32",
        DType::I64 => "i64",
    }
}
fn dtype_parse(s: &str) -> Option<DType> {
    Some(
        match s {
            "f32" => DType::F32,
            "f16" => DType::F16,
            "bf16" => DType::Bf16,
            "u8" => DType::U8,
            "u32" => DType::U32,
            "i64" => DType::I64,
            _ => return None,
        },
    )
}
fn pooling_name(p: Pooling) -> &'static str {
    match p {
        Pooling::MeanMasked => "mean_masked",
        Pooling::PerToken => "per_token",
    }
}
fn pooling_parse(s: &str) -> Option<Pooling> {
    match s {
        "mean_masked" => Some(Pooling::MeanMasked),
        "per_token" => Some(Pooling::PerToken),
        _ => None,
    }
}
fn normalize_name(n: Normalize) -> &'static str {
    match n {
        Normalize::L2 => "l2",
        Normalize::None => "none",
    }
}
fn normalize_parse(s: &str) -> Option<Normalize> {
    match s {
        "l2" => Some(Normalize::L2),
        "none" => Some(Normalize::None),
        _ => None,
    }
}
fn hash_algo_name(a: HashAlgo) -> &'static str {
    match a {
        HashAlgo::Xxh3_64 => "xxh3_64",
        HashAlgo::Xxh3_128 => "xxh3_128",
        HashAlgo::Blake3 => "blake3",
    }
}
fn hash_algo_parse(s: &str) -> Option<HashAlgo> {
    match s {
        "xxh3_64" => Some(HashAlgo::Xxh3_64),
        "xxh3_128" => Some(HashAlgo::Xxh3_128),
        "blake3" => Some(HashAlgo::Blake3),
        _ => None,
    }
}
