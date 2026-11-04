//! The only place where candle tensors and the BERT implementation are used.
use std::sync::Arc;
use candle_core::{DType as CDType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use super::{Backbone, Encoder, ModelError, ModelSpec, PooledEncoder, ProjectionHead};
use crate::resident::memory::DeviceKind;
fn backbone_dtype(spec: &ModelSpec, kind: DeviceKind) -> Result<CDType, ModelError> {
    CDType::try_from(spec.compute.for_device(kind))
        .map_err(|e| ModelError::Incompatible {
            reason: e.to_string(),
        })
}
pub struct CandleBert {
    model: BertModel,
    device: Device,
    dim: usize,
}
fn load_err(e: impl std::fmt::Display) -> ModelError {
    ModelError::Load {
        reason: e.to_string(),
    }
}
fn device_for(kind: DeviceKind) -> Result<Device, ModelError> {
    match kind {
        DeviceKind::Cpu => Ok(Device::Cpu),
        #[cfg(feature = "cuda")]
        DeviceKind::Gpu => Device::new_cuda(0).map_err(load_err),
        #[allow(unreachable_patterns)]
        _ => {
            Err(ModelError::Incompatible {
                reason: "GPU encoding requires the `cuda` feature".into(),
            })
        }
    }
}
impl CandleBert {
    pub fn load(spec: &ModelSpec, kind: DeviceKind) -> Result<Self, ModelError> {
        let text = std::fs::read_to_string(&spec.paths.config).map_err(load_err)?;
        let raw: serde_json::Value = serde_json::from_str(&text).map_err(load_err)?;
        let model_type = raw
            .get("model_type")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        if model_type != "bert" {
            return Err(ModelError::Incompatible {
                reason: format!("model_type `{model_type}` is not `bert`"),
            });
        }
        let config: Config = serde_json::from_value(raw).map_err(load_err)?;
        if config.hidden_size != spec.dim {
            return Err(ModelError::Incompatible {
                reason: format!(
                    "hidden_size {} does not match spec dim {}", config.hidden_size, spec
                    .dim
                ),
            });
        }
        if spec.max_seq_len > config.max_position_embeddings {
            return Err(ModelError::Incompatible {
                reason: format!(
                    "max_seq_len {} exceeds max_position_embeddings {}", spec
                    .max_seq_len, config.max_position_embeddings
                ),
            });
        }
        let device = device_for(kind)?;
        let dtype = backbone_dtype(spec, kind)?;
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&spec.paths.weights, dtype, &device)
        }
            .map_err(load_err)?;
        let model = BertModel::load(vb, &config)
            .map_err(|e| {
                let msg = e.to_string();
                match msg.find("cannot find tensor") {
                    Some(at) => {
                        ModelError::WeightNameMismatch {
                            expected: msg[at + "cannot find tensor".len()..]
                                .trim()
                                .to_owned(),
                        }
                    }
                    None => load_err(msg),
                }
            })?;
        Ok(Self {
            model,
            device,
            dim: config.hidden_size,
        })
    }
}
impl Backbone for CandleBert {
    fn dim(&self) -> usize {
        self.dim
    }
    fn forward(
        &self,
        ids: &[u32],
        mask: &[u32],
        batch: usize,
        seq: usize,
    ) -> Result<Vec<f32>, ModelError> {
        self.run(ids, Some(mask), batch, seq)
    }
}
impl CandleBert {
    fn run(
        &self,
        ids: &[u32],
        mask: Option<&[u32]>,
        batch: usize,
        seq: usize,
    ) -> Result<Vec<f32>, ModelError> {
        let fail = |e: candle_core::Error| ModelError::EncodeFailed {
            reason: e.to_string(),
        };
        let ids = Tensor::from_slice(ids, (batch, seq), &self.device).map_err(fail)?;
        let mask = mask
            .map(|m| Tensor::from_slice(m, (batch, seq), &self.device))
            .transpose()
            .map_err(fail)?;
        let types = ids.zeros_like().map_err(fail)?;
        self.model
            .forward(&ids, &types, mask.as_ref())
            .and_then(|h| h.to_dtype(CDType::F32)?.flatten_all()?.to_vec1::<f32>())
            .map_err(fail)
    }
}
pub fn bert_factory(
    spec: &ModelSpec,
    kind: DeviceKind,
) -> Result<Arc<dyn Encoder>, ModelError> {
    let backbone = CandleBert::load(spec, kind)?;
    let head = spec
        .head
        .as_ref()
        .map(|h| {
            let path = spec
                .paths
                .weights
                .first()
                .ok_or_else(|| ModelError::Load {
                    reason: "no weight files".into(),
                })?;
            ProjectionHead::from_file(path, h)
        })
        .transpose()?;
    let encoder = PooledEncoder::new(backbone, head, spec.pooling, spec.dtype)?
        .with_normalize(spec.normalize)
        .with_run_info(kind, spec.compute.for_device(kind));
    Ok(Arc::new(encoder))
}
#[cfg(test)]
#[path = "test/candle_adapter.rs"]
mod tests;
