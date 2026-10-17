//! Mock/test-double conventions (Q3).
//!
//! Narrow traits for GPU/measurement interfaces use generics (static dispatch) in the hot
//! path; `dyn` only at the edges. Deterministic fakes (fixed output, simulated failure) are
//! hand-written rather than `mockall`-generated, since that composes more simply across
//! `async`/`Send` boundaries; `mockall::automock` is reserved for orchestrator-level traits.
//! Everything here is gated so it never ships in a production build without the `mock` feature.
#![cfg(any(test, feature = "mock"))]
/// Example of a narrow trait a hot-path component depends on generically.
pub trait Clock: Send + Sync {
    fn now_millis(&self) -> u64;
}
/// Hand-written deterministic fake, per the convention above.
pub struct FakeClock {
    pub fixed_millis: u64,
}
impl Clock for FakeClock {
    fn now_millis(&self) -> u64 {
        self.fixed_millis
    }
}
/// Backend abstraction the orchestration layer runs against; `cuda` selects the real
/// CUDA implementation elsewhere, this crate only needs a CPU/mock impl to stay testable.
pub trait ComputeBackend {
    fn run_batch(&self, input: &[f32]) -> Vec<f32>;
}
/// Identity backend: sufficient for orchestration tests that do not care about numerics.
pub struct MockBackend;
impl ComputeBackend for MockBackend {
    fn run_batch(&self, input: &[f32]) -> Vec<f32> {
        input.to_vec()
    }
}
/// Kernel-level trait (batch lifecycle + results), abstracted so orchestration/sync logic
/// is testable without a GPU. Concrete kernel implementations define their own result type.
pub trait Kernel {
    type Hit;
    fn batch_start(&mut self) -> u64;
    fn batch_end(&mut self, batch_id: u64);
    fn results(&self, batch_id: u64) -> Vec<Self::Hit>;
    fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)>;
    fn progress(&self, batch_id: u64) -> usize;
}
/// Scriptable kernel fake: results/ranges/progress for a batch are programmed up front
/// rather than computed, so sync-point tests control exactly what the kernel "found".
#[derive(Default)]
pub struct FakeKernel<H> {
    next_batch_id: u64,
    scripted: std::collections::HashMap<u64, (Vec<H>, Vec<(usize, usize)>, usize)>,
}
impl<H> FakeKernel<H> {
    pub fn new() -> Self {
        Self {
            next_batch_id: 0,
            scripted: std::collections::HashMap::new(),
        }
    }
    /// Programs the outcome of the next-started batch before it runs.
    pub fn script(
        &mut self,
        batch_id: u64,
        hits: Vec<H>,
        ranges: Vec<(usize, usize)>,
        progress: usize,
    ) {
        self.scripted.insert(batch_id, (hits, ranges, progress));
    }
}
impl<H: Clone> Kernel for FakeKernel<H> {
    type Hit = H;
    fn batch_start(&mut self) -> u64 {
        let id = self.next_batch_id;
        self.next_batch_id += 1;
        id
    }
    fn batch_end(&mut self, _batch_id: u64) {}
    fn results(&self, batch_id: u64) -> Vec<H> {
        self.scripted.get(&batch_id).map(|(hits, _, _)| hits.clone()).unwrap_or_default()
    }
    fn written_ranges(&self, batch_id: u64) -> Vec<(usize, usize)> {
        self.scripted
            .get(&batch_id)
            .map(|(_, ranges, _)| ranges.clone())
            .unwrap_or_default()
    }
    fn progress(&self, batch_id: u64) -> usize {
        self.scripted.get(&batch_id).map(|(_, _, p)| *p).unwrap_or(0)
    }
}
/// Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
/// hardware probing with seeded, reproducible noise.
pub trait Measure {
    fn sample(&mut self) -> f64;
}
/// Noise models covering the calibration-stability test matrix (F2-F5).
pub enum NoiseModel {
    Constant,
    Outlier { every_n: u32, magnitude: f64 },
    Drift { per_sample: f64 },
}
/// Seeded-RNG measurement fake: `base` plus a `NoiseModel`-shaped perturbation, so
/// stability/convergence tests are deterministic across runs for a fixed seed.
pub struct FakeMeasure {
    base: f64,
    noise: NoiseModel,
    rng: rand::rngs::StdRng,
    sample_count: u32,
    drift_accum: f64,
}
impl FakeMeasure {
    pub fn new(base: f64, noise: NoiseModel, seed: u64) -> Self {
        use rand::SeedableRng;
        Self {
            base,
            noise,
            rng: rand::rngs::StdRng::seed_from_u64(seed),
            sample_count: 0,
            drift_accum: 0.0,
        }
    }
}
impl Measure for FakeMeasure {
    fn sample(&mut self) -> f64 {
        use rand::Rng;
        self.sample_count += 1;
        match &self.noise {
            NoiseModel::Constant => self.base + self.rng.gen_range(-0.01..0.01),
            NoiseModel::Outlier { every_n, magnitude } => {
                if self.sample_count % every_n == 0 {
                    self.base + magnitude
                } else {
                    self.base + self.rng.gen_range(-0.01..0.01)
                }
            }
            NoiseModel::Drift { per_sample } => {
                self.drift_accum += per_sample;
                self.base + self.drift_accum + self.rng.gen_range(-0.01..0.01)
            }
        }
    }
}
/// `std::sync` vs. `loom::sync` shim for CAS-protocol modules (B1-style queues): swap the
/// import under `cfg(loom)` so the same production code runs exhaustively under loom without
/// a second implementation. Run with `RUSTFLAGS="--cfg loom" cargo test --test <loom_test>`.
#[cfg(loom)]
pub mod sync {
    pub use loom::sync::{atomic, Arc};
}
#[cfg(not(loom))]
pub mod sync {
    pub use std::sync::{atomic, Arc};
}
