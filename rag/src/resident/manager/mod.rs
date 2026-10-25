//! StreamedIndexManager, ScopeResolver, update queue, residency.
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
