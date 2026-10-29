//! Engine-owned element type; backend dtypes are converted at the edge only.

use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
#[error("unsupported dtype: {0}")]
pub struct UnsupportedDType(pub &'static str);

/// Additive attention-mask value that stays finite in f16 (`-inf` would turn rows into NaN).
pub const F16_MASK_VALUE: f32 = -1e4;

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DType {
    F32,
    F16,
    Bf16,
    U8,
    U32,
    I64,
}

impl DType {
    pub const fn size_bytes(self) -> usize {
        match self {
            DType::U8 => 1,
            DType::F16 | DType::Bf16 => 2,
            DType::F32 | DType::U32 => 4,
            DType::I64 => 8,
        }
    }
}

impl TryFrom<safetensors::Dtype> for DType {
    type Error = UnsupportedDType;

    fn try_from(d: safetensors::Dtype) -> Result<Self, Self::Error> {
        use safetensors::Dtype as S;
        match d {
            S::F32 => Ok(DType::F32),
            S::F16 => Ok(DType::F16),
            S::BF16 => Ok(DType::Bf16),
            S::U8 => Ok(DType::U8),
            S::U32 => Ok(DType::U32),
            S::I64 => Ok(DType::I64),
            _ => Err(UnsupportedDType("safetensors")),
        }
    }
}

#[cfg(feature = "candle")]
pub(crate) mod candle_adapter {
    use super::{DType, UnsupportedDType};
    use candle_core::DType as C;

    impl TryFrom<DType> for C {
        type Error = UnsupportedDType;

        fn try_from(d: DType) -> Result<Self, Self::Error> {
            match d {
                DType::F32 => Ok(C::F32),
                DType::F16 => Ok(C::F16),
                DType::Bf16 => Ok(C::BF16),
                DType::U8 => Ok(C::U8),
                DType::U32 => Ok(C::U32),
                DType::I64 => Ok(C::I64),
            }
        }
    }

    impl TryFrom<C> for DType {
        type Error = UnsupportedDType;

        fn try_from(d: C) -> Result<Self, Self::Error> {
            match d {
                C::F32 => Ok(DType::F32),
                C::F16 => Ok(DType::F16),
                C::BF16 => Ok(DType::Bf16),
                C::U8 => Ok(DType::U8),
                C::U32 => Ok(DType::U32),
                C::I64 => Ok(DType::I64),
                _ => Err(UnsupportedDType("candle")),
            }
        }
    }

    /// Candle layouts count elements; the engine counts bytes.
    // TODO: confirm the bytes-vs-elements convention against candle narrow/transpose/permute.
    pub(crate) fn elems_to_bytes(elems: usize, dtype: DType) -> usize {
        elems * dtype.size_bytes()
    }

    pub(crate) fn bytes_to_elems(bytes: usize, dtype: DType) -> usize {
        bytes / dtype.size_bytes()
    }
}

#[cfg(test)]
#[path = "test/dtype.rs"]
mod tests;
