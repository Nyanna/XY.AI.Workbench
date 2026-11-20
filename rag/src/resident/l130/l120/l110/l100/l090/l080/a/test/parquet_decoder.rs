use super::*;
use std::path::PathBuf;
use arrow::array::{ArrayRef, Int64Array};
use arrow::datatypes::{DataType, Field, Schema};
use half::f16;
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::metadata::{KeyValue, ParquetMetaDataReader};
use parquet::file::properties::WriterProperties;
use super::super::index_format::{
    column_schema, HashAlgorithm, TokenizerFingerprint, KEY_COMPUTE_DTYPE, KEY_DIMENSION,
    KEY_HASH_ALGORITHM, KEY_HASH_VERSION, KEY_MAX_SEQ_LEN, KEY_MAX_STAMP, KEY_MODEL_ID,
    KEY_MODEL_REVISION, KEY_NORMALIZATION, KEY_PASSAGE_PREFIX, KEY_POOLING,
    KEY_PREPROCESSING_ID, KEY_QUERY_PREFIX, KEY_SCHEMA_VERSION, KEY_STORAGE_DTYPE,
    KEY_TOKENIZER_FINGERPRINT, TOKENIZER_FP_ALGORITHM,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::index_open::{
    LazyIndexFile, OpenOptions,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::{
    EntryId, Payload,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::ModelDescriptor;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
const DIM: usize = 4;
static_assertions::assert_impl_all!(ParquetDecoder : Send, Sync);
fn checksum_of(vector: &[u8], file: &str, start: u64, end: u64) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    let mut feed = |bytes: &[u8]| {
        for b in bytes {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x100000001b3);
        }
    };
    feed(vector);
    feed(&(file.len() as u64).to_le_bytes());
    feed(file.as_bytes());
    feed(&start.to_le_bytes());
    feed(&end.to_le_bytes());
    h
}
fn header(max_stamp: u64) -> IndexHeader {
    let mut h = IndexHeader::from_descriptor(
            &ModelDescriptor::e5_small_repo_default(DType::F16),
            "rev-1",
            TokenizerFingerprint::new(TOKENIZER_FP_ALGORITHM, 0x2a),
            "prep",
        )
        .unwrap();
    h.dimension = DIM;
    h.max_stamp = max_stamp;
    h
}
fn decoder() -> ParquetDecoder {
    ParquetDecoder::new(header(0), Arc::new(checksum_of))
}
#[derive(Clone, Debug)]
struct Row {
    id: u64,
    stamp: u64,
    vector: Vec<u16>,
    file: String,
    start: u64,
    end: u64,
    checksum: Option<u64>,
}
impl Row {
    fn with_dim(id: u64, stamp: u64, dim: usize) -> Row {
        Row {
            id,
            stamp,
            vector: (0..dim as u64)
                .map(|i| f16::from_f32((id * 7 + i) as f32 * 0.25).to_bits())
                .collect(),
            file: format!("dir/file{id}.txt"),
            start: id * 10,
            end: id * 10 + 5,
            checksum: None,
        }
    }
    fn new(id: u64, stamp: u64) -> Row {
        Row::with_dim(id, stamp, DIM)
    }
    fn vector_bytes(&self) -> Vec<u8> {
        self.vector.iter().flat_map(|b| b.to_le_bytes()).collect()
    }
    fn stored_checksum(&self) -> u64 {
        self.checksum
            .unwrap_or_else(|| checksum_of(
                &self.vector_bytes(),
                &self.file,
                self.start,
                self.end,
            ))
    }
}
fn rows(n: u64) -> Vec<Row> {
    (1..=n).map(|i| Row::new(i * 10, i)).collect()
}
fn columns(rows: &[Row], dim: usize) -> Vec<ArrayRef> {
    let flat: Vec<f16> = rows
        .iter()
        .flat_map(|r| r.vector.iter().map(|b| f16::from_bits(*b)))
        .collect();
    let item = Field::new("item", DataType::Float16, false);
    let list = FixedSizeListArray::try_new(
            Arc::new(item),
            dim as i32,
            Arc::new(Float16Array::from(flat)),
            None,
        )
        .unwrap();
    vec![
        Arc::new(UInt64Array::from_iter_values(rows.iter().map(| r | r.id))),
        Arc::new(UInt64Array::from_iter_values(rows.iter().map(| r | r.stamp))),
        Arc::new(list), Arc::new(StringArray::from_iter_values(rows.iter().map(| r | r
        .file.as_str()))), Arc::new(UInt64Array::from_iter_values(rows.iter().map(| r | r
        .start))), Arc::new(UInt64Array::from_iter_values(rows.iter().map(| r | r.end))),
        Arc::new(UInt64Array::from_iter_values(rows.iter().map(| r | r
        .stored_checksum()))),
    ]
}
fn write_parts(
    schema: Schema,
    columns: Vec<ArrayRef>,
    kv: &[(String, String)],
    group_rows: usize,
    compression: Compression,
) -> Vec<u8> {
    let schema = Arc::new(schema);
    let props = WriterProperties::builder()
        .set_key_value_metadata(
            Some(kv.iter().map(|(k, v)| KeyValue::new(k.clone(), v.clone())).collect()),
        )
        .set_max_row_group_row_count(Some(group_rows))
        .set_compression(compression)
        .build();
    let mut out = Vec::new();
    let mut writer = ArrowWriter::try_new(&mut out, schema.clone(), Some(props))
        .unwrap();
    if columns[0].len() > 0 {
        writer.write(&RecordBatch::try_new(schema, columns).unwrap()).unwrap();
    }
    writer.close().unwrap();
    out
}
fn write_kv(
    kv: &[(String, String)],
    dim: usize,
    rows: &[Row],
    group_rows: usize,
) -> Vec<u8> {
    write_parts(
        column_schema(dim).unwrap(),
        columns(rows, dim),
        kv,
        group_rows,
        Compression::UNCOMPRESSED,
    )
}
fn write_file(max_stamp: u64, rows: &[Row], group_rows: usize) -> Vec<u8> {
    write_kv(&header(max_stamp).to_kv(), DIM, rows, group_rows)
}
fn kv_with(key: &str, value: &str) -> Vec<(String, String)> {
    header(100)
        .to_kv()
        .into_iter()
        .map(|(k, v)| if k == key { (k, value.to_string()) } else { (k, v) })
        .collect()
}
fn info_for(rows: u64) -> FileInfo {
    FileInfo {
        schema_version: 1,
        model_id: String::new(),
        model_revision: String::new(),
        row_groups: 0,
        rows,
        stored_max_stamp: None,
    }
}
fn mismatch_text(parts: &[(&str, String, String)]) -> String {
    let mut text = "Indexdatei nicht kompatibel:".to_string();
    for (key, expected, found) in parts {
        text.push_str(&format!(" [{key}: erwartet '{expected}', gefunden '{found}']"));
    }
    text
}
fn kv_value(h: &IndexHeader, key: &str) -> String {
    h.to_kv().into_iter().find(|(k, _)| k == key).unwrap().1
}
fn chunk(bytes: &[u8], group: usize, root: usize) -> Range<usize> {
    let meta = ParquetMetaDataReader::new()
        .parse_and_finish(&Bytes::copy_from_slice(bytes))
        .unwrap();
    let (start, length) = meta.row_group(group).column(root).byte_range();
    start as usize..(start + length) as usize
}
const ROOT_ENTRY_ID: usize = 0;
const ROOT_CHANGE_STAMP: usize = 1;
const ROOT_VECTOR: usize = 2;
const ROOT_TARGET_FILE: usize = 3;
const ROOT_CHECKSUM: usize = 6;
fn corrupt_chunk(bytes: &mut [u8], group: usize, root: usize) {
    let range = chunk(bytes, group, root);
    bytes[range].fill(0xAB);
}
fn temp_file(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("parquet_decoder_{}_{}", std::process::id(), name))
}
fn format_error(e: DecodeError) -> FormatError {
    match e {
        DecodeError::Format(f) => f,
        other => panic!("expected format error, got {other:?}"),
    }
}
fn assert_data<T: std::fmt::Debug>(r: Result<T, DecodeError>) {
    match r {
        Err(DecodeError::Data(_)) => {}
        other => panic!("expected data error, got {other:?}"),
    }
}
#[test]
fn header_round_trip_with_all_keys() {
    let h = header(42);
    assert_eq!(h.to_kv().len(), 16);
    let bytes = write_file(42, &rows(3), 2);
    assert_eq!(decoder().read_header(& bytes), Ok(h));
}
#[test]
fn missing_key_reports_not_present() {
    let kv: Vec<_> = header(5)
        .to_kv()
        .into_iter()
        .filter(|(k, _)| k != KEY_MODEL_ID)
        .collect();
    let bytes = write_kv(&kv, DIM, &rows(2), 8);
    match format_error(decoder().read_header(&bytes).unwrap_err()) {
        FormatError::SchemaIncompatible { mismatches } => {
            assert_eq!(mismatches.len(), 1);
            assert_eq!(mismatches[0].key, KEY_MODEL_ID);
            assert_eq!(mismatches[0].found, "<fehlt>");
        }
        other => panic!("{other:?}"),
    }
}
#[test]
fn extra_key_is_ignored() {
    let mut kv = header(5).to_kv();
    kv.push(("unrelated".into(), "value".into()));
    let bytes = write_kv(&kv, DIM, &rows(2), 8);
    assert_eq!(decoder().read_header(& bytes), Ok(header(5)));
}
#[test]
fn wrong_dimension_in_schema_fails() {
    let kv = kv_with(KEY_DIMENSION, "8");
    let bytes = write_kv(&kv, DIM, &rows(2), 8);
    assert!(
        matches!(format_error(decoder().read_header(& bytes).unwrap_err()),
        FormatError::ColumnSchemaMismatch { .. })
    );
}
#[test]
fn deviating_columns_fail() {
    let kv = header(5).to_kv();
    let data = rows(2);
    let good = column_schema(DIM).unwrap();
    let variants: Vec<(Schema, Vec<ArrayRef>)> = vec![
        { let mut fields : Vec < Field > = good.fields().iter().map(| f | f.as_ref()
        .clone()).collect(); fields[1] = fields[1].clone().with_name("stamp");
        (Schema::new(fields), columns(& data, DIM)) }, { let mut fields : Vec < Field > =
        good.fields().iter().map(| f | f.as_ref().clone()).collect(); fields[4] =
        Field::new(COL_CHAR_START, DataType::Int64, false); let mut cols = columns(&
        data, DIM); cols[4] = Arc::new(Int64Array::from(vec![1i64, 2]));
        (Schema::new(fields), cols) }, { let mut fields : Vec < Field > = good.fields()
        .iter().map(| f | f.as_ref().clone()).collect(); fields[0] = fields[0].clone()
        .with_nullable(true); (Schema::new(fields), columns(& data, DIM)) },
    ];
    for (schema, cols) in variants {
        let bytes = write_parts(schema, cols, &kv, 8, Compression::UNCOMPRESSED);
        assert!(
            matches!(format_error(decoder().read_header(& bytes).unwrap_err()),
            FormatError::ColumnSchemaMismatch { .. })
        );
    }
}
#[test]
fn legacy_file_lists_every_key_as_missing() {
    let schema = Schema::new(
        vec![
            Field::new("id", DataType::UInt64, false), Field::new("stamp",
            DataType::UInt64, false), Field::new("vector", DataType::Binary, false),
            Field::new("meta", DataType::Binary, false),
        ],
    );
    let cols: Vec<ArrayRef> = vec![
        Arc::new(UInt64Array::from(vec![1u64])), Arc::new(UInt64Array::from(vec![1u64])),
        Arc::new(arrow::array::BinaryArray::from_vec(vec![& b"\0\0"[..]])),
        Arc::new(arrow::array::BinaryArray::from_vec(vec![& b"m"[..]])),
    ];
    let kv: Vec<(String, String)> = [
        ("index.model_id", "m"),
        ("index.dimension", "4"),
        ("index.dtype", "F16"),
        ("index.max_stamp", "1"),
    ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    let bytes = write_parts(schema, cols, &kv, 8, Compression::UNCOMPRESSED);
    let d = decoder();
    match format_error(d.read_header(&bytes).unwrap_err()) {
        FormatError::SchemaIncompatible { mismatches } => {
            assert_eq!(mismatches.len(), 16);
            assert!(mismatches.iter().all(| m | m.found == "<fehlt>"));
        }
        other => panic!("{other:?}"),
    }
    let text = d.read_info(&bytes).unwrap_err();
    assert!(text.starts_with("Indexdatei nicht kompatibel:"));
    assert_eq!(text.matches("<fehlt>").count(), 16);
}
#[test]
fn each_bound_key_yields_exactly_its_mismatch() {
    let d = decoder();
    let other_fp = TokenizerFingerprint::new(TOKENIZER_FP_ALGORITHM, 1);
    let cases: Vec<(&str, String)> = vec![
        (KEY_SCHEMA_VERSION, "2".into()), (KEY_HASH_ALGORITHM, "fast_128_bit".into()),
        (KEY_HASH_VERSION, "3".into()), (KEY_MODEL_ID, "other".into()),
        (KEY_MODEL_REVISION, "rev-2".into()), (KEY_POOLING, "per_token".into()),
        (KEY_NORMALIZATION, "none".into()), (KEY_QUERY_PREFIX, "q".into()),
        (KEY_PASSAGE_PREFIX, "p".into()), (KEY_STORAGE_DTYPE, "F32".into()),
        (KEY_MAX_SEQ_LEN, "999".into()), (KEY_TOKENIZER_FINGERPRINT, other_fp.as_str()
        .to_string()), (KEY_PREPROCESSING_ID, "changed".into()),
    ];
    for (key, value) in cases {
        let bytes = write_kv(&kv_with(key, &value), DIM, &rows(2), 8);
        let text = d.read_info(&bytes).unwrap_err();
        let wanted = mismatch_text(&[(key, kv_value(d.expected(), key), value)]);
        assert_eq!(text, wanted, "{key}");
    }
}
#[test]
fn deviating_dimension_yields_exactly_its_mismatch() {
    let d = decoder();
    let kv = kv_with(KEY_DIMENSION, "8");
    let bytes = write_kv(&kv, 8, &rows_with_dim(2, 8), 8);
    let text = d.read_info(&bytes).unwrap_err();
    assert_eq!(text, mismatch_text(& [(KEY_DIMENSION, "4".into(), "8".into())]));
}
fn rows_with_dim(n: u64, dim: usize) -> Vec<Row> {
    (1..=n).map(|i| Row::with_dim(i * 10, i, dim)).collect()
}
#[test]
fn stamp_and_compute_dtype_differences_pass() {
    let mut kv = kv_with(KEY_MAX_STAMP, "77");
    for pair in kv.iter_mut() {
        if pair.0 == KEY_COMPUTE_DTYPE {
            pair.1 = "cpu=F16,gpu=F32".into();
        }
    }
    let bytes = write_kv(&kv, DIM, &rows(2), 8);
    let info = decoder().read_info(&bytes).unwrap();
    assert_eq!(info.stored_max_stamp, Some(77));
}
#[test]
fn several_differences_are_reported_together() {
    let d = decoder();
    let mut kv = kv_with(KEY_MODEL_ID, "other");
    for pair in kv.iter_mut() {
        if pair.0 == KEY_PASSAGE_PREFIX {
            pair.1 = "p".into();
        }
    }
    let bytes = write_kv(&kv, DIM, &rows(2), 8);
    let text = d.read_info(&bytes).unwrap_err();
    let wanted = mismatch_text(
        &[
            (KEY_MODEL_ID, kv_value(d.expected(), KEY_MODEL_ID), "other".into()),
            (KEY_PASSAGE_PREFIX, kv_value(d.expected(), KEY_PASSAGE_PREFIX), "p".into()),
        ],
    );
    assert_eq!(text, wanted);
}
#[test]
fn format_error_display_is_unchanged() {
    let e = FormatError::ColumnSchemaMismatch {
        expected: "a".into(),
        found: "b".into(),
    };
    assert_eq!(DecodeError::Format(e.clone()).to_string(), e.to_string());
}
#[test]
fn read_info_reports_counts() {
    let bytes = write_file(9, &rows(5), 2);
    let info = decoder().read_info(&bytes).unwrap();
    assert_eq!(info.rows, 5);
    assert_eq!(info.row_groups, 3);
    assert_eq!(info.stored_max_stamp, Some(9));
    assert_eq!(info.schema_version, 1);
    assert_eq!(info.model_id, header(0).model_id);
    assert_eq!(info.model_revision, "rev-1");
}
#[test]
fn read_info_for_zero_rows() {
    let bytes = write_file(0, &[], 8);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    assert_eq!((info.rows, info.row_groups), (0, 0));
    assert_eq!(info.stored_max_stamp, Some(0));
    assert_eq!(d.read_ids_stamps(& bytes, & info), Ok(Vec::new()));
    assert!(d.verify(& bytes, & info).is_ok());
    assert!(d.decode_rows(& bytes, & info, 0, 0).unwrap().vectors.is_empty());
    assert_eq!(d.read_stored_checksums(& bytes), Ok(Vec::new()));
}
#[test]
fn read_info_converts_schema_version() {
    let mut expected = header(0);
    expected.schema_version = 7;
    let d = ParquetDecoder::new(expected, Arc::new(checksum_of));
    let bytes = write_kv(&kv_with(KEY_SCHEMA_VERSION, "7"), DIM, &rows(1), 8);
    assert_eq!(d.read_info(& bytes).unwrap().schema_version, 7);
    let big = u64::from(u32::MAX) + 1;
    let mut expected = header(0);
    expected.schema_version = big;
    let d = ParquetDecoder::new(expected, Arc::new(checksum_of));
    let bytes = write_kv(
        &kv_with(KEY_SCHEMA_VERSION, &big.to_string()),
        DIM,
        &rows(1),
        8,
    );
    let text = d.read_info(&bytes).unwrap_err();
    assert!(text.starts_with("invalid data:"), "{text}");
}
#[test]
fn compressed_files_are_accepted() {
    let data = rows(6);
    let kv = header(6).to_kv();
    for compression in [Compression::SNAPPY, Compression::UNCOMPRESSED] {
        let bytes = write_parts(
            column_schema(DIM).unwrap(),
            columns(&data, DIM),
            &kv,
            4,
            compression,
        );
        let d = decoder();
        let info = d.read_info(&bytes).unwrap();
        d.verify(&bytes, &info).unwrap();
        let batch = d.decode_rows(&bytes, &info, 1, 6).unwrap();
        assert_eq!(&* batch.vectors[0], data[1].vector_bytes().as_slice());
    }
}
#[test]
fn ids_and_stamps_are_returned_in_file_order() {
    let data = vec![Row::new(50, 3), Row::new(7, 1), Row::new(90, 2), Row::new(20, 4)];
    let bytes = write_file(4, &data, 3);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    assert_eq!(
        d.read_ids_stamps(& bytes, & info).unwrap(), vec![(50, 3), (7, 1), (90, 2), (20,
        4)]
    );
}
#[test]
fn ids_and_stamps_never_read_the_vector_chunks() {
    let data = rows(7);
    let mut bytes = write_file(7, &data, 3);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    let meta = load_metadata(&bytes).unwrap();
    let roots = root_indices(&meta, &[COL_ENTRY_ID, COL_CHANGE_STAMP]).unwrap();
    let groups: Vec<usize> = (0..info.row_groups).collect();
    let planned = chunk_ranges(meta.metadata(), &groups, &roots, bytes.len() as u64)
        .unwrap();
    assert_eq!(planned.len(), 2 * info.row_groups);
    for group in 0..info.row_groups {
        let vector = chunk(&bytes, group, ROOT_VECTOR);
        for r in &planned {
            assert!(r.end as usize <= vector.start || r.start as usize >= vector.end);
        }
    }
    for group in 0..info.row_groups {
        corrupt_chunk(&mut bytes, group, ROOT_VECTOR);
        corrupt_chunk(&mut bytes, group, ROOT_TARGET_FILE);
    }
    let pairs = d.read_ids_stamps(&bytes, &info).unwrap();
    assert_eq!(pairs.len(), 7);
    assert_eq!(pairs[3], (40, 4));
    assert_eq!(d.read_stored_checksums(& bytes).unwrap().len(), 7);
}
#[test]
fn decode_rows_never_reads_other_chunks() {
    let data = rows(7);
    let mut bytes = write_file(7, &data, 3);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    for group in 0..info.row_groups {
        corrupt_chunk(&mut bytes, group, ROOT_ENTRY_ID);
        corrupt_chunk(&mut bytes, group, ROOT_CHANGE_STAMP);
        corrupt_chunk(&mut bytes, group, ROOT_CHECKSUM);
    }
    let batch = d.decode_rows(&bytes, &info, 0, 7).unwrap();
    assert_eq!(batch.vectors.len(), 7);
}
#[test]
fn decode_rows_copies_only_overlapping_row_groups() {
    let data = rows(9);
    let mut bytes = write_file(9, &data, 3);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    corrupt_chunk(&mut bytes, 0, ROOT_VECTOR);
    corrupt_chunk(&mut bytes, 2, ROOT_VECTOR);
    let batch = d.decode_rows(&bytes, &info, 3, 6).unwrap();
    assert_eq!(&* batch.vectors[2], data[5].vector_bytes().as_slice());
    assert!(d.decode_rows(& bytes, & info, 0, 6).is_err());
}
#[test]
fn ids_row_count_must_match_info() {
    let bytes = write_file(3, &rows(3), 8);
    assert_data(decoder().read_ids_stamps_inner(&bytes, &info_for(4)));
}
#[test]
fn ids_with_nulls_or_missing_columns_are_data_errors() {
    let mut fields: Vec<Field> = column_schema(DIM)
        .unwrap()
        .fields()
        .iter()
        .map(|f| f.as_ref().clone())
        .collect();
    fields[0] = fields[0].clone().with_nullable(true);
    let mut cols = columns(&rows(2), DIM);
    cols[0] = Arc::new(UInt64Array::from(vec![Some(1u64), None]));
    let bytes = write_parts(
        Schema::new(fields),
        cols,
        &header(2).to_kv(),
        8,
        Compression::UNCOMPRESSED,
    );
    assert_data(decoder().read_ids_stamps_inner(&bytes, &info_for(2)));
    let schema = Schema::new(vec![Field::new("other", DataType::UInt64, false)]);
    let cols: Vec<ArrayRef> = vec![Arc::new(UInt64Array::from(vec![1u64]))];
    let bytes = write_parts(
        schema,
        cols,
        &header(1).to_kv(),
        8,
        Compression::UNCOMPRESSED,
    );
    assert_data(decoder().read_ids_stamps_inner(&bytes, &info_for(1)));
}
fn check_range(
    d: &ParquetDecoder,
    bytes: &[u8],
    info: &FileInfo,
    data: &[Row],
    lo: u64,
    hi: u64,
) {
    let batch = d.decode_rows(bytes, info, lo, hi).unwrap();
    assert_eq!(batch.vectors.len() as u64, hi - lo, "{lo}..{hi}");
    assert_eq!(batch.metas.len() as u64, hi - lo);
    for (i, row) in data[lo as usize..hi as usize].iter().enumerate() {
        assert_eq!(&* batch.vectors[i], row.vector_bytes().as_slice());
        assert_eq!(
            &* batch.metas[i], encode_meta_parts(& row.file, row.start, row.end)
            .as_slice()
        );
    }
}
#[test]
fn decode_rows_ranges() {
    let data = rows(10);
    let bytes = write_file(10, &data, 4);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    assert_eq!(info.row_groups, 3);
    for (lo, hi) in [
        (1, 3),
        (2, 6),
        (0, 10),
        (0, 1),
        (9, 10),
        (4, 8),
        (7, 9),
        (3, 4),
        (4, 5),
        (8, 10),
    ] {
        check_range(&d, &bytes, &info, &data, lo, hi);
    }
}
#[test]
fn decode_rows_empty_and_invalid_ranges() {
    let data = rows(4);
    let bytes = write_file(4, &data, 2);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    assert!(d.decode_rows(& bytes, & info, 2, 2).unwrap().vectors.is_empty());
    assert!(d.decode_rows(& bytes, & info, 4, 4).unwrap().metas.is_empty());
    for (lo, hi) in [(0, 5), (3, 2), (5, 5)] {
        let text = d.decode_rows(&bytes, &info, lo, hi).err().unwrap();
        assert!(text.starts_with("invalid data:"), "{text}");
    }
    let mut lying = info.clone();
    lying.rows = 9;
    assert_data(d.decode_rows_inner(&bytes, &lying, 0, 9).map(|_| ()));
}
#[test]
fn decode_rows_is_bit_exact() {
    let patterns: Vec<u16> = vec![
        0x7e01, 0x7c01, 0xfe00, 0x8000, 0x0000, 0x0001, 0x03ff, 0x7c00, 0xfc00, 0x3c00,
        0xabcd, 0x0400,
    ];
    let data: Vec<Row> = patterns
        .chunks(DIM)
        .enumerate()
        .map(|(i, chunk)| {
            let mut row = Row::new(i as u64 + 1, i as u64 + 1);
            row.vector = chunk.to_vec();
            row
        })
        .collect();
    let bytes = write_file(3, &data, 2);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    check_range(&d, &bytes, &info, &data, 0, 3);
    d.verify(&bytes, &info).unwrap();
}
#[test]
fn decode_rows_metas_cover_default_and_unicode_sources() {
    let mut data = rows(3);
    data[0].file = String::new();
    data[0].start = 0;
    data[0].end = 0;
    data[1].file = "日本語/🦀\0x".into();
    data[1].start = 9;
    data[1].end = 2;
    let bytes = write_file(3, &data, 8);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    check_range(&d, &bytes, &info, &data, 0, 3);
    let batch = d.decode_rows(&bytes, &info, 0, 3).unwrap();
    assert!(batch.metas[0].is_empty());
}
#[test]
fn decode_rows_rejects_other_dimension() {
    let bytes = write_file(2, &rows(2), 8);
    let mut expected = header(0);
    expected.dimension = 8;
    let d = ParquetDecoder::new(expected, Arc::new(checksum_of));
    assert_data(d.decode_rows_inner(&bytes, &info_for(2), 0, 2).map(|_| ()));
}
#[test]
fn decode_rows_spans_many_batches() {
    let n = (READ_BATCH_ROWS * 2 + 100) as u64;
    let data: Vec<Row> = (1..=n).map(|i| Row::new(i, i)).collect();
    let bytes = write_file(n, &data, 3000);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    check_range(&d, &bytes, &info, &data, 2990, 6010);
    check_range(&d, &bytes, &info, &data, 4090, 4100);
}
fn verify_error(bytes: &[u8], rows: u64) -> DecodeError {
    decoder().verify_inner(bytes, &info_for(rows)).unwrap_err()
}
#[test]
fn verify_accepts_valid_file() {
    let bytes = write_file(10, &rows(10), 4);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    assert_eq!(d.verify(& bytes, & info), Ok(()));
}
#[test]
fn verify_detects_checksum_deviations() {
    let clean = rows(6);
    let good = clean[3].stored_checksum();
    let mut flipped = clean.clone();
    flipped[3].checksum = Some(good);
    flipped[3].vector[1] ^= 0x0100;
    let bytes = write_file(6, &flipped, 4);
    match verify_error(&bytes, 6) {
        DecodeError::Checksum { row, entry_id, stored, computed } => {
            assert_eq!((row, entry_id, stored), (3, 40, good));
            assert_ne!(computed, good);
        }
        other => panic!("{other:?}"),
    }
    let mut moved = clean.clone();
    moved[5].checksum = Some(clean[5].stored_checksum());
    moved[5].end += 1;
    match verify_error(&write_file(6, &moved, 4), 6) {
        DecodeError::Checksum { row, entry_id, .. } => {
            assert_eq!((row, entry_id), (5, 60))
        }
        other => panic!("{other:?}"),
    }
    let mut stored = clean.clone();
    stored[0].checksum = Some(clean[0].stored_checksum() ^ 1);
    match verify_error(&write_file(6, &stored, 4), 6) {
        DecodeError::Checksum { row, entry_id, stored: s, computed } => {
            assert_eq!((row, entry_id), (0, 10));
            assert_eq!(s, computed ^ 1);
        }
        other => panic!("{other:?}"),
    }
}
#[test]
fn checksum_error_text_is_formatted() {
    let e = DecodeError::Checksum {
        row: 3,
        entry_id: 40,
        stored: 0xab,
        computed: 0xcd,
    };
    assert_eq!(
        e.to_string(),
        "checksum mismatch at row 3 (entry 40): stored 00000000000000ab, computed 00000000000000cd"
    );
}
#[test]
fn verify_rejects_inconsistent_data() {
    let mut unsorted = rows(4);
    unsorted.swap(1, 2);
    assert_data(Err::<(), _>(verify_error(&write_file(4, &unsorted, 8), 4)));
    let mut duplicate = rows(4);
    duplicate[2].id = duplicate[1].id;
    assert_data(Err::<(), _>(verify_error(&write_file(4, &duplicate, 3), 4)));
    let mut zero_stamp = rows(4);
    zero_stamp[1].stamp = 0;
    assert_data(Err::<(), _>(verify_error(&write_file(4, &zero_stamp, 8), 4)));
    let mut high_stamp = rows(4);
    high_stamp[3].stamp = 5;
    assert_data(Err::<(), _>(verify_error(&write_file(4, &high_stamp, 8), 4)));
    let bytes = write_file(4, &rows(4), 8);
    assert_data(Err::<(), _>(verify_error(&bytes, 5)));
    assert_data(Err::<(), _>(verify_error(&bytes, 3)));
}
#[test]
fn verify_accepts_stamp_equal_to_max_stamp() {
    let bytes = write_file(4, &rows(4), 8);
    assert_eq!(decoder().verify_inner(& bytes, & info_for(4)), Ok(()));
}
#[test]
fn stored_checksums_in_file_order_without_binding() {
    let data = vec![Row::new(9, 1), Row::new(3, 2), Row::new(5, 3)];
    let bytes = write_file(3, &data, 2);
    let mut other = header(0);
    other.model_id = "another".into();
    let d = ParquetDecoder::new(other, Arc::new(checksum_of));
    assert!(d.read_info(& bytes).is_err());
    let expected: Vec<(u64, u64)> = data
        .iter()
        .map(|r| (r.id, r.stored_checksum()))
        .collect();
    assert_eq!(d.read_stored_checksums(& bytes), Ok(expected));
}
#[test]
fn stored_checksums_report_format_errors() {
    let schema = Schema::new(vec![Field::new("other", DataType::UInt64, false)]);
    let cols: Vec<ArrayRef> = vec![Arc::new(UInt64Array::from(vec![1u64]))];
    let bytes = write_parts(
        schema,
        cols,
        &header(1).to_kv(),
        8,
        Compression::UNCOMPRESSED,
    );
    assert!(
        matches!(decoder().read_stored_checksums(& bytes), Err(DecodeError::Format(_)))
    );
}
#[test]
fn truncated_files_give_footer_errors() {
    let bytes = write_file(8, &rows(8), 3);
    let len = bytes.len();
    let step = len / 150 + 1;
    let lengths: Vec<usize> = (0..len.min(64))
        .chain((0..len).step_by(step))
        .chain(len.saturating_sub(64)..len)
        .collect();
    let d = decoder();
    for cut in lengths {
        let prefix = &bytes[..cut];
        assert!(
            matches!(d.read_header(prefix), Err(DecodeError::Footer(_))), "cut {cut}"
        );
        let text = d.read_info(prefix).unwrap_err();
        assert!(text.starts_with("unreadable file:"), "{text}");
    }
}
#[test]
fn garbled_files_never_panic() {
    let bytes = write_file(8, &rows(8), 3);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    let len = bytes.len();
    let positions: Vec<usize> = (len.saturating_sub(160)..len)
        .chain((0..len).step_by(len / 120 + 1))
        .collect();
    for pos in positions {
        for mask in [0xffu8, 0x01, 0x80] {
            let mut garbled = bytes.clone();
            garbled[pos] ^= mask;
            let _ = d.read_header(&garbled);
            let _ = d.read_info(&garbled);
            let _ = d.read_ids_stamps(&garbled, &info);
            let _ = d.decode_rows(&garbled, &info, 0, 8);
            let _ = d.decode_rows(&garbled, &info, 3, 5);
            let _ = d.verify(&garbled, &info);
            let _ = d.read_stored_checksums(&garbled);
        }
    }
    for junk in [&b""[..], b"PAR1", b"PAR1PAR1", &[0u8; 100], &[0xffu8; 100]] {
        assert!(d.read_info(junk).is_err());
        assert!(d.verify(junk, & info).is_err());
    }
}
#[test]
fn garbled_data_pages_are_reported_not_panicked() {
    let mut bytes = write_file(8, &rows(8), 3);
    let d = decoder();
    let info = d.read_info(&bytes).unwrap();
    corrupt_chunk(&mut bytes, 1, ROOT_VECTOR);
    assert!(d.decode_rows(& bytes, & info, 3, 6).is_err());
    assert!(d.verify(& bytes, & info).is_err());
}
#[test]
fn column_chunks_outside_the_file_are_rejected() {
    let bytes = write_file(4, &rows(4), 8);
    let meta = load_metadata(&bytes).unwrap();
    let roots = root_indices(&meta, &[COL_VECTOR]).unwrap();
    assert_data(chunk_ranges(meta.metadata(), &[0], &roots, 10));
    assert_data(chunk_ranges(meta.metadata(), &[5], &roots, bytes.len() as u64));
}
#[test]
fn missing_columns_are_data_errors() {
    let bytes = write_file(4, &rows(4), 8);
    let meta = load_metadata(&bytes).unwrap();
    assert_data(root_indices(&meta, &["nope"]));
}
#[test]
fn copied_ranges_serve_only_copied_bytes() {
    let source: Vec<u8> = (0..100u8).collect();
    let copied = CopiedRanges::copy(&source, vec![40..50, 10..20, 15..30]).unwrap();
    assert_eq!(copied.len(), 100);
    assert_eq!(copied.parts.len(), 2);
    assert_eq!(copied.parts.iter().map(| (_, b) | b.len()).sum::< usize > (), 30);
    assert_eq!(& copied.get_bytes(12, 10).unwrap() [..], & source[12..22]);
    assert_eq!(& copied.get_bytes(20, 10).unwrap() [..], & source[20..30]);
    assert_eq!(& copied.get_bytes(40, 10).unwrap() [..], & source[40..50]);
    assert!(copied.get_bytes(25, 10).is_err());
    assert!(copied.get_bytes(35, 1).is_err());
    assert!(copied.get_bytes(0, 1).is_err());
    assert!(copied.get_bytes(95, 10).is_err());
    assert!(copied.get_bytes(u64::MAX, 2).is_err());
    assert!(copied.get_bytes(35, 0).unwrap().is_empty());
    let mut read = Vec::new();
    std::io::Read::read_to_end(&mut copied.get_read(45).unwrap(), &mut read).unwrap();
    assert_eq!(read, & source[45..50]);
    assert!(copied.get_read(60).is_err());
}
#[test]
fn copied_ranges_reject_ranges_beyond_the_source() {
    let source = [0u8; 10];
    assert_data(CopiedRanges::copy(&source, vec![5..11]).map(|_| ()));
}
#[test]
fn slice_source_is_bounds_checked() {
    let source: Vec<u8> = (0..20u8).collect();
    let reader = SliceSource(&source);
    assert_eq!(reader.len(), 20);
    assert_eq!(& reader.get_bytes(5, 5).unwrap() [..], & source[5..10]);
    assert!(reader.get_bytes(15, 6).is_err());
    assert!(reader.get_bytes(u64::MAX, 1).is_err());
    assert!(reader.get_bytes(1, usize::MAX).is_err());
    assert!(reader.get_read(20).is_ok());
    assert!(reader.get_read(21).is_err());
}
fn write_temp(name: &str, bytes: &[u8]) -> PathBuf {
    let path = temp_file(name);
    std::fs::write(&path, bytes).unwrap();
    path
}
fn open(
    path: &PathBuf,
    d: ParquetDecoder,
    use_mmap: bool,
    verify: bool,
) -> LazyIndexFile {
    LazyIndexFile::new(path, Arc::new(d), OpenOptions { use_mmap, verify })
}
#[test]
fn lazy_index_file_reads_decoded_rows() {
    let data = rows(9);
    let path = write_temp("read", &write_file(30, &data, 4));
    for use_mmap in [true, false] {
        for verify in [true, false] {
            let file = open(&path, decoder(), use_mmap, verify);
            file.ensure_open().unwrap();
            for row in &data {
                let payload = file.read(EntryId(row.id));
                assert_eq!(&* payload.vector, row.vector_bytes().as_slice());
                assert_eq!(
                    &* payload.meta, encode_meta_parts(& row.file, row.start, row.end)
                    .as_slice()
                );
                assert_eq!(&* file.read_vector(EntryId(row.id)), &* payload.vector);
                assert_eq!(&* file.read_meta(EntryId(row.id)).unwrap(), &* payload.meta);
            }
            assert!(file.read_meta(EntryId(5)).is_none());
            assert_eq!(file.read(EntryId(5)), Payload::default());
            assert_eq!(file.stamp_counter().unwrap().max_raw(), Some(30));
        }
    }
    let _ = std::fs::remove_file(path);
}
#[test]
fn lazy_index_file_crosses_batch_boundaries() {
    let n = (BATCH_ROWS_FOR_TEST + 904) as u64;
    let data: Vec<Row> = (0..n).map(|i| Row::new(i * 2 + 1, i + 1)).collect();
    let path = write_temp("batches", &write_file(n, &data, 1000));
    let file = open(&path, decoder(), true, true);
    file.ensure_open().unwrap();
    for i in [0usize, 4095, 4096, 4100, n as usize - 1] {
        let payload = file.read(EntryId(data[i].id));
        assert_eq!(&* payload.vector, data[i].vector_bytes().as_slice(), "row {i}");
    }
    let _ = std::fs::remove_file(path);
}
const BATCH_ROWS_FOR_TEST: usize = 4096;
#[test]
fn lazy_index_file_resumes_stamps_above_both_maxima() {
    let data = rows(7);
    let path = write_temp("stamps_low", &write_file(3, &data, 4));
    let file = open(&path, decoder(), true, false);
    assert_eq!(file.stamp_counter().unwrap().max_raw(), Some(7));
    let _ = std::fs::remove_file(path);
    let path = write_temp("stamps_high", &write_file(20, &data, 4));
    let file = open(&path, decoder(), true, false);
    assert_eq!(file.stamp_counter().unwrap().max_raw(), Some(20));
    let _ = std::fs::remove_file(path);
    let path = write_temp("stamps_empty", &write_file(0, &[], 4));
    let file = open(&path, decoder(), true, true);
    file.ensure_open().unwrap();
    assert_eq!(file.stamp_counter().unwrap().max_raw(), None);
    let _ = std::fs::remove_file(path);
}
#[test]
fn lazy_index_file_reports_binding_failure() {
    let path = write_temp("binding", &write_file(5, &rows(3), 4));
    let mut expected = header(0);
    expected.model_id = "another-model".into();
    let d = ParquetDecoder::new(expected, Arc::new(checksum_of));
    let file = open(&path, d, true, true);
    let text = file.ensure_open().unwrap_err().to_string();
    assert!(text.contains("Indexdatei nicht kompatibel:"), "{text}");
    assert!(text.contains("[model_id: erwartet 'another-model'"), "{text}");
    assert!(file.ensure_open().is_err());
    let _ = std::fs::remove_file(path);
}
#[test]
fn lazy_index_file_verify_reports_checksum_failure() {
    let mut data = rows(5);
    data[2].checksum = Some(1);
    let path = write_temp("badsum", &write_file(5, &data, 4));
    let strict = open(&path, decoder(), true, true);
    let text = strict.ensure_open().unwrap_err().to_string();
    assert!(text.contains("checksum mismatch at row 2 (entry 30)"), "{text}");
    let lax = open(&path, decoder(), true, false);
    lax.ensure_open().unwrap();
    assert_eq!(&* lax.read(EntryId(30)).vector, data[2].vector_bytes().as_slice());
    let _ = std::fs::remove_file(path);
}
#[test]
fn lazy_index_file_rejects_duplicate_ids_and_legacy_files() {
    let mut data = rows(3);
    data[2].id = data[0].id;
    let path = write_temp("dups", &write_file(5, &data, 4));
    let text = open(&path, decoder(), true, false)
        .ensure_open()
        .unwrap_err()
        .to_string();
    assert!(text.contains("duplicate id"), "{text}");
    let _ = std::fs::remove_file(path);
    let path = write_temp("garbage", b"not a parquet file at all");
    let text = open(&path, decoder(), false, false)
        .ensure_open()
        .unwrap_err()
        .to_string();
    assert!(text.contains("unreadable file:"), "{text}");
    let _ = std::fs::remove_file(path);
}
#[test]
fn decoder_is_deterministic_and_parallel() {
    let bytes = write_file(8, &rows(8), 3);
    let d = Arc::new(decoder());
    let info = d.read_info(&bytes).unwrap();
    let first = d.decode_rows(&bytes, &info, 0, 8).unwrap();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope
                .spawn(|| {
                    let again = d.decode_rows(&bytes, &info, 0, 8).unwrap();
                    assert_eq!(
                        again.vectors.iter().map(| v | v.to_vec()).collect::< Vec < _ >>
                        (), first.vectors.iter().map(| v | v.to_vec()).collect::< Vec < _
                        >> ()
                    );
                    assert!(d.verify(& bytes, & info).is_ok());
                });
        }
    });
    assert_eq!(d.expected(), & header(0));
}
#[test]
fn hash_algorithm_difference_is_a_binding_error() {
    let mut expected = header(0);
    expected.hash_algorithm = HashAlgorithm::Fast128;
    let d = ParquetDecoder::new(expected, Arc::new(checksum_of));
    let bytes = write_file(2, &rows(2), 4);
    assert!(d.read_info(& bytes).unwrap_err().contains(KEY_HASH_ALGORITHM));
}
