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
#[path = "test/mod.rs"]
mod tests;
