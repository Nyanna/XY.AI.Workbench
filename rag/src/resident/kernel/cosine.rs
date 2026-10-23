//! Dense scoring path: cosine similarity as the dot product of L2-normalised vectors.
use std::sync::Arc;
use super::cpu_simd::{dot, scan_slots};
use super::{KernelError, QueryBatch, SliceRange, SlotScorer, SlotSource, TopKScorer};
pub struct CosineSlots {
    source: Arc<dyn SlotSource>,
}
impl CosineSlots {
    pub fn new(source: Arc<dyn SlotSource>) -> Self {
        Self { source }
    }
}
impl SlotScorer for CosineSlots {
    fn scores(
        &self,
        queries: &QueryBatch,
        range: SliceRange,
    ) -> Result<Vec<Vec<f32>>, KernelError> {
        let dim = queries.dim();
        for qi in 0..queries.len() {
            if queries.tokens(qi) != 1 {
                return Err(KernelError::ShapeMismatch {
                    expected: dim,
                    actual: queries.data(qi).len(),
                });
            }
        }
        scan_slots(
            self.source.as_ref(),
            queries,
            range,
            |q, d| {
                if d.len() != dim {
                    return Err(KernelError::ShapeMismatch {
                        expected: dim,
                        actual: d.len(),
                    });
                }
                Ok(dot(q, d))
            },
        )
    }
    fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
        self.source.slot_offset(segment, slot)
    }
}
pub type CosineScorer = TopKScorer<CosineSlots>;
pub fn cosine_scorer(source: Arc<dyn SlotSource>, k: usize) -> CosineScorer {
    TopKScorer::new(CosineSlots::new(source), k)
}
#[cfg(test)]
#[path = "test/cosine.rs"]
mod tests;
