# Spec: Textnormalisierung (module `text_normalization`, build order 98)

## 1. Task, restrictions and boundaries

Implement in Rust:
- **Tokenizer normalization config.** The module turns parameters (`NormalizationParams`) into a resolved config (`ResolvedNormalization`) for the tokenizer's own normalizer. The config is built lazily, on the CPU only.
- **External normalizer.** The module optionally applies a swappable, thread-safe `TextNormalizer` to chunk text. It runs before prefix/marker prepending and before tokenization.
- **No double normalization.** A step the tokenizer performs is not performed a second time. The external normalizer only adds transformations.
- **Identity normalizer.** `IdentityNormalizer` ships with the module.

Out of scope:
- Tokenization, prefixes and markers, chunking.
- Running the tokenizer's own steps (Unicode form, lowercasing, accent stripping). The tokenizer does that. This module never applies them itself.
- Threads, GPU, I/O.

Restrictions:
- No `unsafe`.
- Use the `log` crate.
- The only dependency is `resource_lifecycle`.

## 2. Context

- Chunk text passes through `TextPreprocessor::apply` first. After that, the caller prepends prefixes or markers, then tokenizes.
- The tokenizer applies its own normalizer, configured with `ResolvedNormalization`.
- The tokenizer-normalizer config is a loadable resource of class `MemoryClass::Ram`. `NormalizerLoader` implements `ResourceLoader<ResolvedNormalization>`. The caller registers it in the `ResourceRegistry` and gets the config through `request_init`, which is lazy. This module neither registers nor evicts anything.
- `TextPreprocessor` itself is cheap and holds no lifecycle state.

## 3. Interface

```rust
use std::borrow::Cow;
use std::sync::Arc;
use resource_lifecycle::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnicodeForm { #[default] None, Nfc, Nfd, Nfkc, Nfkd }

/// Set of normalization steps. Used both as the parameters for the
/// tokenizer normalizer and as the steps an external normalizer declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NormalizationParams {
    pub unicode_form: UnicodeForm,   // None = step off
    pub lowercase: bool,
    pub strip_accents: bool,
}

/// Config for the tokenizer's own normalizer. All steps are applied by the tokenizer, never by this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResolvedNormalization {
    pub unicode_form: UnicodeForm,
    pub lowercase: bool,
    pub strip_accents: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizeError(pub String);

/// External text normalizer. Swappable and usable concurrently.
pub trait TextNormalizer: Send + Sync {
    fn normalize<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError>;
    /// Steps this normalizer performs itself (default: none).
    fn performed_steps(&self) -> NormalizationParams { NormalizationParams::default() }
}

/// Returns the input unchanged (Cow::Borrowed).
#[derive(Debug, Clone, Copy, Default)]
pub struct IdentityNormalizer;
impl TextNormalizer for IdentityNormalizer { /* ... */ }

#[derive(Clone)]
pub struct TextPreprocessor { /* Option<Arc<dyn TextNormalizer>> */ }
impl TextPreprocessor {
    /// None = no external normalizer (text stays unchanged).
    pub fn new(external: Option<Arc<dyn TextNormalizer>>) -> Self;
    /// Applies the external normalizer. Without one, returns Cow::Borrowed(text).
    pub fn apply<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError>;
    /// Resolves the tokenizer config without double steps (see section 4).
    pub fn resolve(&self, params: &NormalizationParams) -> ResolvedNormalization;
}

/// Lazy loader of the tokenizer-normalizer config. CPU only, no I/O.
pub struct NormalizerLoader { /* params + preprocessor */ }
impl NormalizerLoader {
    pub fn new(params: NormalizationParams, preprocessor: TextPreprocessor) -> Self;
}
impl ResourceLoader<ResolvedNormalization> for NormalizerLoader {
    fn load(&self) -> Result<ResolvedNormalization, String>;
}
```

`TextPreprocessor` and `NormalizerLoader` are `Send + Sync`.

## 4. Behavior and invariants

**Applying the external normalizer**
- `apply` with `external == None` returns `Ok(Cow::Borrowed(text))`. The text is unchanged.
- `apply` with `IdentityNormalizer` returns `Ok(Cow::Borrowed(text))`.
- `apply` with an external normalizer returns exactly its result. `apply` is called once per chunk and never repeats the normalizer.
- An `Err(NormalizeError)` from the normalizer is returned as is. The text is not changed, and the module does not retry or swallow the error.
- A panic in the external normalizer is not caught.
- Empty text is passed through. Behavior stays the same.

**Avoiding double normalization (`resolve`)**
- `resolve(params)` starts from `params` and removes every step that `external.performed_steps()` already declares:
  - `unicode_form`: if the external form is not `None`, the resolved form is `None`.
  - `lowercase`: if declared by the external normalizer, the resolved value is `false`.
  - `strip_accents`: if declared by the external normalizer, the resolved value is `false`.
- Steps that the external normalizer does not declare stay unchanged. The external normalizer runs before the tokenizer and wins on overlap.
- Without an external normalizer, or with `IdentityNormalizer`, `resolve(params)` equals `params` field by field.
- `resolve` is pure, deterministic and idempotent. It does not fail.
- `apply` never performs `unicode_form`, `lowercase` or `strip_accents`. These steps run only once, in the tokenizer, according to `ResolvedNormalization`.

**Lazy initialization**
- Neither `NormalizerLoader::new` nor `TextPreprocessor::new` does work. Work happens only in `load()`.
- `load()` returns `Ok(preprocessor.resolve(&params))`, is deterministic and has no side effects. Repeated calls return equal values. It does not fail and uses no GPU or I/O.
- The result is `Send + Sync`, which satisfies `ResourceRegistry<ResolvedNormalization>`.

**Concurrency**
- `TextPreprocessor` is `Clone`, and the clones share the same `Arc` normalizer. `apply` and `resolve` take `&self` and can run concurrently from any number of threads. They hold no mutable state and no locks.
- An external normalizer must itself be `Send + Sync`. This module adds no synchronization.

**Errors**
- The only error type is `NormalizeError`. It comes only from the external normalizer.
- `NormalizerLoader::load` never returns `Err`.

## 5. Relied-on elements

From Ressourcen-Lebenszyklus (`resource_lifecycle`):
- `pub trait ResourceLoader<H>: Send + Sync { fn load(&self) -> Result<H, String>; }`. It runs synchronously inside `LoadTicket::run`.
- `ResourceRegistry<H: Send + Sync + 'static>::register(key, MemoryClass, Arc<dyn ResourceLoader<H>>)` is called by the caller, not by this module. Nothing is loaded at `register`.
- `MemoryClass::Ram` is the class for the normalizer config.