# Spec: Encoding-Schemata (module `encoding_schemata`, build order 112)

## 1. Task, restrictions and boundaries
Turn text plus an `EmbedMode` (Query or Passage/Document) into a token-ID sequence, using one of two schemas:
- **Prefix schema:** prepend the mode's prefix.
- **Multi-vector schema (late interaction):** prepend a marker. Queries are truncated and padded with the mask token. Documents are truncated and not padded.

Restrictions:
- Pure and synchronous. No I/O, no global state, no threads, no `unsafe`.
- Does not load tokenizers, batch, run the model, pool or normalize.
- Does not read `ModelDescriptor` itself. The caller passes values in.
- Invents no defaults for marker text, lengths or mask ID.

## 2. Context
- `tokenizer_loader::LoadedTokenizer` is the ready tokenizer. Padding is off, truncation is set to `max_seq_len` with the final special token preserved, and the pad token is known. It also carries the prefix schema (`schema().prefix(mode)`).
- `ModelDescriptor.multi_vector: Option<MultiVectorConfig>` holds the marker texts and lengths. It has no mask-token ID, so the caller supplies that ID.
- Downstream, the model consumes `EncodedSequence`. In multi-vector query mode it must see the padded positions (query augmentation), so `attention_mask` is 1 there.

## 3. Interface
```rust
use crate::model_descriptor::MultiVectorConfig;
use crate::tokenizer_loader::{EmbedMode, LoadedTokenizer};

/// Multi-vector parameters per model variant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiVectorSchema {
    pub query_marker: String,
    pub document_marker: String,
    pub max_query_len: usize,     // fixed query length, > 0
    pub max_document_len: usize,  // > 0
    pub mask_token_id: u32,
}
impl MultiVectorSchema {
    /// Copies markers and lengths unchanged from the descriptor config.
    pub fn from_config(cfg: &MultiVectorConfig, mask_token_id: u32) -> Self;
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
    /// Display: "ungültiges Encoding-Schema: {reason}"
    InvalidSchema { reason: String },
    /// Display: "Tokenisierung fehlgeschlagen: {cause}"
    Tokenize { cause: String },
}
impl std::fmt::Display for EncodingError { /* as above */ }
impl std::error::Error for EncodingError {}

impl EncodingSchema {
    /// Checks the multi-vector schema: max_query_len > 0 and max_document_len > 0,
    /// otherwise InvalidSchema (reason names the field). Prefix is always Ok.
    pub fn validate(&self) -> Result<(), EncodingError>;

    /// Encodes one text. Calls validate() first.
    pub fn encode(&self, tokenizer: &LoadedTokenizer, text: &str, mode: EmbedMode)
        -> Result<EncodedSequence, EncodingError>;
}
```
All types are `Send + Sync + 'static` value types. `encode` takes `&self` and has no internal state.

## 4. Behavior and invariants
**Common**
- The input string is `lead + text`. `lead` is described per schema below.
- The input is encoded via `tokenizer.tokenizer().encode(input, true)`, so the tokenizer's special tokens are added and its general truncation (`max_seq_len`) applies. Any tokenizer error gives `Tokenize { cause }`.
- The multi-vector truncations apply in addition to the general one. The result is therefore never longer than `min(max_seq_len, mv_limit)` before query padding.
- `attention_mask` always has the length of `ids` and is all 1.
- Deterministic and pure: the same inputs give the same output. Empty text is valid.

**Prefix schema**
- `lead = tokenizer.schema().prefix(mode)`, with Query → `query_prefix` and Passage → `passage_prefix`. An empty prefix gives `lead = ""`.
- The token sequence from the tokenizer is returned unchanged. There is no extra truncation, padding or pad removal.

**Multi-vector schema**
- `lead` is the marker (Query → `query_marker`, Passage → `document_marker`) followed by one space `' '`. If the marker is empty, `lead = ""`. The marker is therefore prepended as text and tokenized together with the input.
- Shared truncation rule `cut(ids, n)`:
  - If `len <= n`, do nothing.
  - Otherwise keep the first `n` tokens.
  - If the last token of the original encoding is a special token (`special_tokens_mask == 1`) and `n >= 1`, the result is the first `n-1` tokens followed by that last token. The result has length exactly `n`.
- **Query mode** (`Query`):
  1. Apply `cut(ids, max_query_len)`.
  2. Append `mask_token_id` until the length is exactly `max_query_len`.
  3. Result: `ids.len() == max_query_len` and `attention_mask.len() == max_query_len`, all 1. Padded positions are not masked out.
  4. If the general truncation already shortened the sequence below `max_query_len` (because `max_seq_len < max_query_len`), padding still brings it to exactly `max_query_len`.
- **Document mode** (`Passage`):
  1. Remove every token whose ID equals `tokenizer.pad_token_id()`.
  2. Apply `cut(ids, max_document_len)`.
  3. No padding. The length is at most `max_document_len`.
  4. The `attention_mask` is all 1 and as long as `ids`.
- `mask_token_id` is used as given. It is not checked against the vocabulary.

**Errors**
- `InvalidSchema` is returned only for a multi-vector schema with a length of 0, and it is returned before any tokenization.
- Nothing panics.

**Concurrency**
- `encode` may be called concurrently from many threads on a shared `&EncodingSchema` and a shared `&LoadedTokenizer`. It uses no locks.

## 5. Relied-on elements
From `tokenizer_loader` (verbatim):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbedMode { Query, Passage }
pub struct TokenizerSchema { pub query_prefix: String, pub passage_prefix: String }
impl TokenizerSchema { pub fn prefix(&self, mode: EmbedMode) -> &str; }
impl LoadedTokenizer {
    pub fn schema(&self) -> &TokenizerSchema;
    pub fn max_seq_len(&self) -> usize;
    pub fn pad_token_id(&self) -> u32;
    pub fn tokenizer(&self) -> &tokenizers::Tokenizer;
}
```
Guarantees relied on:
- `LoadedTokenizer` has padding disabled.
- Its truncation is set to `max_seq_len`, Right, LongestFirst, and keeps the final special token. A truncated encoding has at most `max_seq_len` tokens, special tokens included.
- `LoadedTokenizer` is immutable and `Send + Sync`.

From `model_descriptor` (verbatim):
```rust
pub struct MultiVectorConfig {
    pub max_query_len: usize, pub max_document_len: usize,
    pub query_marker: String, pub document_marker: String,
    pub dimension: usize, pub skiplist: Vec<String>,
}
```
Only `max_query_len`, `max_document_len`, `query_marker` and `document_marker` are used. `dimension` and `skiplist` are ignored by this module. The descriptor's `validate()` does not check `multi_vector`, so this module's `validate()` does.

From the `tokenizers` crate: `Tokenizer::encode(input, add_special_tokens) -> Result<Encoding, _>`, `Encoding::get_ids()`, `Encoding::get_special_tokens_mask()`.