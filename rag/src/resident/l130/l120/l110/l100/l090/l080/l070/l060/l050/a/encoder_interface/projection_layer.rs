//! Linear per-token projection `input_dim -> output_dim` loaded from safetensors as FP32.
use std::fmt;
use std::path::{Path, PathBuf};
use serde_json::{Map, Value};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::weight_source::{
    WeightError, WeightFile,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::ProjectionLayer;
#[derive(Debug, Clone, PartialEq)]
pub struct Projection {
    weight: Vec<f32>,
    bias: Option<Vec<f32>>,
    input_dim: usize,
    output_dim: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    WeightNameMismatch { name: String },
    IncompatibleShape { tensor: String, actual: Vec<usize>, expected: Vec<usize> },
    IncompatibleDType { tensor: String, dtype: String },
    Load { path: Option<PathBuf>, reason: String },
    InputDimMismatch { expected: usize, actual: usize },
}
impl fmt::Display for ProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectionError::WeightNameMismatch { name } => {
                write!(f, "Gewichtsname passt nicht: {name}")
            }
            ProjectionError::IncompatibleShape { tensor, actual, expected } => {
                write!(
                    f,
                    "inkompatibel: Tensor {tensor} hat Shape {actual:?}, erwartet {expected:?}"
                )
            }
            ProjectionError::IncompatibleDType { tensor, dtype } => {
                write!(
                    f,
                    "inkompatibel: Tensor {tensor} hat nicht unterstützten Typ {dtype}"
                )
            }
            ProjectionError::Load { path: Some(path), reason } => {
                write!(f, "Load-Error: {reason} ({})", path.display())
            }
            ProjectionError::Load { path: None, reason } => {
                write!(f, "Load-Error: {reason}")
            }
            ProjectionError::InputDimMismatch { expected, actual } => {
                write!(f, "Eingabedimension {actual} statt {expected}")
            }
        }
    }
}
impl std::error::Error for ProjectionError {}
impl ProjectionError {
    fn with_path(self, path: &Path) -> ProjectionError {
        match self {
            ProjectionError::Load { path: None, reason } => {
                ProjectionError::Load {
                    path: Some(path.to_path_buf()),
                    reason,
                }
            }
            other => other,
        }
    }
}
fn load_error(reason: impl Into<String>) -> ProjectionError {
    ProjectionError::Load {
        path: None,
        reason: reason.into(),
    }
}
pub fn bias_tensor_name(weight_name: &str) -> String {
    match weight_name.strip_suffix("weight") {
        Some(stem) => format!("{stem}bias"),
        None => format!("{weight_name}bias"),
    }
}
#[derive(Clone, Copy)]
enum Kind {
    F32,
    F16,
    Bf16,
}
impl Kind {
    fn parse(dtype: &str) -> Option<Kind> {
        match dtype {
            "F32" => Some(Kind::F32),
            "F16" => Some(Kind::F16),
            "BF16" => Some(Kind::Bf16),
            _ => None,
        }
    }
    fn size(self) -> usize {
        match self {
            Kind::F32 => 4,
            Kind::F16 | Kind::Bf16 => 2,
        }
    }
}
struct Entry<'a> {
    dtype: &'a str,
    shape: Vec<usize>,
    bytes: &'a [u8],
}
type Fetch<'f, 'a> = &'f dyn Fn(&str) -> Result<Option<Entry<'a>>, ProjectionError>;
fn f16_to_f32(h: u16) -> f32 {
    let sign = ((h as u32) & 0x8000) << 16;
    let exp = ((h >> 10) & 0x1f) as u32;
    let man = (h & 0x3ff) as u32;
    let bits = if exp == 0 {
        if man == 0 {
            sign
        } else {
            let shift = man.leading_zeros() - 21;
            let mantissa = (man << shift) & 0x3ff;
            sign | ((113 - shift) << 23) | (mantissa << 13)
        }
    } else if exp == 0x1f {
        sign | 0x7f80_0000 | (man << 13)
    } else {
        sign | ((exp + 112) << 23) | (man << 13)
    };
    f32::from_bits(bits)
}
fn bf16_to_f32(b: u16) -> f32 {
    f32::from_bits((b as u32) << 16)
}
fn decode(bytes: &[u8], kind: Kind) -> Vec<f32> {
    match kind {
        Kind::F32 => {
            bytes
                .chunks_exact(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect()
        }
        Kind::F16 => {
            bytes
                .chunks_exact(2)
                .map(|c| f16_to_f32(u16::from_le_bytes([c[0], c[1]])))
                .collect()
        }
        Kind::Bf16 => {
            bytes
                .chunks_exact(2)
                .map(|c| bf16_to_f32(u16::from_le_bytes([c[0], c[1]])))
                .collect()
        }
    }
}
fn check_entry(
    entry: &Entry<'_>,
    name: &str,
    expected: &[usize],
) -> Result<Kind, ProjectionError> {
    if entry.shape != expected {
        return Err(ProjectionError::IncompatibleShape {
            tensor: name.to_owned(),
            actual: entry.shape.clone(),
            expected: expected.to_vec(),
        });
    }
    Kind::parse(entry.dtype)
        .ok_or_else(|| ProjectionError::IncompatibleDType {
            tensor: name.to_owned(),
            dtype: entry.dtype.to_owned(),
        })
}
fn check_len(entry: &Entry<'_>, kind: Kind, name: &str) -> Result<(), ProjectionError> {
    let elems = entry.shape.iter().try_fold(1usize, |acc, d| acc.checked_mul(*d));
    let want = elems.and_then(|e| e.checked_mul(kind.size()));
    if want != Some(entry.bytes.len()) {
        return Err(
            load_error(
                format!(
                    "tensor {name}: byte length {} does not match shape and dtype", entry
                    .bytes.len()
                ),
            ),
        );
    }
    Ok(())
}
fn build(
    fetch: Fetch<'_, '_>,
    layer: &ProjectionLayer,
    input_dim: usize,
    output_dim: usize,
) -> Result<Projection, ProjectionError> {
    let weight_name = layer.tensor_name.as_str();
    let weight = fetch(weight_name)?
        .ok_or_else(|| ProjectionError::WeightNameMismatch {
            name: weight_name.to_owned(),
        })?;
    let weight_kind = check_entry(&weight, weight_name, &[output_dim, input_dim])?;
    check_len(&weight, weight_kind, weight_name)?;
    let bias_name = bias_tensor_name(weight_name);
    let bias = match fetch(&bias_name)? {
        Some(entry) => {
            let kind = check_entry(&entry, &bias_name, &[output_dim])?;
            check_len(&entry, kind, &bias_name)?;
            Some((entry, kind))
        }
        None => None,
    };
    Ok(Projection {
        weight: decode(weight.bytes, weight_kind),
        bias: bias.map(|(entry, kind)| decode(entry.bytes, kind)),
        input_dim,
        output_dim,
    })
}
fn parse_entry<'a>(
    name: &str,
    value: &'a Value,
    data: &'a [u8],
) -> Result<Entry<'a>, ProjectionError> {
    let obj = value
        .as_object()
        .ok_or_else(|| load_error(
            format!("tensor {name}: header entry is not an object"),
        ))?;
    let dtype = obj
        .get("dtype")
        .and_then(Value::as_str)
        .ok_or_else(|| load_error(format!("tensor {name}: missing dtype")))?;
    let shape = obj
        .get("shape")
        .and_then(Value::as_array)
        .ok_or_else(|| load_error(format!("tensor {name}: missing shape")))?
        .iter()
        .map(|d| {
            d.as_u64()
                .and_then(|d| usize::try_from(d).ok())
                .ok_or_else(|| load_error(format!("tensor {name}: invalid shape")))
        })
        .collect::<Result<Vec<usize>, _>>()?;
    let offsets = obj
        .get("data_offsets")
        .and_then(Value::as_array)
        .filter(|a| a.len() == 2)
        .ok_or_else(|| load_error(format!("tensor {name}: invalid data_offsets")))?;
    let (begin, end) = match (offsets[0].as_u64(), offsets[1].as_u64()) {
        (Some(b), Some(e)) => (b, e),
        _ => return Err(load_error(format!("tensor {name}: invalid data_offsets"))),
    };
    if begin > end {
        return Err(
            load_error(format!("tensor {name}: begin offset exceeds end offset")),
        );
    }
    if end > data.len() as u64 {
        return Err(load_error(format!("tensor {name}: end offset beyond data section")));
    }
    Ok(Entry {
        dtype,
        shape,
        bytes: &data[begin as usize..end as usize],
    })
}
fn fetch_from_header<'a>(
    header: &'a Map<String, Value>,
    data: &'a [u8],
    name: &str,
) -> Result<Option<Entry<'a>>, ProjectionError> {
    if name == "__metadata__" {
        return Ok(None);
    }
    match header.get(name) {
        Some(value) => parse_entry(name, value, data).map(Some),
        None => Ok(None),
    }
}
fn fetch_from_file<'a>(
    file: &'a WeightFile,
    name: &str,
) -> Result<Option<Entry<'a>>, ProjectionError> {
    let Some(info) = file.info(name) else {
        return Ok(None);
    };
    let bytes = file
        .raw_bytes(name)
        .map_err(|e| match e {
            WeightError::NotFound { name } => {
                ProjectionError::WeightNameMismatch {
                    name,
                }
            }
            other => load_error(other.to_string()),
        })?;
    Ok(
        Some(Entry {
            dtype: info.dtype.to_safetensors(),
            shape: info.shape.clone(),
            bytes,
        }),
    )
}
impl Projection {
    pub fn from_weight_file(
        file: &WeightFile,
        layer: &ProjectionLayer,
        input_dim: usize,
        output_dim: usize,
    ) -> Result<Projection, ProjectionError> {
        let fetch = |name: &str| fetch_from_file(file, name);
        build(&fetch, layer, input_dim, output_dim).map_err(|e| e.with_path(file.path()))
    }
    pub fn from_file(
        path: &Path,
        layer: &ProjectionLayer,
        input_dim: usize,
        output_dim: usize,
    ) -> Result<Projection, ProjectionError> {
        let file = WeightFile::open(path)
            .map_err(|e| ProjectionError::Load {
                path: Some(path.to_path_buf()),
                reason: e.to_string(),
            })?;
        Projection::from_weight_file(&file, layer, input_dim, output_dim)
    }
    pub fn from_bytes(
        bytes: &[u8],
        layer: &ProjectionLayer,
        input_dim: usize,
        output_dim: usize,
    ) -> Result<Projection, ProjectionError> {
        let Some(prefix) = bytes.get(..8) else {
            return Err(load_error("input shorter than 8 bytes"));
        };
        let mut len_bytes = [0u8; 8];
        len_bytes.copy_from_slice(prefix);
        let header_len = u64::from_le_bytes(len_bytes);
        let data_start = header_len
            .checked_add(8)
            .filter(|end| *end <= bytes.len() as u64)
            .ok_or_else(|| load_error("header length exceeds input size"))? as usize;
        let value: Value = serde_json::from_slice(&bytes[8..data_start])
            .map_err(|e| load_error(format!("header JSON: {e}")))?;
        let header = value
            .as_object()
            .ok_or_else(|| load_error("header is not a JSON object"))?;
        let data = &bytes[data_start..];
        let fetch = |name: &str| fetch_from_header(header, data, name);
        build(&fetch, layer, input_dim, output_dim)
    }
    pub fn input_dim(&self) -> usize {
        self.input_dim
    }
    pub fn output_dim(&self) -> usize {
        self.output_dim
    }
    pub fn apply(&self, vectors: &[Vec<f32>]) -> Result<Vec<Vec<f32>>, ProjectionError> {
        if let Some(bad) = vectors.iter().find(|v| v.len() != self.input_dim) {
            return Err(ProjectionError::InputDimMismatch {
                expected: self.input_dim,
                actual: bad.len(),
            });
        }
        Ok(vectors.iter().map(|v| self.project(v)).collect())
    }
    fn project(&self, v: &[f32]) -> Vec<f32> {
        let mut out = Vec::with_capacity(self.output_dim);
        for row in 0..self.output_dim {
            let w = &self.weight[row * self.input_dim..(row + 1) * self.input_dim];
            let mut acc = 0.0f32;
            for (a, b) in w.iter().zip(v) {
                acc += a * b;
            }
            if let Some(bias) = &self.bias {
                acc += bias[row];
            }
            out.push(acc);
        }
        out
    }
}
#[cfg(test)]
#[path = "test/projection_layer.rs"]
mod tests;
