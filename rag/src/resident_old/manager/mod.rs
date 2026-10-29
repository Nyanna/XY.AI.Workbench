//! StreamedIndexManager, ScopeResolver, update queue, residency.
//!
//! # Consistency model
//!
//! - The index is the source of truth. VRAM is a derived, discardable mirror. On divergence the
//!   index wins.
//! - Identity is `(EntryId, Stamp)`. Stamps are compared with `!=` only, never with `<`.
//! - Reconciliation is state-based. The `watch` signal only wakes the scan
//!   ([`sync::SyncScanner::scan`]). Only resident entries are compared.
//! - Write order in the index is data, then stamp, then signal. Read order on load is stamp,
//!   data, stamp again ([`sync::load_entry`]).
//! - Segment tables are copy-on-write with a root pointer swap ([`swap`]). A retired version is
//!   freed once all readers have reached the new epoch. Mapping mutations follow the table in
//!   the slot gap ([`slot`]).
//! - Last writer wins. [`apply`](crate::resident::index::Index::apply) is serialized.
pub mod advice;
pub mod blocking;
pub mod doorbell;
pub mod build;
pub mod arena;
pub mod scope;
pub mod slot;
pub mod swap;
pub mod runtime;
#[cfg(test)]
#[path = "test/flat.rs"]
pub(crate) mod flat_index;
pub mod streamed;
pub mod seqlock;
pub mod sync;
pub mod tracker;
pub mod evict;
pub(crate) mod flight;
pub mod residency;
pub mod upload;
pub mod transfer;
