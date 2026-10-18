//! Optional linear projection applied to token embeddings (late-interaction checkpoints).

use std::path::Path;

use half::{bf16, f16};
use safetensors::{tensor::TensorView, Dtype, SafeTensors};

use super::{HeadSpec, ModelError};

#[derive(Debug)]
pub struct ProjectionHead {
    /// Row-major `[out_dim, in_dim]`.
    weight: Vec<f32>,
    bias: Option<Vec<f32>>,
    in_dim: usize,
    out_dim: usize,
}

fn to_f32(name: &str, view: &TensorView<'_>) -> Result<Vec<f32>, ModelError> {
    let data = view.data();
    Ok(match view.dtype() {
        Dtype::F32 => data.chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect(),
        Dtype::F16 => data.chunks_exact(2).map(|b| f16::from_le_bytes([b[0], b[1]]).to_f32()).collect(),
        Dtype::BF16 => data.chunks_exact(2).map(|b| bf16::from_le_bytes([b[0], b[1]]).to_f32()).collect(),
        other => {
            return Err(ModelError::Incompatible {
                reason: format!("tensor `{name}` has unsupported dtype {other:?}"),
            })
        }
    })
}

fn fetch(st: &SafeTensors<'_>, name: &str, shape: &[usize]) -> Result<Vec<f32>, ModelError> {
    let view = st
        .tensor(name)
        .map_err(|_| ModelError::WeightNameMismatch { expected: name.to_owned() })?;
    if view.shape() != shape {
        return Err(ModelError::Incompatible {
            reason: format!("tensor `{name}` has shape {:?}, expected {shape:?}", view.shape()),
        });
    }
    to_f32(name, &view)
}

impl ProjectionHead {
    pub fn from_safetensors(data: &[u8], spec: &HeadSpec) -> Result<Self, ModelError> {
        let st = SafeTensors::deserialize(data).map_err(|e| ModelError::Load { reason: e.to_string() })?;
        let weight = fetch(&st, &spec.weight_name, &[spec.out_dim, spec.in_dim])?;
        let bias = if spec.bias {
            let name = spec.weight_name.strip_suffix("weight").unwrap_or(&spec.weight_name).to_owned() + "bias";
            Some(fetch(&st, &name, &[spec.out_dim])?)
        } else {
            None
        };
        Ok(Self { weight, bias, in_dim: spec.in_dim, out_dim: spec.out_dim })
    }

    pub fn from_file(path: &Path, spec: &HeadSpec) -> Result<Self, ModelError> {
        let file = std::fs::File::open(path).map_err(|e| ModelError::Load { reason: e.to_string() })?;
        // SAFETY: the checkpoint is treated as read-only for the lifetime of the map.
        let map = unsafe { memmap2::Mmap::map(&file) }.map_err(|e| ModelError::Load { reason: e.to_string() })?;
        Self::from_safetensors(&map, spec)
    }

    pub fn in_dim(&self) -> usize {
        self.in_dim
    }

    pub fn out_dim(&self) -> usize {
        self.out_dim
    }

    /// `rows` holds consecutive `in_dim`-sized vectors.
    pub fn apply(&self, rows: &[f32]) -> Vec<f32> {
        let mut out = Vec::with_capacity(rows.len() / self.in_dim * self.out_dim);
        for row in rows.chunks_exact(self.in_dim) {
            for (o, w) in self.weight.chunks_exact(self.in_dim).enumerate() {
                let dot: f32 = w.iter().zip(row).map(|(a, b)| a * b).sum();
                out.push(dot + self.bias.as_ref().map_or(0.0, |b| b[o]));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checkpoint(with_bias: bool) -> Vec<u8> {
        let w: Vec<u8> = [1.0f32, 0.0, 0.0, 1.0, 1.0, 1.0].iter().flat_map(|v| v.to_le_bytes()).collect();
        let b: Vec<u8> = [0.5f32, 0.0, -1.0].iter().flat_map(|v| v.to_le_bytes()).collect();
        let mut tensors = vec![("linear.weight", TensorView::new(Dtype::F32, vec![3, 2], &w).unwrap())];
        if with_bias {
            tensors.push(("linear.bias", TensorView::new(Dtype::F32, vec![3], &b).unwrap()));
        }
        safetensors::serialize(tensors, None).unwrap()
    }

    fn spec(bias: bool) -> HeadSpec {
        HeadSpec { weight_name: "linear.weight".into(), in_dim: 2, out_dim: 3, bias }
    }

    #[test]
    fn projects_with_and_without_bias() {
        let plain = ProjectionHead::from_safetensors(&checkpoint(false), &spec(false)).unwrap();
        assert_eq!(plain.apply(&[2.0, 3.0]), vec![2.0, 3.0, 5.0]);
        let biased = ProjectionHead::from_safetensors(&checkpoint(true), &spec(true)).unwrap();
        assert_eq!(biased.apply(&[2.0, 3.0]), vec![2.5, 3.0, 4.0]);
    }

    #[test]
    fn missing_or_misshaped_tensors_are_reported() {
        let mut s = spec(false);
        s.weight_name = "dense.weight".into();
        assert!(matches!(
            ProjectionHead::from_safetensors(&checkpoint(false), &s),
            Err(ModelError::WeightNameMismatch { .. })
        ));
        let mut s = spec(false);
        s.out_dim = 4;
        assert!(matches!(
            ProjectionHead::from_safetensors(&checkpoint(false), &s),
            Err(ModelError::Incompatible { .. })
        ));
    }
}
