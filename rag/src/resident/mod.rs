//! Resident engine: embedding/similarity substrate, kept warm.
pub mod backend;
pub mod error;
pub mod failpoint;
pub mod telemetry;
#[cfg(any(test, feature = "mock"))]
pub mod mock;
pub mod queue;
pub mod memory;
pub mod op;
pub mod index;
pub mod kernel;
pub mod precision;
pub mod facade;
pub mod processor;
pub mod planner;
pub mod subengine;
pub mod model;
pub mod tokenize;
pub mod manager;
pub mod segment;
