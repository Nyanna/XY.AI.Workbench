use std::future::IntoFuture;
use std::io;

use tokio::runtime::{Builder, Runtime};

use super::error::TaskError;
use super::handle::TaskHandle;
use super::params::{
    EmbeddingParams, InferenceParams, Output, PairInput, SimilarityParams, TextInput, VectorInput,
};
use super::resident::Resident;

/// Owns a private runtime; must not be used from inside another tokio runtime.
pub struct ResidentBlocking {
    rt: Runtime,
    inner: Resident,
}

impl ResidentBlocking {
    pub fn new(inner: Resident) -> io::Result<Self> {
        let rt = Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("resident-blocking")
            .enable_all()
            .build()?;
        Ok(Self { rt, inner })
    }

    pub fn resident(&self) -> &Resident {
        &self.inner
    }

    pub fn embedding(
        &self,
        input: TextInput,
        params: EmbeddingParams,
    ) -> Result<Output, TaskError> {
        self.wait(self.inner.submit_embedding(input, params))
    }

    pub fn similarity(
        &self,
        input: VectorInput,
        params: SimilarityParams,
    ) -> Result<Output, TaskError> {
        self.wait(self.inner.submit_similarity(input, params))
    }

    pub fn inference(
        &self,
        input: PairInput,
        params: InferenceParams,
    ) -> Result<Output, TaskError> {
        self.wait(self.inner.submit_inference(input, params))
    }

    fn wait(&self, handle: TaskHandle) -> Result<Output, TaskError> {
        self.rt.block_on(handle.into_future())
    }
}
