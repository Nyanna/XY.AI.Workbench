//! Writes encoder output into a slot: L2-normalised and converted to the slot dtype.

use half::{bf16, f16};

use super::pooling::l2_normalize_rows;
use super::{ModelError, RegionIo, TensorRef};
use crate::resident::memory::DType;

pub fn encode_bytes(values: &[f32], dtype: DType) -> Result<Vec<u8>, ModelError> {
    Ok(match dtype {
        DType::F32 => values.iter().flat_map(|v| v.to_le_bytes()).collect(),
        DType::F16 => values.iter().flat_map(|v| f16::from_f32(*v).to_le_bytes()).collect(),
        DType::Bf16 => values.iter().flat_map(|v| bf16::from_f32(*v).to_le_bytes()).collect(),
        other => {
            return Err(ModelError::Incompatible {
                reason: format!("embeddings cannot be stored as {other:?}"),
            })
        }
    })
}

/// Normalises `rows` (row length `dim`) and stores them contiguously into `out`.
pub fn write_slot(
    mem: &dyn RegionIo,
    out: &TensorRef,
    rows: &mut [f32],
    dim: usize,
) -> Result<(), ModelError> {
    l2_normalize_rows(rows, dim);
    let bytes = encode_bytes(rows, out.layout.dtype)?;
    mem.write(out.region, &bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dtype_conversion_sizes() {
        assert_eq!(encode_bytes(&[1.0, 2.0], DType::F32).unwrap().len(), 8);
        assert_eq!(encode_bytes(&[1.0, 2.0], DType::F16).unwrap().len(), 4);
        assert!(encode_bytes(&[1.0], DType::U8).is_err());
    }

    #[test]
    fn f16_roundtrip_is_within_tolerance() {
        let bytes = encode_bytes(&[0.1234567], DType::F16).unwrap();
        let back = f16::from_le_bytes([bytes[0], bytes[1]]).to_f32();
        assert!((back - 0.1234567).abs() < 1e-3);
    }
}
