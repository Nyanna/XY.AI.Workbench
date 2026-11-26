use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

use super::_5::_4::_3::a::kernel_layout::Slot;
use super::_5::query_batch::{check_slot_range, DocSource, SegmentId, SourceError, Workspace};

/// One hit: byte offset of the slot and its score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    pub offset: u64,
    pub score: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopKError {
    Source(SourceError),
    ScoreLen { expected: usize, got: usize },
    Mismatch { what: &'static str },
}

impl fmt::Display for TopKError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TopKError::Source(e) => write!(f, "source error: {e}"),
            TopKError::ScoreLen { expected, got } => {
                write!(f, "score length mismatch: expected {expected}, got {got}")
            }
            TopKError::Mismatch { what } => write!(f, "cannot merge: {what} differs"),
        }
    }
}

impl std::error::Error for TopKError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TopKError::Source(e) => Some(e),
            _ => None,
        }
    }
}

impl From<SourceError> for TopKError {
    fn from(e: SourceError) -> Self {
        TopKError::Source(e)
    }
}

/// Heap entry ordered so that the worst hit compares greatest.
#[derive(Debug, Clone, Copy)]
struct Entry {
    offset: u64,
    score: f32,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .partial_cmp(&self.score)
            .unwrap_or(Ordering::Equal)
            .then(self.offset.cmp(&other.offset))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Entry {}

/// Bounded set of the best hits of one query.
#[derive(Debug, Clone)]
pub struct TopK {
    k: usize,
    heap: BinaryHeap<Entry>,
}

impl TopK {
    pub fn new(k: usize) -> TopK {
        TopK {
            k,
            heap: BinaryHeap::new(),
        }
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Returns true if the hit was kept, including by displacing the worst one.
    pub fn push(&mut self, offset: u64, score: f32) -> bool {
        if self.k == 0 || score.is_nan() {
            return false;
        }
        let candidate = Entry { offset, score };
        if self.heap.len() < self.k {
            self.heap.push(candidate);
            return true;
        }
        match self.heap.peek_mut() {
            Some(mut worst) if candidate < *worst => {
                *worst = candidate;
                true
            }
            _ => false,
        }
    }

    pub fn extend<I: IntoIterator<Item = (u64, f32)>>(&mut self, items: I) {
        for (offset, score) in items {
            self.push(offset, score);
        }
    }

    /// Takes all hits of `other`; `k` of `self` is kept.
    pub fn merge(&mut self, other: TopK) {
        for e in other.heap {
            self.push(e.offset, e.score);
        }
    }

    /// Best hit first.
    pub fn into_sorted_vec(self) -> Vec<Hit> {
        self.heap
            .into_sorted_vec()
            .into_iter()
            .map(|e| Hit {
                offset: e.offset,
                score: e.score,
            })
            .collect()
    }
}

/// Top-k per query of a batch, accumulated over several slot ranges.
#[derive(Debug, Clone)]
pub struct RangeScorer {
    tops: Vec<TopK>,
    k: usize,
}

impl RangeScorer {
    pub fn new(num_queries: usize, k: usize) -> RangeScorer {
        RangeScorer {
            tops: (0..num_queries).map(|_| TopK::new(k)).collect(),
            k,
        }
    }

    pub fn num_queries(&self) -> usize {
        self.tops.len()
    }

    pub fn k(&self) -> usize {
        self.k
    }

    /// `scores` is query-major: the score of query `q` for slot `ws.start() + i`
    /// is `scores[q * ws.len() + i]`.
    pub fn add_range(
        &mut self,
        src: &dyn DocSource,
        segment: SegmentId,
        ws: Workspace,
        scores: &[f32],
    ) -> Result<(), TopKError> {
        check_slot_range(src, segment, ws.range())?;
        let width = ws.len() as usize;
        let expected = self
            .tops
            .len()
            .checked_mul(width)
            .ok_or(TopKError::ScoreLen {
                expected: usize::MAX,
                got: scores.len(),
            })?;
        if scores.len() != expected {
            return Err(TopKError::ScoreLen {
                expected,
                got: scores.len(),
            });
        }
        if width == 0 || self.k == 0 {
            return Ok(());
        }

        let offsets: Vec<Option<u64>> = (0..ws.len())
            .map(|i| src.slot_offset(segment, Slot(ws.start() + i)))
            .collect();
        for (top, row) in self.tops.iter_mut().zip(scores.chunks_exact(width)) {
            let mut local = TopK::new(self.k);
            for (offset, &score) in offsets.iter().zip(row) {
                if let Some(offset) = *offset {
                    local.push(offset, score);
                }
            }
            top.merge(local);
        }
        Ok(())
    }

    /// Both sides must have the same `k` and the same number of queries.
    pub fn merge(&mut self, other: RangeScorer) -> Result<(), TopKError> {
        if self.k != other.k {
            return Err(TopKError::Mismatch { what: "k" });
        }
        if self.tops.len() != other.tops.len() {
            return Err(TopKError::Mismatch {
                what: "query count",
            });
        }
        for (mine, theirs) in self.tops.iter_mut().zip(other.tops) {
            mine.merge(theirs);
        }
        Ok(())
    }

    /// Sorted hits per query, indexed by query index.
    pub fn into_results(self) -> Vec<Vec<Hit>> {
        self.tops.into_iter().map(TopK::into_sorted_vec).collect()
    }
}

#[cfg(test)]
#[path = "test/topk.rs"]
mod tests;
