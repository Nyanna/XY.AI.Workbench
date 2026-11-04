use std::time::Duration;

pub const BUCKETS: [usize; 5] = [32, 64, 128, 256, 512];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchError {
    /// token_budget == 0 or max_seq_len == 0
    InvalidConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchConfig {
    pub token_budget: usize,
    pub max_seq_len: usize,
    pub pad_token_id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    rows: usize,
    seq_len: usize,
    input_ids: Vec<u32>,
    attention_mask: Vec<u32>,
    positions: Vec<u32>,
}

impl Batch {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn seq_len(&self) -> usize {
        self.seq_len
    }

    pub fn input_ids(&self) -> &[u32] {
        &self.input_ids
    }

    pub fn attention_mask(&self) -> &[u32] {
        &self.attention_mask
    }

    pub fn positions(&self) -> &[u32] {
        &self.positions
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity((self.input_ids.len() + self.attention_mask.len()) * 4);
        for v in self.input_ids.iter().chain(self.attention_mask.iter()) {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }

    pub fn positions_to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.positions.len() * 4);
        for v in &self.positions {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }
}

pub fn padded_len(len: usize, max_seq_len: usize) -> usize {
    let t = len.min(max_seq_len);
    let p = BUCKETS.iter().copied().find(|&b| b >= t).unwrap_or(max_seq_len);
    p.min(max_seq_len).max(1)
}

fn finish_batch(
    seqs: &[Vec<u32>],
    members: &[usize],
    cfg: &BatchConfig,
) -> Batch {
    let last = members.last().copied().unwrap_or(0);
    let seq_len = padded_len(seqs[last].len(), cfg.max_seq_len);
    let rows = members.len();
    let mut input_ids = Vec::with_capacity(rows * seq_len);
    let mut attention_mask = Vec::with_capacity(rows * seq_len);
    let mut positions = Vec::with_capacity(rows);
    for &idx in members {
        let seq = &seqs[idx];
        let real = seq.len().min(cfg.max_seq_len).min(seq_len);
        input_ids.extend_from_slice(&seq[..real]);
        attention_mask.extend(std::iter::repeat(1u32).take(real));
        let pad = seq_len - real;
        input_ids.extend(std::iter::repeat(cfg.pad_token_id).take(pad));
        attention_mask.extend(std::iter::repeat(0u32).take(pad));
        positions.push(idx as u32);
    }
    Batch {
        rows,
        seq_len,
        input_ids,
        attention_mask,
        positions,
    }
}

pub fn build_batches(seqs: &[Vec<u32>], cfg: &BatchConfig) -> Result<Vec<Batch>, BatchError> {
    if cfg.token_budget == 0 || cfg.max_seq_len == 0 {
        return Err(BatchError::InvalidConfig);
    }
    let mut order: Vec<usize> = (0..seqs.len()).collect();
    order.sort_by_key(|&i| seqs[i].len());

    let mut batches = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    for idx in order {
        let p = padded_len(seqs[idx].len(), cfg.max_seq_len);
        if !current.is_empty() && (current.len() + 1).saturating_mul(p) > cfg.token_budget {
            batches.push(finish_batch(seqs, &current, cfg));
            current.clear();
        }
        current.push(idx);
    }
    if !current.is_empty() {
        batches.push(finish_batch(seqs, &current, cfg));
    }
    Ok(batches)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchClass {
    Query,
    Bulk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassDeadline {
    pub max_batch_duration: Option<Duration>,
    pub cost_per_token: Duration,
}

pub fn batch_size_for(token_budget: usize, bucket_len: usize, deadline: &ClassDeadline) -> usize {
    let by_budget = if bucket_len == 0 {
        1
    } else {
        (token_budget / bucket_len).max(1)
    };
    let Some(d) = deadline.max_batch_duration else {
        return by_budget;
    };
    let denom = (bucket_len as u128).saturating_mul(deadline.cost_per_token.as_nanos());
    if denom == 0 {
        return by_budget;
    }
    let by_deadline = usize::try_from(d.as_nanos() / denom)
        .unwrap_or(usize::MAX)
        .max(1);
    by_budget.min(by_deadline)
}

#[cfg(test)]
#[path = "test/batching.rs"]
mod tests;
