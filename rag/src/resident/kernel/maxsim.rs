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
mod tests {
    use super::*;
    use crate::resident::kernel::InMemorySlots;
    use crate::resident::memory::{ArenaId, Region};
    use crate::resident::subengine::{LaneClass, LaneConfig, RecordingLane};
    fn lane() -> RecordingLane {
        let scratch = Region {
            arena: ArenaId(0),
            offset: 0,
            len: 0,
            gen: 0,
        };
        RecordingLane::new(LaneClass::Query, LaneConfig::new(scratch))
    }
    fn tokens(n: usize, dim: usize, seed: u32) -> Vec<f32> {
        (0..n * dim)
            .map(|i| {
                (((i as u32).wrapping_mul(2654435761).wrapping_add(seed * 97)) % 1000)
                    as f32 / 500.0 - 1.0
            })
            .collect()
    }
    fn source(dim: usize) -> Arc<InMemorySlots> {
        let docs = vec![
            tokens(5, dim, 1), tokens(12, dim, 2), tokens(1, dim, 3), Vec::new(),
            tokens(8, dim, 4)
        ];
        Arc::new(InMemorySlots::new(dim, vec![docs]))
    }
    const R: SliceRange = SliceRange {
        segment: 0,
        slot_from: 0,
        slot_to: 5,
    };
    #[test]
    fn cpu_op_ranks_by_maxsim_and_empty_doc_is_last() {
        let dim = 4;
        let op = MaxSimOp::cpu(source(dim), 5);
        let q = QueryBatch::new(dim, vec![tokens(3, dim, 9)]).unwrap();
        let hits = op.score(&q, R, &mut lane()).unwrap();
        assert_eq!(hits[0].len(), 5);
        assert!(hits[0].windows(2).all(| w | w[0].score >= w[1].score));
        let last = hits[0].last().unwrap();
        assert_eq!(last.offset, source(dim).slot_offset(0, 3));
        assert!(last.score.is_finite());
    }
    #[test]
    fn cpu_op_top_k_limits_results() {
        let dim = 4;
        let op = MaxSimOp::cpu(source(dim), 2);
        let q = QueryBatch::new(dim, vec![tokens(2, dim, 5), tokens(4, dim, 6)])
            .unwrap();
        let hits = op.score(&q, R, &mut lane()).unwrap();
        assert!(hits.iter().all(| h | h.len() == 2));
    }
    #[cfg(feature = "candle")]
    mod candle {
        use super::*;
        use crate::resident::memory::DType;
        fn per_slot(op: &dyn SlotScorer, q: &QueryBatch) -> Vec<Vec<f32>> {
            op.scores(q, R).unwrap()
        }
        #[test]
        fn f32_tile_chain_matches_cpu_reference_incl_padding() {
            let dim = 8;
            let src: Arc<dyn SlotSource> = source(dim);
            let q = QueryBatch::new(dim, vec![tokens(4, dim, 7), tokens(2, dim, 8)])
                .unwrap();
            let want = per_slot(&CpuMaxSim::new(src.clone()), &q);
            for tile in [1usize, 3, 4, 64] {
                let chain = candle_chain::CandleMaxSim::new(
                        src.clone(),
                        candle_core::Device::Cpu,
                        DType::F32,
                        tile,
                    )
                    .unwrap();
                let got = per_slot(&chain, &q);
                for (g, w) in got.iter().flatten().zip(want.iter().flatten()) {
                    assert!((g - w).abs() < 1e-3, "tile={tile} got={g} want={w}");
                }
            }
        }
        #[test]
        fn f16_masking_never_produces_nan() {
            let dim = 8;
            let src: Arc<dyn SlotSource> = source(dim);
            let q = QueryBatch::new(dim, vec![tokens(4, dim, 7)]).unwrap();
            let want = per_slot(&CpuMaxSim::new(src.clone()), &q);
            let chain = candle_chain::CandleMaxSim::new(
                    src,
                    candle_core::Device::Cpu,
                    DType::F16,
                    4,
                )
                .unwrap();
            let got = per_slot(&chain, &q);
            for (g, w) in got[0].iter().zip(&want[0]) {
                assert!(g.is_finite());
                assert!((g - w).abs() < 0.1 + 0.01 * w.abs(), "got={g} want={w}");
            }
        }
        #[test]
        fn op_wraps_candle_chain() {
            let dim = 8;
            let op = MaxSimOp::candle_cpu(source(dim), 3, DType::F32, 4).unwrap();
            let q = QueryBatch::new(dim, vec![tokens(4, dim, 7)]).unwrap();
            assert_eq!(op.score(& q, R, & mut lane()).unwrap() [0].len(), 3);
        }
    }
}
