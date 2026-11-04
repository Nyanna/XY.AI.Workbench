//! Token-budget batching with length bucketing.

pub const BUCKETS: [usize; 5] = [32, 64, 128, 256, 512];

#[derive(Clone, Copy, Debug)]
pub struct BatchConfig {
    /// Upper bound for `batch * seq_len` of one batch.
    pub token_budget: usize,
    pub max_seq_len: usize,
    pub pad_token_id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Batch {
    /// Positions of the rows in the caller's input order.
    pub indices: Vec<usize>,
    pub seq_len: usize,
    /// Row-major `[rows, seq_len]`.
    pub ids: Vec<u32>,
    /// Row-major `[rows, seq_len]`, 1 for real tokens and 0 for padding.
    pub mask: Vec<u32>,
}

impl Batch {
    pub fn rows(&self) -> usize {
        self.indices.len()
    }

    /// Little-endian `ids` followed by `mask`; matches a `[2, rows, seq_len]` u32 layout.
    pub fn pack(&self) -> Vec<u8> {
        self.ids.iter().chain(&self.mask).flat_map(|v| v.to_le_bytes()).collect()
    }
}

fn bucket_len(len: usize, max_seq_len: usize) -> usize {
    let bucket = BUCKETS.iter().copied().find(|b| *b >= len).unwrap_or(max_seq_len);
    bucket.min(max_seq_len).max(1)
}

/// Sorts by length, truncates to `max_seq_len` and groups rows so that no batch exceeds the
/// token budget. A single row larger than the budget forms its own batch.
pub fn plan_batches(seqs: &[Vec<u32>], cfg: &BatchConfig) -> Vec<Batch> {
    let mut order: Vec<usize> = (0..seqs.len()).collect();
    order.sort_by_key(|i| seqs[*i].len());

    let mut batches = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut current_len = 0usize;
    for i in order {
        let len = bucket_len(seqs[i].len().min(cfg.max_seq_len), cfg.max_seq_len);
        if !current.is_empty() && (current.len() + 1) * len > cfg.token_budget {
            batches.push(build(&current, current_len, seqs, cfg));
            current.clear();
        }
        current.push(i);
        current_len = len;
    }
    if !current.is_empty() {
        batches.push(build(&current, current_len, seqs, cfg));
    }
    batches
}

fn build(indices: &[usize], seq_len: usize, seqs: &[Vec<u32>], cfg: &BatchConfig) -> Batch {
    let mut ids = Vec::with_capacity(indices.len() * seq_len);
    let mut mask = Vec::with_capacity(indices.len() * seq_len);
    for &i in indices {
        let real = seqs[i].len().min(seq_len);
        ids.extend_from_slice(&seqs[i][..real]);
        ids.resize(ids.len() + seq_len - real, cfg.pad_token_id);
        mask.extend(std::iter::repeat(1).take(real));
        mask.extend(std::iter::repeat(0).take(seq_len - real));
    }
    Batch { indices: indices.to_vec(), seq_len, ids, mask }
}

#[cfg(test)]
#[path = "test/batch.rs"]
mod tests;
