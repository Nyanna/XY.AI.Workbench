use std::fmt;

use log::{debug, trace};

use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::text_normalization::{
    NormalizeError, TextPreprocessor,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::tokenizer_loader::{
    EmbedMode, LoadedTokenizer,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::batching::{
    build_batches, Batch, BatchConfig, BatchError,
};

/// Schema-dependent handling. Chosen by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodingSchema {
    /// The text gets `tokenizer.schema().prefix(mode)` prepended. No post-processing.
    SingleVector,
    /// The text gets the marker of the mode prepended. Query sequences are
    /// post-padded with `pad_token_id` to `query_len` tokens; passages are not padded.
    MultiVector {
        query_marker: String,
        passage_marker: String,
        query_len: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodingError {
    NoInputTexts,
    Normalize(NormalizeError),
    Tokenization { cause: String },
    Batch(BatchError),
}

impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodingError::NoInputTexts => write!(f, "keine Eingabetexte"),
            EncodingError::Normalize(e) => {
                write!(f, "Normalisierung fehlgeschlagen: {}", e.0)
            }
            EncodingError::Tokenization { cause } => {
                write!(f, "Tokenisierung fehlgeschlagen: {cause}")
            }
            EncodingError::Batch(e) => write!(f, "Batch-Bildung fehlgeschlagen: {e:?}"),
        }
    }
}

impl std::error::Error for EncodingError {}

fn prefix_for<'a>(
    tokenizer: &'a LoadedTokenizer,
    schema: &'a EncodingSchema,
    mode: EmbedMode,
) -> &'a str {
    match schema {
        EncodingSchema::SingleVector => tokenizer.schema().prefix(mode),
        EncodingSchema::MultiVector {
            query_marker,
            passage_marker,
            ..
        } => match mode {
            EmbedMode::Query => query_marker,
            EmbedMode::Passage => passage_marker,
        },
    }
}

/// Encodes all texts as one batch. `result[i]` belongs to `texts[i]`.
pub fn encode_texts<S: AsRef<str>>(
    tokenizer: &LoadedTokenizer,
    preprocessor: &TextPreprocessor,
    schema: &EncodingSchema,
    texts: &[S],
    mode: EmbedMode,
) -> Result<Vec<Vec<u32>>, EncodingError> {
    if texts.is_empty() {
        return Err(EncodingError::NoInputTexts);
    }
    let prefix = prefix_for(tokenizer, schema, mode);
    let mut prepared: Vec<String> = Vec::with_capacity(texts.len());
    for text in texts {
        let normalized = preprocessor
            .apply(text.as_ref())
            .map_err(EncodingError::Normalize)?;
        let mut full = String::with_capacity(prefix.len() + normalized.len());
        full.push_str(prefix);
        full.push_str(&normalized);
        prepared.push(full);
    }
    let encodings = tokenizer
        .tokenizer()
        .encode_batch(prepared, true)
        .map_err(|e| EncodingError::Tokenization {
            cause: e.to_string(),
        })?;
    let truncated = encodings
        .iter()
        .filter(|e| !e.get_overflowing().is_empty())
        .count();
    let mut ids: Vec<Vec<u32>> = encodings.iter().map(|e| e.get_ids().to_vec()).collect();
    if let (EncodingSchema::MultiVector { query_len, .. }, EmbedMode::Query) = (schema, mode) {
        let pad = tokenizer.pad_token_id();
        for seq in &mut ids {
            if seq.len() < *query_len {
                seq.resize(*query_len, pad);
            }
        }
    }
    debug!(
        "encoded texts: truncated={} mode={:?} texts={}",
        truncated,
        mode,
        texts.len()
    );
    trace!(
        "longest sequence: {}",
        ids.iter().map(Vec::len).max().unwrap_or(0)
    );
    Ok(ids)
}

/// `encode_texts`, then `build_batches`. Rows map to input indices via `Batch::positions()`.
pub fn encode_and_batch<S: AsRef<str>>(
    tokenizer: &LoadedTokenizer,
    preprocessor: &TextPreprocessor,
    schema: &EncodingSchema,
    texts: &[S],
    mode: EmbedMode,
    batch_config: &BatchConfig,
) -> Result<Vec<Batch>, EncodingError> {
    let ids = encode_texts(tokenizer, preprocessor, schema, texts, mode)?;
    let batches = build_batches(&ids, batch_config).map_err(EncodingError::Batch)?;
    debug!("built batches: count={}", batches.len());
    Ok(batches)
}

const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<EncodingSchema>();
    assert_send_sync::<EncodingError>();
};

#[cfg(test)]
#[path = "test/text_encoding.rs"]
mod tests;
