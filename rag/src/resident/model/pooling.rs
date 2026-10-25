//! Host reference for masked mean pooling and L2 normalisation.
const L2_EPS: f32 = 1e-12;
/// `hidden` is `[batch, seq, dim]`, `mask` is `[batch, seq]` with 0/1 entries.
pub fn mean_pool(
    hidden: &[f32],
    mask: &[u32],
    batch: usize,
    seq: usize,
    dim: usize,
) -> Vec<f32> {
    debug_assert_eq!(hidden.len(), batch * seq * dim);
    debug_assert_eq!(mask.len(), batch * seq);
    let mut out = vec![0f32; batch * dim];
    for b in 0..batch {
        let acc = &mut out[b * dim..(b + 1) * dim];
        let mut count = 0u32;
        for t in 0..seq {
            if mask[b * seq + t] == 0 {
                continue;
            }
            count += 1;
            let row = &hidden[(b * seq + t) * dim..(b * seq + t + 1) * dim];
            acc.iter_mut().zip(row).for_each(|(a, x)| *a += x);
        }
        let inv = 1.0 / count.max(1) as f32;
        acc.iter_mut().for_each(|a| *a *= inv);
    }
    out
}
/// Normalises each `dim`-sized row in place; all-zero rows stay zero and are counted in the result.
pub fn l2_normalize_rows(rows: &mut [f32], dim: usize) -> usize {
    let mut degenerate = 0;
    for row in rows.chunks_exact_mut(dim) {
        let norm = row.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm < L2_EPS {
            degenerate += 1;
        }
        let norm = norm.max(L2_EPS);
        row.iter_mut().for_each(|x| *x /= norm);
    }
    degenerate
}
#[cfg(test)]
#[path = "test/pooling.rs"]
mod tests;
