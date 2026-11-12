use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
fn model_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("model/multilingual-e5-small")
}
fn load(max_len: usize) -> Tokenizer {
    Tokenizer::from_file(&model_dir().join(TOKENIZER_FILE_NAME), max_len).unwrap()
}
#[test]
fn zero_max_len_is_invalid() {
    let err = Tokenizer::from_file(&model_dir().join(TOKENIZER_FILE_NAME), 0)
        .err()
        .unwrap();
    assert!(matches!(err, TokenizerError::InvalidConfig { .. }));
}
#[test]
fn missing_file_is_load_error() {
    let err = Tokenizer::from_file(Path::new("/nonexistent/tokenizer.json"), 8)
        .err()
        .unwrap();
    assert!(matches!(err, TokenizerError::Load { .. }));
    assert!(err.to_string().starts_with("Tokenizer-Datei nicht ladbar"));
}
#[test]
fn descriptor_zero_seq_len_is_invalid() {
    let mut d = ModelDescriptor::e5_small_default(&model_dir(), DType::F32);
    d.max_seq_len = 0;
    let err = Tokenizer::from_descriptor(&d).err().unwrap();
    assert!(matches!(err, TokenizerError::InvalidConfig { .. }));
}
#[test]
fn encode_adds_special_tokens_and_is_deterministic() {
    let d = ModelDescriptor::e5_small_default(&model_dir(), DType::F32);
    let t = Tokenizer::shared_from_descriptor(&d).unwrap();
    assert_eq!(t.max_len(), 512);
    let a = t.encode("query: hello world").unwrap();
    assert_eq!(a, t.encode("query: hello world").unwrap());
    assert_eq!(a[0], 0);
    assert_eq!(* a.last().unwrap(), 2);
    assert!(a.iter().all(|& i | (i as usize) < t.vocab_size()));
}
#[test]
fn empty_text_yields_only_special_tokens() {
    let t = load(512);
    assert_eq!(t.encode("").unwrap(), vec![0, 2]);
}
#[test]
fn truncation_keeps_trailing_special_token() {
    let t = load(8);
    let ids = t.encode(&"word ".repeat(100)).unwrap();
    assert_eq!(ids.len(), 8);
    assert_eq!(* ids.last().unwrap(), 2);
}
#[test]
fn batch_matches_single_and_preserves_order() {
    let t = load(16);
    assert!(t.encode_batch(& []).unwrap().is_empty());
    let texts = ["a", "hello world", "", "passage: Guten Tag"];
    let batch = t.encode_batch(&texts).unwrap();
    for (i, text) in texts.iter().enumerate() {
        assert_eq!(batch[i], t.encode(text).unwrap());
    }
}
