//! Writes encoder output into a slot: L2-normalised and converted to the slot dtype.
use half::{bf16, f16};
use super::pooling::l2_normalize_rows;
use super::{ModelError, Normalize, RegionIo, TensorRef};
use crate::resident::memory::DType;
pub fn encode_bytes(values: &[f32], dtype: DType) -> Result<Vec<u8>, ModelError> {
    Ok(
        match dtype {
            DType::F32 => values.iter().flat_map(|v| v.to_le_bytes()).collect(),
            DType::F16 => {
                values.iter().flat_map(|v| f16::from_f32(*v).to_le_bytes()).collect()
            }
            DType::Bf16 => {
                values.iter().flat_map(|v| bf16::from_f32(*v).to_le_bytes()).collect()
            }
            other => {
                return Err(ModelError::Incompatible {
                    reason: format!("embeddings cannot be stored as {other:?}"),
                });
            }
        },
    )
}
/// Normalises `rows` (row length `dim`) as requested and stores them contiguously into `out`.
pub fn write_slot(
    mem: &dyn RegionIo,
    out: &TensorRef,
    rows: &mut [f32],
    dim: usize,
    normalize: Normalize,
) -> Result<(), ModelError> {
    if normalize == Normalize::L2 {
        let empty = l2_normalize_rows(rows, dim);
        if empty > 0 {
            tracing::warn!(empty, "empty embedding stored as zero vector");
        }
    }
    let bytes = encode_bytes(rows, out.layout.dtype)?;
    mem.write(out.region, &bytes)?;
    Ok(())
}
#[cfg(test)]
#[path = "test/slot.rs"]
mod tests;
