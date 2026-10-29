//! Late-interaction scoring (ColBERT-style MaxSim) behind one op; the implementation is
//! swappable (CPU SIMD, Candle tile chain, fused kernel).
use std::sync::Arc;
use super::cpu_simd::CpuMaxSim;
use super::{
    KernelHit, KernelError, QueryBatch, Scorer, SliceRange, SlotScorer, SlotSource,
    TopKScorer,
};
use crate::resident::subengine::ExecContext;
impl SlotScorer for Box<dyn SlotScorer> {
    fn scores(
        &self,
        queries: &QueryBatch,
        range: SliceRange,
    ) -> Result<Vec<Vec<f32>>, KernelError> {
        (**self).scores(queries, range)
    }
    fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
        (**self).slot_offset(segment, slot)
    }
}
pub struct MaxSimOp {
    inner: TopKScorer<Box<dyn SlotScorer>>,
}
impl MaxSimOp {
    pub fn with_slots(slots: Box<dyn SlotScorer>, k: usize) -> Self {
        Self {
            inner: TopKScorer::new(slots, k),
        }
    }
    pub fn cpu(source: Arc<dyn SlotSource>, k: usize) -> Self {
        Self::with_slots(Box::new(CpuMaxSim::new(source)), k)
    }
    #[cfg(feature = "candle")]
    pub fn candle_cpu(
        source: Arc<dyn SlotSource>,
        k: usize,
        dtype: crate::resident::memory::DType,
        tile_tokens: usize,
    ) -> Result<Self, KernelError> {
        let chain = candle_chain::CandleMaxSim::new(
            source,
            candle_core::Device::Cpu,
            dtype,
            tile_tokens,
        )?;
        Ok(Self::with_slots(Box::new(chain), k))
    }
    #[cfg(feature = "cuda")]
    pub fn candle_cuda(
        source: Arc<dyn SlotSource>,
        k: usize,
        ordinal: usize,
        dtype: crate::resident::memory::DType,
        tile_tokens: usize,
    ) -> Result<Self, KernelError> {
        let device = candle_core::Device::new_cuda(ordinal)
            .map_err(|e| KernelError::Unavailable {
                reason: e.to_string(),
            })?;
        let chain = candle_chain::CandleMaxSim::new(source, device, dtype, tile_tokens)?;
        Ok(Self::with_slots(Box::new(chain), k))
    }
}
impl Scorer for MaxSimOp {
    fn score(
        &self,
        queries: &QueryBatch,
        range: SliceRange,
        lane: &mut dyn ExecContext,
    ) -> Result<Vec<Vec<KernelHit>>, KernelError> {
        self.inner.score(queries, range, lane)
    }
}
#[cfg(feature = "candle")]
pub(crate) mod candle_chain {
    use std::sync::Arc;
    use candle_core::{DType as CDType, Device, Tensor};
    use crate::resident::kernel::slots::check_range;
    use crate::resident::kernel::{
        KernelError, QueryBatch, SliceRange, SlotScorer, SlotSource,
    };
    use crate::resident::memory::{DType, F16_MASK_VALUE};
    fn fail(e: candle_core::Error) -> KernelError {
        KernelError::LaunchFailed {
            source: Box::new(e),
        }
    }
    struct Tile {
        /// `dim x tile`, already transposed for the matmul.
        docs_t: Tensor,
        /// Additive mask for padded columns of the last tile.
        mask: Option<Tensor>,
    }
    /// `matmul -> mask -> max` per document tile with a running maximum, then a sum over query
    /// tokens. Intermediate similarity matrices stay at `query_tokens x tile`.
    pub struct CandleMaxSim {
        source: Arc<dyn SlotSource>,
        device: Device,
        dtype: CDType,
        tile: usize,
    }
    impl CandleMaxSim {
        pub fn new(
            source: Arc<dyn SlotSource>,
            device: Device,
            dtype: DType,
            tile: usize,
        ) -> Result<Self, KernelError> {
            let dtype = CDType::try_from(dtype)
                .map_err(|e| KernelError::Unavailable {
                    reason: e.to_string(),
                })?;
            Ok(Self {
                source,
                device,
                dtype,
                tile: tile.max(1),
            })
        }
        fn tiles(&self, doc: &[f32], dim: usize) -> candle_core::Result<Vec<Tile>> {
            let tokens = doc.len() / dim;
            let mut tiles = Vec::new();
            for start in (0..tokens).step_by(self.tile) {
                let take = self.tile.min(tokens - start);
                let mut buf = vec![0f32; self.tile * dim];
                buf[..take * dim]
                    .copy_from_slice(&doc[start * dim..(start + take) * dim]);
                let docs_t = Tensor::from_vec(buf, (self.tile, dim), &self.device)?
                    .to_dtype(self.dtype)?
                    .t()?
                    .contiguous()?;
                let mask = if take < self.tile {
                    let m: Vec<f32> = (0..self.tile)
                        .map(|i| if i < take { 0.0 } else { F16_MASK_VALUE })
                        .collect();
                    Some(
                        Tensor::from_vec(m, (1, self.tile), &self.device)?
                            .to_dtype(self.dtype)?,
                    )
                } else {
                    None
                };
                tiles.push(Tile { docs_t, mask });
            }
            Ok(tiles)
        }
        fn score(&self, query: &Tensor, tiles: &[Tile]) -> candle_core::Result<f32> {
            let mut running: Option<Tensor> = None;
            for t in tiles {
                let mut sim = query.matmul(&t.docs_t)?;
                if let Some(mask) = &t.mask {
                    sim = sim.broadcast_add(mask)?;
                }
                let best = sim.max(1)?;
                running = Some(
                    match running {
                        None => best,
                        Some(r) => r.maximum(&best)?,
                    },
                );
            }
            match running {
                Some(r) => r.to_dtype(CDType::F32)?.sum_all()?.to_scalar::<f32>(),
                None => Ok(F16_MASK_VALUE * query.dim(0)? as f32),
            }
        }
    }
    impl SlotScorer for CandleMaxSim {
        fn scores(
            &self,
            queries: &QueryBatch,
            range: SliceRange,
        ) -> Result<Vec<Vec<f32>>, KernelError> {
            check_range(self.source.as_ref(), range)?;
            let dim = queries.dim();
            if dim != self.source.dim() {
                return Err(KernelError::ShapeMismatch {
                    expected: self.source.dim(),
                    actual: dim,
                });
            }
            let qs = (0..queries.len())
                .map(|i| {
                    Tensor::from_slice(
                            queries.data(i),
                            (queries.tokens(i), dim),
                            &self.device,
                        )?
                        .to_dtype(self.dtype)
                })
                .collect::<candle_core::Result<Vec<_>>>()
                .map_err(fail)?;
            let mut out = vec![Vec::with_capacity(range.len() as usize); qs.len()];
            for slot in range.slot_from..range.slot_to {
                let doc = self.source.slot_vectors(range.segment, slot).unwrap_or(&[]);
                if doc.len() % dim != 0 {
                    return Err(KernelError::ShapeMismatch {
                        expected: dim,
                        actual: doc.len(),
                    });
                }
                let tiles = self.tiles(doc, dim).map_err(fail)?;
                for (qi, q) in qs.iter().enumerate() {
                    out[qi].push(self.score(q, &tiles).map_err(fail)?);
                }
            }
            Ok(out)
        }
        fn slot_offset(&self, segment: u32, slot: u32) -> u64 {
            self.source.slot_offset(segment, slot)
        }
    }
}
#[cfg(test)]
#[path = "test/maxsim.rs"]
mod tests;
