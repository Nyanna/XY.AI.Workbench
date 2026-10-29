use super::error::TaskError;
use super::params::{
    EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, VectorInput,
};
use crate::resident::queue::Queue;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TaskKind {
    Embedding,
    Similarity,
    Inference,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TaskMeta {
    pub id: u64,
    pub kind: TaskKind,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Payload {
    Embedding {
        input: TextInput,
        params: EmbeddingParams,
    },
    Similarity {
        input: VectorInput,
        params: SimilarityParams,
    },
    Inference {
        input: PairInput,
        params: InferenceParams,
    },
}

pub type TaskQueue = Queue<TaskMeta, Payload, Output, TaskError>;
