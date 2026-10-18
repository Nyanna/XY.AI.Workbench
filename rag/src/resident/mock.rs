//! Mock/test-double conventions.
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
use crate::resident::backend::ComputeBackend;
/// Identity backend: sufficient for orchestration tests that do not care about numerics.
pub struct MockBackend;
impl ComputeBackend for MockBackend {
    fn run_batch(&self, input: &[f32]) -> Vec<f32> {
        input.to_vec()
    }
}
/// Measurement trait calibration logic depends on; `FakeMeasure` stands in for real
/// hardware probing with seeded, reproducible noise.
pub trait Measure {
    fn sample(&mut self) -> f64;
}
/// Noise models for calibration-stability tests.
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
/// `std::sync` vs. `loom::sync` shim for CAS-protocol modules: swap the
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
