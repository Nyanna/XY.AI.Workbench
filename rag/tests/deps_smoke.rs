//! Smoke tests for third-party format support the resident engine relies on.

use std::fs::File;
use std::sync::Arc;

use arrow::array::{Array, FixedSizeListArray, Float16Array, RecordBatch};
use arrow::datatypes::{DataType, Field, Schema};
use half::f16;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ArrowWriter;
use tokenizers::models::wordlevel::WordLevel;
use tokenizers::Tokenizer;

const DIM: i32 = 4;

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("xyrag-deps-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

#[test]
fn parquet_f16_fixed_size_list_round_trips_bit_exact() {
    let values: Vec<f16> = (0..12).map(|i| f16::from_f32(i as f32 * 0.37 - 1.5)).collect();
    let item = Arc::new(Field::new("item", DataType::Float16, false));
    let col = FixedSizeListArray::new(item.clone(), DIM, Arc::new(Float16Array::from(values.clone())), None);
    let schema = Arc::new(Schema::new(vec![Field::new("emb", DataType::FixedSizeList(item, DIM), false)]));
    let batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(col)]).unwrap();

    let path = scratch("f16.parquet");
    let mut w = ArrowWriter::try_new(File::create(&path).unwrap(), schema, None).unwrap();
    w.write(&batch).unwrap();
    w.close().unwrap();

    let reader = ParquetRecordBatchReaderBuilder::try_new(File::open(&path).unwrap())
        .unwrap()
        .build()
        .unwrap();
    let mut back: Vec<f16> = Vec::new();
    for b in reader {
        let b = b.unwrap();
        let list = b.column(0).as_any().downcast_ref::<FixedSizeListArray>().unwrap();
        assert_eq!(list.value_length(), DIM);
        let flat = list.values().as_any().downcast_ref::<Float16Array>().unwrap();
        back.extend(flat.values().iter().copied());
    }
    let _ = std::fs::remove_file(&path);
    assert_eq!(back.len(), values.len());
    for (a, b) in values.iter().zip(&back) {
        assert_eq!(a.to_bits(), b.to_bits());
    }
}

#[test]
fn tokenizer_loads_from_serialized_json() {
    let vocab = [("[UNK]", 0), ("hello", 1), ("world", 2)].into_iter().map(|(k, v)| (k.to_string(), v));
    let model = WordLevel::builder().vocab(vocab.collect()).unk_token("[UNK]".into()).build().unwrap();
    let json = Tokenizer::new(model).to_string(false).unwrap();

    let path = scratch("tokenizer.json");
    std::fs::write(&path, json).unwrap();
    let tok = Tokenizer::from_file(&path).unwrap();
    let _ = std::fs::remove_file(&path);

    let enc = tok.encode("hello", false).unwrap();
    assert_eq!(enc.get_ids(), &[1]);
    let enc = tok.encode("nope", false).unwrap();
    assert_eq!(enc.get_ids(), &[0]);
}
