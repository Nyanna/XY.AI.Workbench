//! StreamedIndexManager, ScopeResolver, update queue, residency.
pub mod advice;
pub mod blocking;
pub mod doorbell;
pub mod build;
pub(crate) mod arena;
pub mod scope;
pub mod slot;
pub(crate) mod swap;
pub mod streamed;
pub mod seqlock;
pub mod sync;
pub mod tracker;
pub(crate) mod evict;
pub(crate) mod flight;
pub(crate) mod residency;
pub(crate) mod upload;
pub mod transfer;
