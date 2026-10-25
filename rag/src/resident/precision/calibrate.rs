//! Calibrator: enumerates combinations of a capability set, measures them interleaved against
//! the fp32 reference, rejects invalid ones and keeps the Pareto front over (time, deviation).
//!
//! Scale: position 0 is the fastest, 1 the most precise combination. Calibration is best effort
//! and never blocks: caps end a measurement with the best intermediate state marked unstable.
use std::collections::{BTreeSet, HashSet};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, info};
use crate::resident::error::PrecisionError;
use crate::resident::subengine::SubengineKind;
use super::axis::{AxisId, ValueId};
use super::capability::CapabilitySet;
use super::known::{BATCH, THREADS, TILE};
use super::measure::{
    Cfg, Clock, Measure, MeasureResult, NeverYield, Sample, SystemClock, Validity,
    WindowTuning, YieldCheck,
};
use super::pareto::pareto_by;
use super::profile::ProfileConfig;
use super::stats::{estimate, spread, Estimator, Spread};
/// Ends a single combination's measurement, whichever limit hits first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HardCap {
    /// Counts every attempt, interrupted ones included.
    pub max_reps: u32,
    pub max_time: Option<Duration>,
}
#[derive(Clone, Debug)]
pub struct CalibratorConfig {
    /// Above this deviation a combination counts as incompatible and is not activated.
    pub max_deviation: f64,
    /// Raised to 3 if lower.
    pub min_reps: u32,
    pub window_ms: f64,
    /// Relative scatter below which a series counts as stable.
    pub cv_threshold: f64,
    pub spread: Spread,
    pub estimator: Estimator,
    pub hard_cap: HardCap,
    pub slice_budget_ms: f64,
    /// Budget of a whole calibration run; the rest is skipped when it is used up.
    pub total_budget: Option<Duration>,
    /// A combination is dominated only if a peer is faster by more than this share.
    pub dominance_margin: f64,
    /// Pause after an interrupted sample so a waiting production task gets the device.
    pub interrupt_backoff: Duration,
    /// Axes measured one at a time; only their winners are combined and verified.
    pub separable_axes: BTreeSet<AxisId>,
    /// Measures the full cartesian product instead (reference for the pruned result).
    pub exhaustive: bool,
    pub seq_lens: Vec<usize>,
}
impl CalibratorConfig {
    /// Lengths in steps of four up to and including `max`.
    pub fn seq_lens_up_to(max: usize) -> Vec<usize> {
        let mut lens = Vec::new();
        let mut n = 16usize;
        while n < max {
            lens.push(n);
            n = n.saturating_mul(4);
        }
        lens.push(max);
        lens
    }
    pub fn tuning(&self) -> WindowTuning {
        WindowTuning {
            window_ms: self.window_ms,
            slice_budget_ms: self.slice_budget_ms,
        }
    }
}
impl Default for CalibratorConfig {
    fn default() -> Self {
        Self {
            max_deviation: 5e-2,
            min_reps: 3,
            window_ms: 10.0,
            cv_threshold: 0.05,
            spread: Spread::Cv,
            estimator: Estimator::Median,
            hard_cap: HardCap {
                max_reps: 40,
                max_time: Some(Duration::from_secs(30)),
            },
            slice_budget_ms: 5.0,
            total_budget: Some(Duration::from_secs(600)),
            dominance_margin: 0.1,
            interrupt_backoff: Duration::from_millis(5),
            separable_axes: [&*BATCH, &*TILE, &*THREADS].into_iter().cloned().collect(),
            exhaustive: false,
            seq_lens: Self::seq_lens_up_to(512),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalibrationTarget {
    pub engine: SubengineKind,
    pub model_id: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// One axis varied, the others at their base value.
    Axis,
    /// Combined winners (or the full product), measured together.
    Verify,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    /// `result.stable == false` marks an unstable, best-effort measurement.
    Measured(MeasureResult),
    Rejected(Validity),
    /// Stopped early: slower and not more precise than a peer.
    Dominated,
    /// Not measured: budget used up or the device never became available.
    Skipped,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub stage: Stage,
    pub cfg: Cfg,
    pub outcome: Outcome,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ParetoPoint {
    pub cfg: Cfg,
    pub time: Duration,
    pub deviation: f64,
    pub stable: bool,
}
#[derive(Clone, Debug)]
pub struct CalibrationReport {
    pub target: CalibrationTarget,
    pub entries: Vec<Entry>,
    /// Ascending time, strictly decreasing deviation.
    pub front: Vec<ParetoPoint>,
    pub budget_exhausted: bool,
    /// Size of the full cartesian product that pruning avoided measuring.
    pub exhaustive_size: usize,
    pub elapsed: Duration,
}
impl CalibrationReport {
    fn count(&self, f: impl Fn(&Outcome) -> bool) -> usize {
        self.entries.iter().filter(|e| f(&e.outcome)).count()
    }
    pub fn measured(&self) -> usize {
        self.count(|o| matches!(o, Outcome::Measured(_)))
    }
    pub fn unstable(&self) -> usize {
        self.count(|o| matches!(o, Outcome::Measured(r) if ! r.stable))
    }
    pub fn rejected(&self) -> usize {
        self.count(|o| matches!(o, Outcome::Rejected(_)))
    }
    pub fn dominated(&self) -> usize {
        self.count(|o| matches!(o, Outcome::Dominated))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Active,
    Done { stable: bool },
    Dominated,
    Rejected(Validity),
}
struct Track {
    cfg: Cfg,
    deviation: f64,
    samples: Vec<f64>,
    /// Sample over the group anchor's sample of the same round.
    ratios: Vec<f64>,
    attempts: u32,
    started: Option<Duration>,
    state: State,
}
impl Track {
    fn active(&self) -> bool {
        self.state == State::Active
    }
}
pub struct Calibrator<M: Measure> {
    cfg: CalibratorConfig,
    measure: M,
    clock: Arc<dyn Clock>,
    yield_check: Arc<dyn YieldCheck>,
    run_started: Duration,
}
type Axes = [(AxisId, Vec<ValueId>)];
fn ordered(values: &BTreeSet<ValueId>) -> Vec<ValueId> {
    let mut v: Vec<ValueId> = values.iter().cloned().collect();
    v.sort_by_key(|x| match x.as_str().parse::<u64>() {
        Ok(n) => (0, n, String::new()),
        Err(_) => (1, 0, x.as_str().to_owned()),
    });
    v
}
fn cartesian_from(start: &ProfileConfig, axes: &Axes) -> Vec<ProfileConfig> {
    let mut acc = vec![start.clone()];
    for (axis, values) in axes {
        acc = acc
            .iter()
            .flat_map(|c| values.iter().map(move |v| c.clone().with(axis, v)))
            .collect();
    }
    acc
}
fn product_size(axes: &[&Axes]) -> usize {
    axes.iter()
        .flat_map(|a| a.iter())
        .fold(1usize, |n, (_, v)| n.saturating_mul(v.len()))
}
impl<M: Measure> Calibrator<M> {
    pub fn new(mut cfg: CalibratorConfig, measure: M) -> Self {
        cfg.min_reps = cfg.min_reps.max(3);
        Self {
            cfg,
            measure,
            clock: Arc::new(SystemClock::new()),
            yield_check: Arc::new(NeverYield),
            run_started: Duration::ZERO,
        }
    }
    pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.clock = clock;
        self
    }
    pub fn with_yield_check(mut self, yield_check: Arc<dyn YieldCheck>) -> Self {
        self.yield_check = yield_check;
        self
    }
    pub fn config(&self) -> &CalibratorConfig {
        &self.cfg
    }
    pub fn measure(&self) -> &M {
        &self.measure
    }
    pub fn run(
        &mut self,
        target: &CalibrationTarget,
        caps: &CapabilitySet,
    ) -> Result<CalibrationReport, PrecisionError> {
        let span = tracing::info_span!(
            "calibrate", engine = ? target.engine, model_id = % target.model_id
        );
        let _entered = span.enter();
        self.run_started = self.clock.elapsed();
        let (core, seps) = self.split_axes(caps)?;
        let mut report = CalibrationReport {
            target: target.clone(),
            entries: Vec::new(),
            front: Vec::new(),
            budget_exhausted: false,
            exhaustive_size: product_size(&[core.as_slice(), seps.as_slice()]),
            elapsed: Duration::ZERO,
        };
        let candidates = if self.cfg.exhaustive {
            let all = [core.as_slice(), seps.as_slice()].concat();
            self.cfgs(cartesian_from(&ProfileConfig::new(), &all))
        } else {
            self.axis_stage(&core, &seps, &mut report)?
        };
        let verified = self.measure_group(candidates, true)?;
        self.record(&mut report, Stage::Verify, verified);
        let points: Vec<ParetoPoint> = report
            .entries
            .iter()
            .filter(|e| e.stage == Stage::Verify)
            .filter_map(|e| match &e.outcome {
                Outcome::Measured(r) => {
                    Some(ParetoPoint {
                        cfg: e.cfg.clone(),
                        time: r.time,
                        deviation: r.deviation,
                        stable: r.stable,
                    })
                }
                _ => None,
            })
            .collect();
        report.front = pareto_by(points, |p| (p.time.as_secs_f64(), p.deviation));
        report.budget_exhausted |= self.over_budget();
        report.elapsed = self.clock.elapsed().saturating_sub(self.run_started);
        info!(
            front = report.front.len(), measured = report.measured(), unstable = report
            .unstable(), rejected = report.rejected(), dominated = report.dominated(),
            exhaustive_size = report.exhaustive_size, budget_exhausted = report
            .budget_exhausted, "calibration finished"
        );
        Ok(report)
    }
    fn split_axes(
        &self,
        caps: &CapabilitySet,
    ) -> Result<
        (Vec<(AxisId, Vec<ValueId>)>, Vec<(AxisId, Vec<ValueId>)>),
        PrecisionError,
    > {
        let (mut core, mut seps) = (Vec::new(), Vec::new());
        for (axis, values) in caps.iter().filter(|(_, v)| !v.is_empty()) {
            let entry = (axis.clone(), ordered(values));
            if self.cfg.separable_axes.contains(axis) {
                seps.push(entry);
            } else {
                core.push(entry);
            }
        }
        if core.is_empty() && seps.is_empty() {
            return Err(PrecisionError::Capability {
                reason: "empty capability set".into(),
            });
        }
        Ok((core, seps))
    }
    fn cfgs(&self, combos: Vec<ProfileConfig>) -> Vec<Cfg> {
        combos.into_iter().map(|c| Cfg::new(c, self.cfg.seq_lens.clone())).collect()
    }
    /// Measures each separable axis alone per core combination and returns the cartesian
    /// product of the per-axis winners.
    fn axis_stage(
        &mut self,
        core: &Axes,
        seps: &Axes,
        report: &mut CalibrationReport,
    ) -> Result<Vec<Cfg>, PrecisionError> {
        let mut candidates: Vec<ProfileConfig> = Vec::new();
        for core_combo in cartesian_from(&ProfileConfig::new(), core) {
            if self.over_budget() {
                report.budget_exhausted = true;
                break;
            }
            let base = seps
                .iter()
                .fold(core_combo.clone(), |c, (axis, values)| c.with(axis, &values[0]));
            let mut winners: Vec<(AxisId, Vec<ValueId>)> = Vec::new();
            for (axis, values) in seps {
                if values.len() == 1 {
                    winners.push((axis.clone(), values.clone()));
                    continue;
                }
                let group = self
                    .cfgs(values.iter().map(|v| base.clone().with(axis, v)).collect());
                let results = self.measure_group(group, true)?;
                let won = self.winners(axis, &results);
                self.record(report, Stage::Axis, results);
                winners.push((axis.clone(), won));
            }
            if winners.iter().any(|(_, v)| v.is_empty()) {
                continue;
            }
            candidates.extend(cartesian_from(&core_combo, &winners));
        }
        let mut seen = HashSet::new();
        candidates.retain(|c| seen.insert(c.clone()));
        Ok(self.cfgs(candidates))
    }
    fn winners(&self, axis: &AxisId, results: &[(Cfg, Outcome)]) -> Vec<ValueId> {
        let points: Vec<(ValueId, f64, f64)> = results
            .iter()
            .filter_map(|(cfg, outcome)| match (outcome, cfg.combo.get(axis)) {
                (Outcome::Measured(r), Some(v)) => {
                    Some((v.clone(), r.time.as_secs_f64(), r.deviation))
                }
                _ => None,
            })
            .collect();
        pareto_by(points, |p| (p.1, p.2)).into_iter().map(|p| p.0).collect()
    }
    fn record(
        &self,
        report: &mut CalibrationReport,
        stage: Stage,
        results: Vec<(Cfg, Outcome)>,
    ) {
        for (cfg, outcome) in results {
            if stage == Stage::Axis
                && report.entries.iter().any(|e| e.stage == stage && e.cfg == cfg)
            {
                continue;
            }
            match &outcome {
                Outcome::Measured(r) => {
                    debug!(
                        % cfg, time_us = r.time.as_micros() as u64, deviation = r
                        .deviation, stable = r.stable, samples = r.samples,
                        "combination measured"
                    )
                }
                Outcome::Rejected(v) => {
                    info!(% cfg, reason = v.as_str(), "combination rejected")
                }
                Outcome::Dominated => {
                    debug!(% cfg, "combination dominated, stopped early")
                }
                Outcome::Skipped => info!(% cfg, "combination skipped"),
            }
            if let Outcome::Measured(r) = &outcome {
                if !r.stable {
                    info!(
                        % cfg, error = % PrecisionError::CalibrationUnstable { reason :
                        "cap reached, best intermediate state used".into(), },
                        "combination unstable"
                    );
                }
            }
            report.entries.push(Entry { stage, cfg, outcome });
        }
    }
    fn over_budget(&self) -> bool {
        self.cfg
            .total_budget
            .is_some_and(|b| self.clock.elapsed().saturating_sub(self.run_started) >= b)
    }
    fn capped(&self, t: &Track, now: Duration) -> bool {
        let cap = self.cfg.hard_cap;
        t.attempts >= cap.max_reps
            || cap
                .max_time
                .is_some_and(|m| {
                    t.started.is_some_and(|s| now.saturating_sub(s) >= m)
                })
    }
    /// Deviation phase: cheap, and decides rejection before any time is spent.
    fn judge(&mut self, cfg: &Cfg) -> Result<(f64, State), PrecisionError> {
        let (deviation, validity) = match self.measure.deviation(cfg) {
            Ok(r) if r.valid.is_ok() && r.deviation.is_nan() => {
                (f64::NAN, Validity::NonFinite)
            }
            Ok(r) if r.valid.is_ok() && r.deviation > self.cfg.max_deviation => {
                (r.deviation, Validity::OverMaxDeviation)
            }
            Ok(r) => (r.deviation, r.valid),
            Err(PrecisionError::Incompatible { reason }) => {
                debug!(% cfg, % reason, "combination incompatible");
                (f64::INFINITY, Validity::Failed)
            }
            Err(e) => return Err(e),
        };
        let state = if validity.is_ok() {
            State::Active
        } else {
            State::Rejected(validity)
        };
        Ok((deviation, state))
    }
    fn take_sample(&mut self, cfg: &Cfg) -> Result<Option<f64>, PrecisionError> {
        let sample = if self.yield_check.should_yield() {
            Sample::Interrupted
        } else {
            self.measure.sample(cfg)?
        };
        match sample {
            Sample::Time(d) => Ok(Some(d.as_secs_f64())),
            Sample::Interrupted => {
                if !self.cfg.interrupt_backoff.is_zero() {
                    std::thread::sleep(self.cfg.interrupt_backoff);
                }
                Ok(None)
            }
        }
    }
    /// Measures `cfgs` round-robin so load fluctuations hit all of them alike. Odd rounds run in
    /// reverse order (ABBA). Stability is judged on the time ratio to the first combination,
    /// since only relations between combinations matter.
    fn measure_group(
        &mut self,
        cfgs: Vec<Cfg>,
        prune: bool,
    ) -> Result<Vec<(Cfg, Outcome)>, PrecisionError> {
        let mut tracks = Vec::with_capacity(cfgs.len());
        for cfg in cfgs {
            let (deviation, state) = self.judge(&cfg)?;
            tracks
                .push(Track {
                    cfg,
                    deviation,
                    samples: Vec::new(),
                    ratios: Vec::new(),
                    attempts: 0,
                    started: None,
                    state,
                });
        }
        let anchor = tracks.iter().position(Track::active);
        let had_peers = tracks.iter().filter(|t| t.active()).count() > 1;
        let mut round = 0usize;
        while tracks.iter().any(Track::active) && !self.over_budget() {
            let mut order: Vec<usize> = (0..tracks.len())
                .filter(|&i| tracks[i].active())
                .collect();
            if round % 2 == 1 {
                order.reverse();
            }
            let mut taken: Vec<Option<f64>> = vec![None; tracks.len()];
            for i in order {
                let now = self.clock.elapsed();
                tracks[i].attempts += 1;
                tracks[i].started.get_or_insert(now);
                let cfg = tracks[i].cfg.clone();
                match self.take_sample(&cfg) {
                    Ok(Some(s)) => {
                        tracks[i].samples.push(s);
                        taken[i] = Some(s);
                    }
                    Ok(None) => {}
                    Err(PrecisionError::Incompatible { reason }) => {
                        debug!(% cfg, % reason, "combination failed while sampling");
                        tracks[i].state = State::Rejected(Validity::Failed);
                    }
                    Err(e) => return Err(e),
                }
            }
            if let Some(a) = anchor {
                if let Some(base) = taken[a].filter(|s| *s > 0.0) {
                    for (i, t) in tracks.iter_mut().enumerate().filter(|(i, _)| *i != a)
                    {
                        if let Some(s) = taken[i] {
                            t.ratios.push(s / base);
                        }
                    }
                }
            }
            self.settle(&mut tracks, anchor, had_peers, prune);
            round += 1;
        }
        Ok(self.finish(tracks))
    }
    fn stable_series(&self, series: &[f64]) -> bool {
        series.len() >= self.cfg.min_reps as usize
            && spread(series, self.cfg.spread) <= self.cfg.cv_threshold
    }
    fn dominated(&self, tracks: &[Track], i: usize) -> bool {
        let t = &tracks[i];
        if t.samples.len() < self.cfg.min_reps as usize {
            return false;
        }
        let mine = estimate(&t.samples, self.cfg.estimator);
        tracks
            .iter()
            .enumerate()
            .any(|(j, o)| {
                j != i && o.samples.len() >= self.cfg.min_reps as usize
                    && !matches!(o.state, State::Dominated | State::Rejected(_))
                    && o.deviation <= t.deviation
                    && estimate(&o.samples, self.cfg.estimator)
                        * (1.0 + self.cfg.dominance_margin) < mine
            })
    }
    fn settle(
        &self,
        tracks: &mut [Track],
        anchor: Option<usize>,
        had_peers: bool,
        prune: bool,
    ) {
        let now = self.clock.elapsed();
        for i in 0..tracks.len() {
            if !tracks[i].active() || Some(i) == anchor {
                continue;
            }
            let next = if prune && self.dominated(tracks, i) {
                State::Dominated
            } else if self.stable_series(&tracks[i].ratios) {
                State::Done { stable: true }
            } else if self.capped(&tracks[i], now) {
                State::Done { stable: false }
            } else {
                continue;
            };
            tracks[i].state = next;
        }
        let Some(a) = anchor.filter(|&a| tracks[a].active()) else { return };
        if tracks.iter().enumerate().any(|(j, t)| j != a && t.active()) {
            return;
        }
        let own_ok = self.stable_series(&tracks[a].samples);
        let next = if had_peers {
            let peers = tracks
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != a)
                .map(|(_, t)| t.state);
            let (mut any_stable, mut any_unstable) = (false, false);
            for s in peers {
                match s {
                    State::Done { stable: true } => any_stable = true,
                    State::Done { stable: false } => any_unstable = true,
                    _ => {}
                }
            }
            State::Done {
                stable: (any_stable && !any_unstable) || own_ok,
            }
        } else if own_ok {
            State::Done { stable: true }
        } else if self.capped(&tracks[a], now) {
            State::Done { stable: false }
        } else {
            return;
        };
        tracks[a].state = next;
    }
    fn finish(&self, tracks: Vec<Track>) -> Vec<(Cfg, Outcome)> {
        tracks
            .into_iter()
            .map(|t| {
                let stable = matches!(t.state, State::Done { stable : true });
                let outcome = match t.state {
                    State::Dominated => Outcome::Dominated,
                    State::Rejected(v) => Outcome::Rejected(v),
                    State::Active | State::Done { .. } if t.samples.is_empty() => {
                        Outcome::Skipped
                    }
                    State::Active | State::Done { .. } => {
                        Outcome::Measured(MeasureResult {
                            time: Duration::from_secs_f64(
                                estimate(&t.samples, self.cfg.estimator).max(0.0),
                            ),
                            deviation: t.deviation,
                            valid: Validity::Ok,
                            stable,
                            samples: t.samples.len() as u32,
                        })
                    }
                };
                (t.cfg, outcome)
            })
            .collect()
    }
}
#[cfg(test)]
#[path = "test/calibrate.rs"]
mod tests;
