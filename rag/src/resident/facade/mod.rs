//! Public resident facade API.

mod blocking;
mod error;
mod handle;
mod params;
mod resident;
mod task;

pub use blocking::ResidentBlocking;
pub use error::TaskError;
pub use handle::TaskHandle;
pub use params::{
    EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, TextRole,
    VectorInput,
};
pub use resident::{
    IndexInfo, Residency, Resident, ResidentBuilder, ResidentConfig, WorkerConfig,
};
pub use task::{Payload, TaskKind, TaskMeta, TaskQueue};
