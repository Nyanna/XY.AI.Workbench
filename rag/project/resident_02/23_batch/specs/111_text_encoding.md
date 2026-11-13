# Spec: Text-Encoding (module `text_encoding`, build order 111)

## 1. Task, restrictions and boundaries
Implement the following in Rust:
- Encode a list of texts in one mode (Query or Passage) as a batch into token-ID sequences. The output order equals the input order.
- A combined function that encodes and then splits the result into padded batches via `build_batches`. It passes encoding errors through unchanged and logs the batch count.

Restrictions:
- Stateless free functions. No `unsafe`, no threads, no global state, no I/O. Logging uses the `log` crate.
- Do not load tokenizers, do not implement normalization or padding logic, and do not pool or run inference. Use the dependency APIs as specified below.
- Do not add behavior beyond this spec.

## 2. Context
- `tokenizer_loader` provides `LoadedTokenizer`. It holds the tokenizer, with padding off and truncation set to `max_seq_len` (special tokens included), plus `pad_token_id`, the prefix schema and `max_seq_len`.
- `text_normalization` provides `TextPreprocessor::apply`. It is the external normalizer. The tokenizer runs its own normalization.
- `batching` (Batch-Bildung) provides `build_batches(&[Vec<u32>], &BatchConfig)`. It does the sorting by length, the padding and the `positions` mapping.
- The caller supplies the `EncodingSchema` (below), the `BatchConfig` and the loaded dependencies.

## 3. Interface
```rust
use crate::tokenizer_loader::{LoadedTokenizer, EmbedMode};
use crate::text_normalization::{TextPreprocessor, NormalizeError};
use crate::batching::{Batch, BatchConfig, BatchError, build_batches};

/// Schema-dependent handling. Chosen by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodingSchema {
    /// Single-vector: the text gets `tokenizer.schema().prefix(mode)` prepended. No post-processing.
    SingleVector,
    /// Multi-vector: the text gets the marker of the mode prepended.
    /// Query sequences are post-padded with `pad_token_id` to `query_len` tokens (fixed query padding).
    /// Passage sequences are not padded.
    MultiVector { query_marker: String, passage_marker: String, query_len: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodingError {
    /// Empty list. Display: "keine Eingabetexte"
    NoInputTexts,
    /// External normalizer failed. Display: "Normalisierung fehlgeschlagen: {0.0}"
    Normalize(NormalizeError),
    /// Tokenization failed. Display: "Tokenisierung fehlgeschlagen: {cause}"
    Tokenization { cause: String },
    /// Batch config rejected by build_batches. Display: "Batch-Bildung fehlgeschlagen: {0:?}"
    Batch(BatchError),
}
impl std::fmt::Display for EncodingError { /* as above */ }
impl std::error::Error for EncodingError {}

/// Encode all texts as one batch. Result[i] belongs to texts[i].
pub fn encode_texts<S: AsRef<str>>(
    tokenizer: &LoadedTokenizer,
    preprocessor: &TextPreprocessor,
    schema: &EncodingSchema,
    texts: &[S],
    mode: EmbedMode,
) -> Result<Vec<Vec<u32>>, EncodingError>;

/// encode_texts, then build_batches(&ids, batch_config).
/// Every Batch maps its rows to input indices via Batch::positions().
pub fn encode_and_batch<S: AsRef<str>>(
    tokenizer: &LoadedTokenizer,
    preprocessor: &TextPreprocessor,
    schema: &EncodingSchema,
    texts: &[S],
    mode: EmbedMode,
    batch_config: &BatchConfig,
) -> Result<Vec<Batch>, EncodingError>;
```
Both functions are reentrant and `Send + Sync`-safe: they take only shared references and hold no state.

## 4. Behavior and invariants
**`encode_texts`**
- `texts.is_empty()` → `Err(NoInputTexts)`. Nothing is tokenized or logged as a success.
- Per text, in this order:
  1. `preprocessor.apply(text)`. On `Err(e)` return `Normalize(e)`, with no retry and no skipping.
  2. Prepend the prefix or marker by plain concatenation, with no separator. `SingleVector` uses `tokenizer.schema().prefix(mode)`. `MultiVector` uses `query_marker` for Query and `passage_marker` for Passage. An empty prefix or marker adds nothing.
  3. Tokenize all prepared texts together in one `encode_batch` call of `tokenizer.tokenizer()` with `add_special_tokens = true`. Any tokenizer error → `Tokenization { cause: error.to_string() }`. No partial results.
  4. Post-process with `MultiVector` only. If the mode is Query and `len < query_len`, append `pad_token_id` until `len == query_len`. Longer sequences stay unchanged (they are already limited to `max_seq_len` by the tokenizer). Passages and `SingleVector` stay unchanged.
- Result: `Vec<Vec<u32>>` of the same length as `texts`, taken from `Encoding::get_ids()` in input order. There is no padding except the Query padding above, because the tokenizer's own padding is off.
- Truncated count: the number of encodings with a non-empty `get_overflowing()`. This is measured before the post-processing in step 4.
- Logging on success:
  - `debug!` always, including when the count is 0: the number of truncated texts, the mode (`{:?}`) and the number of texts.
  - `trace!`: the largest sequence length in the result, taken after post-processing.

**`encode_and_batch`**
- Calls `encode_texts` with the same arguments. Any `EncodingError` (including `NoInputTexts`) is returned unchanged, and no batch is built.
- On success it calls `build_batches(&ids, batch_config)`. `Err(e)` becomes `Batch(e)`.
- Logs the number of batches with `debug!` after a successful build.
- Batches are padded, grouped by length, and every input index appears in exactly one batch's `positions()`. These are the guarantees of `build_batches`. The batch order is not the input order, and the mapping is `positions()`.
- `batch_config.pad_token_id` and `max_seq_len` are used as given. This module does not check them against the tokenizer.

## 5. Relied-on elements
From `tokenizer_loader`:
- `enum EmbedMode { Query, Passage }` (Debug, Clone, Copy).
- `LoadedTokenizer::{tokenizer(&self) -> &tokenizers::Tokenizer, pad_token_id(&self) -> u32, schema(&self) -> &TokenizerSchema, max_seq_len(&self) -> usize}`.
- `TokenizerSchema::prefix(&self, mode: EmbedMode) -> &str`.
- Guarantees: padding is off. Truncation is `LongestFirst`, `Right`, `stride 0`, at `max_seq_len` including special tokens. The type is `Send + Sync`.

From `text_normalization`:
- `TextPreprocessor::apply<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError>`.
- `NormalizeError(pub String)`: Debug, Clone, PartialEq, Eq.
- The tokenizer does its own normalization. Do not repeat it here.

From `batching`:
- `build_batches(seqs: &[Vec<u32>], cfg: &BatchConfig) -> Result<Vec<Batch>, BatchError>`.
- `BatchConfig { token_budget: usize, max_seq_len: usize, pad_token_id: u32 }`.
- `Batch::positions(&self) -> &[u32]` gives the original input indices per row.
- `BatchError::InvalidConfig` is the only error. An empty input returns `Ok(vec![])`, but this module never passes an empty input.
- `BatchError` and `Batch` derive Debug, Clone, PartialEq, Eq.