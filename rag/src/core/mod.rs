//! Core building blocks of the RAG engine: query, result set, layer
//! contract, engine (orchestrator) and persistence.
pub mod engine;
pub mod executor;
pub mod layer;
pub mod persistence;
pub mod query;
pub mod registry;
pub mod result;
pub mod topology;
