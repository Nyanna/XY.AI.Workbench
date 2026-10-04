//! xy.ai.rag - Layered Anytime Retrieval Engine.
//!
//! Used both as a library (`xy_ai_rag::core::...`) and as an on-demand CLI
//! utility (`xyrag`, see `src/bin/xyrag.rs`) - no daemon or server process.

pub mod core;
pub mod layers;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
