//! Trigram layer: text-pattern search gated by a per-file trigram signature.
//!
//! Part B (this submodule tree) provides the set data structure used by the
//! layer: a quantised radix-4 trie (`CompactTrie`) as the serialisable form and
//! a flat bitmask (`CellMask`) as the in-memory form.

pub mod trie;
