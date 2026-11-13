# Spec: Tokenizer-Laden (module `tokenizer_loader`, build order 99)

## 1. Task, restrictions and boundaries
Create a ready-to-use tokenizer in Rust, either from a tokenizer config file or from an already loaded tokenizer. Also:
- determine the pad token,
- disable automatic padding,
- set up truncation.

Restrictions:
- The tokenizer type is `tokenizers::Tokenizer` (HuggingFace `tokenizers` crate, `tokenizer.json` format). No other crates are needed besides `std` and the dependencies below.
- No `unsafe`, no global state, no threads.
- The module does not encode, batch, pool or apply prefixes. It only stores the schema.
- It does not manage lifecycle state. State, TTL and backoff belong to `resource_lifecycle`. This module only supplies a `ResourceLoader` for it.
- It does not call the registry itself.

## 2. Context
- `ModelDescriptor` (module `model_descriptor`) supplies `config_path`, `query_prefix`, `passage_prefix` and `max_seq_len`.
- `ResourceRegistry<H>` (module `resource_lifecycle`) loads resources lazily through `Arc<dyn ResourceLoader<H>>`. Here `H = LoadedTokenizer`.
- The caller registers `TokenizerLoader` with the registry. Nothing is read from disk before the first `load()`.
- Consumers later use `LoadedTokenizer` for encoding. They read the tokenizer, pad token, schema and `max_seq_len` from it.

## 3. Interface
```rust
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokenizers::Tokenizer;
use crate::model_descriptor::ModelDescriptor;
use crate::resource_lifecycle::ResourceLoader;

/// Fixed file name in the directory of the model config (parent of `config_path`).
pub const TOKENIZER_FILE_NAME: &str = "tokenizer.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbedMode { Query, Passage }

/// Schema for mode selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenizerSchema { pub query_prefix: String, pub passage_prefix: String }
impl TokenizerSchema {
    pub fn prefix(&self, mode: EmbedMode) -> &str; // Query -> query_prefix, Passage -> passage_prefix
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizerLoadError {
    /// File missing or unreadable/unparseable. Display: "Tokenizer konnte nicht geladen werden ({path}): {cause}"
    Load { path: PathBuf, cause: String },
    /// No pad token found. Display: "kein Pad-Token gefunden (Tokenizer {id})"
    NoPadToken { id: String },
    /// Truncation rejected. Display: "Trunkierung abgelehnt (Tokenizer {id}): {cause}"
    Truncation { id: String, cause: String },
}
impl std::fmt::Display for TokenizerLoadError { /* as above */ }
impl std::error::Error for TokenizerLoadError {}

/// Ready tokenizer: padding off, truncation set, pad token known. Immutable after construction.
pub struct LoadedTokenizer { /* private */ }
impl LoadedTokenizer {
    /// Reads `<parent of descriptor.config_path>/tokenizer.json`.
    /// id = that file path as `to_string_lossy()`. schema = descriptor prefixes. max_seq_len = descriptor.max_seq_len.
    pub fn from_descriptor(descriptor: &ModelDescriptor) -> Result<Self, TokenizerLoadError>;
    /// Reads the tokenizer file at an explicit path (id derived from the path as above).
    pub fn from_file(path: &Path, schema: TokenizerSchema, max_seq_len: usize) -> Result<Self, TokenizerLoadError>;
    /// From an already loaded tokenizer, freely chosen id.
    pub fn from_tokenizer(tokenizer: Tokenizer, id: impl Into<String>, schema: TokenizerSchema,
                          max_seq_len: usize) -> Result<Self, TokenizerLoadError>;

    pub fn id(&self) -> &str;
    pub fn schema(&self) -> &TokenizerSchema;
    pub fn max_seq_len(&self) -> usize;
    pub fn pad_token(&self) -> &str;
    pub fn pad_token_id(&self) -> u32;
    pub fn tokenizer(&self) -> &Tokenizer;
}

/// Lazy loader for `ResourceRegistry<LoadedTokenizer>`. Construction does no I/O.
pub struct TokenizerLoader { /* private */ }
impl TokenizerLoader {
    pub fn from_descriptor(descriptor: &ModelDescriptor) -> Self;   // path/schema/max_seq_len captured, file not touched
    pub fn from_file(path: PathBuf, schema: TokenizerSchema, max_seq_len: usize) -> Self;
    pub fn from_tokenizer(tokenizer: Tokenizer, id: String, schema: TokenizerSchema, max_seq_len: usize) -> Self;
}
impl ResourceLoader<LoadedTokenizer> for TokenizerLoader {
    /// Builds a LoadedTokenizer; Err(msg) = TokenizerLoadError.to_string().
    fn load(&self) -> Result<LoadedTokenizer, String>;
}
```
`LoadedTokenizer` and `TokenizerLoader` are `Send + Sync + 'static`. The registry stores `Arc<LoadedTokenizer>`.

## 4. Behavior and invariants
**Loading from a file**
- `from_descriptor` uses the directory `descriptor.config_path.parent()`. If there is no parent, it uses `.`. The file name is always `TOKENIZER_FILE_NAME`.
- A missing, unreadable or unparseable file returns `Load { path, cause }`. `path` is the full file path and `cause` is the underlying error text. Nothing panics.
- The id is deterministic: the same path gives the same id, and different paths give different ids.
- The schema is built from `query_prefix` and `passage_prefix` of the descriptor, unchanged (an empty prefix stays empty). `max_seq_len` is taken over unchanged.

**From a loaded tokenizer**
- `from_tokenizer` uses the given id, schema and `max_seq_len` unchanged. It does no file access.
- The remaining steps are identical to loading from a file.

**Steps after getting the tokenizer, in this order**
1. **Pad token**, first hit wins:
   1. the pad token configured in the tokenizer's padding parameters (`get_padding()`, `pad_token` with `pad_id`),
   2. the token `<pad>`,
   3. the token `[PAD]`.
   - For 2 and 3, the token counts as present only if `token_to_id` returns an id.
   - If none is found, return `NoPadToken { id }`.
   - `pad_token()` and `pad_token_id()` return the result. The id belongs to that token.
2. **Disable padding**: `with_padding(None)`. Afterwards `get_padding()` is `None`, so encoding never pads on its own. The pad token stays queryable.
3. **Truncation**: `max_seq_len == 0` returns `Truncation` without calling the tokenizer. Otherwise call `with_truncation(Some(TruncationParams { max_length: max_seq_len, strategy: LongestFirst, direction: Right, stride: 0 }))`.
   - If the tokenizer returns an error, return `Truncation { id, cause }`.
   - The tokenizer's own truncation reserves room for the special tokens its post-processor adds. So the final special token (e.g. `</s>`/`[SEP]`) is preserved and a truncated encoding has at most `max_seq_len` tokens in total, special tokens included.
   - Truncation applies to every input.
- A step that fails aborts the whole load. No partly configured `LoadedTokenizer` is returned.

**Lazy loading and concurrency**
- `TokenizerLoader::from_*` and registration do not read the file. Only `load()` does.
- `load()` can be called repeatedly. Each call builds a new `LoadedTokenizer`. `from_tokenizer` clones the stored `Tokenizer` on each call, so the loader's copy stays unchanged.
- A failed `load()` returns `Err(String)`, and `resource_lifecycle` handles backoff.
- `LoadedTokenizer` is immutable after construction and may be shared by `Arc` between threads without a lock.
- There is no internal synchronization.

## 5. Relied-on elements
From `model_descriptor` (verbatim fields used):
```rust
pub struct ModelDescriptor { pub query_prefix: String, pub passage_prefix: String,
    pub max_seq_len: usize, pub config_path: PathBuf, /* other fields unused */ }
```
`ModelDescriptor` is a plain value type (`Send + Sync + 'static`). This module performs no validation of it.

From `resource_lifecycle` (verbatim):
```rust
/// Loads the resource. Runs synchronously inside `LoadTicket::run`.
pub trait ResourceLoader<H>: Send + Sync {
    fn load(&self) -> Result<H, String>;
}
```
`ResourceRegistry<H>` needs `H: Send + Sync + 'static`. It runs the loader synchronously on the caller's thread and catches panics. On `Err(msg)` it goes to Backoff. The registry owns registration (`register(key, class, Arc<dyn ResourceLoader<H>>)`), not this module.

From the `tokenizers` crate: `Tokenizer::from_file`, `with_padding(Option<PaddingParams>)`, `with_truncation(Option<TruncationParams>) -> Result<_, _>`, `get_padding()`, `token_to_id`, `Clone`. `Tokenizer` is `Send + Sync`.