use std::fmt;
use log::{debug, trace};
use crate::resident::l130::l120::l110::l100::l090::l080::a::encoding_schemata::{
    EncodingError as SchemaError, EncodingSchema,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::text_normalization::{
    NormalizeError, TextPreprocessor,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::tokenizer_loader::{
    EmbedMode, LoadedTokenizer,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::batching::{
    build_batches, Batch, BatchConfig, BatchError,
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextEncodingError {
    NoInputTexts,
    Normalize(NormalizeError),
    Schema(SchemaError),
    Batch(BatchError),
}
impl fmt::Display for TextEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextEncodingError::NoInputTexts => write!(f, "keine Eingabetexte"),
            TextEncodingError::Normalize(e) => {
                write!(f, "Normalisierung fehlgeschlagen: {}", e.0)
            }
            TextEncodingError::Schema(e) => write!(f, "{e}"),
            TextEncodingError::Batch(e) => {
                write!(f, "Batch-Bildung fehlgeschlagen: {e:?}")
            }
        }
    }
}
impl std::error::Error for TextEncodingError {}
/// Normalizes and encodes all texts with `schema`. `result[i]` belongs to `texts[i]`.
/// The attention mask of every sequence is all ones, so the ids alone are sufficient
/// input for `build_batches`.
pub fn encode_texts<S: AsRef<str>>(
    tokenizer: &LoadedTokenizer,
    preprocessor: &TextPreprocessor,
    schema: &EncodingSchema,
    texts: &[S],
    mode: EmbedMode,
) -> Result<Vec<Vec<u32>>, TextEncodingError> {
    if texts.is_empty() {
        return Err(TextEncodingError::NoInputTexts);
    }
    schema.validate().map_err(TextEncodingError::Schema)?;
    let mut ids: Vec<Vec<u32>> = Vec::with_capacity(texts.len());
    for text in texts {
        let normalized = preprocessor
            .apply(text.as_ref())
            .map_err(TextEncodingError::Normalize)?;
        let sequence = schema
            .encode(tokenizer, &normalized, mode)
            .map_err(TextEncodingError::Schema)?;
        ids.push(sequence.ids);
    }
    debug!("encoded texts: mode={:?} texts={}", mode, texts.len());
    trace!("longest sequence: {}", ids.iter().map(Vec::len).max().unwrap_or(0));
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
) -> Result<Vec<Batch>, TextEncodingError> {
    let ids = encode_texts(tokenizer, preprocessor, schema, texts, mode)?;
    let batches = build_batches(&ids, batch_config).map_err(TextEncodingError::Batch)?;
    debug!("built batches: count={}", batches.len());
    Ok(batches)
}
const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TextEncodingError>();
};
#[cfg(test)]
#[path = "test/text_encoding.rs"]
mod tests;
