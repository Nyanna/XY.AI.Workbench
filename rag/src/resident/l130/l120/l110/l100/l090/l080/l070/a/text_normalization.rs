use std::borrow::Cow;
use std::sync::Arc;

use log::{debug, warn};

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::a::resource_lifecycle::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnicodeForm {
    #[default]
    None,
    Nfc,
    Nfd,
    Nfkc,
    Nfkd,
}

/// Set of normalization steps. Used both as the parameters for the tokenizer
/// normalizer and as the steps an external normalizer declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NormalizationParams {
    pub unicode_form: UnicodeForm,
    pub lowercase: bool,
    pub strip_accents: bool,
}

/// Config for the tokenizer's own normalizer. All steps are applied by the
/// tokenizer, never by this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResolvedNormalization {
    pub unicode_form: UnicodeForm,
    pub lowercase: bool,
    pub strip_accents: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizeError(pub String);

impl std::fmt::Display for NormalizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "normalization failed: {}", self.0)
    }
}

impl std::error::Error for NormalizeError {}

/// External text normalizer. Swappable and usable concurrently.
pub trait TextNormalizer: Send + Sync {
    fn normalize<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError>;

    /// Steps this normalizer performs itself (default: none).
    fn performed_steps(&self) -> NormalizationParams {
        NormalizationParams::default()
    }
}

/// Returns the input unchanged.
#[derive(Debug, Clone, Copy, Default)]
pub struct IdentityNormalizer;

impl TextNormalizer for IdentityNormalizer {
    fn normalize<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError> {
        Ok(Cow::Borrowed(text))
    }
}

#[derive(Clone)]
pub struct TextPreprocessor {
    external: Option<Arc<dyn TextNormalizer>>,
}

impl TextPreprocessor {
    pub fn new(external: Option<Arc<dyn TextNormalizer>>) -> Self {
        Self { external }
    }

    pub fn apply<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError> {
        match &self.external {
            None => Ok(Cow::Borrowed(text)),
            Some(normalizer) => normalizer.normalize(text).inspect_err(|error| {
                warn!("external normalizer failed: {}", error.0);
            }),
        }
    }

    pub fn resolve(&self, params: &NormalizationParams) -> ResolvedNormalization {
        let performed = match &self.external {
            Some(normalizer) => normalizer.performed_steps(),
            None => NormalizationParams::default(),
        };
        ResolvedNormalization {
            unicode_form: if performed.unicode_form != UnicodeForm::None {
                UnicodeForm::None
            } else {
                params.unicode_form
            },
            lowercase: params.lowercase && !performed.lowercase,
            strip_accents: params.strip_accents && !performed.strip_accents,
        }
    }
}

/// Lazy loader of the tokenizer-normalizer config. CPU only, no I/O.
pub struct NormalizerLoader {
    params: NormalizationParams,
    preprocessor: TextPreprocessor,
}

impl NormalizerLoader {
    pub fn new(params: NormalizationParams, preprocessor: TextPreprocessor) -> Self {
        Self {
            params,
            preprocessor,
        }
    }
}

impl ResourceLoader<ResolvedNormalization> for NormalizerLoader {
    fn load(&self) -> Result<ResolvedNormalization, String> {
        let resolved = self.preprocessor.resolve(&self.params);
        debug!("resolved tokenizer normalization: {:?}", resolved);
        Ok(resolved)
    }
}

const _: fn() = || {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TextPreprocessor>();
    assert_send_sync::<NormalizerLoader>();
    assert_send_sync::<ResolvedNormalization>();
};

#[cfg(test)]
#[path = "test/text_normalization.rs"]
mod tests;
