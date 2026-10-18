//! The only place where candle tensors and the BERT implementation are used.
use std::sync::Arc;
use candle_core::{DType as CDType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use super::{Backbone, Encoder, ModelError, ModelSpec, PooledEncoder, ProjectionHead};
use crate::resident::memory::DeviceKind;
/// candle's `get_extended_attention_mask` computes `(1 - mask) * f32::MIN` in the model dtype, which is
/// `0 * -inf = NaN` in f16 for every position (also without a caller mask, as it builds a ones mask).
/// The backbone therefore runs in f32; only the stored embeddings use the slot dtype.
const BACKBONE_DTYPE: CDType = CDType::F32;
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
        let device = device_for(kind)?;
        let dtype = BACKBONE_DTYPE;
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
    Ok(Arc::new(PooledEncoder::new(backbone, head, spec.pooling, spec.dtype)?))
}
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    use crate::resident::memory::layout::Layout;
    use crate::resident::memory::{
        ArenaKind, ArenaSizes, CpuMemoryManager, DType, DeviceMemory,
    };
    use crate::resident::model::{
        plan_batches, BatchConfig, ModelRegistry, RegionIo, TensorRef,
    };
    use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};
    fn model_dir() -> Option<PathBuf> {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("model/multilingual-e5-small");
        dir.join("model.safetensors").exists().then_some(dir)
    }
    fn embed(texts: &[&str], dir: &std::path::Path, dtype: DType, device: DeviceKind) -> Vec<Vec<f32>> {
        embed_with(&ModelSpec::e5_small(dir, dtype), texts, device)
    }

    fn embed_with(spec: &ModelSpec, texts: &[&str], device: DeviceKind) -> Vec<Vec<f32>> {
        let dir = spec.paths.config.parent().unwrap();
        let dim = spec.dim;
        let loaded = ModelRegistry::with_defaults().load(spec, device).unwrap();
        let tok = tokenizers::Tokenizer::from_file(dir.join("tokenizer.json")).unwrap();
        let seqs: Vec<Vec<u32>> = texts
            .iter()
            .map(|t| tok.encode(*t, true).unwrap().get_ids().to_vec())
            .collect();
        let batches = plan_batches(
            &seqs,
            &BatchConfig {
                token_budget: 1 << 14,
                max_seq_len: 512,
                pad_token_id: 1,
            },
        );
        let sizes = ArenaSizes {
            weights: 1 << 10,
            index_segment: 1 << 10,
            workspace: 1 << 20,
            staging: 1 << 20,
            param_block: 1 << 12,
        };
        let mem = Arc::new(CpuMemoryManager::new(sizes, 1));
        let scratch = mem.alloc_in(ArenaKind::Workspace, 1024, 16).unwrap();
        let lane = RecordingLane::new(LaneClass::Query, LaneConfig::new(scratch));
        let io: Arc<dyn RegionIo> = mem.clone();
        let mut result = vec![Vec::new(); texts.len()];
        for b in &batches {
            let t_layout = Layout::contiguous(&[2, b.rows(), b.seq_len], DType::U32);
            let t_region = mem
                .alloc_in(ArenaKind::Staging, t_layout.byte_len() as u64, 16)
                .unwrap();
            mem.write(t_region, &b.pack()).unwrap();
            let o_layout = Layout::contiguous(&[b.rows(), dim], DType::F32);
            let o_region = mem
                .alloc_in(ArenaKind::Workspace, o_layout.byte_len() as u64, 16)
                .unwrap();
            let out = TensorRef {
                region: o_region,
                layout: o_layout,
            };
            loaded
                .encoder()
                .encode_into(
                    &io,
                    &TensorRef {
                        region: t_region,
                        layout: t_layout,
                    },
                    &out,
                    &lane,
                )
                .unwrap()
                .wait()
                .unwrap();
            let mut raw = vec![0u8; out.layout.byte_len()];
            mem.read(o_region, &mut raw).unwrap();
            let flat: Vec<f32> = raw
                .chunks_exact(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect();
            for (row, idx) in b.indices.iter().enumerate() {
                result[*idx] = flat[row * dim..(row + 1) * dim].to_vec();
            }
        }
        result
    }
    fn cosine(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(x, y)| x * y).sum()
    }
    #[test]
    fn e5_small_embeddings_are_normalised_and_padding_invariant() {
        let Some(dir) = model_dir() else { return };
        let texts = [
            "query: hello world",
            "passage: a much longer passage about retrieval of documents with embeddings",
        ];
        let batched = embed(&texts, &dir, DType::F32, DeviceKind::Cpu);
        let single = embed(&texts[..1], &dir, DType::F32, DeviceKind::Cpu);
        for v in &batched {
            assert!((cosine(v, v) - 1.0).abs() < 1e-4);
        }
        assert!(cosine(& batched[0], & single[0]) > 0.9999);
    }
    #[test]
    fn e5_small_ranks_related_text_higher() {
        let Some(dir) = model_dir() else { return };
        let v = embed(
            &[
                "query: how do plants make energy",
                "passage: Photosynthesis converts light into chemical energy in plants.",
                "passage: The stock market closed lower on Tuesday.",
            ],
            &dir,
            DType::F32,
            DeviceKind::Cpu,
        );
        assert!(cosine(& v[0], & v[1]) > cosine(& v[0], & v[2]));
    }
    #[test]
    fn wrong_hidden_size_is_incompatible() {
        let Some(dir) = model_dir() else { return };
        let mut spec = ModelSpec::e5_small(&dir, DType::F32);
        spec.dim = 128;
        assert!(
            matches!(CandleBert::load(& spec, DeviceKind::Cpu),
            Err(ModelError::Incompatible { .. }))
        );
    }
    fn spec_for(name: &str, dir: &std::path::Path) -> Option<ModelSpec> {
        match name {
            "multilingual-e5-small" => Some(ModelSpec::e5_small(dir, DType::F32)),
            _ => None,
        }
    }

    fn installed_models() -> Vec<(String, PathBuf)> {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("model");
        let mut found: Vec<_> = std::fs::read_dir(root)
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| e.path().join("config.json").exists())
            .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path()))
            .collect();
        found.sort();
        found
    }

    fn reference(name: &str) -> Option<(Vec<String>, Vec<Vec<f32>>)> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/data/reference/{name}.json"));
        let json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()?;
        let texts = json["texts"].as_array()?.iter().map(|t| t.as_str().map(str::to_owned)).collect::<Option<_>>()?;
        let embeddings = json["embeddings"]
            .as_array()?
            .iter()
            .map(|row| row.as_array()?.iter().map(|x| x.as_f64().map(|x| x as f32)).collect::<Option<Vec<_>>>())
            .collect::<Option<_>>()?;
        Some((texts, embeddings))
    }

    fn assert_matches_reference(device: DeviceKind) {
        let models = installed_models();
        assert!(!models.is_empty(), "no model installed under model/ (see model/README.md)");
        for (name, dir) in models {
            let (texts, expected) = reference(&name)
                .unwrap_or_else(|| panic!("model `{name}` has no tests/data/reference/{name}.json (see model/README.md)"));
            let spec = spec_for(&name, &dir).unwrap_or_else(|| panic!("no ModelSpec registered for `{name}` in spec_for"));
            let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
            let got = embed_with(&spec, &refs, device);
            assert_eq!(got.len(), expected.len(), "{name}");
            for (i, (g, e)) in got.iter().zip(&expected).enumerate() {
                let c = cosine(g, e);
                assert!(c > 0.9999, "{name} text {i}: cosine {c}");
            }
        }
    }

    #[test]
    fn installed_models_cpu_match_reference() {
        assert_matches_reference(DeviceKind::Cpu);
    }

    #[cfg(feature = "cuda")]
    #[test]
    #[ignore = "requires a CUDA device"]
    fn installed_models_gpu_match_reference() {
        assert_matches_reference(DeviceKind::Gpu);
    }

    #[cfg(feature = "cuda")]
    #[test]
    #[ignore = "requires a CUDA device"]
    fn e5_small_gpu_matches_cpu() {
        let Some(dir) = model_dir() else { return };
        let texts = [
            "query: hello world",
            "passage: a much longer passage about retrieval of documents with embeddings",
        ];
        let cpu = embed(&texts, &dir, DType::F32, DeviceKind::Cpu);
        let gpu = embed(&texts, &dir, DType::F32, DeviceKind::Gpu);
        for (c, g) in cpu.iter().zip(&gpu) {
            assert!(g.iter().all(| x | x.is_finite()));
            assert!(cosine(c, g) > 0.9999, "cosine {}", cosine(c, g));
        }
    }
}
