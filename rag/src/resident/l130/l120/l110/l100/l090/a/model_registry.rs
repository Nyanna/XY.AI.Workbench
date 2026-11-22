//! Fixed set of models: maps architecture ids to encoder factories and scoring kinds to scorers.
use std::collections::HashMap;
use std::sync::Arc;
use crate::resident::l130::l120::l110::l100::l090::l080::a::kernel_scoring::{
    Metric, Scorer,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::a::resource_lifecycle::{
    ResourceKey, ResourceLoader,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::b::encoder_build::{
    build_encoder, BuiltEncoder,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::a::encoder_interface::Encoder;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::{
    DeviceClass, ModelConfigError, ModelDescriptor,
};
pub type ScoringKind = Metric;
pub const ENCODER_INIT_COST: u32 = 100;
const BERT_ARCHITECTURE: &str = "bert";
/// Builds an encoder for one architecture. Blocks until the weights are ready.
pub trait EncoderFactory: Send + Sync {
    fn build(
        &self,
        descriptor: &ModelDescriptor,
        class: DeviceClass,
    ) -> Result<Encoder, String>;
}
impl<F> EncoderFactory for F
where
    F: Fn(&ModelDescriptor, DeviceClass) -> Result<Encoder, String> + Send + Sync,
{
    fn build(
        &self,
        descriptor: &ModelDescriptor,
        class: DeviceClass,
    ) -> Result<Encoder, String> {
        self(descriptor, class)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RegistryError {
    #[error("nicht unterstützte Architektur: {0}")]
    UnsupportedArchitecture(String),
    #[error("{0}")]
    Config(ModelConfigError),
    #[error("Encoder-Laden fehlgeschlagen ({architecture}): {reason}")]
    Load { architecture: String, reason: String },
    #[error("Scorer nicht verfügbar: {scorer}")]
    ScorerUnavailable { kind: ScoringKind, scorer: String },
}
/// Resource handle of `ResourceRegistry<LoadedEncoder>`. Dropping it releases the weights.
pub struct LoadedEncoder {
    descriptor: Arc<ModelDescriptor>,
    class: DeviceClass,
    encoder: Encoder,
}
impl LoadedEncoder {
    pub fn descriptor(&self) -> &ModelDescriptor {
        &self.descriptor
    }
    pub fn device_class(&self) -> DeviceClass {
        self.class
    }
    pub fn encoder(&self) -> &Encoder {
        &self.encoder
    }
}
pub struct EncoderResource {
    pub key: ResourceKey,
    pub init_cost: u32,
    pub loader: Arc<dyn ResourceLoader<LoadedEncoder>>,
}
pub fn encoder_key(architecture: &str, class: DeviceClass) -> ResourceKey {
    let class = match class {
        DeviceClass::Cpu => "cpu",
        DeviceClass::Gpu => "gpu",
    };
    format!("{architecture}:{class}")
}
pub struct ModelRegistry {
    factories: HashMap<String, Arc<dyn EncoderFactory>>,
    scorers: HashMap<ScoringKind, Arc<Scorer>>,
}
impl Default for ModelRegistry {
    fn default() -> Self {
        ModelRegistry::new()
    }
}
impl ModelRegistry {
    pub fn new() -> ModelRegistry {
        ModelRegistry {
            factories: HashMap::new(),
            scorers: HashMap::new(),
        }
    }
    pub fn with_builtin() -> ModelRegistry {
        let mut registry = ModelRegistry::new();
        registry.register_architecture(BERT_ARCHITECTURE, Arc::new(build_bert_encoder));
        registry
    }
    pub fn register_architecture(
        &mut self,
        architecture: impl Into<String>,
        factory: Arc<dyn EncoderFactory>,
    ) -> Option<Arc<dyn EncoderFactory>> {
        self.factories.insert(architecture.into(), factory)
    }
    pub fn register_scorer(
        &mut self,
        kind: ScoringKind,
        scorer: Arc<Scorer>,
    ) -> Option<Arc<Scorer>> {
        self.scorers.insert(kind, scorer)
    }
    pub fn supports_architecture(&self, architecture: &str) -> bool {
        self.factories.contains_key(architecture)
    }
    pub fn architectures(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.factories.keys().cloned().collect();
        ids.sort();
        ids
    }
    pub fn scorer_available(&self, kind: ScoringKind) -> bool {
        self.scorers.contains_key(&kind)
    }
    pub fn scorer(&self, kind: ScoringKind) -> Result<Arc<Scorer>, RegistryError> {
        self.scorers
            .get(&kind)
            .cloned()
            .ok_or_else(|| RegistryError::ScorerUnavailable {
                kind,
                scorer: kind.kernel_name().to_string(),
            })
    }
    pub fn load_encoder(
        &self,
        descriptor: &ModelDescriptor,
        class: DeviceClass,
    ) -> Result<LoadedEncoder, RegistryError> {
        descriptor.validate().map_err(RegistryError::Config)?;
        let factory = self
            .factories
            .get(&descriptor.architecture)
            .ok_or_else(|| RegistryError::UnsupportedArchitecture(
                descriptor.architecture.clone(),
            ))?;
        let encoder = factory
            .build(descriptor, class)
            .map_err(|reason| RegistryError::Load {
                architecture: descriptor.architecture.clone(),
                reason,
            })?;
        Ok(LoadedEncoder {
            descriptor: Arc::new(descriptor.clone()),
            class,
            encoder,
        })
    }
    pub fn encoder_resource(
        self: &Arc<Self>,
        descriptor: Arc<ModelDescriptor>,
        class: DeviceClass,
    ) -> EncoderResource {
        EncoderResource {
            key: encoder_key(&descriptor.architecture, class),
            init_cost: ENCODER_INIT_COST,
            loader: Arc::new(EncoderLoader {
                registry: Arc::clone(self),
                descriptor,
                class,
            }),
        }
    }
}
struct EncoderLoader {
    registry: Arc<ModelRegistry>,
    descriptor: Arc<ModelDescriptor>,
    class: DeviceClass,
}
impl ResourceLoader<LoadedEncoder> for EncoderLoader {
    fn load(&self) -> Result<LoadedEncoder, String> {
        self.registry
            .load_encoder(&self.descriptor, self.class)
            .map_err(|e| e.to_string())
    }
}
fn build_bert_encoder(
    descriptor: &ModelDescriptor,
    class: DeviceClass,
) -> Result<Encoder, String> {
    build_encoder(descriptor, class)
        .map(BuiltEncoder::into_encoder)
        .map_err(|e| e.to_string())
}
#[cfg(test)]
#[path = "test/model_registry.rs"]
mod tests;
