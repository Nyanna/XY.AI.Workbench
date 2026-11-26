use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;

fn model_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("model/multilingual-e5-small")
}

fn schema() -> TokenizerSchema {
    TokenizerSchema {
        query_prefix: "query: ".to_string(),
        passage_prefix: "passage: ".to_string(),
    }
}

fn word_level(vocab: &[(&str, u32)], normalizer: &str) -> Tokenizer {
    let entries: Vec<String> = vocab.iter().map(|(t, i)| format!("\"{t}\":{i}")).collect();
    let json = format!(
        concat!(
            "{{\"version\":\"1.0\",\"truncation\":null,\"padding\":null,\"added_tokens\":[],",
            "\"normalizer\":{},\"pre_tokenizer\":{{\"type\":\"Whitespace\"}},\"post_processor\":null,",
            "\"decoder\":null,\"model\":{{\"type\":\"WordLevel\",\"vocab\":{{{}}},\"unk_token\":\"[UNK]\"}}}}"
        ),
        normalizer,
        entries.join(",")
    );
    Tokenizer::from_bytes(json.as_bytes()).unwrap()
}

const BASE_VOCAB: [(&str, u32); 6] = [
    ("[UNK]", 0),
    ("[PAD]", 1),
    ("hello", 2),
    ("world", 3),
    ("cafe", 4),
    ("Hello", 5),
];

fn base() -> Tokenizer {
    word_level(&BASE_VOCAB, "null")
}

fn loaded(tokenizer: Tokenizer, max_seq_len: usize) -> LoadedTokenizer {
    LoadedTokenizer::from_tokenizer(tokenizer, "test", schema(), max_seq_len).unwrap()
}

fn ids(t: &LoadedTokenizer, text: &str) -> Vec<u32> {
    t.tokenizer().encode(text, true).unwrap().get_ids().to_vec()
}

struct FixedSource(Result<ResolvedNormalization, String>);

impl ResourceLoader<ResolvedNormalization> for FixedSource {
    fn load(&self) -> Result<ResolvedNormalization, String> {
        self.0.clone()
    }
}

#[test]
fn schema_selects_prefix_by_mode() {
    let s = schema();
    assert_eq!(s.prefix(EmbedMode::Query), "query: ");
    assert_eq!(s.prefix(EmbedMode::Passage), "passage: ");
}

#[test]
fn error_display_texts() {
    let p = PathBuf::from("/x/tokenizer.json");
    assert_eq!(
        TokenizerLoadError::Load { path: p, cause: "boom".into() }.to_string(),
        "Tokenizer konnte nicht geladen werden (/x/tokenizer.json): boom"
    );
    assert_eq!(
        TokenizerLoadError::NoPadToken { id: "t".into() }.to_string(),
        "kein Pad-Token gefunden (Tokenizer t)"
    );
    assert_eq!(
        TokenizerLoadError::Truncation { id: "t".into(), cause: "c".into() }.to_string(),
        "Trunkierung abgelehnt (Tokenizer t): c"
    );
    assert_eq!(
        TokenizerLoadError::Normalization { id: "t".into(), cause: "c".into() }.to_string(),
        "Normalisierung abgelehnt (Tokenizer t): c"
    );
}

#[test]
fn missing_file_is_load_error_with_full_path() {
    let path = model_dir().join("does-not-exist").join(TOKENIZER_FILE_NAME);
    match LoadedTokenizer::from_file(&path, schema(), 8) {
        Err(TokenizerLoadError::Load { path: p, cause }) => {
            assert_eq!(p, path);
            assert!(!cause.is_empty());
        }
        other => panic!("unexpected: {other:?}"),
    }
}

#[test]
fn real_model_is_ready() {
    let path = model_dir().join(TOKENIZER_FILE_NAME);
    let t = LoadedTokenizer::from_file(&path, schema(), 8).unwrap();
    assert_eq!(t.id(), path.to_string_lossy());
    assert_eq!(t.pad_token(), "<pad>");
    assert_eq!(t.pad_token_id(), 1);
    assert_eq!(t.tokenizer().token_to_id(t.pad_token()), Some(t.pad_token_id()));
    assert!(t.tokenizer().get_padding().is_none());
    assert_eq!(t.max_seq_len(), 8);

    let encoded = ids(&t, &"word ".repeat(100));
    assert_eq!(encoded.len(), 8);
    assert_eq!(*encoded.last().unwrap(), 2);
}

#[test]
fn from_descriptor_uses_config_directory_and_descriptor_values() {
    let mut d = ModelDescriptor::e5_small_default(&model_dir(), DType::F32);
    d.passage_prefix = String::new();
    let t = LoadedTokenizer::from_descriptor(&d).unwrap();
    assert_eq!(t.id(), model_dir().join(TOKENIZER_FILE_NAME).to_string_lossy());
    assert_eq!(t.max_seq_len(), 512);
    assert_eq!(t.schema().query_prefix, "query: ");
    assert_eq!(t.schema().passage_prefix, "");
}

#[test]
fn pad_token_priority() {
    let both = [("[UNK]", 0), ("[PAD]", 1), ("<pad>", 4)];
    let t = loaded(word_level(&both, "null"), 4);
    assert_eq!((t.pad_token(), t.pad_token_id()), ("<pad>", 4));

    let mut configured = word_level(&both, "null");
    configured.with_padding(Some(tokenizers::PaddingParams {
        pad_id: 1,
        pad_token: "[PAD]".to_string(),
        ..Default::default()
    }));
    let t = loaded(configured, 4);
    assert_eq!((t.pad_token(), t.pad_token_id()), ("[PAD]", 1));
    assert!(t.tokenizer().get_padding().is_none());

    let t = loaded(base(), 4);
    assert_eq!((t.pad_token(), t.pad_token_id()), ("[PAD]", 1));
}

#[test]
fn missing_pad_token_is_error() {
    let tokenizer = word_level(&[("[UNK]", 0), ("hello", 1)], "null");
    let err = LoadedTokenizer::from_tokenizer(tokenizer, "x", schema(), 4).unwrap_err();
    assert_eq!(err, TokenizerLoadError::NoPadToken { id: "x".to_string() });
}

#[test]
fn zero_max_seq_len_is_truncation_error() {
    let err = LoadedTokenizer::from_tokenizer(base(), "x", schema(), 0).unwrap_err();
    assert!(matches!(err, TokenizerLoadError::Truncation { ref id, .. } if id == "x"));
}

#[test]
fn truncation_applies_to_every_input() {
    let t = loaded(base(), 2);
    assert_eq!(ids(&t, "hello world hello world").len(), 2);
    assert_eq!(ids(&t, "hello").len(), 1);
}

#[test]
fn default_normalization_leaves_tokenizer_untouched() {
    let plain = loaded(base(), 4);
    let applied = loaded(base(), 4).with_normalization(ResolvedNormalization::default()).unwrap();
    assert_eq!(
        plain.tokenizer().to_string(false).unwrap(),
        applied.tokenizer().to_string(false).unwrap()
    );
    assert_eq!(applied.normalization(), ResolvedNormalization::default());
}

#[test]
fn lowercase_and_strip_accents_extend_normalizer() {
    let n = ResolvedNormalization {
        unicode_form: UnicodeForm::None,
        lowercase: true,
        strip_accents: true,
    };
    let t = loaded(base(), 4);
    assert_eq!(ids(&t, "Caf\u{e9}"), vec![0]);
    let t = t.with_normalization(n).unwrap();
    assert_eq!(ids(&t, "Caf\u{e9}"), vec![4]);
    assert_eq!(t.normalization(), n);
}

#[test]
fn extension_steps_precede_existing_normalizer() {
    let strip = "{\"type\":\"Strip\",\"strip_left\":true,\"strip_right\":true}";
    let t = loaded(word_level(&BASE_VOCAB, strip), 4);
    let n = ResolvedNormalization {
        unicode_form: UnicodeForm::Nfc,
        lowercase: true,
        strip_accents: false,
    };
    let json = t.with_normalization(n).unwrap().tokenizer().to_string(false).unwrap();
    let lower = json.find("\"type\":\"Lowercase\"").unwrap();
    let nfc = json.find("\"type\":\"NFC\"").unwrap();
    let existing = json.find("\"type\":\"Strip\"").unwrap();
    assert!(lower < nfc && nfc < existing);
}

#[test]
fn compatibility_form_uses_nfkd_for_accent_stripping() {
    let n = ResolvedNormalization {
        unicode_form: UnicodeForm::Nfkc,
        lowercase: false,
        strip_accents: true,
    };
    let json = loaded(base(), 4).with_normalization(n).unwrap().tokenizer().to_string(false).unwrap();
    let nfkd = json.find("\"type\":\"NFKD\"").unwrap();
    let strip = json.find("\"type\":\"StripAccents\"").unwrap();
    let nfkc = json.find("\"type\":\"NFKC\"").unwrap();
    assert!(nfkd < strip && strip < nfkc);
}

#[test]
fn loader_is_lazy_and_reports_load_errors() {
    let path = model_dir().join("missing").join(TOKENIZER_FILE_NAME);
    let loader = TokenizerLoader::from_file(path.clone(), schema(), 8);
    let msg = loader.load().unwrap_err();
    assert!(msg.starts_with(&format!("Tokenizer konnte nicht geladen werden ({})", path.display())));
}

#[test]
fn loader_from_descriptor_reads_real_file_on_load() {
    let d = ModelDescriptor::e5_small_default(&model_dir(), DType::F32);
    let t = TokenizerLoader::from_descriptor(&d).load().unwrap();
    assert_eq!(t.pad_token_id(), 1);
    assert_eq!(t.max_seq_len(), 512);
}

#[test]
fn loader_from_tokenizer_builds_independent_values() {
    let loader = TokenizerLoader::from_tokenizer(base(), "mem".to_string(), schema(), 3);
    let a = loader.load().unwrap();
    let b = loader.load().unwrap();
    assert_eq!(a.id(), "mem");
    assert_eq!(b.max_seq_len(), 3);
    assert_eq!(ids(&a, "hello world hello world"), ids(&b, "hello world hello world"));
}

#[test]
fn loader_applies_normalization_source() {
    let n = ResolvedNormalization {
        unicode_form: UnicodeForm::None,
        lowercase: true,
        strip_accents: false,
    };
    let loader = TokenizerLoader::from_tokenizer(base(), "mem".to_string(), schema(), 4)
        .with_normalization(Arc::new(FixedSource(Ok(n))));
    let t = loader.load().unwrap();
    assert_eq!(t.normalization(), n);
    assert_eq!(ids(&t, "HELLO"), vec![2]);
}

#[test]
fn loader_reports_normalization_source_error() {
    let loader = TokenizerLoader::from_tokenizer(base(), "mem".to_string(), schema(), 4)
        .with_normalization(Arc::new(FixedSource(Err("nope".to_string()))));
    assert_eq!(
        loader.load().unwrap_err(),
        "Normalisierung abgelehnt (Tokenizer mem): nope"
    );
}
