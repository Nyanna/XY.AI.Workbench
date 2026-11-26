use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokenizers::normalizers::{
    Lowercase, NormalizerWrapper, Sequence, StripAccents, NFC, NFD, NFKC, NFKD,
};
use tokenizers::{Tokenizer, TruncationDirection, TruncationParams, TruncationStrategy};
use super::text_normalization::{ResolvedNormalization, UnicodeForm};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::resource_lifecycle::ResourceLoader;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::ModelDescriptor;
pub use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::tokenizer::TOKENIZER_FILE_NAME;
const PAD_TOKEN_CANDIDATES: [&str; 2] = ["<pad>", "[PAD]"];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbedMode {
    Query,
    Passage,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenizerSchema {
    pub query_prefix: String,
    pub passage_prefix: String,
}
impl TokenizerSchema {
    pub fn prefix(&self, mode: EmbedMode) -> &str {
        match mode {
            EmbedMode::Query => &self.query_prefix,
            EmbedMode::Passage => &self.passage_prefix,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizerLoadError {
    Load { path: PathBuf, cause: String },
    NoPadToken { id: String },
    Truncation { id: String, cause: String },
    Normalization { id: String, cause: String },
}
impl fmt::Display for TokenizerLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load { path, cause } => {
                write!(
                    f, "Tokenizer konnte nicht geladen werden ({}): {}", path.display(),
                    cause
                )
            }
            Self::NoPadToken { id } => {
                write!(f, "kein Pad-Token gefunden (Tokenizer {id})")
            }
            Self::Truncation { id, cause } => {
                write!(f, "Trunkierung abgelehnt (Tokenizer {id}): {cause}")
            }
            Self::Normalization { id, cause } => {
                write!(f, "Normalisierung abgelehnt (Tokenizer {id}): {cause}")
            }
        }
    }
}
impl std::error::Error for TokenizerLoadError {}
/// Ready tokenizer: padding off, truncation set, pad token known. Immutable after construction.
pub struct LoadedTokenizer {
    tokenizer: Tokenizer,
    id: String,
    schema: TokenizerSchema,
    max_seq_len: usize,
    pad_token: String,
    pad_token_id: u32,
    normalization: ResolvedNormalization,
}
impl fmt::Debug for LoadedTokenizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoadedTokenizer")
            .field("id", &self.id)
            .field("schema", &self.schema)
            .field("max_seq_len", &self.max_seq_len)
            .field("pad_token", &self.pad_token)
            .field("pad_token_id", &self.pad_token_id)
            .field("normalization", &self.normalization)
            .finish()
    }
}
fn descriptor_tokenizer_path(descriptor: &ModelDescriptor) -> PathBuf {
    let dir = descriptor.config_path.parent().unwrap_or_else(|| Path::new("."));
    dir.join(TOKENIZER_FILE_NAME)
}
fn path_id(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
fn read_tokenizer(path: &Path) -> Result<Tokenizer, TokenizerLoadError> {
    Tokenizer::from_file(path)
        .map_err(|e| TokenizerLoadError::Load {
            path: path.to_path_buf(),
            cause: e.to_string(),
        })
}
fn find_pad_token(tokenizer: &Tokenizer) -> Option<(String, u32)> {
    if let Some(padding) = tokenizer.get_padding() {
        return Some((padding.pad_token.clone(), padding.pad_id));
    }
    PAD_TOKEN_CANDIDATES
        .iter()
        .find_map(|token| {
            tokenizer.token_to_id(token).map(|id| ((*token).to_string(), id))
        })
}
fn extension_steps(n: ResolvedNormalization) -> Vec<NormalizerWrapper> {
    let mut steps: Vec<NormalizerWrapper> = Vec::new();
    if n.strip_accents {
        match n.unicode_form {
            UnicodeForm::Nfkc | UnicodeForm::Nfkd => steps.push(NFKD.into()),
            _ => steps.push(NFD.into()),
        }
        steps.push(StripAccents.into());
    }
    if n.lowercase {
        steps.push(Lowercase.into());
    }
    match n.unicode_form {
        UnicodeForm::None => {}
        UnicodeForm::Nfc => steps.push(NFC.into()),
        UnicodeForm::Nfd => steps.push(NFD.into()),
        UnicodeForm::Nfkc => steps.push(NFKC.into()),
        UnicodeForm::Nfkd => steps.push(NFKD.into()),
    }
    steps
}
fn extend_normalizer(
    tokenizer: &mut Tokenizer,
    id: &str,
    n: ResolvedNormalization,
) -> Result<(), TokenizerLoadError> {
    if n == ResolvedNormalization::default() {
        return Ok(());
    }
    let mut steps = extension_steps(n);
    if let Some(existing) = tokenizer.get_normalizer() {
        steps.push(existing.clone());
    }
    tokenizer
        .with_normalizer(Some(Sequence::new(steps)))
        .map(|_| ())
        .map_err(|e| TokenizerLoadError::Normalization {
            id: id.to_string(),
            cause: e.to_string(),
        })
}
impl LoadedTokenizer {
    pub fn from_descriptor(
        descriptor: &ModelDescriptor,
    ) -> Result<Self, TokenizerLoadError> {
        Self::from_file(
            &descriptor_tokenizer_path(descriptor),
            schema_of(descriptor),
            descriptor.max_seq_len,
        )
    }
    pub fn from_file(
        path: &Path,
        schema: TokenizerSchema,
        max_seq_len: usize,
    ) -> Result<Self, TokenizerLoadError> {
        Self::assemble(read_tokenizer(path)?, path_id(path), schema, max_seq_len, None)
    }
    pub fn from_tokenizer(
        tokenizer: Tokenizer,
        id: impl Into<String>,
        schema: TokenizerSchema,
        max_seq_len: usize,
    ) -> Result<Self, TokenizerLoadError> {
        Self::assemble(tokenizer, id.into(), schema, max_seq_len, None)
    }
    fn assemble(
        mut tokenizer: Tokenizer,
        id: String,
        schema: TokenizerSchema,
        max_seq_len: usize,
        normalization: Option<ResolvedNormalization>,
    ) -> Result<Self, TokenizerLoadError> {
        let normalization = normalization.unwrap_or_default();
        extend_normalizer(&mut tokenizer, &id, normalization)?;
        let (pad_token, pad_token_id) = find_pad_token(&tokenizer)
            .ok_or_else(|| TokenizerLoadError::NoPadToken {
                id: id.clone(),
            })?;
        tokenizer.with_padding(None);
        if max_seq_len == 0 {
            return Err(TokenizerLoadError::Truncation {
                id,
                cause: "max_seq_len must be greater than zero".to_string(),
            });
        }
        tokenizer
            .with_truncation(
                Some(TruncationParams {
                    max_length: max_seq_len,
                    strategy: TruncationStrategy::LongestFirst,
                    direction: TruncationDirection::Right,
                    stride: 0,
                }),
            )
            .map_err(|e| TokenizerLoadError::Truncation {
                id: id.clone(),
                cause: e.to_string(),
            })?;
        Ok(Self {
            tokenizer,
            id,
            schema,
            max_seq_len,
            pad_token,
            pad_token_id,
            normalization,
        })
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn schema(&self) -> &TokenizerSchema {
        &self.schema
    }
    pub fn max_seq_len(&self) -> usize {
        self.max_seq_len
    }
    pub fn pad_token(&self) -> &str {
        &self.pad_token
    }
    pub fn pad_token_id(&self) -> u32 {
        self.pad_token_id
    }
    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }
    /// Extends the tokenizer's normalizer with the configured steps; consumes self so it runs before sharing.
    pub fn with_normalization(
        mut self,
        n: ResolvedNormalization,
    ) -> Result<Self, TokenizerLoadError> {
        extend_normalizer(&mut self.tokenizer, &self.id, n)?;
        self.normalization = n;
        Ok(self)
    }
    /// Config applied to the tokenizer's own normalizer (already reduced by the external normalizer's steps).
    pub fn normalization(&self) -> ResolvedNormalization {
        self.normalization
    }
}
fn schema_of(descriptor: &ModelDescriptor) -> TokenizerSchema {
    TokenizerSchema {
        query_prefix: descriptor.query_prefix.clone(),
        passage_prefix: descriptor.passage_prefix.clone(),
    }
}
enum TokenizerSource {
    File(PathBuf),
    Loaded { tokenizer: Tokenizer, id: String },
}
/// Lazy loader for `ResourceRegistry<LoadedTokenizer>`. Construction does no I/O.
pub struct TokenizerLoader {
    source: TokenizerSource,
    schema: TokenizerSchema,
    max_seq_len: usize,
    normalization: Option<Arc<dyn ResourceLoader<ResolvedNormalization>>>,
}
impl TokenizerLoader {
    fn new(
        source: TokenizerSource,
        schema: TokenizerSchema,
        max_seq_len: usize,
    ) -> Self {
        Self {
            source,
            schema,
            max_seq_len,
            normalization: None,
        }
    }
    /// Config source for the tokenizer's own normalizer; the external `TextNormalizer` is not involved here.
    pub fn with_normalization(
        mut self,
        source: Arc<dyn ResourceLoader<ResolvedNormalization>>,
    ) -> Self {
        self.normalization = Some(source);
        self
    }
    pub fn from_descriptor(descriptor: &ModelDescriptor) -> Self {
        Self::from_file(
            descriptor_tokenizer_path(descriptor),
            schema_of(descriptor),
            descriptor.max_seq_len,
        )
    }
    pub fn from_file(
        path: PathBuf,
        schema: TokenizerSchema,
        max_seq_len: usize,
    ) -> Self {
        Self::new(TokenizerSource::File(path), schema, max_seq_len)
    }
    pub fn from_tokenizer(
        tokenizer: Tokenizer,
        id: String,
        schema: TokenizerSchema,
        max_seq_len: usize,
    ) -> Self {
        Self::new(
            TokenizerSource::Loaded {
                tokenizer,
                id,
            },
            schema,
            max_seq_len,
        )
    }
    fn build(&self) -> Result<LoadedTokenizer, TokenizerLoadError> {
        let (tokenizer, id) = match &self.source {
            TokenizerSource::File(path) => (read_tokenizer(path)?, path_id(path)),
            TokenizerSource::Loaded { tokenizer, id } => (tokenizer.clone(), id.clone()),
        };
        let normalization = match &self.normalization {
            Some(source) => {
                Some(
                    source
                        .load()
                        .map_err(|cause| TokenizerLoadError::Normalization {
                            id: id.clone(),
                            cause,
                        })?,
                )
            }
            None => None,
        };
        LoadedTokenizer::assemble(
            tokenizer,
            id,
            self.schema.clone(),
            self.max_seq_len,
            normalization,
        )
    }
}
impl ResourceLoader<LoadedTokenizer> for TokenizerLoader {
    fn load(&self) -> Result<LoadedTokenizer, String> {
        self.build().map_err(|e| e.to_string())
    }
}
const _: fn() = || {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<LoadedTokenizer>();
    assert_send_sync::<TokenizerLoader>();
};
#[cfg(test)]
#[path = "test/tokenizer_loader.rs"]
mod tests;
