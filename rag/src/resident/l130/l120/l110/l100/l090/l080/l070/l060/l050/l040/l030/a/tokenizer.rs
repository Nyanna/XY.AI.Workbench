use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokenizers::{PaddingParams, TruncationDirection, TruncationParams, TruncationStrategy};

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::ModelDescriptor;

pub const TOKENIZER_FILE_NAME: &str = "tokenizer.json";

pub type SharedTokenizer = Arc<Tokenizer>;

#[derive(Debug)]
pub enum TokenizerError {
    Load { path: PathBuf, reason: String },
    Encode { reason: String },
    InvalidConfig { reason: String },
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenizerError::Load { path, reason } => write!(
                f,
                "Tokenizer-Datei nicht ladbar ({}): {}",
                path.display(),
                reason
            ),
            TokenizerError::Encode { reason } => {
                write!(f, "Tokenisierung fehlgeschlagen: {}", reason)
            }
            TokenizerError::InvalidConfig { reason } => {
                write!(f, "ungültige Tokenizer-Konfiguration: {}", reason)
            }
        }
    }
}

impl std::error::Error for TokenizerError {}

/// Immutable after loading.
pub struct Tokenizer {
    inner: tokenizers::Tokenizer,
    max_len: usize,
}

impl Tokenizer {
    pub fn from_descriptor(descriptor: &ModelDescriptor) -> Result<Tokenizer, TokenizerError> {
        if descriptor.max_seq_len == 0 {
            return Err(TokenizerError::InvalidConfig {
                reason: "max_seq_len must be greater than zero".to_string(),
            });
        }
        let path = match descriptor.config_path.parent() {
            Some(dir) => dir.join(TOKENIZER_FILE_NAME),
            None => PathBuf::from(TOKENIZER_FILE_NAME),
        };
        Self::from_file(&path, descriptor.max_seq_len)
    }

    pub fn shared_from_descriptor(
        descriptor: &ModelDescriptor,
    ) -> Result<SharedTokenizer, TokenizerError> {
        Self::from_descriptor(descriptor).map(Arc::new)
    }

    pub fn from_file(path: &Path, max_len: usize) -> Result<Tokenizer, TokenizerError> {
        if max_len == 0 {
            return Err(TokenizerError::InvalidConfig {
                reason: "max_len must be greater than zero".to_string(),
            });
        }
        let load_err = |reason: String| TokenizerError::Load {
            path: path.to_path_buf(),
            reason,
        };
        let mut inner = tokenizers::Tokenizer::from_file(path).map_err(|e| load_err(e.to_string()))?;
        inner
            .with_truncation(Some(TruncationParams {
                max_length: max_len,
                strategy: TruncationStrategy::LongestFirst,
                stride: 0,
                direction: TruncationDirection::Right,
            }))
            .map_err(|e| load_err(e.to_string()))?;
        inner.with_padding(None::<PaddingParams>);
        Ok(Tokenizer { inner, max_len })
    }

    pub fn max_len(&self) -> usize {
        self.max_len
    }

    pub fn vocab_size(&self) -> usize {
        self.inner.get_vocab_size(true)
    }

    pub fn encode(&self, text: &str) -> Result<Vec<u32>, TokenizerError> {
        let encoding = self
            .inner
            .encode(text, true)
            .map_err(|e| TokenizerError::Encode {
                reason: e.to_string(),
            })?;
        let mut ids = encoding.get_ids().to_vec();
        ids.truncate(self.max_len);
        Ok(ids)
    }

    pub fn encode_batch(&self, texts: &[&str]) -> Result<Vec<Vec<u32>>, TokenizerError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        let encodings = self
            .inner
            .encode_batch(texts.to_vec(), true)
            .map_err(|e| TokenizerError::Encode {
                reason: e.to_string(),
            })?;
        Ok(encodings
            .into_iter()
            .map(|e| {
                let mut ids = e.get_ids().to_vec();
                ids.truncate(self.max_len);
                ids
            })
            .collect())
    }
}

const _: fn() = || {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<Tokenizer>();
};

#[cfg(test)]
#[path = "test/tokenizer.rs"]
mod tests;
