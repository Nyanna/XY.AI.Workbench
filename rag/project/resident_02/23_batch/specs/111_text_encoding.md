# Spec: Text-Encoding (module `text_encoding`, build order 111)

## 1. Task, restrictions and boundaries
Implement the following in Rust:
- Encode a list of texts in one mode (Query or Passage) as a batch into token-ID sequences. The output order equals the input order.
- A combined function that encodes and then splits the result into padded batches via `build_batches`. It passes encoding errors through unchanged and logs the batch count.

Restrictions:
- Stateless free functions. No `unsafe`, no threads, no global state, no I/O. Logging uses the `log` crate.
- Do not load tokenizers, do not implement normalization, schema or padding logic, and do not pool or run inference. Use the dependency APIs as specified below.
- Do not add behavior beyond this spec.

## 2. Context
- `tokenizer_loader` provides `LoadedTokenizer`. It holds the tokenizer, with padding off and truncation set to `max_seq_len` (special tokens included), plus `pad_token_id`, the prefix schema and `max_seq_len`.
- `text_normalization` provides `TextPreprocessor::apply`. It is the external normalizer. The tokenizer runs its own normalization, including the steps configured through `tokenizer_loader`.
- `batching` (Batch-Bildung) provides `build_batches(&[Vec<u32>], &BatchConfig)`. It does the sorting by length, the padding and the `positions` mapping.
- `encoding_schemata` provides `EncodingSchema` and `EncodingSchema::encode`, which turns one already normalized text into an `EncodedSequence`. It is the only definition of the schema and of the markers, padding and cutting rules. This module does not duplicate them.
- The caller supplies the `EncodingSchema`, the `BatchConfig` and the loaded dependencies.

## 3. Interface
```rust
use crate::encoding_schemata::{EncodingError as SchemaError, EncodingSchema};
use crate::tokenizer_loader::{LoadedTokenizer, EmbedMode};
use crate::text_normalization::{TextPreprocessor, NormalizeError};
use crate::batching::{Batch, BatchConfig, BatchError, build_batches};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextEncodingError {
    /// Empty list. Display: "keine Eingabetexte"
    NoInputTexts,
    /// External normalizer failed. Display: "Normalisierung fehlgeschlagen: {0.0}"
    Normalize(NormalizeError),
    /// Invalid schema or tokenization failure reported by `encoding_schemata`.
    /// Display: the Display of the wrapped error, unchanged.
    Schema(SchemaError),
    /// Batch config rejected by build_batches. Display: "Batch-Bildung fehlgeschlagen: {0:?}"
    Batch(BatchError),
}
impl std::fmt::Display for TextEncodingError { /* as above */ }
impl std::error::Error for TextEncodingError {}

/// Normalize and encode all texts with `schema`. Result[i] belongs to texts[i].
/// The ids are the `EncodedSequence::ids` of the single-text encoding; the attention mask
/// is all ones and is not returned.
pub fn encode_texts<S: AsRef<str>>(
    tokenizer: &LoadedTokenizer,
    preprocessor: &TextPreprocessor,
    schema: &EncodingSchema,
    texts: &[S],
    mode: EmbedMode,
) -> Result<Vec<Vec<u32>>, TextEncodingError>;

/// encode_texts, then build_batches(&ids, batch_config).
/// Every Batch maps its rows to input indices via Batch::positions().
pub fn encode_and_batch<S: AsRef<str>>(
    tokenizer: &LoadedTokenizer,
    preprocessor: &TextPreprocessor,
    schema: &EncodingSchema,
    texts: &[S],
    mode: EmbedMode,
    batch_config: &BatchConfig,
) -> Result<Vec<Batch>, TextEncodingError>;
```
Both functions are reentrant and `Send + Sync`-safe: they take only shared references and hold no state.

## 4. Behavior and invariants
**`encode_texts`**
- `texts.is_empty()` → `Err(NoInputTexts)`. Nothing is tokenized or logged as a success.
- After the empty-list check and before any text is touched, `schema.validate()` runs once. `Err(e)` becomes `Schema(e)`, and nothing is normalized or tokenized.
- Per text, in this order:
  1. `preprocessor.apply(text)`. On `Err(e)` return `Normalize(e)`, with no retry and no skipping.
  2. `schema.encode(tokenizer, &normalized, mode)`. On `Err(e)` return `Schema(e)`. Prefix or marker handling, truncation, padding and the mask token are defined by `encoding_schemata` only.
  3. Keep `EncodedSequence::ids`. The attention mask is all ones by the `encoding_schemata` contract, and `build_batches` derives the same mask for every position inside a sequence, so padded multi-vector query positions stay attended. No partial results on any error.
  
- Result: `Vec<Vec<u32>>` of the same length as `texts`, taken from `EncodedSequence::ids` in input order. Padding and cutting happen only as defined by `encoding_schemata`.
- Logging on success:
  - `debug!` always, including when the count is 0: the mode (`{:?}`) and the number of texts.
  - `trace!`: the largest sequence length in the result.

**`encode_and_batch`**
- Calls `encode_texts` with the same arguments. Any `TextEncodingError` (including `NoInputTexts`) is returned unchanged, and no batch is built.
- On success it calls `build_batches(&ids, batch_config)`. `Err(e)` becomes `Batch(e)`.
- Logs the number of batches with `debug!` after a successful build.
- Batches are padded, grouped by length, and every input index appears in exactly one batch's `positions()`. These are the guarantees of `build_batches`. The batch order is not the input order, and the mapping is `positions()`.
- `batch_config.pad_token_id` and `max_seq_len` are used as given. This module does not check them against the tokenizer.

## 5. Relied-on elements
From `encoding_schemata`:
- `EncodingSchema::{Prefix, MultiVector(MultiVectorSchema)}`, `EncodingSchema::validate(&self) -> Result<(), EncodingError>` and `EncodingSchema::encode(&self, &LoadedTokenizer, &str, EmbedMode) -> Result<EncodedSequence, EncodingError>`.
- `EncodedSequence { ids: Vec<u32>, attention_mask: Vec<u32> }`, with `attention_mask` all ones.
- `EncodingError::{InvalidSchema { reason }, Tokenize { cause }}`: Debug, Clone, PartialEq, Eq, with a Display.

From `tokenizer_loader`:
- `enum EmbedMode { Query, Passage }` (Debug, Clone, Copy).
- `LoadedTokenizer::{tokenizer(&self) -> &tokenizers::Tokenizer, pad_token_id(&self) -> u32, schema(&self) -> &TokenizerSchema, max_seq_len(&self) -> usize}`.
- `TokenizerSchema::prefix(&self, mode: EmbedMode) -> &str`.
- Guarantees: padding is off. Truncation is `LongestFirst`, `Right`, `stride 0`, at `max_seq_len` including special tokens. The type is `Send + Sync`.

From `text_normalization`:
- `TextPreprocessor::apply<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError>`.
- `NormalizeError(pub String)`: Debug, Clone, PartialEq, Eq.
- The tokenizer's normalizer (built-in plus configured extension) runs inside `EncodingSchema::encode`. This module neither configures nor inspects it. It only applies the external `TextPreprocessor`.

From `batching`:
- `build_batches(seqs: &[Vec<u32>], cfg: &BatchConfig) -> Result<Vec<Batch>, BatchError>`.
- `BatchConfig { token_budget: usize, max_seq_len: usize, pad_token_id: u32 }`.
- `Batch::positions(&self) -> &[u32]` gives the original input indices per row.
- `BatchError::InvalidConfig` is the only error. An empty input returns `Ok(vec![])`, but this module never passes an empty input.
- `BatchError` and `Batch` derive Debug, Clone, PartialEq, Eq.