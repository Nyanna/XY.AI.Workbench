//! Backbone-agnostic encode pipeline: tokens -> hidden states -> pooling -> slot.

use std::sync::Arc;

use parking_lot::Mutex;

use super::head::ProjectionHead;
use super::pooling::mean_pool;
use super::slot::write_slot;
use super::{EncodeEvent, Encoder, ModelError, Pooling, RegionIo, TensorRef};
use crate::resident::memory::DType;
use crate::resident::subengine::ExecContext;

/// Produces token-level hidden states on the host; framework types never cross this boundary.
pub trait Backbone: Send + Sync + 'static {
    fn dim(&self) -> usize;

    /// `ids` and `mask` are `[batch, seq]`; the result is `[batch, seq, dim]`.
    fn forward(&self, ids: &[u32], mask: &[u32], batch: usize, seq: usize) -> Result<Vec<f32>, ModelError>;
}

pub struct PooledEncoder<B: Backbone> {
    backbone: Arc<B>,
    head: Option<Arc<ProjectionHead>>,
    pooling: Pooling,
    dtype: DType,
}

impl<B: Backbone> PooledEncoder<B> {
    pub fn new(
        backbone: B,
        head: Option<ProjectionHead>,
        pooling: Pooling,
        dtype: DType,
    ) -> Result<Self, ModelError> {
        if let Some(h) = &head {
            if h.in_dim() != backbone.dim() {
                return Err(ModelError::Incompatible {
                    reason: format!("head expects {} inputs, backbone yields {}", h.in_dim(), backbone.dim()),
                });
            }
        }
        Ok(Self { backbone: Arc::new(backbone), head: head.map(Arc::new), pooling, dtype })
    }
}

fn token_dims(tokens: &TensorRef) -> Result<(usize, usize), ModelError> {
    match tokens.layout.shape.as_slice() {
        [2, b, t] if tokens.layout.dtype == DType::U32 => Ok((*b, *t)),
        other => Err(ModelError::Incompatible {
            reason: format!("tokens must be u32 [2, batch, seq], got {other:?} {:?}", tokens.layout.dtype),
        }),
    }
}

fn run(
    backbone: &dyn Backbone,
    head: Option<&ProjectionHead>,
    pooling: Pooling,
    mem: &dyn RegionIo,
    tokens: &TensorRef,
    out: &TensorRef,
    dim: usize,
) -> Result<(), ModelError> {
    let (batch, seq) = token_dims(tokens)?;
    let mut raw = vec![0u8; 2 * batch * seq * 4];
    mem.read(tokens.region, &mut raw)?;
    let words: Vec<u32> = raw.chunks_exact(4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect();
    let (ids, mask) = words.split_at(batch * seq);

    let mut hidden = backbone.forward(ids, mask, batch, seq)?;
    if let Some(h) = head {
        hidden = h.apply(&hidden);
    }
    let mut rows = match pooling {
        Pooling::MeanMasked => mean_pool(&hidden, mask, batch, seq, dim),
        Pooling::PerToken => hidden,
    };
    write_slot(mem, out, &mut rows, dim)
}

impl<B: Backbone> Encoder for PooledEncoder<B> {
    fn dim(&self) -> usize {
        self.head.as_ref().map_or_else(|| self.backbone.dim(), |h| h.out_dim())
    }

    fn dtype(&self) -> DType {
        self.dtype
    }

    fn encode_into(
        &self,
        mem: &Arc<dyn RegionIo>,
        tokens: &TensorRef,
        out: &TensorRef,
        lane: &dyn ExecContext,
    ) -> Result<EncodeEvent, ModelError> {
        let (batch, seq) = token_dims(tokens)?;
        let dim = self.dim();
        let expected: &[usize] = &match self.pooling {
            Pooling::MeanMasked => vec![batch, dim],
            Pooling::PerToken => vec![batch, seq, dim],
        };
        if out.layout.shape != expected || out.layout.dtype != self.dtype {
            return Err(ModelError::Incompatible {
                reason: format!(
                    "output must be {:?} {:?}, got {:?} {:?}",
                    expected, self.dtype, out.layout.shape, out.layout.dtype
                ),
            });
        }

        let failure = Arc::new(Mutex::new(None));
        let job = {
            let (backbone, head, pooling) = (self.backbone.clone(), self.head.clone(), self.pooling);
            let (mem, tokens, out, failure) = (mem.clone(), tokens.clone(), out.clone(), failure.clone());
            move || {
                if let Err(e) = run(&*backbone, head.as_deref(), pooling, &*mem, &tokens, &out, dim) {
                    *failure.lock() = Some(e);
                }
                Ok(())
            }
        };
        lane.submit(Box::new(job))?;
        Ok(EncodeEvent::new(lane.record_event()?, failure))
    }
}
