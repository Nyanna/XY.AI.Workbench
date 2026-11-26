use std::fmt;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::tokenizer_loader::{
    EmbedMode, LoadedTokenizer,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::MultiVectorConfig;
/// Multi-vector parameters per model variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiVectorSchema {
    pub query_marker: String,
    pub document_marker: String,
    pub max_query_len: usize,
    pub max_document_len: usize,
    pub mask_token_id: u32,
}
impl MultiVectorSchema {
    /// Copies markers and lengths unchanged from the descriptor config.
    pub fn from_config(cfg: &MultiVectorConfig, mask_token_id: u32) -> Self {
        Self {
            query_marker: cfg.query_marker.clone(),
            document_marker: cfg.document_marker.clone(),
            max_query_len: cfg.max_query_len,
            max_document_len: cfg.max_document_len,
            mask_token_id,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodingSchema {
    /// Prefix from `LoadedTokenizer::schema().prefix(mode)`.
    Prefix,
    MultiVector(MultiVectorSchema),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedSequence {
    pub ids: Vec<u32>,
    /// Same length as `ids`; every entry is 1 (padded query positions included).
    pub attention_mask: Vec<u32>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodingError {
    InvalidSchema { reason: String },
    Tokenize { cause: String },
}
impl fmt::Display for EncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodingError::InvalidSchema { reason } => {
                write!(f, "ungültiges Encoding-Schema: {reason}")
            }
            EncodingError::Tokenize { cause } => {
                write!(f, "Tokenisierung fehlgeschlagen: {cause}")
            }
        }
    }
}
impl std::error::Error for EncodingError {}
/// Keeps the first `n` ids. A special last token is preserved as the final element.
fn cut(mut ids: Vec<u32>, special: &[u32], n: usize) -> Vec<u32> {
    if ids.len() <= n {
        return ids;
    }
    let last_is_special = n >= 1 && special.last() == Some(&1);
    let last = ids[ids.len() - 1];
    ids.truncate(n);
    if last_is_special {
        ids[n - 1] = last;
    }
    ids
}
impl EncodingSchema {
    /// Checks the multi-vector schema: both lengths must be greater than zero.
    pub fn validate(&self) -> Result<(), EncodingError> {
        if let EncodingSchema::MultiVector(mv) = self {
            if mv.max_query_len == 0 {
                return Err(EncodingError::InvalidSchema {
                    reason: "max_query_len must be greater than zero".to_string(),
                });
            }
            if mv.max_document_len == 0 {
                return Err(EncodingError::InvalidSchema {
                    reason: "max_document_len must be greater than zero".to_string(),
                });
            }
        }
        Ok(())
    }
    /// Encodes one text. Calls `validate()` first.
    pub fn encode(
        &self,
        tokenizer: &LoadedTokenizer,
        text: &str,
        mode: EmbedMode,
    ) -> Result<EncodedSequence, EncodingError> {
        self.validate()?;
        let lead = match self {
            EncodingSchema::Prefix => tokenizer.schema().prefix(mode).to_string(),
            EncodingSchema::MultiVector(mv) => {
                let marker = match mode {
                    EmbedMode::Query => &mv.query_marker,
                    EmbedMode::Passage => &mv.document_marker,
                };
                if marker.is_empty() { String::new() } else { format!("{marker} ") }
            }
        };
        let encoding = tokenizer
            .tokenizer()
            .encode(format!("{lead}{text}"), true)
            .map_err(|e| EncodingError::Tokenize {
                cause: e.to_string(),
            })?;
        let ids = encoding.get_ids();
        let special = encoding.get_special_tokens_mask();
        let ids = match self {
            EncodingSchema::Prefix => ids.to_vec(),
            EncodingSchema::MultiVector(mv) => {
                match mode {
                    EmbedMode::Query => {
                        let mut out = cut(ids.to_vec(), special, mv.max_query_len);
                        out.resize(mv.max_query_len, mv.mask_token_id);
                        out
                    }
                    EmbedMode::Passage => {
                        let pad = tokenizer.pad_token_id();
                        let (kept, kept_special): (Vec<u32>, Vec<u32>) = ids
                            .iter()
                            .zip(special.iter())
                            .filter(|(id, _)| **id != pad)
                            .map(|(id, s)| (*id, *s))
                            .unzip();
                        cut(kept, &kept_special, mv.max_document_len)
                    }
                }
            }
        };
        let attention_mask = vec![1; ids.len()];
        Ok(EncodedSequence {
            ids,
            attention_mask,
        })
    }
}
#[cfg(test)]
#[path = "test/encoding_schemata.rs"]
mod tests;
