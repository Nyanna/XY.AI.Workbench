//! One shared CPU tokenizer. Token ids reach the device only as `u32` inputs of the encoder;
//! there is deliberately no device-side tokenizer or id-to-address gather.

mod error;
mod modes;
mod normalize;

use std::sync::Arc;

use tokenizers::{Tokenizer, TruncationParams};

use crate::resident::model::{plan_batches, Batch, BatchConfig, ModelSpec};
use crate::resident::subengine::{Resource, ResourceKey};

pub use error::TokenizeError;
pub use modes::{ColbertScheme, EncodeMode, ModeScheme};
pub use normalize::{Identity, TextNormalizer};

pub struct SharedTokenizer {
    key: ResourceKey,
    tokenizer: Tokenizer,
    scheme: ModeScheme,
    normalizer: Option<Arc<dyn TextNormalizer>>,
    pad_id: u32,
}

impl SharedTokenizer {
    /// Loads `tokenizer.json` next to the model's config file.
    pub fn from_spec(spec: &ModelSpec) -> Result<Self, TokenizeError> {
        let path = spec.paths.config.with_file_name("tokenizer.json");
        let tokenizer = Tokenizer::from_file(&path)
            .map_err(|e| TokenizeError::Load { reason: format!("{}: {e}", path.display()) })?;
        Self::new(
            format!("tokenizer:{}", path.display()),
            tokenizer,
            ModeScheme::Prefix(spec.prefixes.clone()),
            spec.max_seq_len,
        )
    }

    /// Padding is left to the batcher; truncation to `max_len` keeps the closing special token.
    pub fn new(
        id: impl Into<String>,
        mut tokenizer: Tokenizer,
        scheme: ModeScheme,
        max_len: usize,
    ) -> Result<Self, TokenizeError> {
        let pad_id = tokenizer
            .get_padding()
            .map(|p| p.pad_id)
            .or_else(|| tokenizer.token_to_id("<pad>"))
            .or_else(|| tokenizer.token_to_id("[PAD]"))
            .ok_or_else(|| TokenizeError::Load { reason: "no padding token".into() })?;
        tokenizer.with_padding(None);
        tokenizer
            .with_truncation(Some(TruncationParams { max_length: max_len, ..Default::default() }))
            .map_err(|e| TokenizeError::Load { reason: e.to_string() })?;
        Ok(Self { key: ResourceKey::new(id), tokenizer, scheme, normalizer: None, pad_id })
    }

    pub fn with_normalizer(mut self, normalizer: Arc<dyn TextNormalizer>) -> Self {
        self.normalizer = Some(normalizer);
        self
    }

    pub fn pad_id(&self) -> u32 {
        self.pad_id
    }

    /// Unpadded id sequences in input order.
    #[tracing::instrument(level = "debug", skip_all, fields(?mode, n_texts = texts.len()))]
    pub fn encode<S: AsRef<str>>(
        &self,
        mode: EncodeMode,
        texts: &[S],
    ) -> Result<Vec<Vec<u32>>, TokenizeError> {
        if texts.is_empty() {
            return Err(TokenizeError::EmptyInput);
        }
        let inputs: Vec<String> = texts
            .iter()
            .map(|t| {
                let text = t.as_ref();
                match &self.normalizer {
                    Some(n) => self.scheme.prepare(mode, &n.normalize(text)),
                    None => self.scheme.prepare(mode, text),
                }
            })
            .collect();
        let encodings = self
            .tokenizer
            .encode_batch(inputs, true)
            .map_err(|e| TokenizeError::EncodeFailed { reason: e.to_string() })?;
        let seqs: Vec<Vec<u32>> = encodings
            .iter()
            .map(|e| self.scheme.finish(mode, e.get_ids().to_vec()))
            .collect();
        tracing::trace!(
            max_len = seqs.iter().map(Vec::len).max().unwrap_or(0),
            "encoded"
        );
        Ok(seqs)
    }

    /// Encodes and groups into padded, length-bucketed batches; `Batch::indices` maps rows back
    /// to the input order.
    #[tracing::instrument(level = "debug", skip_all, fields(?mode, n_texts = texts.len()))]
    pub fn encode_batches<S: AsRef<str>>(
        &self,
        mode: EncodeMode,
        texts: &[S],
        cfg: &BatchConfig,
    ) -> Result<Vec<Batch>, TokenizeError> {
        let batches = plan_batches(&self.encode(mode, texts)?, cfg);
        tracing::trace!(n_batches = batches.len(), "bucketed");
        Ok(batches)
    }
}

impl Resource for SharedTokenizer {
    fn key(&self) -> ResourceKey {
        self.key.clone()
    }

    fn init_cost_hint(&self) -> u64 {
        10
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::resident::memory::DType;

    const PAD: u32 = 1;
    const MASK: u32 = 4;
    const QUERY: u32 = 5;
    const PASSAGE: u32 = 6;
    const HELLO: u32 = 7;
    const WORLD: u32 = 8;

    fn special(id: u32, content: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id, "content": content, "single_word": false, "lstrip": false,
            "rstrip": false, "normalized": false, "special": true
        })
    }

    fn word_level() -> Tokenizer {
        let template = |ids: [u32; 2]| {
            serde_json::json!({
                "<s>": {"id": "<s>", "ids": [ids[0]], "tokens": ["<s>"]},
                "</s>": {"id": "</s>", "ids": [ids[1]], "tokens": ["</s>"]}
            })
        };
        let single = serde_json::json!([
            {"SpecialToken": {"id": "<s>", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}},
            {"SpecialToken": {"id": "</s>", "type_id": 0}}
        ]);
        let json = serde_json::json!({
            "version": "1.0", "truncation": null, "padding": null,
            "added_tokens": [special(0, "<s>"), special(1, "<pad>"), special(2, "</s>")],
            "normalizer": null,
            "pre_tokenizer": {"type": "WhitespaceSplit"},
            "post_processor": {
                "type": "TemplateProcessing", "single": single, "pair": single,
                "special_tokens": template([0, 2])
            },
            "decoder": null,
            "model": {
                "type": "WordLevel", "unk_token": "<unk>",
                "vocab": {
                    "<s>": 0, "<pad>": 1, "</s>": 2, "<unk>": 3, "<mask>": 4,
                    "query:": 5, "passage:": 6, "hello": 7, "world": 8
                }
            }
        });
        Tokenizer::from_bytes(json.to_string()).expect("fixture tokenizer")
    }

    fn e5_scheme() -> ModeScheme {
        ModeScheme::Prefix(crate::resident::model::Prefixes {
            query: "query: ".into(),
            passage: "passage: ".into(),
        })
    }

    fn colbert_scheme() -> ModeScheme {
        ModeScheme::Colbert(ColbertScheme {
            query_marker: "query: ".into(),
            doc_marker: "passage: ".into(),
            query_maxlen: 8,
            doc_maxlen: 4,
            mask_token_id: MASK,
        })
    }

    fn shared(scheme: ModeScheme, max_len: usize) -> SharedTokenizer {
        SharedTokenizer::new("tokenizer:test", word_level(), scheme, max_len).unwrap()
    }

    fn cfg(budget: usize) -> BatchConfig {
        BatchConfig { token_budget: budget, max_seq_len: 512, pad_token_id: PAD }
    }

    #[test]
    fn query_and_passage_prefixes_are_prepended() {
        let t = shared(e5_scheme(), 512);
        let q = t.encode(EncodeMode::Query, &["hello world"]).unwrap();
        let p = t.encode(EncodeMode::Passage, &["hello world"]).unwrap();
        assert_eq!(q[0], vec![0, QUERY, HELLO, WORLD, 2]);
        assert_eq!(p[0], vec![0, PASSAGE, HELLO, WORLD, 2]);
    }

    #[test]
    fn truncation_keeps_closing_special_token() {
        let t = shared(e5_scheme(), 4);
        let ids = t.encode(EncodeMode::Query, &["hello world hello"]).unwrap();
        assert_eq!(ids[0].len(), 4);
        assert_eq!((ids[0][0], ids[0][3]), (0, 2));
    }

    #[test]
    fn empty_input_is_rejected() {
        let t = shared(e5_scheme(), 512);
        let none: [&str; 0] = [];
        assert!(matches!(t.encode(EncodeMode::Query, &none), Err(TokenizeError::EmptyInput)));
    }

    #[test]
    fn batches_pad_to_bucket_with_mask_and_map_back_to_inputs() {
        let t = shared(e5_scheme(), 512);
        let texts = ["hello world hello world", "hello"];
        let batches = t.encode_batches(EncodeMode::Passage, &texts, &cfg(10_000)).unwrap();
        assert_eq!(batches.len(), 1);
        let b = &batches[0];
        assert_eq!(b.seq_len, 32);
        assert_eq!(b.indices, vec![1, 0]);
        assert_eq!(b.ids.len(), 2 * 32);
        assert_eq!(&b.ids[..4], &[0, PASSAGE, HELLO, 2]);
        assert_eq!(&b.mask[..5], &[1, 1, 1, 1, 0]);
        assert_eq!(b.ids[4], PAD);
        assert_eq!(b.mask[32..32 + 7].iter().sum::<u32>(), 7);
    }

    #[test]
    fn colbert_query_is_padded_with_mask_tokens_and_passage_is_cut() {
        let t = shared(colbert_scheme(), 512);
        let q = t.encode(EncodeMode::Query, &["hello"]).unwrap();
        assert_eq!(q[0], vec![0, QUERY, HELLO, 2, MASK, MASK, MASK, MASK]);
        let p = t.encode(EncodeMode::Passage, &["hello world hello world"]).unwrap();
        assert_eq!(p[0], vec![0, PASSAGE, HELLO, WORLD]);
    }

    #[test]
    fn normalizer_runs_before_prefixing() {
        struct Lowercase;
        impl TextNormalizer for Lowercase {
            fn normalize<'a>(&self, text: &'a str) -> std::borrow::Cow<'a, str> {
                text.to_lowercase().into()
            }
        }
        let t = shared(e5_scheme(), 512).with_normalizer(Arc::new(Lowercase));
        let ids = t.encode(EncodeMode::Query, &["HELLO"]).unwrap();
        assert_eq!(ids[0], vec![0, QUERY, HELLO, 2]);
    }

    #[test]
    fn resource_key_is_stable() {
        assert_eq!(shared(e5_scheme(), 512).key(), ResourceKey::new("tokenizer:test"));
    }

    #[test]
    fn e5_small_checkpoint_tokenizes_with_special_tokens() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("model/multilingual-e5-small");
        if !dir.join("tokenizer.json").exists() {
            return;
        }
        let t = SharedTokenizer::from_spec(&ModelSpec::e5_small(&dir, DType::F32)).unwrap();
        let q = t.encode(EncodeMode::Query, &["hello world"]).unwrap();
        let p = t.encode(EncodeMode::Passage, &["hello world"]).unwrap();
        assert_eq!((q[0][0], *q[0].last().unwrap()), (0, 2));
        assert_ne!(q[0], p[0]);
        assert_eq!(t.pad_id(), 1);
    }
}
