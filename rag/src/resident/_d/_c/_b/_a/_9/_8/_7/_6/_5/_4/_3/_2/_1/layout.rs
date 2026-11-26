use std::fmt;

pub const MASK_VALUE: f32 = -10_000.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DType {
    F32,
    F16,
    BF16,
    U8,
    U32,
    I64,
}

impl DType {
    pub const fn size_in_bytes(self) -> usize {
        match self {
            DType::F32 => 4,
            DType::F16 => 2,
            DType::BF16 => 2,
            DType::U8 => 1,
            DType::U32 => 4,
            DType::I64 => 8,
        }
    }

    pub fn from_safetensors(dtype: &str, source: &str) -> Result<DType, LayoutError> {
        match dtype {
            "F32" => Ok(DType::F32),
            "F16" => Ok(DType::F16),
            "BF16" => Ok(DType::BF16),
            "U8" => Ok(DType::U8),
            "U32" => Ok(DType::U32),
            "I64" => Ok(DType::I64),
            _ => Err(LayoutError::UnsupportedDType {
                dtype: dtype.to_owned(),
                source: source.to_owned(),
            }),
        }
    }

    pub const fn to_safetensors(self) -> &'static str {
        match self {
            DType::F32 => "F32",
            DType::F16 => "F16",
            DType::BF16 => "BF16",
            DType::U8 => "U8",
            DType::U32 => "U32",
            DType::I64 => "I64",
        }
    }

    pub const fn is_float(self) -> bool {
        matches!(self, DType::F32 | DType::F16 | DType::BF16)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutError {
    UnsupportedDType { dtype: String, source: String },
    RankMismatch { shape_rank: usize, strides_rank: usize },
    DimOutOfRange { dim: usize, rank: usize },
    NarrowOutOfRange { dim: usize, start: usize, len: usize, size: usize },
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayoutError::UnsupportedDType { dtype, source } => {
                write!(f, "nicht unterstützter Datentyp: {dtype} (Quelle: {source})")
            }
            LayoutError::RankMismatch {
                shape_rank,
                strides_rank,
            } => write!(
                f,
                "rank mismatch: shape rank {shape_rank}, strides rank {strides_rank}"
            ),
            LayoutError::DimOutOfRange { dim, rank } => {
                write!(f, "dimension {dim} out of range for rank {rank}")
            }
            LayoutError::NarrowOutOfRange {
                dim,
                start,
                len,
                size,
            } => write!(
                f,
                "narrow out of range: dim {dim}, start {start}, len {len}, size {size}"
            ),
        }
    }
}

impl std::error::Error for LayoutError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Layout {
    shape: Vec<usize>,
    strides: Vec<usize>,
    dtype: DType,
}

fn contiguous_strides(shape: &[usize], dtype: DType) -> Vec<usize> {
    let mut strides = vec![0; shape.len()];
    let mut acc = dtype.size_in_bytes();
    for i in (0..shape.len()).rev() {
        strides[i] = acc;
        acc *= shape[i].max(1);
    }
    strides
}

impl Layout {
    pub fn new(shape: Vec<usize>, strides: Vec<usize>, dtype: DType) -> Result<Layout, LayoutError> {
        if shape.len() != strides.len() {
            return Err(LayoutError::RankMismatch {
                shape_rank: shape.len(),
                strides_rank: strides.len(),
            });
        }
        Ok(Layout {
            shape,
            strides,
            dtype,
        })
    }

    pub fn contiguous(shape: &[usize], dtype: DType) -> Layout {
        Layout {
            shape: shape.to_vec(),
            strides: contiguous_strides(shape, dtype),
            dtype,
        }
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn strides(&self) -> &[usize] {
        &self.strides
    }

    pub fn dtype(&self) -> DType {
        self.dtype
    }

    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    pub fn elem_count(&self) -> usize {
        self.shape.iter().product()
    }

    pub fn byte_len(&self) -> usize {
        self.elem_count() * self.dtype.size_in_bytes()
    }

    pub fn is_contiguous(&self) -> bool {
        let expected = contiguous_strides(&self.shape, self.dtype);
        self.shape
            .iter()
            .zip(self.strides.iter().zip(expected.iter()))
            .all(|(&size, (s, e))| size <= 1 || s == e)
            || self.elem_count() == 0
    }

    pub fn transpose(&self, dim1: usize, dim2: usize) -> Result<Layout, LayoutError> {
        let rank = self.rank();
        for dim in [dim1, dim2] {
            if dim >= rank {
                return Err(LayoutError::DimOutOfRange { dim, rank });
            }
        }
        let mut out = self.clone();
        out.shape.swap(dim1, dim2);
        out.strides.swap(dim1, dim2);
        Ok(out)
    }

    pub fn narrow(&self, dim: usize, start: usize, len: usize) -> Result<Layout, LayoutError> {
        let rank = self.rank();
        if dim >= rank {
            return Err(LayoutError::DimOutOfRange { dim, rank });
        }
        let size = self.shape[dim];
        match start.checked_add(len) {
            Some(end) if end <= size => {}
            _ => {
                return Err(LayoutError::NarrowOutOfRange {
                    dim,
                    start,
                    len,
                    size,
                })
            }
        }
        let mut out = self.clone();
        out.shape[dim] = len;
        Ok(out)
    }

    pub fn narrow_offset_bytes(&self, dim: usize, start: usize) -> Result<usize, LayoutError> {
        let rank = self.rank();
        if dim >= rank {
            return Err(LayoutError::DimOutOfRange { dim, rank });
        }
        let size = self.shape[dim];
        if start > size {
            return Err(LayoutError::NarrowOutOfRange {
                dim,
                start,
                len: 0,
                size,
            });
        }
        Ok(start * self.strides[dim])
    }
}

#[cfg(test)]
#[path = "test/layout.rs"]
mod tests;
