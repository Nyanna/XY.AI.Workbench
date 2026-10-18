use thiserror::Error;

use crate::resident::error::IsRetryable;

/// Tokenizer loading and encoding errors.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum TokenizeError {
    #[error("tokenizer load failed: {reason}")]
    Load { reason: String },
    #[error("tokenization failed: {reason}")]
    EncodeFailed { reason: String },
    #[error("no input texts")]
    EmptyInput,
}

impl IsRetryable for TokenizeError {
    fn is_retryable(&self) -> bool {
        false
    }
}
