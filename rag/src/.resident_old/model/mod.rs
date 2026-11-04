//! Model resources: specs, registry, encoder contract and loaded-encoder resources.
mod batch;
#[cfg(feature = "candle")]
pub(crate) mod candle_adapter;
mod encoder;
mod error;
mod head;
mod pooling;
mod slot;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use parking_lot::Mutex;
pub use batch::{plan_batches, Batch, BatchConfig, BUCKETS};
pub use encoder::{Backbone, PooledEncoder};
pub use error::ModelError;
pub use head::ProjectionHead;
pub use pooling::{l2_normalize_rows, mean_pool};
pub use slot::{encode_bytes, write_slot};
use crate::resident::memory::layout::Layout;
use crate::resident::memory::{CpuMemoryManager, DType, DeviceKind, MemoryError, Region};
use crate::resident::subengine::{EventHandle, ExecContext, Resource, ResourceKey};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pooling {
    /// One vector per sequence: mean over unmasked tokens.
    MeanMasked,
    /// One vector per token.
    PerToken,
}
/// Scaling applied to every produced vector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Normalize {
    /// Unit Euclidean length; a dot product then equals the cosine.
    L2,
    None,
}
/// Backbone element type per device class; independent of the storage dtype.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeDtypes {
    pub cpu: DType,
    pub gpu: DType,
}
impl ComputeDtypes {
    pub fn for_device(&self, device: DeviceKind) -> DType {
        match device {
            DeviceKind::Cpu => self.cpu,
            DeviceKind::Gpu => self.gpu,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prefixes {
    pub query: String,
    pub passage: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadSpec {
    pub weight_name: String,
    pub in_dim: usize,
    pub out_dim: usize,
    pub bias: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelPaths {
    pub config: PathBuf,
    pub weights: Vec<PathBuf>,
}
/// Plain description of a model; owns no resources.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelSpec {
    pub model_id: String,
    pub arch_id: String,
    pub dim: usize,
    pub pooling: Pooling,
    pub normalize: Normalize,
    pub prefixes: Prefixes,
    /// Element type of the produced embeddings (storage).
    pub dtype: DType,
    /// Element type of the backbone forward pass per device class.
    pub compute: ComputeDtypes,
    pub max_seq_len: usize,
    pub head: Option<HeadSpec>,
    pub paths: ModelPaths,
}
impl ModelSpec {
    /// `intfloat/multilingual-e5-small` checkout in `dir`.
    pub fn e5_small(dir: &Path, dtype: DType) -> Self {
        Self {
            model_id: "intfloat/multilingual-e5-small".into(),
            arch_id: "bert".into(),
            normalize: Normalize::L2,
            compute: ComputeDtypes {
                cpu: DType::F32,
                gpu: DType::F32,
            },
            dim: 384,
            pooling: Pooling::MeanMasked,
            prefixes: Prefixes {
                query: "query: ".into(),
                passage: "passage: ".into(),
            },
            dtype,
            max_seq_len: 512,
            head: None,
            paths: ModelPaths {
                config: dir.join("config.json"),
                weights: vec![dir.join("model.safetensors")],
            },
        }
    }
    /// Rejects configurations that would silently degrade embeddings.
    pub fn validate(&self) -> Result<(), ModelError> {
        let invalid = |reason: String| Err(ModelError::Incompatible { reason });
        if self.dim == 0 || self.max_seq_len == 0 {
            return invalid(
                format!(
                    "dim ({}) and max_seq_len ({}) must be positive", self.dim, self
                    .max_seq_len
                ),
            );
        }
        for (name, prefix) in [
            ("query", &self.prefixes.query),
            ("passage", &self.prefixes.passage),
        ] {
            if !prefix.is_empty() && !prefix.ends_with(' ') {
                return invalid(
                    format!("{name} prefix `{prefix}` must end with a space"),
                );
            }
        }
        Ok(())
    }
}
/// A region together with its layout; the only way tensors cross the encoder API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TensorRef {
    pub region: Region,
    pub layout: Layout,
}
/// Byte access to regions of one memory domain.
pub trait RegionIo: Send + Sync {
    fn read(&self, region: Region, out: &mut [u8]) -> Result<(), MemoryError>;
    fn write(&self, region: Region, data: &[u8]) -> Result<(), MemoryError>;
}
impl RegionIo for CpuMemoryManager {
    fn read(&self, region: Region, out: &mut [u8]) -> Result<(), MemoryError> {
        CpuMemoryManager::read(self, region, out)
    }
    fn write(&self, region: Region, data: &[u8]) -> Result<(), MemoryError> {
        CpuMemoryManager::write(self, region, data)
    }
}
/// Completion of one `encode_into` call.
pub struct EncodeEvent {
    event: EventHandle,
    failure: Arc<Mutex<Option<ModelError>>>,
}
impl EncodeEvent {
    pub(crate) fn new(
        event: EventHandle,
        failure: Arc<Mutex<Option<ModelError>>>,
    ) -> Self {
        Self { event, failure }
    }
    pub fn is_complete(&self) -> bool {
        self.event.is_complete()
    }
    /// Blocks until the work finished and reports its failure, if any.
    pub fn wait(&self) -> Result<(), ModelError> {
        self.event.synchronize()?;
        self.failure.lock().take().map_or(Ok(()), Err)
    }
}
pub trait Encoder: Send + Sync {
    fn dim(&self) -> usize;
    fn dtype(&self) -> DType;
    /// `tokens` is a u32 `[2, batch, seq]` region (ids, then mask). `out` receives the
    /// L2-normalised embeddings in the encoder's dtype.
    fn encode_into(
        &self,
        mem: &Arc<dyn RegionIo>,
        tokens: &TensorRef,
        out: &TensorRef,
        lane: &dyn ExecContext,
    ) -> Result<EncodeEvent, ModelError>;
}
pub type EncoderFactory = Box<
    dyn Fn(&ModelSpec, DeviceKind) -> Result<Arc<dyn Encoder>, ModelError> + Send + Sync,
>;
/// Architecture id -> encoder constructor.
#[derive(Default)]
pub struct ModelRegistry {
    factories: HashMap<String, EncoderFactory>,
}
impl ModelRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    /// Registry with every architecture compiled into this build.
    pub fn with_defaults() -> Self {
        #[allow(unused_mut)]
        let mut registry = Self::new();
        #[cfg(feature = "candle")]
        registry.register("bert", Box::new(candle_adapter::bert_factory));
        registry
    }
    pub fn register(&mut self, arch_id: impl Into<String>, factory: EncoderFactory) {
        self.factories.insert(arch_id.into(), factory);
    }
    pub fn load(
        &self,
        spec: &ModelSpec,
        device: DeviceKind,
    ) -> Result<LoadedEncoder, ModelError> {
        spec.validate()?;
        let factory = self
            .factories
            .get(&spec.arch_id)
            .ok_or_else(|| ModelError::UnsupportedArch {
                arch: spec.arch_id.clone(),
            })?;
        Ok(LoadedEncoder {
            spec: spec.clone(),
            device,
            encoder: factory(spec, device)?,
        })
    }
}
/// Architecture plus weights, ready to encode; dropping it unloads the weights.
pub struct LoadedEncoder {
    spec: ModelSpec,
    device: DeviceKind,
    encoder: Arc<dyn Encoder>,
}
impl LoadedEncoder {
    pub fn spec(&self) -> &ModelSpec {
        &self.spec
    }
    pub fn device(&self) -> DeviceKind {
        self.device
    }
    pub fn encoder(&self) -> &Arc<dyn Encoder> {
        &self.encoder
    }
}
impl Resource for LoadedEncoder {
    fn key(&self) -> ResourceKey {
        ResourceKey::new(format!("model:{}:{:?}", self.spec.arch_id, self.device))
    }
    fn init_cost_hint(&self) -> u64 {
        100
    }
}
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
