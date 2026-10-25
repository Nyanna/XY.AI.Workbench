//! Public resident facade API.
mod blocking;
mod error;
mod extensions;
mod handle;
mod params;
mod resident;
mod task;
#[cfg(test)]
#[path = "test/addon.rs"]
mod addon_tests;
pub use blocking::ResidentBlocking;
pub use error::TaskError;
pub use extensions::Extensions;
pub use handle::TaskHandle;
pub use params::{
    EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput,
    TextRole, VectorInput,
};
pub use resident::{IndexInfo, Residency, Resident, ResidentBuilder, ResidentConfig};
pub use crate::resident::processor::WorkerConfig;
pub use task::{Payload, TaskKind, TaskMeta, TaskQueue};
