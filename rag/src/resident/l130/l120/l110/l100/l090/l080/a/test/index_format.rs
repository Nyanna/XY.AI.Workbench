use std::collections::HashMap;
use std::sync::Arc;
use tokenizers::Tokenizer;
use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::tokenizer_loader::TokenizerSchema;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::ComputeDTypes;
fn tokenizer(extra: &str) -> Tokenizer {
    let json = format!(
        concat!("{{\"version\":\"1.0\",\"truncation\":null,\"padding\":null,\"added_tokens\":[],",
        "\"normalizer\":null,\"pre_tokenizer\":{{\"type\":\"Whitespace\"}},\"post_processor\":null,",
        "\"decoder\":null,\"model\":{{\"type\":\"WordLevel\",",
        "\"vocab\":{{\"[PAD]\":0,\"[UNK]\":1,\"hello\":2{}}},\"unk_token\":\"[UNK]\"}}}}"),
        extra
    );
    Tokenizer::from_bytes(json.as_bytes()).unwrap()
}
fn loaded(extra: &str, max_seq_len: usize) -> LoadedTokenizer {
    let schema = TokenizerSchema {
        query_prefix: "query: ".to_string(),
        passage_prefix: "passage: ".to_string(),
    };
    LoadedTokenizer::from_tokenizer(tokenizer(extra), "test", schema, max_seq_len)
        .unwrap()
}
fn descriptor() -> ModelDescriptor {
    let mut d = ModelDescriptor::e5_small_repo_default(DType::F16);
    d.query_prefix = "query: ".to_string();
    d.passage_prefix = "passage: ".to_string();
    d.compute_dtypes = ComputeDTypes {
        cpu: DType::F32,
        gpu: DType::F16,
    };
    d
}
fn fingerprint() -> TokenizerFingerprint {
    TokenizerFingerprint::new(TOKENIZER_FP_ALGORITHM, 0x2a)
}
fn header() -> IndexHeader {
    IndexHeader::from_descriptor(&descriptor(), "rev-1", fingerprint(), "prep").unwrap()
}
fn keys(m: &[Mismatch]) -> Vec<&str> {
    m.iter().map(|m| m.key.as_str()).collect()
}
fn incompatible(r: Result<impl std::fmt::Debug, FormatError>) -> Vec<Mismatch> {
    match r {
        Err(FormatError::SchemaIncompatible { mismatches }) => {
            assert!(! mismatches.is_empty());
            mismatches
        }
        other => panic!("expected SchemaIncompatible, got {other:?}"),
    }
}
fn with_kv(h: &IndexHeader, key: &str, value: &str) -> Vec<(String, String)> {
    h.to_kv()
        .into_iter()
        .map(|(k, v)| if k == key { (k, value.to_string()) } else { (k, v) })
        .collect()
}
fn schema_with(index: usize, replacement: Field) -> Schema {
    let mut fields: Vec<Field> = column_schema(4)
        .unwrap()
        .fields()
        .iter()
        .map(|f| f.as_ref().clone())
        .collect();
    fields[index] = replacement;
    Schema::new(fields)
}
#[test]
fn types_are_send_sync() {
    fn check<T: Send + Sync + 'static>() {}
    check::<IndexHeader>();
    check::<TokenizerFingerprint>();
    check::<FormatError>();
    check::<Mismatch>();
    check::<Pooling>();
    check::<HashAlgorithm>();
}
#[test]
fn constants_match_contract() {
    assert_eq!(SCHEMA_VERSION, 1);
    assert_eq!(HASH_ALGORITHM, HashAlgorithm::Fast64);
    assert_eq!(HASH_VERSION, 1);
    assert_eq!(TOKENIZER_FP_ALGORITHM, "fnv1a64");
}
#[test]
fn column_schema_layout() {
    let s = column_schema(384).unwrap();
    let names: Vec<&str> = s.fields().iter().map(|f| f.name().as_str()).collect();
    assert_eq!(
        names, [COL_ENTRY_ID, COL_CHANGE_STAMP, COL_VECTOR, COL_TARGET_FILE,
        COL_CHAR_START, COL_CHAR_END, COL_CHECKSUM]
    );
    assert!(s.fields().iter().all(| f | ! f.is_nullable()));
    assert_eq!(s.field(0).data_type(), & DataType::UInt64);
    assert_eq!(s.field(1).data_type(), & DataType::UInt64);
    match s.field(2).data_type() {
        DataType::FixedSizeList(item, n) => {
            assert_eq!(* n, 384);
            assert_eq!(item.name(), "item");
            assert_eq!(item.data_type(), & DataType::Float16);
            assert!(! item.is_nullable());
        }
        other => panic!("unexpected vector type {other:?}"),
    }
    assert_eq!(s.field(3).data_type(), & DataType::Utf8);
    for i in 4..7 {
        assert_eq!(s.field(i).data_type(), & DataType::UInt64);
    }
}
#[test]
fn column_schema_rejects_invalid_dimension() {
    for dim in [0usize, i32::MAX as usize + 1, usize::MAX] {
        let m = incompatible(column_schema(dim));
        assert_eq!(keys(& m), [KEY_DIMENSION]);
        assert_eq!(m[0].found, dim.to_string());
        assert!(
            incompatible(check_column_schema(& column_schema(1).unwrap(), dim)).len() ==
            1
        );
    }
    assert!(column_schema(i32::MAX as usize).is_ok());
}
#[test]
fn header_column_schema_uses_dimension() {
    let h = header();
    assert_eq!(h.column_schema().unwrap(), column_schema(h.dimension()).unwrap());
}
#[test]
fn check_column_schema_accepts_expected() {
    check_column_schema(&column_schema(8).unwrap(), 8).unwrap();
}
#[test]
fn check_column_schema_ignores_metadata_and_child_name() {
    let base = column_schema(4).unwrap();
    let item = Field::new("other_name", DataType::Float16, false);
    let vector = Field::new(
            COL_VECTOR,
            DataType::FixedSizeList(Arc::new(item), 4),
            false,
        )
        .with_metadata(HashMap::from([("k".to_string(), "v".to_string())]));
    let found = schema_with(2, vector)
        .with_metadata(HashMap::from([("ARROW".to_string(), "x".to_string())]));
    assert_eq!(found.fields().len(), base.fields().len());
    check_column_schema(&found, 4).unwrap();
}
#[test]
fn check_column_schema_detects_deviations() {
    let item = |t: DataType, nullable: bool| Arc::new(Field::new("item", t, nullable));
    let cases: Vec<Schema> = vec![
        schema_with(0, Field::new(COL_ENTRY_ID, DataType::UInt32, false)), schema_with(0,
        Field::new(COL_ENTRY_ID, DataType::UInt64, true)), schema_with(0,
        Field::new("renamed", DataType::UInt64, false)), schema_with(3,
        Field::new(COL_TARGET_FILE, DataType::LargeUtf8, false)), schema_with(2,
        Field::new(COL_VECTOR, DataType::FixedSizeList(item(DataType::Float16, false),
        5), false,),), schema_with(2, Field::new(COL_VECTOR,
        DataType::FixedSizeList(item(DataType::Float32, false), 4), false,),),
        schema_with(2, Field::new(COL_VECTOR,
        DataType::FixedSizeList(item(DataType::Float16, true), 4), false,),),
        schema_with(2, Field::new(COL_VECTOR,
        DataType::FixedSizeList(item(DataType::Float16, false), 4), true,),),
        Schema::new(Vec::< Field >::new()),
    ];
    for found in &cases {
        match check_column_schema(found, 4) {
            Err(FormatError::ColumnSchemaMismatch { expected, found: text }) => {
                assert!(expected.contains("entry_id: UInt64, nullable=false"));
                assert!(expected.contains("size=4"));
                assert_ne!(expected, text);
            }
            other => panic!("expected ColumnSchemaMismatch, got {other:?}"),
        }
    }
}
#[test]
fn check_column_schema_detects_order_and_count() {
    let base = column_schema(4).unwrap();
    let mut swapped: Vec<Field> = base
        .fields()
        .iter()
        .map(|f| f.as_ref().clone())
        .collect();
    swapped.swap(0, 1);
    assert!(
        matches!(check_column_schema(& Schema::new(swapped.clone()), 4),
        Err(FormatError::ColumnSchemaMismatch { .. }))
    );
    swapped.pop();
    assert!(
        matches!(check_column_schema(& Schema::new(swapped), 4),
        Err(FormatError::ColumnSchemaMismatch { .. }))
    );
}
#[test]
fn fingerprint_format_and_parse() {
    assert_eq!(fingerprint().as_str(), "fnv1a64:000000000000002a");
    assert_eq!(TokenizerFingerprint::new("x", u64::MAX).as_str(), "x:ffffffffffffffff");
    let fp = fingerprint();
    assert_eq!(TokenizerFingerprint::parse(fp.as_str()), Some(fp));
    assert!(TokenizerFingerprint::parse("a:b:0123456789abcdef").is_none());
    for bad in [
        "",
        ":0123456789abcdef",
        "fnv1a64",
        "fnv1a64:",
        "fnv1a64:0123456789abcde",
        "fnv1a64:0123456789abcdef0",
        "fnv1a64:0123456789ABCDEF",
        "fnv1a64:0123456789abcdeg",
        "fnv1a64: 123456789abcdef",
    ] {
        assert!(TokenizerFingerprint::parse(bad).is_none(), "{bad}");
    }
}
#[test]
fn fnv1a64_known_vectors() {
    assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
    assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
    assert_eq!(fnv1a64(b"foobar"), 0x8594_4171_f739_67e8);
}
#[test]
fn fingerprint_from_loaded_is_deterministic_and_discriminating() {
    let a = TokenizerFingerprint::from_loaded(&loaded("", 8)).unwrap();
    let b = TokenizerFingerprint::from_loaded(&loaded("", 8)).unwrap();
    assert_eq!(a, b);
    assert!(a.as_str().starts_with("fnv1a64:"));
    assert_eq!(TokenizerFingerprint::parse(a.as_str()), Some(a.clone()));
    let other = TokenizerFingerprint::from_loaded(&loaded(",\"world\":3", 8)).unwrap();
    assert_ne!(a, other);
    let shared = Arc::new(loaded("", 8));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let shared = Arc::clone(&shared);
            std::thread::spawn(move || {
                TokenizerFingerprint::from_loaded(&shared).unwrap()
            })
        })
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), a);
    }
}
#[test]
fn from_descriptor_derives_fields() {
    let d = descriptor();
    let h = header();
    assert_eq!(h.schema_version, 1);
    assert_eq!(h.max_stamp, 0);
    assert_eq!(h.hash_algorithm, HashAlgorithm::Fast64);
    assert_eq!(h.hash_version, 1);
    assert_eq!(h.model_id, d.model_id);
    assert_eq!(h.model_revision, "rev-1");
    assert_eq!(h.dimension, d.dimension);
    assert_eq!(h.pooling, Pooling::Mean);
    assert_eq!(h.normalization, d.normalization);
    assert_eq!(h.query_prefix, "query: ");
    assert_eq!(h.passage_prefix, "passage: ");
    assert_eq!(h.storage_dtype, DType::F16);
    assert_eq!(h.compute_dtype, "cpu=F32,gpu=F16");
    assert_eq!(h.max_seq_len, d.max_seq_len);
    assert_eq!(h.tokenizer_fingerprint, fingerprint());
    assert_eq!(h.preprocessing_id, "prep");
    assert_eq!(h.vector_bytes(), d.dimension * 2);
}
#[test]
fn from_descriptor_keeps_prefixes_untrimmed_and_empty_preprocessing() {
    let mut d = descriptor();
    d.query_prefix = "  q ".to_string();
    d.passage_prefix = String::new();
    let h = IndexHeader::from_descriptor(&d, "", fingerprint(), "").unwrap();
    assert_eq!(h.query_prefix, "  q ");
    assert_eq!(h.passage_prefix, "");
    assert_eq!(h.model_revision, "");
    assert_eq!(h.preprocessing_id, "");
}
#[test]
fn from_descriptor_rejects_cls_pooling() {
    let mut d = descriptor();
    d.pooling = Pooling::Cls;
    let m = incompatible(IndexHeader::from_descriptor(&d, "r", fingerprint(), ""));
    assert_eq!(keys(& m), [KEY_POOLING]);
}
#[test]
fn from_descriptor_rejects_non_f16_storage() {
    let mut d = descriptor();
    d.storage_dtype = DType::F32;
    let m = incompatible(IndexHeader::from_descriptor(&d, "r", fingerprint(), ""));
    assert_eq!(keys(& m), [KEY_STORAGE_DTYPE]);
    assert_eq!(m[0].expected, "F16");
    assert_eq!(m[0].found, "F32");
}
#[test]
fn from_descriptor_collects_all_violations() {
    let mut d = descriptor();
    d.dimension = 0;
    d.pooling = Pooling::Cls;
    d.storage_dtype = DType::BF16;
    let m = incompatible(IndexHeader::from_descriptor(&d, "r", fingerprint(), ""));
    assert_eq!(keys(& m), [KEY_DIMENSION, KEY_POOLING, KEY_STORAGE_DTYPE]);
}
#[test]
fn derived_values() {
    let h = header();
    assert_eq!(h.dimension(), h.dimension);
    let stamped = h.with_max_stamp(77);
    assert_eq!(stamped.max_stamp, 77);
    assert_eq!(stamped.with_max_stamp(0), h);
    let mut small = h.clone();
    small.dimension = 3;
    assert_eq!(small.vector_bytes(), 6);
    small.dimension = usize::MAX;
    assert_eq!(small.vector_bytes(), usize::MAX);
}
#[test]
fn to_kv_has_sixteen_pairs_in_key_order() {
    let kv = header().to_kv();
    let got: Vec<&str> = kv.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(
        got, [KEY_SCHEMA_VERSION, KEY_MAX_STAMP, KEY_HASH_ALGORITHM, KEY_HASH_VERSION,
        KEY_MODEL_ID, KEY_MODEL_REVISION, KEY_DIMENSION, KEY_POOLING, KEY_NORMALIZATION,
        KEY_QUERY_PREFIX, KEY_PASSAGE_PREFIX, KEY_STORAGE_DTYPE, KEY_COMPUTE_DTYPE,
        KEY_MAX_SEQ_LEN, KEY_TOKENIZER_FINGERPRINT, KEY_PREPROCESSING_ID]
    );
    let map: HashMap<_, _> = kv.into_iter().collect();
    assert_eq!(map[KEY_SCHEMA_VERSION], "1");
    assert_eq!(map[KEY_HASH_ALGORITHM], "fast_64_bit");
    assert_eq!(map[KEY_POOLING], "masked_mean");
    assert_eq!(map[KEY_STORAGE_DTYPE], "F16");
    assert_eq!(map[KEY_TOKENIZER_FINGERPRINT], "fnv1a64:000000000000002a");
}
#[test]
fn kv_roundtrip_is_lossless() {
    let h = header();
    assert_eq!(IndexHeader::from_kv(& h.to_kv()), Ok(h.clone()));
    let mut odd = h.with_max_stamp(u64::MAX);
    odd.model_revision = String::new();
    odd.query_prefix = String::new();
    odd.passage_prefix = " tail \n".to_string();
    odd.preprocessing_id = String::new();
    odd.compute_dtype = "anything, goes=1".to_string();
    odd.hash_algorithm = HashAlgorithm::Cryptographic;
    odd.pooling = Pooling::PerToken;
    odd.normalization = Normalization::None;
    odd.storage_dtype = DType::BF16;
    odd.schema_version = 9;
    odd.dimension = 1;
    assert_eq!(IndexHeader::from_kv(& odd.to_kv()), Ok(odd));
    for algorithm in [
        HashAlgorithm::Fast64,
        HashAlgorithm::Fast128,
        HashAlgorithm::Cryptographic,
    ] {
        let mut x = h.clone();
        x.hash_algorithm = algorithm;
        assert_eq!(IndexHeader::from_kv(& x.to_kv()), Ok(x));
    }
    for dtype in [
        DType::F32,
        DType::F16,
        DType::BF16,
        DType::U8,
        DType::U32,
        DType::I64,
    ] {
        let mut x = h.clone();
        x.storage_dtype = dtype;
        assert_eq!(IndexHeader::from_kv(& x.to_kv()), Ok(x));
    }
}
#[test]
fn from_kv_ignores_unknown_keys_and_last_duplicate_wins() {
    let h = header();
    let mut kv = h.to_kv();
    kv.push(("ARROW:schema".to_string(), "blob".to_string()));
    kv.insert(0, (KEY_MODEL_ID.to_string(), "stale".to_string()));
    assert_eq!(IndexHeader::from_kv(& kv), Ok(h.clone()));
    kv.push((KEY_MAX_STAMP.to_string(), "5".to_string()));
    assert_eq!(IndexHeader::from_kv(& kv).unwrap().max_stamp, 5);
}
#[test]
fn from_kv_reports_every_missing_key() {
    let m = incompatible(IndexHeader::from_kv(&[]));
    assert_eq!(m.len(), 16);
    assert!(m.iter().all(| m | m.found == "<fehlt>"));
    let expected: HashMap<&str, &str> = m
        .iter()
        .map(|m| (m.key.as_str(), m.expected.as_str()))
        .collect();
    assert_eq!(expected[KEY_SCHEMA_VERSION], "u64");
    assert_eq!(expected[KEY_POOLING], "masked_mean|per_token");
    let order: Vec<String> = header().to_kv().into_iter().map(|(k, _)| k).collect();
    assert_eq!(keys(& m), order);
}
#[test]
fn from_kv_reports_invalid_numbers() {
    let h = header();
    for bad in ["abc", "", "-1", "+5", " 5", "5 ", "1.5", "18446744073709551616"] {
        for key in [
            KEY_SCHEMA_VERSION,
            KEY_MAX_STAMP,
            KEY_HASH_VERSION,
            KEY_DIMENSION,
            KEY_MAX_SEQ_LEN,
        ] {
            let m = incompatible(IndexHeader::from_kv(&with_kv(&h, key, bad)));
            assert_eq!(keys(& m), [key]);
            assert_eq!(m[0].found, bad);
        }
    }
}
#[test]
fn from_kv_reports_zero_dimension() {
    let m = incompatible(IndexHeader::from_kv(&with_kv(&header(), KEY_DIMENSION, "0")));
    assert_eq!(keys(& m), [KEY_DIMENSION]);
    assert_eq!(m[0].found, "0");
}
#[test]
fn from_kv_reports_unknown_enum_values_case_sensitive() {
    let h = header();
    let cases = [
        (KEY_POOLING, "cls"),
        (KEY_POOLING, "MASKED_MEAN"),
        (KEY_NORMALIZATION, "L2"),
        (KEY_HASH_ALGORITHM, "fast64"),
        (KEY_STORAGE_DTYPE, "f16"),
        (KEY_STORAGE_DTYPE, "F64"),
    ];
    for (key, value) in cases {
        let m = incompatible(IndexHeader::from_kv(&with_kv(&h, key, value)));
        assert_eq!(keys(& m), [key]);
        assert_eq!(m[0].found, value);
    }
}
#[test]
fn from_kv_reports_invalid_fingerprint() {
    let m = incompatible(
        IndexHeader::from_kv(
            &with_kv(&header(), KEY_TOKENIZER_FINGERPRINT, "fnv1a64:XYZ"),
        ),
    );
    assert_eq!(keys(& m), [KEY_TOKENIZER_FINGERPRINT]);
    assert_eq!(m[0].found, "fnv1a64:XYZ");
}
#[test]
fn from_kv_collects_all_problems_in_key_order() {
    let h = header();
    let kv: Vec<(String, String)> = h
        .to_kv()
        .into_iter()
        .filter(|(k, _)| k != KEY_MODEL_ID)
        .map(|(k, v)| match k.as_str() {
            KEY_MAX_STAMP => (k, "x".to_string()),
            KEY_POOLING => (k, "bad".to_string()),
            _ => (k, v),
        })
        .collect();
    let m = incompatible(IndexHeader::from_kv(&kv));
    assert_eq!(keys(& m), [KEY_MAX_STAMP, KEY_MODEL_ID, KEY_POOLING]);
}
#[test]
fn from_kv_accepts_any_compute_dtype_text() {
    let h = header();
    let parsed = IndexHeader::from_kv(&with_kv(&h, KEY_COMPUTE_DTYPE, "")).unwrap();
    assert_eq!(parsed.compute_dtype, "");
}
#[test]
fn binding_accepts_identical_and_ignores_stamp_and_compute() {
    let h = header();
    h.check_binding(&h).unwrap();
    let mut other = h.with_max_stamp(999);
    other.compute_dtype = "cpu=F16,gpu=F32".to_string();
    other.check_binding(&h).unwrap();
    h.check_binding(&other).unwrap();
}
#[test]
fn binding_reports_each_deviating_key_in_order() {
    let expected = header();
    let mut file = expected.clone();
    file.schema_version = 2;
    file.hash_algorithm = HashAlgorithm::Fast128;
    file.hash_version = 3;
    file.model_id = "other".to_string();
    file.model_revision = "rev-2".to_string();
    file.dimension = 768;
    file.pooling = Pooling::PerToken;
    file.normalization = Normalization::None;
    file.query_prefix = "q".to_string();
    file.passage_prefix = "p".to_string();
    file.storage_dtype = DType::F32;
    file.max_seq_len += 1;
    file.tokenizer_fingerprint = TokenizerFingerprint::new(TOKENIZER_FP_ALGORITHM, 1);
    file.preprocessing_id = "changed".to_string();
    let m = incompatible(file.check_binding(&expected));
    assert_eq!(
        keys(& m), [KEY_SCHEMA_VERSION, KEY_HASH_ALGORITHM, KEY_HASH_VERSION,
        KEY_MODEL_ID, KEY_MODEL_REVISION, KEY_DIMENSION, KEY_POOLING, KEY_NORMALIZATION,
        KEY_QUERY_PREFIX, KEY_PASSAGE_PREFIX, KEY_STORAGE_DTYPE, KEY_MAX_SEQ_LEN,
        KEY_TOKENIZER_FINGERPRINT, KEY_PREPROCESSING_ID]
    );
    assert_eq!(m[0].expected, "1");
    assert_eq!(m[0].found, "2");
    assert_eq!(m[3].expected, expected.model_id);
    assert_eq!(m[3].found, "other");
}
#[test]
fn binding_detects_single_deviation() {
    let expected = header();
    let mut file = expected.clone();
    file.preprocessing_id = String::new();
    let m = incompatible(file.check_binding(&expected));
    assert_eq!(keys(& m), [KEY_PREPROCESSING_ID]);
    assert_eq!(m[0].expected, "prep");
    assert_eq!(m[0].found, "");
}
#[test]
fn display_names_all_details() {
    let expected = header();
    let mut file = expected.clone();
    file.model_id = "other".to_string();
    file.dimension = 7;
    let text = file.check_binding(&expected).unwrap_err().to_string();
    assert!(text.contains(KEY_MODEL_ID) && text.contains("other"));
    assert!(text.contains(KEY_DIMENSION) && text.contains('7'));
    assert!(text.contains(& expected.model_id));
    let schema_err = check_column_schema(
            &schema_with(0, Field::new("x", DataType::UInt64, false)),
            4,
        )
        .unwrap_err()
        .to_string();
    assert!(schema_err.contains("entry_id") && schema_err.contains("x: UInt64"));
    let tok = FormatError::TokenizerDefinition("boom".to_string()).to_string();
    assert!(tok.contains("boom"));
    let _: &dyn std::error::Error = &FormatError::TokenizerDefinition(String::new());
}
