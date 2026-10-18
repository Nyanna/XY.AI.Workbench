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
mod tests {
    use super::*;

    #[test]
    fn sizes() {
        assert_eq!(DType::U8.size_bytes(), 1);
        assert_eq!(DType::F16.size_bytes(), 2);
        assert_eq!(DType::Bf16.size_bytes(), 2);
        assert_eq!(DType::F32.size_bytes(), 4);
        assert_eq!(DType::U32.size_bytes(), 4);
        assert_eq!(DType::I64.size_bytes(), 8);
    }

    #[test]
    fn safetensors_convert_and_reject() {
        assert_eq!(DType::try_from(safetensors::Dtype::F16), Ok(DType::F16));
        assert!(DType::try_from(safetensors::Dtype::F64).is_err());
    }

    #[cfg(feature = "candle")]
    #[test]
    fn candle_roundtrip_and_reject() {
        for d in [DType::F32, DType::F16, DType::Bf16, DType::U8, DType::U32, DType::I64] {
            let c = candle_core::DType::try_from(d).unwrap();
            assert_eq!(DType::try_from(c), Ok(d));
        }
        assert!(DType::try_from(candle_core::DType::F64).is_err());
    }
}
