use tokenizers::Tokenizer;
use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::tokenizer_loader::TokenizerSchema;
#[test]
fn types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<MultiVectorSchema>();
    assert_send_sync::<EncodingSchema>();
    assert_send_sync::<EncodedSequence>();
    assert_send_sync::<EncodingError>();
}
const CLS: u32 = 10;
const SEP: u32 = 11;
const PAD: u32 = 1;
const MASK: u32 = 99;
const VOCAB: [(&str, u32); 10] = [
    ("[UNK]", 0),
    ("[PAD]", 1),
    ("hello", 2),
    ("world", 3),
    ("query", 4),
    ("passage", 5),
    (":", 6),
    ("Q", 7),
    ("P", 8),
    ("[CLS]", CLS),
];
fn tokenizer(with_specials: bool) -> Tokenizer {
    let mut entries: Vec<String> = VOCAB
        .iter()
        .map(|(t, i)| format!("\"{t}\":{i}"))
        .collect();
    entries.push(format!("\"[SEP]\":{SEP}"));
    let post = if with_specials {
        concat!(
            "{\"type\":\"TemplateProcessing\",",
            "\"single\":[{\"SpecialToken\":{\"id\":\"[CLS]\",\"type_id\":0}},",
            "{\"Sequence\":{\"id\":\"A\",\"type_id\":0}},",
            "{\"SpecialToken\":{\"id\":\"[SEP]\",\"type_id\":0}}],",
            "\"pair\":[{\"Sequence\":{\"id\":\"A\",\"type_id\":0}},",
            "{\"Sequence\":{\"id\":\"B\",\"type_id\":1}}],", "\"special_tokens\":{",
            "\"[CLS]\":{\"id\":\"[CLS]\",\"ids\":[10],\"tokens\":[\"[CLS]\"]},",
            "\"[SEP]\":{\"id\":\"[SEP]\",\"ids\":[11],\"tokens\":[\"[SEP]\"]}}}"
        )
    } else {
        "null"
    };
    let json = format!(
        concat!("{{\"version\":\"1.0\",\"truncation\":null,\"padding\":null,",
        "\"added_tokens\":[{{\"id\":1,\"content\":\"[PAD]\",\"single_word\":false,",
        "\"lstrip\":false,\"rstrip\":false,\"normalized\":false,\"special\":true}}],",
        "\"normalizer\":null,\"pre_tokenizer\":{{\"type\":\"Whitespace\"}},\"post_processor\":{},",
        "\"decoder\":null,\"model\":{{\"type\":\"WordLevel\",\"vocab\":{{{}}},\"unk_token\":\"[UNK]\"}}}}"),
        post, entries.join(",")
    );
    Tokenizer::from_bytes(json.as_bytes()).unwrap()
}
fn loaded_with(
    max_seq_len: usize,
    with_specials: bool,
    query_prefix: &str,
) -> LoadedTokenizer {
    let schema = TokenizerSchema {
        query_prefix: query_prefix.to_string(),
        passage_prefix: "passage: ".to_string(),
    };
    LoadedTokenizer::from_tokenizer(
            tokenizer(with_specials),
            "test",
            schema,
            max_seq_len,
        )
        .unwrap()
}
fn loaded(max_seq_len: usize) -> LoadedTokenizer {
    loaded_with(max_seq_len, true, "query: ")
}
fn mv(query_len: usize, document_len: usize) -> EncodingSchema {
    EncodingSchema::MultiVector(MultiVectorSchema {
        query_marker: "Q".to_string(),
        document_marker: "P".to_string(),
        max_query_len: query_len,
        max_document_len: document_len,
        mask_token_id: MASK,
    })
}
fn assert_mask(seq: &EncodedSequence) {
    assert_eq!(seq.attention_mask, vec![1; seq.ids.len()]);
}
#[test]
fn prefix_schema_prepends_mode_prefix_and_keeps_tokens() {
    let t = loaded(32);
    let q = EncodingSchema::Prefix.encode(&t, "hello", EmbedMode::Query).unwrap();
    assert_eq!(q.ids, vec![CLS, 4, 6, 2, SEP]);
    assert_mask(&q);
    let p = EncodingSchema::Prefix.encode(&t, "world", EmbedMode::Passage).unwrap();
    assert_eq!(p.ids, vec![CLS, 5, 6, 3, SEP]);
    assert_mask(&p);
}
#[test]
fn prefix_schema_with_empty_prefix_and_empty_text() {
    let t = loaded_with(32, true, "");
    let q = EncodingSchema::Prefix.encode(&t, "", EmbedMode::Query).unwrap();
    assert_eq!(q.ids, vec![CLS, SEP]);
    assert_mask(&q);
}
#[test]
fn prefix_schema_uses_general_truncation_only() {
    let t = loaded(4);
    let q = EncodingSchema::Prefix
        .encode(&t, "hello world hello world", EmbedMode::Query)
        .unwrap();
    assert_eq!(q.ids, vec![CLS, 4, 6, SEP]);
    assert_mask(&q);
}
#[test]
fn prefix_schema_keeps_pad_tokens_in_text() {
    let t = loaded_with(32, true, "");
    let q = EncodingSchema::Prefix.encode(&t, "hello [PAD]", EmbedMode::Query).unwrap();
    assert_eq!(q.ids, vec![CLS, 2, PAD, SEP]);
}
#[test]
fn query_is_padded_with_mask_token_to_fixed_length() {
    let t = loaded(32);
    let q = mv(6, 8).encode(&t, "hello", EmbedMode::Query).unwrap();
    assert_eq!(q.ids, vec![CLS, 7, 2, SEP, MASK, MASK]);
    assert_mask(&q);
}
#[test]
fn query_longer_than_limit_is_cut_and_keeps_final_special_token() {
    let t = loaded(32);
    let q = mv(4, 8).encode(&t, "hello world hello", EmbedMode::Query).unwrap();
    assert_eq!(q.ids, vec![CLS, 7, 2, SEP]);
    assert_mask(&q);
    let one = mv(1, 8).encode(&t, "hello world", EmbedMode::Query).unwrap();
    assert_eq!(one.ids, vec![SEP]);
}
#[test]
fn query_is_padded_even_when_general_truncation_is_shorter() {
    let t = loaded(4);
    let q = mv(6, 8).encode(&t, "hello world hello", EmbedMode::Query).unwrap();
    assert_eq!(q.ids, vec![CLS, 7, 2, SEP, MASK, MASK]);
    assert_mask(&q);
}
#[test]
fn query_with_exact_length_is_unchanged() {
    let t = loaded(32);
    let q = mv(4, 8).encode(&t, "hello", EmbedMode::Query).unwrap();
    assert_eq!(q.ids, vec![CLS, 7, 2, SEP]);
}
#[test]
fn document_is_cut_but_not_padded() {
    let t = loaded(32);
    let long = mv(6, 5)
        .encode(&t, "hello world hello world", EmbedMode::Passage)
        .unwrap();
    assert_eq!(long.ids, vec![CLS, 8, 2, 3, SEP]);
    assert_mask(&long);
    let short = mv(6, 5).encode(&t, "hello", EmbedMode::Passage).unwrap();
    assert_eq!(short.ids, vec![CLS, 8, 2, SEP]);
    assert_mask(&short);
}
#[test]
fn document_removes_pad_tokens_before_cutting() {
    let t = loaded(32);
    let d = mv(6, 4)
        .encode(&t, "hello [PAD] world [PAD] hello", EmbedMode::Passage)
        .unwrap();
    assert_eq!(d.ids, vec![CLS, 8, 2, SEP]);
    let short = mv(6, 8).encode(&t, "[PAD] hello [PAD]", EmbedMode::Passage).unwrap();
    assert_eq!(short.ids, vec![CLS, 8, 2, SEP]);
    assert_mask(&short);
}
#[test]
fn cut_without_special_last_token_keeps_first_tokens() {
    let t = loaded_with(32, false, "query: ");
    let d = mv(6, 2).encode(&t, "hello world", EmbedMode::Passage).unwrap();
    assert_eq!(d.ids, vec![8, 2]);
    let q = mv(3, 2).encode(&t, "hello world", EmbedMode::Query).unwrap();
    assert_eq!(q.ids, vec![7, 2, 3]);
}
#[test]
fn empty_marker_adds_no_lead() {
    let t = loaded(32);
    let schema = EncodingSchema::MultiVector(MultiVectorSchema {
        query_marker: String::new(),
        document_marker: String::new(),
        max_query_len: 4,
        max_document_len: 4,
        mask_token_id: MASK,
    });
    let q = schema.encode(&t, "hello", EmbedMode::Query).unwrap();
    assert_eq!(q.ids, vec![CLS, 2, SEP, MASK]);
    let d = schema.encode(&t, "", EmbedMode::Passage).unwrap();
    assert_eq!(d.ids, vec![CLS, SEP]);
}
#[test]
fn encode_is_deterministic() {
    let t = loaded(32);
    let schema = mv(6, 5);
    let a = schema.encode(&t, "hello world", EmbedMode::Query).unwrap();
    let b = schema.encode(&t, "hello world", EmbedMode::Query).unwrap();
    assert_eq!(a, b);
}
#[test]
fn validate_rejects_zero_lengths_naming_the_field() {
    assert_eq!(EncodingSchema::Prefix.validate(), Ok(()));
    assert_eq!(mv(1, 1).validate(), Ok(()));
    match mv(0, 4).validate().unwrap_err() {
        EncodingError::InvalidSchema { reason } => {
            assert!(reason.contains("max_query_len"))
        }
        other => panic!("unexpected {other:?}"),
    }
    match mv(4, 0).validate().unwrap_err() {
        EncodingError::InvalidSchema { reason } => {
            assert!(reason.contains("max_document_len"))
        }
        other => panic!("unexpected {other:?}"),
    }
}
#[test]
fn encode_validates_first_in_both_modes() {
    let t = loaded(32);
    for mode in [EmbedMode::Query, EmbedMode::Passage] {
        let err = mv(0, 4).encode(&t, "hello", mode).unwrap_err();
        assert!(matches!(err, EncodingError::InvalidSchema { .. }));
    }
}
#[test]
fn from_config_copies_values() {
    let cfg = MultiVectorConfig {
        max_query_len: 32,
        max_document_len: 180,
        query_marker: "[Q]".to_string(),
        document_marker: "[D]".to_string(),
        dimension: 128,
        skiplist: vec![".".to_string()],
    };
    let schema = MultiVectorSchema::from_config(&cfg, 103);
    assert_eq!(
        schema, MultiVectorSchema { query_marker : "[Q]".to_string(), document_marker :
        "[D]".to_string(), max_query_len : 32, max_document_len : 180, mask_token_id :
        103, }
    );
}
#[test]
fn error_display_texts() {
    assert_eq!(
        EncodingError::InvalidSchema { reason : "r".to_string() } .to_string(),
        "ungültiges Encoding-Schema: r"
    );
    assert_eq!(
        EncodingError::Tokenize { cause : "c".to_string() } .to_string(),
        "Tokenisierung fehlgeschlagen: c"
    );
}
