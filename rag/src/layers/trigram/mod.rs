//! Trigram layer: text-pattern search gated by a per-file trigram signature.
//!
//! Part A (this layer) builds on part B's set data structure: a quantised
//! radix-4 trie (`CompactTrie`) as the serialisable form and a flat bitmask
//! (`CellMask`) as the in-memory form.
//!
//! Decisions from spec section 9:
//! - 9.3 Unicode: NFKD then combining marks dropped (`normalize`).
//! - 9.4 CamelCase: split at `[a-z0-9][A-Z]` and `[A-Z][A-Z][a-z]`, digits
//!   stay attached except at a case transition (`normalize::split_camel_case`).
//! - 9.5 Keyspace: more distinct trigrams than `2^key_bits` collide; this acts
//!   like quantisation (false positives only) and is not an error. IDs are
//!   assigned by probing for a free slot (`vocab::Vocab::allocate_id`).
//! - 9.6 Param constancy: `TrieParams` are fixed per index. The vocabulary
//!   stores them plus a creation timestamp; a parameter change rebuilds the
//!   vocabulary (new timestamp), and any signature older than that timestamp
//!   is treated as stale and re-indexed on visit (`vocab`, `layer::run`).
//! - 9.7 Binary files: NUL bytes or invalid UTF-8 make normalisation fail; the
//!   file is stored with an empty signature and is never searched
//!   (`normalize::normalize_bytes`, `layer::TrigramLayer::reindex`).
pub mod index;
pub mod layer;
pub mod normalize;
pub mod signature;
pub mod trie;
pub mod vocab;
pub use layer::TrigramLayer;
