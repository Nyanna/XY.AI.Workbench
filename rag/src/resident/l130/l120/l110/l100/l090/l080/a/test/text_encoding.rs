use std::borrow::Cow;
use std::sync::Arc;
use tokenizers::Tokenizer;
use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::a::encoding_schemata::MultiVectorSchema;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::text_normalization::TextNormalizer;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::tokenizer_loader::TokenizerSchema;
const PAD: u32 = 1;
const MASK: u32 = 9;
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
    ("[MASK]", 9),
];
fn word_level() -> Tokenizer {
    let entries: Vec<String> = VOCAB
        .iter()
        .map(|(t, i)| format!("\"{t}\":{i}"))
        .collect();
    let json = format!(
        concat!("{{\"version\":\"1.0\",\"truncation\":null,\"padding\":null,\"added_tokens\":[],",
        "\"normalizer\":null,\"pre_tokenizer\":{{\"type\":\"Whitespace\"}},\"post_processor\":null,",
        "\"decoder\":null,\"model\":{{\"type\":\"WordLevel\",\"vocab\":{{{}}},\"unk_token\":\"[UNK]\"}}}}"),
        entries.join(",")
    );
    Tokenizer::from_bytes(json.as_bytes()).unwrap()
}
fn loaded(max_seq_len: usize) -> LoadedTokenizer {
    let schema = TokenizerSchema {
        query_prefix: "query: ".to_string(),
        passage_prefix: "passage: ".to_string(),
    };
    LoadedTokenizer::from_tokenizer(word_level(), "test", schema, max_seq_len).unwrap()
}
fn identity() -> TextPreprocessor {
    TextPreprocessor::new(None)
}
fn multi(query_len: usize) -> EncodingSchema {
    multi_with_markers("Q", "P", query_len, 16)
}
fn multi_with_markers(
    query_marker: &str,
    document_marker: &str,
    max_query_len: usize,
    max_document_len: usize,
) -> EncodingSchema {
    EncodingSchema::MultiVector(MultiVectorSchema {
        query_marker: query_marker.to_string(),
        document_marker: document_marker.to_string(),
        max_query_len,
        max_document_len,
        mask_token_id: MASK,
    })
}
fn config() -> BatchConfig {
    BatchConfig {
        token_budget: 64,
        max_seq_len: 32,
        pad_token_id: PAD,
    }
}
struct Upper;
impl TextNormalizer for Upper {
    fn normalize<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError> {
        Ok(Cow::Owned(text.replace("HELLO", "hello")))
    }
}
struct Failing;
impl TextNormalizer for Failing {
    fn normalize<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError> {
        if text.contains("bad") {
            Err(NormalizeError("boom".to_string()))
        } else {
            Ok(Cow::Borrowed(text))
        }
    }
}
#[test]
fn empty_input_is_error() {
    let t = loaded(16);
    let texts: [&str; 0] = [];
    let err = encode_texts(
            &t,
            &identity(),
            &EncodingSchema::Prefix,
            &texts,
            EmbedMode::Query,
        )
        .unwrap_err();
    assert_eq!(err, TextEncodingError::NoInputTexts);
    let err = encode_and_batch(
            &t,
            &identity(),
            &EncodingSchema::Prefix,
            &texts,
            EmbedMode::Query,
            &config(),
        )
        .unwrap_err();
    assert_eq!(err, TextEncodingError::NoInputTexts);
}
#[test]
fn prefix_schema_prepends_prefix_by_mode() {
    let t = loaded(16);
    let schema = EncodingSchema::Prefix;
    let q = encode_texts(&t, &identity(), &schema, &["hello"], EmbedMode::Query)
        .unwrap();
    assert_eq!(q, vec![vec![4, 6, 2]]);
    let p = encode_texts(&t, &identity(), &schema, &["world"], EmbedMode::Passage)
        .unwrap();
    assert_eq!(p, vec![vec![5, 6, 3]]);
}
#[test]
fn result_keeps_input_order_and_accepts_owned_strings() {
    let t = loaded(16);
    let texts = vec![
        "hello world hello".to_string(), "world".to_string(), "hello".to_string()
    ];
    let out = encode_texts(
            &t,
            &identity(),
            &EncodingSchema::Prefix,
            &texts,
            EmbedMode::Passage,
        )
        .unwrap();
    assert_eq!(out.len(), 3);
    assert_eq!(out[0], vec![5, 6, 2, 3, 2]);
    assert_eq!(out[1], vec![5, 6, 3]);
    assert_eq!(out[2], vec![5, 6, 2]);
}
#[test]
fn multi_vector_uses_markers_and_pads_only_queries() {
    let t = loaded(16);
    let schema = multi(5);
    let q = encode_texts(&t, &identity(), &schema, &["hello"], EmbedMode::Query)
        .unwrap();
    assert_eq!(q, vec![vec![7, 2, MASK, MASK, MASK]]);
    let p = encode_texts(&t, &identity(), &schema, &["hello"], EmbedMode::Passage)
        .unwrap();
    assert_eq!(p, vec![vec![8, 2]]);
}
#[test]
fn long_queries_are_cut_to_max_query_len() {
    let t = loaded(16);
    let out = encode_texts(
            &t,
            &identity(),
            &multi(2),
            &["hello world hello"],
            EmbedMode::Query,
        )
        .unwrap();
    assert_eq!(out, vec![vec![7, 2]]);
}
#[test]
fn query_padding_equal_length_is_unchanged() {
    let t = loaded(16);
    let out = encode_texts(&t, &identity(), &multi(2), &["hello"], EmbedMode::Query)
        .unwrap();
    assert_eq!(out, vec![vec![7, 2]]);
}
#[test]
fn empty_marker_adds_nothing() {
    let t = loaded(16);
    let schema = multi_with_markers("", "", 1, 1);
    let out = encode_texts(&t, &identity(), &schema, &["hello"], EmbedMode::Query)
        .unwrap();
    assert_eq!(out, vec![vec![2]]);
}
#[test]
fn truncation_limits_to_max_seq_len() {
    let t = loaded(3);
    let out = encode_texts(
            &t,
            &identity(),
            &EncodingSchema::Prefix,
            &["hello world hello world"],
            EmbedMode::Query,
        )
        .unwrap();
    assert_eq!(out, vec![vec![4, 6, 2]]);
}
#[test]
fn query_padding_applies_after_truncation_up_to_query_len() {
    let t = loaded(3);
    let out = encode_texts(
            &t,
            &identity(),
            &multi(5),
            &["hello world hello"],
            EmbedMode::Query,
        )
        .unwrap();
    assert_eq!(out, vec![vec![7, 2, 3, MASK, MASK]]);
}
#[test]
fn external_preprocessor_runs_before_prefixing() {
    let t = loaded(16);
    let pre = TextPreprocessor::new(Some(Arc::new(Upper)));
    let out = encode_texts(
            &t,
            &pre,
            &EncodingSchema::Prefix,
            &["HELLO"],
            EmbedMode::Passage,
        )
        .unwrap();
    assert_eq!(out, vec![vec![5, 6, 2]]);
}
#[test]
fn normalizer_failure_is_passed_through() {
    let t = loaded(16);
    let pre = TextPreprocessor::new(Some(Arc::new(Failing)));
    let err = encode_texts(
            &t,
            &pre,
            &EncodingSchema::Prefix,
            &["hello", "bad text"],
            EmbedMode::Query,
        )
        .unwrap_err();
    assert_eq!(err, TextEncodingError::Normalize(NormalizeError("boom".to_string())));
    let err = encode_and_batch(
            &t,
            &pre,
            &EncodingSchema::Prefix,
            &["bad"],
            EmbedMode::Query,
            &config(),
        )
        .unwrap_err();
    assert_eq!(err, TextEncodingError::Normalize(NormalizeError("boom".to_string())));
}
#[test]
fn invalid_schema_is_reported_before_any_encoding() {
    let t = loaded(16);
    let pre = TextPreprocessor::new(Some(Arc::new(Failing)));
    for schema in [
        multi_with_markers("Q", "P", 0, 4),
        multi_with_markers("Q", "P", 4, 0),
    ] {
        let err = encode_texts(&t, &pre, &schema, &["bad"], EmbedMode::Query)
            .unwrap_err();
        assert!(
            matches!(err, TextEncodingError::Schema(SchemaError::InvalidSchema { .. }))
        );
    }
}
#[test]
fn padded_query_positions_stay_attended_in_batches() {
    let t = loaded(16);
    let batches = encode_and_batch(
            &t,
            &identity(),
            &multi(5),
            &["hello"],
            EmbedMode::Query,
            &config(),
        )
        .unwrap();
    assert_eq!(batches.len(), 1);
    let b = &batches[0];
    assert_eq!(& b.input_ids() [..5], & [7, 2, MASK, MASK, MASK]);
    assert_eq!(& b.attention_mask() [..5], & [1, 1, 1, 1, 1]);
    assert_eq!(b.attention_mask().iter().sum::< u32 > (), 5);
}
#[test]
fn error_display_texts() {
    assert_eq!(TextEncodingError::NoInputTexts.to_string(), "keine Eingabetexte");
    assert_eq!(
        TextEncodingError::Normalize(NormalizeError("x".to_string())).to_string(),
        "Normalisierung fehlgeschlagen: x"
    );
    assert_eq!(
        TextEncodingError::Schema(SchemaError::Tokenize { cause : "y".to_string() })
        .to_string(), "Tokenisierung fehlgeschlagen: y"
    );
    assert_eq!(
        TextEncodingError::Schema(SchemaError::InvalidSchema { reason : "z".to_string()
        }).to_string(), "ungültiges Encoding-Schema: z"
    );
    assert_eq!(
        TextEncodingError::Batch(BatchError::InvalidConfig).to_string(),
        "Batch-Bildung fehlgeschlagen: InvalidConfig"
    );
}
#[test]
fn invalid_batch_config_is_wrapped() {
    let t = loaded(16);
    let cfg = BatchConfig {
        token_budget: 0,
        max_seq_len: 32,
        pad_token_id: PAD,
    };
    let err = encode_and_batch(
            &t,
            &identity(),
            &EncodingSchema::Prefix,
            &["hello"],
            EmbedMode::Query,
            &cfg,
        )
        .unwrap_err();
    assert_eq!(err, TextEncodingError::Batch(BatchError::InvalidConfig));
}
#[test]
fn batches_cover_every_input_index_exactly_once() {
    let t = loaded(64);
    let texts: Vec<String> = (0..7).map(|i| "hello ".repeat(i + 1)).collect();
    let ids = encode_texts(
            &t,
            &identity(),
            &EncodingSchema::Prefix,
            &texts,
            EmbedMode::Passage,
        )
        .unwrap();
    let cfg = BatchConfig {
        token_budget: 64,
        max_seq_len: 64,
        pad_token_id: PAD,
    };
    let batches = encode_and_batch(
            &t,
            &identity(),
            &EncodingSchema::Prefix,
            &texts,
            EmbedMode::Passage,
            &cfg,
        )
        .unwrap();
    assert_eq!(batches, build_batches(& ids, & cfg).unwrap());
    let mut seen: Vec<u32> = batches
        .iter()
        .flat_map(|b| b.positions().to_vec())
        .collect();
    seen.sort_unstable();
    assert_eq!(seen, (0..7).collect::< Vec < u32 >> ());
}
