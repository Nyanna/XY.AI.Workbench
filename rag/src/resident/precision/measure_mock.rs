//! Scripted fake of the measurement: seeded noise per combination, no device needed.
#![cfg(any(test, feature = "mock"))]

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::resident::error::PrecisionError;
use crate::resident::mock::{self, Measure as _, NoiseModel};

use super::measure::{Cfg, Clock, DeviationReport, Measure, Sample, Validity};

/// Clock that only moves when told to; the fake advances it by every sampled time.
#[derive(Default)]
pub struct ManualClock {
    nanos: AtomicU64,
}

impl ManualClock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn advance(&self, by: Duration) {
        self.nanos.fetch_add(by.as_nanos() as u64, Ordering::Relaxed);
    }
}

impl Clock for ManualClock {
    fn elapsed(&self) -> Duration {
        Duration::from_nanos(self.nanos.load(Ordering::Relaxed))
    }
}

/// Behaviour of one combination. Times are milliseconds; the noise models perturb them by
/// absolute amounts of the same unit.
#[derive(Clone, Debug)]
pub struct FakeCombo {
    pub time_ms: f64,
    pub noise: NoiseModel,
    pub deviation: f64,
    pub valid: Validity,
    pub incompatible: bool,
}

impl FakeCombo {
    pub fn new(time_ms: f64, deviation: f64) -> Self {
        Self {
            time_ms,
            noise: NoiseModel::Constant,
            deviation,
            valid: Validity::Ok,
            incompatible: false,
        }
    }

    pub fn noise(mut self, noise: NoiseModel) -> Self {
        self.noise = noise;
        self
    }

    pub fn valid(mut self, valid: Validity) -> Self {
        self.valid = valid;
        self
    }

    pub fn incompatible(mut self) -> Self {
        self.incompatible = true;
        self
    }
}

pub struct FakeMeasure {
    script: Box<dyn Fn(&Cfg) -> FakeCombo + Send>,
    seed: u64,
    sources: HashMap<Cfg, mock::FakeMeasure>,
    clock: Option<Arc<ManualClock>>,
    load: Option<(usize, Box<dyn Fn(usize) -> f64 + Send>)>,
    calls: usize,
    counts: HashMap<Cfg, u32>,
    order: Vec<Cfg>,
}

impl FakeMeasure {
    pub fn new(seed: u64, script: impl Fn(&Cfg) -> FakeCombo + Send + 'static) -> Self {
        Self {
            script: Box::new(script),
            seed,
            sources: HashMap::new(),
            clock: None,
            load: None,
            calls: 0,
            counts: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn with_clock(mut self, clock: Arc<ManualClock>) -> Self {
        self.clock = Some(clock);
        self
    }

    /// Multiplies every sample by `load(call_index / per_round)`: a disturbance shared by all
    /// combinations of a round, as real background load would be.
    pub fn with_shared_load(
        mut self,
        per_round: usize,
        load: impl Fn(usize) -> f64 + Send + 'static,
    ) -> Self {
        self.load = Some((per_round.max(1), Box::new(load)));
        self
    }

    pub fn samples_taken(&self, cfg: &Cfg) -> u32 {
        self.counts.get(cfg).copied().unwrap_or(0)
    }

    /// Every sampled combination in call order.
    pub fn order(&self) -> &[Cfg] {
        &self.order
    }
}

impl Measure for FakeMeasure {
    fn deviation(&mut self, cfg: &Cfg) -> Result<DeviationReport, PrecisionError> {
        let combo = (self.script)(cfg);
        if combo.incompatible {
            return Err(PrecisionError::Incompatible { reason: "scripted".into() });
        }
        Ok(DeviationReport { deviation: combo.deviation, valid: combo.valid })
    }

    fn sample(&mut self, cfg: &Cfg) -> Result<Sample, PrecisionError> {
        let combo = (self.script)(cfg);
        let seed = {
            let mut h = DefaultHasher::new();
            cfg.hash(&mut h);
            self.seed ^ h.finish()
        };
        let source = self
            .sources
            .entry(cfg.clone())
            .or_insert_with(|| mock::FakeMeasure::new(combo.time_ms, combo.noise.clone(), seed));
        let mut ms = source.sample().max(1e-6);
        if let Some((per_round, load)) = &self.load {
            ms *= load(self.calls / per_round);
        }
        self.calls += 1;
        *self.counts.entry(cfg.clone()).or_default() += 1;
        self.order.push(cfg.clone());
        let time = Duration::from_secs_f64(ms / 1000.0);
        if let Some(clock) = &self.clock {
            clock.advance(time);
        }
        Ok(Sample::Time(time))
    }
}
