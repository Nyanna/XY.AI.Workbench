//! Kernel abstraction: segment descriptors, hits and the `Kernel` trait.
//!
//! The kernel knows only segments, offsets and spans. Mapping an address back to an entry is the
//! manager's job (`entry_of`, `entry_of_fixed`); the public API carries no pointers.

pub mod cuda;

#[cfg(any(test, feature = "mock"))]
pub mod mock;

use std::time::Duration;

use crate::resident::error::IndexError;

/// Address-agnostic arena span; pointer resolution happens only inside a backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RegionSpan {
    pub offset: u64,
    pub len: u64,
}

/// Offset table (`n + 1` prefix sums) living in an arena span.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OffsetRef(pub RegionSpan);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SegmentLayout {
    Fixed { stride: u32 },
    Variable { offsets: OffsetRef },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentDesc {
    pub base: u64,
    pub count: u32,
    pub layout: SegmentLayout,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KernelHit {
    pub offset: u64,
    pub score: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrittenRange {
    pub offset: u64,
    pub len: u64,
}

#[derive(Clone, Debug, Default)]
pub struct BatchDesc {
    pub segments: Vec<SegmentDesc>,
    pub queries: u32,
    pub params: Option<RegionSpan>,
    pub time_budget: Option<Duration>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BatchId(u64);

impl BatchId {
    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum KernelOutput {
    Hits(Vec<KernelHit>),
    /// The granted range is exhausted; the manager grants the next one and resumes.
    Partial,
}

pub trait Kernel: Send + Sync {
    fn begin(&self, batch: &BatchDesc) -> Result<BatchId, IndexError>;
    fn run(&self, batch: BatchId, input: &[RegionSpan]) -> Result<KernelOutput, IndexError>;
    /// Monotonically non-decreasing.
    fn progress(&self, batch: BatchId) -> u64;
    fn end(&self, batch: BatchId) -> Result<Vec<WrittenRange>, IndexError>;
}

/// Entry index for `addr` in a `Fixed` segment.
pub fn entry_of_fixed(base: u64, stride: u32, count: u32, addr: u64) -> Option<usize> {
    if stride == 0 || addr < base {
        return None;
    }
    let idx = (addr - base) / u64::from(stride);
    (idx < u64::from(count)).then_some(idx as usize)
}

/// Entry index for `addr` given `n + 1` ascending offsets; entry `i` covers
/// `[offsets[i], offsets[i + 1])`.
pub fn entry_of(offsets: &[u64], addr: u64) -> Option<usize> {
    let i = offsets.partition_point(|&o| o <= addr);
    (i > 0 && i < offsets.len()).then(|| i - 1)
}

#[cfg(test)]
mod tests {
    use super::mock::{Call, MockKernel};
    use super::*;

    fn spans(n: u64) -> Vec<RegionSpan> {
        (0..n).map(|i| RegionSpan { offset: i * 64, len: 64 }).collect()
    }

    fn desc() -> BatchDesc {
        BatchDesc { queries: 2, ..Default::default() }
    }

    #[test]
    fn entry_of_fixed_bounds() {
        assert_eq!(entry_of_fixed(100, 10, 4, 99), None);
        assert_eq!(entry_of_fixed(100, 10, 4, 100), Some(0));
        assert_eq!(entry_of_fixed(100, 10, 4, 119), Some(1));
        assert_eq!(entry_of_fixed(100, 10, 4, 139), Some(3));
        assert_eq!(entry_of_fixed(100, 10, 4, 140), None);
        assert_eq!(entry_of_fixed(100, 0, 4, 100), None);
    }

    #[test]
    fn entry_of_variable_bounds() {
        let offsets = [10, 15, 15, 40];
        assert_eq!(entry_of(&offsets, 9), None);
        assert_eq!(entry_of(&offsets, 10), Some(0));
        assert_eq!(entry_of(&offsets, 14), Some(0));
        assert_eq!(entry_of(&offsets, 15), Some(2));
        assert_eq!(entry_of(&offsets, 39), Some(2));
        assert_eq!(entry_of(&offsets, 40), None);
        assert_eq!(entry_of(&[], 0), None);
    }

    #[test]
    fn scoring_is_deterministic() {
        let run = || {
            let k = MockKernel::scoring();
            let b = k.begin(&desc()).unwrap();
            let out = k.run(b, &spans(5)).unwrap();
            (out, k.end(b).unwrap())
        };
        let (a, b) = (run(), run());
        assert_eq!(a, b);
        assert!(matches!(&a.0, KernelOutput::Hits(h) if h.len() == 5));
        assert!(a.1.is_empty());
    }

    #[test]
    fn partial_then_follow_up_range_covers_input() {
        let k = MockKernel::embedding(120);
        let b = k.begin(&desc()).unwrap();
        let input = [RegionSpan { offset: 0, len: 100 }, RegionSpan { offset: 100, len: 50 }];
        assert_eq!(k.run(b, &input).unwrap(), KernelOutput::Partial);

        k.grant(100);
        let rest = [RegionSpan { offset: 120, len: 30 }];
        assert!(matches!(k.run(b, &rest).unwrap(), KernelOutput::Hits(_)));

        let written = k.end(b).unwrap();
        assert_eq!(written.iter().map(|w| w.len).sum::<u64>(), 150);
    }

    #[test]
    fn progress_is_monotonic() {
        let k = MockKernel::scoring();
        let b = k.begin(&desc()).unwrap();
        let mut last = k.progress(b);
        for _ in 0..4 {
            k.run(b, &spans(3)).unwrap();
            let now = k.progress(b);
            assert!(now > last);
            last = now;
        }
    }

    #[test]
    fn injected_failure_leaves_batch_endable() {
        let k = MockKernel::scoring().fail_after(1);
        let b = k.begin(&desc()).unwrap();
        assert!(k.run(b, &spans(1)).is_ok());
        assert!(matches!(k.run(b, &spans(1)), Err(IndexError::KernelFailed { .. })));
        assert!(k.end(b).is_ok());
        assert_eq!(k.calls().last(), Some(&Call::End(b)));
        assert!(matches!(k.end(b), Err(IndexError::UnknownBatch { .. })));
    }

    #[test]
    fn usable_as_trait_object() {
        let k: std::sync::Arc<dyn Kernel> = std::sync::Arc::new(MockKernel::scoring());
        let b = k.begin(&desc()).unwrap();
        assert!(k.run(b, &spans(1)).is_ok());
    }
}
