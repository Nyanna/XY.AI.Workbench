//! StreamedIndexManager, ScopeResolver, update queue, residency.
pub mod advice;
pub mod blocking;
pub mod doorbell;
pub(crate) mod arena;
pub mod scope;
pub mod slot;
pub(crate) mod swap;
pub mod seqlock;
pub mod sync;
pub mod tracker;
