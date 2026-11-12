use std::fmt;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::{
    ModelDescriptor, Normalization, Pooling,
};

pub const NORM_EPSILON: f32 = 1e-12;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PoolingKind {
    Mean,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoolNormError {
    UnsupportedPooling(String),
    ShapeMismatch(String),
    ZeroDimension,
}

impl fmt::Display for PoolNormError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PoolNormError::UnsupportedPooling(s) => write!(f, "nicht unterstütztes Pooling: {s}"),
            PoolNormError::ShapeMismatch(s) => write!(f, "ungültige Form: {s}"),
            PoolNormError::ZeroDimension => write!(f, "dimension muss größer als 0 sein"),
        }
    }
}

impl std::error::Error for PoolNormError {}

#[derive(Debug, Clone, PartialEq)]
pub struct PoolNormOutput {
    pub vectors: Vec<f32>,
    pub num_vectors: usize,
    pub degenerate_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolNorm {
    pub pooling: PoolingKind,
    pub normalization: Normalization,
    pub dim: usize,
}

impl PoolNorm {
    pub fn new(
        pooling: PoolingKind,
        normalization: Normalization,
        dim: usize,
    ) -> Result<Self, PoolNormError> {
        if dim == 0 {
            return Err(PoolNormError::ZeroDimension);
        }
        Ok(Self { pooling, normalization, dim })
    }

    pub fn from_descriptor(d: &ModelDescriptor) -> Result<Self, PoolNormError> {
        let pooling = match d.pooling {
            Pooling::Mean => PoolingKind::Mean,
            other => return Err(PoolNormError::UnsupportedPooling(format!("{other:?}"))),
        };
        Self::new(pooling, d.normalization, d.dimension)
    }

    fn check_shapes(
        &self,
        hidden: &[f32],
        mask: &[u8],
        batch: usize,
        seq: usize,
    ) -> Result<(), PoolNormError> {
        if self.dim == 0 {
            return Err(PoolNormError::ZeroDimension);
        }
        let tokens = batch.checked_mul(seq).ok_or_else(|| {
            PoolNormError::ShapeMismatch(format!("batch {batch} * seq {seq} overflows"))
        })?;
        let expected = tokens.checked_mul(self.dim).ok_or_else(|| {
            PoolNormError::ShapeMismatch(format!("batch*seq {tokens} * dim {} overflows", self.dim))
        })?;
        if hidden.len() != expected {
            return Err(PoolNormError::ShapeMismatch(format!(
                "hidden length {} != batch*seq*dim {}",
                hidden.len(),
                expected
            )));
        }
        if mask.len() != tokens {
            return Err(PoolNormError::ShapeMismatch(format!(
                "mask length {} != batch*seq {}",
                mask.len(),
                tokens
            )));
        }
        Ok(())
    }

    pub fn pool(
        &self,
        hidden: &[f32],
        mask: &[u8],
        batch: usize,
        seq: usize,
    ) -> Result<Vec<f32>, PoolNormError> {
        self.check_shapes(hidden, mask, batch, seq)?;
        match self.pooling {
            PoolingKind::None => Ok(hidden.to_vec()),
            PoolingKind::Mean => {
                let dim = self.dim;
                let mut out = vec![0.0f32; batch * dim];
                let mut acc = vec![0.0f64; dim];
                for b in 0..batch {
                    acc.iter_mut().for_each(|a| *a = 0.0);
                    let mut count = 0usize;
                    for s in 0..seq {
                        let t = b * seq + s;
                        if mask[t] == 0 {
                            continue;
                        }
                        count += 1;
                        let row = &hidden[t * dim..(t + 1) * dim];
                        for (a, v) in acc.iter_mut().zip(row) {
                            *a += f64::from(*v);
                        }
                    }
                    if count > 0 {
                        let n = count as f64;
                        let dst = &mut out[b * dim..(b + 1) * dim];
                        for (d, a) in dst.iter_mut().zip(&acc) {
                            *d = (*a / n) as f32;
                        }
                    }
                }
                Ok(out)
            }
        }
    }

    pub fn normalize(&self, vectors: &mut [f32]) -> Result<usize, PoolNormError> {
        if self.dim == 0 {
            return Err(PoolNormError::ZeroDimension);
        }
        if vectors.len() % self.dim != 0 {
            return Err(PoolNormError::ShapeMismatch(format!(
                "vectors length {} is not a multiple of dim {}",
                vectors.len(),
                self.dim
            )));
        }
        if self.normalization == Normalization::None {
            return Ok(0);
        }
        let eps = f64::from(NORM_EPSILON);
        let mut degenerate = 0usize;
        for v in vectors.chunks_exact_mut(self.dim) {
            let sum: f64 = v.iter().map(|x| f64::from(*x) * f64::from(*x)).sum();
            let n = sum.sqrt();
            if n.is_nan() {
                continue;
            }
            if n < eps {
                degenerate += 1;
            }
            let n_c = n.max(eps);
            for x in v.iter_mut() {
                *x = (f64::from(*x) / n_c) as f32;
            }
        }
        Ok(degenerate)
    }

    pub fn apply(
        &self,
        hidden: &[f32],
        mask: &[u8],
        batch: usize,
        seq: usize,
    ) -> Result<PoolNormOutput, PoolNormError> {
        let mut vectors = self.pool(hidden, mask, batch, seq)?;
        let degenerate_count = self.normalize(&mut vectors)?;
        let num_vectors = vectors.len() / self.dim;
        Ok(PoolNormOutput { vectors, num_vectors, degenerate_count })
    }
}

static_assertions::assert_impl_all!(PoolNorm: Send, Sync);
static_assertions::assert_impl_all!(PoolNormOutput: Send, Sync);
static_assertions::assert_impl_all!(PoolNormError: Send, Sync);

#[cfg(test)]
#[path = "test/pooling_norm.rs"]
mod tests;
